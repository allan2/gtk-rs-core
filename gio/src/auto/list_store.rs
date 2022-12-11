// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ListModel;
use glib::{prelude::*, translate::*};
use std::fmt;
#[cfg(any(feature = "v2_64", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
use std::mem;

glib::wrapper! {
    #[doc(alias = "GListStore")]
    pub struct ListStore(Object<ffi::GListStore, ffi::GListStoreClass>) @implements ListModel;

    match fn {
        type_ => || ffi::g_list_store_get_type(),
    }
}

impl ListStore {
    #[doc(alias = "g_list_store_new")]
    pub fn new(item_type: glib::types::Type) -> ListStore {
        unsafe { from_glib_full(ffi::g_list_store_new(item_type.into_glib())) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ListStore`] objects.
    ///
    /// This method returns an instance of [`ListStoreBuilder`](crate::builders::ListStoreBuilder) which can be used to create [`ListStore`] objects.
    pub fn builder() -> ListStoreBuilder {
        ListStoreBuilder::default()
    }

    #[doc(alias = "g_list_store_append")]
    pub fn append(&self, item: &impl IsA<glib::Object>) {
        unsafe {
            ffi::g_list_store_append(self.to_glib_none().0, item.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_64", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
    #[doc(alias = "g_list_store_find")]
    pub fn find(&self, item: &impl IsA<glib::Object>) -> Option<u32> {
        unsafe {
            let mut position = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::g_list_store_find(
                self.to_glib_none().0,
                item.as_ref().to_glib_none().0,
                position.as_mut_ptr(),
            ));
            if ret {
                Some(position.assume_init())
            } else {
                None
            }
        }
    }

    #[doc(alias = "g_list_store_insert")]
    pub fn insert(&self, position: u32, item: &impl IsA<glib::Object>) {
        unsafe {
            ffi::g_list_store_insert(
                self.to_glib_none().0,
                position,
                item.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_list_store_remove")]
    pub fn remove(&self, position: u32) {
        unsafe {
            ffi::g_list_store_remove(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "g_list_store_remove_all")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::g_list_store_remove_all(self.to_glib_none().0);
        }
    }
}

impl Default for ListStore {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ListStore`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ListStoreBuilder {
    item_type: Option<glib::types::Type>,
}

impl ListStoreBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ListStoreBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ListStore`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ListStore {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref item_type) = self.item_type {
            properties.push(("item-type", item_type));
        }
        glib::Object::new::<ListStore>(&properties)
    }

    pub fn item_type(mut self, item_type: glib::types::Type) -> Self {
        self.item_type = Some(item_type);
        self
    }
}

impl fmt::Display for ListStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ListStore")
    }
}
