// Take a look at the license at the top of the repository in the LICENSE file.

use std::{marker, mem};

use super::{InitializingType, Signal};
use crate::{prelude::*, translate::*, Object, ParamSpec, Type, TypeFlags, TypeInfo};

// rustdoc-stripper-ignore-next
/// Trait for a type list of prerequisite object types.
pub trait PrerequisiteList {
    // rustdoc-stripper-ignore-next
    /// Returns the list of types for this list.
    fn types() -> Vec<Type>;
}

impl PrerequisiteList for () {
    fn types() -> Vec<Type> {
        vec![]
    }
}

impl<T: crate::ObjectType> PrerequisiteList for (T,) {
    fn types() -> Vec<Type> {
        vec![T::static_type()]
    }
}

// Generates all the PrerequisiteList impls for prerequisite_lists of arbitrary sizes based on a list of type
// parameters like A B C. It would generate the impl then for (A, B) and (A, B, C).
macro_rules! prerequisite_list_trait(
    ($name1:ident, $name2: ident, $($name:ident),*) => (
        prerequisite_list_trait!(__impl $name1, $name2; $($name),*);
    );
    (__impl $($name:ident),+; $name1:ident, $($name2:ident),*) => (
        prerequisite_list_trait_impl!($($name),+);
        prerequisite_list_trait!(__impl $($name),+ , $name1; $($name2),*);
    );
    (__impl $($name:ident),+; $name1:ident) => (
        prerequisite_list_trait_impl!($($name),+);
        prerequisite_list_trait_impl!($($name),+, $name1);
    );
);

// Generates the impl block for PrerequisiteList on prerequisite_lists or arbitrary sizes based on its
// arguments. Takes a list of type parameters as parameters, e.g. A B C
// and then implements the trait on (A, B, C).
macro_rules! prerequisite_list_trait_impl(
    ($($name:ident),+) => (
        impl<$($name: crate::ObjectType),+> PrerequisiteList for ( $($name),+ ) {
            fn types() -> Vec<Type> {
                vec![$($name::static_type()),+]
            }
        }
    );
);

prerequisite_list_trait!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);

/// Type methods required for an [`ObjectInterface`] implementation.
///
/// This is usually generated by the [`#[object_interface]`](crate::object_interface) attribute macro.
pub unsafe trait ObjectInterfaceType {
    /// Returns the `glib::Type` ID of the interface.
    ///
    /// This will register the type with the type system on the first call.
    #[doc(alias = "get_type")]
    fn type_() -> Type;
}

/// The central trait for defining a `GObject` interface.
///
/// Links together the type name, and the interface struct for type registration and allows hooking
/// into various steps of the type registration and initialization.
///
/// This must only be implemented on `#[repr(C)]` structs and have `gobject_ffi::GTypeInterface` as
/// the first field.
///
/// See [`register_interface`] for registering an implementation of this trait
/// with the type system.
///
/// [`register_interface`]: fn.register_interface.html
pub unsafe trait ObjectInterface: ObjectInterfaceType + Sized + 'static {
    /// `GObject` type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;

    /// Prerequisites for this interface.
    ///
    /// Any implementer of the interface must be a subclass of the prerequisites or implement them
    /// in case of interfaces.
    type Prerequisites: PrerequisiteList;

    /// Additional type initialization.
    ///
    /// This is called right after the type was registered and allows
    /// interfaces to do additional type-specific initialization.
    ///
    /// Optional
    fn type_init(_type_: &mut InitializingType<Self>) {}

    /// Interface initialization.
    ///
    /// This is called after `type_init` and before the first implementor
    /// of the interface is created. Interfaces can use this to do interface-
    /// specific initialization, e.g. for installing signals on the interface,
    /// and for setting default implementations of interface functions.
    ///
    /// Optional
    fn interface_init(&mut self) {}

    /// Properties installed for this interface.
    ///
    /// All implementors of the interface must provide these properties.
    fn properties() -> &'static [ParamSpec] {
        &[]
    }

    /// Signals installed for this interface.
    fn signals() -> &'static [Signal] {
        &[]
    }
}

pub trait ObjectInterfaceExt: ObjectInterface {
    /// Get interface from an instance.
    ///
    /// This will panic if `obj` does not implement the interface.
    #[inline]
    #[deprecated = "Use from_obj() instead"]
    fn from_instance<T: IsA<Object>>(obj: &T) -> &Self {
        Self::from_obj(obj)
    }

    /// Get interface from an instance.
    ///
    /// This will panic if `obj` does not implement the interface.
    #[inline]
    fn from_obj<T: IsA<Object>>(obj: &T) -> &Self {
        assert!(obj.as_ref().type_().is_a(Self::type_()));

        unsafe {
            let klass = (*(obj.as_ptr() as *const gobject_ffi::GTypeInstance)).g_class;
            let interface =
                gobject_ffi::g_type_interface_peek(klass as *mut _, Self::type_().into_glib());
            debug_assert!(!interface.is_null());
            &*(interface as *const Self)
        }
    }
}

impl<T: ObjectInterface> ObjectInterfaceExt for T {}

unsafe extern "C" fn interface_init<T: ObjectInterface>(
    klass: ffi::gpointer,
    _klass_data: ffi::gpointer,
) {
    let iface = &mut *(klass as *mut T);

    let pspecs = <T as ObjectInterface>::properties();
    for pspec in pspecs {
        gobject_ffi::g_object_interface_install_property(
            iface as *mut T as *mut _,
            pspec.to_glib_none().0,
        );
    }

    let type_ = T::type_();
    let signals = <T as ObjectInterface>::signals();
    for signal in signals {
        signal.register(type_);
    }

    iface.interface_init();
}

/// Register a `glib::Type` ID for `T`.
///
/// This must be called only once and will panic on a second call.
///
/// The [`object_interface!`] macro will create a `type_()` function around this, which will
/// ensure that it's only ever called once.
///
/// [`object_interface!`]: ../../macro.object_interface.html
pub fn register_interface<T: ObjectInterface>() -> Type {
    unsafe {
        use std::ffi::CString;

        let type_name = CString::new(T::NAME).unwrap();
        assert_eq!(
            gobject_ffi::g_type_from_name(type_name.as_ptr()),
            gobject_ffi::G_TYPE_INVALID
        );

        let type_ = gobject_ffi::g_type_register_static_simple(
            Type::INTERFACE.into_glib(),
            type_name.as_ptr(),
            mem::size_of::<T>() as u32,
            Some(interface_init::<T>),
            0,
            None,
            0,
        );

        let prerequisites = T::Prerequisites::types();
        for prerequisite in prerequisites {
            gobject_ffi::g_type_interface_add_prerequisite(type_, prerequisite.into_glib());
        }

        let type_ = Type::from_glib(type_);
        assert!(type_.is_valid());

        T::type_init(&mut InitializingType::<T>(type_, marker::PhantomData));

        type_
    }
}

/// Registers a `glib::Type` ID for `T` as a dynamic type.
///
/// An object interface must be explicitly registered as a dynamic type when
/// the system loads the implementation by calling [`TypePluginImpl::use_`] or
/// more specifically [`TypeModuleImpl::load`]. Therefore, unlike for object
/// interfaces registered as static types, object interfaces registered as
/// dynamic types can be registered several times.
///
/// The [`dynamic_object_interface!`] macro will create `register_interface()`
/// and `on_implementation_load()` functions around this, which will ensure
/// that the function is called when necessary.
///
/// [`dynamic_object_interface!`]: ../../../glib_macros/attr.dynamic_object_interface.html
/// [`TypePluginImpl::use_`]: ../type_plugin/trait.TypePluginImpl.html#method.use_
/// [`TypeModuleImpl::load`]: ../type_module/trait.TypeModuleImpl.html#method.load
pub fn register_dynamic_interface<P: DynamicObjectRegisterExt, T: ObjectInterface>(
    type_plugin: &P,
) -> Type {
    unsafe {
        use std::ffi::CString;

        let type_name = CString::new(T::NAME).unwrap();

        let already_registered =
            gobject_ffi::g_type_from_name(type_name.as_ptr()) != gobject_ffi::G_TYPE_INVALID;

        let type_info = TypeInfo(gobject_ffi::GTypeInfo {
            class_size: mem::size_of::<T>() as u16,
            class_init: Some(interface_init::<T>),
            ..TypeInfo::default().0
        });

        // registers the interface within the `type_plugin`
        let type_ = type_plugin.register_dynamic_type(
            Type::INTERFACE,
            type_name.to_str().unwrap(),
            &type_info,
            TypeFlags::ABSTRACT,
        );

        let prerequisites = T::Prerequisites::types();
        for prerequisite in prerequisites {
            // adding prerequisite interface can be done only once
            if !already_registered {
                gobject_ffi::g_type_interface_add_prerequisite(
                    type_.into_glib(),
                    prerequisite.into_glib(),
                );
            }
        }

        assert!(type_.is_valid());

        T::type_init(&mut InitializingType::<T>(type_, marker::PhantomData));

        type_
    }
}
