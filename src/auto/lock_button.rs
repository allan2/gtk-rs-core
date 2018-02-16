// This file was generated by gir (https://github.com/gtk-rs/gir @ ea993ed)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use Widget;
use ffi;
use gio;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct LockButton(Object<ffi::GtkLockButton, ffi::GtkLockButtonClass>): Button, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_lock_button_get_type(),
    }
}

impl LockButton {
    pub fn new<'a, P: IsA<gio::Permission> + 'a, Q: Into<Option<&'a P>>>(permission: Q) -> LockButton {
        assert_initialized_main_thread!();
        let permission = permission.into();
        let permission = permission.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_lock_button_new(permission.0)).downcast_unchecked()
        }
    }
}

pub trait LockButtonExt {
    fn get_permission(&self) -> Option<gio::Permission>;

    fn set_permission<'a, P: IsA<gio::Permission> + 'a, Q: Into<Option<&'a P>>>(&self, permission: Q);

    fn get_property_text_lock(&self) -> Option<String>;

    fn set_property_text_lock(&self, text_lock: Option<&str>);

    fn get_property_text_unlock(&self) -> Option<String>;

    fn set_property_text_unlock(&self, text_unlock: Option<&str>);

    fn get_property_tooltip_lock(&self) -> Option<String>;

    fn set_property_tooltip_lock(&self, tooltip_lock: Option<&str>);

    fn get_property_tooltip_not_authorized(&self) -> Option<String>;

    fn set_property_tooltip_not_authorized(&self, tooltip_not_authorized: Option<&str>);

    fn get_property_tooltip_unlock(&self) -> Option<String>;

    fn set_property_tooltip_unlock(&self, tooltip_unlock: Option<&str>);

    fn connect_property_permission_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_lock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_unlock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tooltip_lock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tooltip_not_authorized_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tooltip_unlock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<LockButton> + IsA<glib::object::Object>> LockButtonExt for O {
    fn get_permission(&self) -> Option<gio::Permission> {
        unsafe {
            from_glib_none(ffi::gtk_lock_button_get_permission(self.to_glib_none().0))
        }
    }

    fn set_permission<'a, P: IsA<gio::Permission> + 'a, Q: Into<Option<&'a P>>>(&self, permission: Q) {
        let permission = permission.into();
        let permission = permission.to_glib_none();
        unsafe {
            ffi::gtk_lock_button_set_permission(self.to_glib_none().0, permission.0);
        }
    }

    fn get_property_text_lock(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text-lock".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_text_lock(&self, text_lock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text-lock".to_glib_none().0, Value::from(text_lock).to_glib_none().0);
        }
    }

    fn get_property_text_unlock(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text-unlock".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_text_unlock(&self, text_unlock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text-unlock".to_glib_none().0, Value::from(text_unlock).to_glib_none().0);
        }
    }

    fn get_property_tooltip_lock(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "tooltip-lock".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_tooltip_lock(&self, tooltip_lock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "tooltip-lock".to_glib_none().0, Value::from(tooltip_lock).to_glib_none().0);
        }
    }

    fn get_property_tooltip_not_authorized(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "tooltip-not-authorized".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_tooltip_not_authorized(&self, tooltip_not_authorized: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "tooltip-not-authorized".to_glib_none().0, Value::from(tooltip_not_authorized).to_glib_none().0);
        }
    }

    fn get_property_tooltip_unlock(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "tooltip-unlock".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_tooltip_unlock(&self, tooltip_unlock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "tooltip-unlock".to_glib_none().0, Value::from(tooltip_unlock).to_glib_none().0);
        }
    }

    fn connect_property_permission_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::permission",
                transmute(notify_permission_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_lock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text-lock",
                transmute(notify_text_lock_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_unlock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text-unlock",
                transmute(notify_text_unlock_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tooltip_lock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tooltip-lock",
                transmute(notify_tooltip_lock_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tooltip_not_authorized_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tooltip-not-authorized",
                transmute(notify_tooltip_not_authorized_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tooltip_unlock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tooltip-unlock",
                transmute(notify_tooltip_unlock_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_permission_trampoline<P>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LockButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_lock_trampoline<P>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LockButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_unlock_trampoline<P>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LockButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tooltip_lock_trampoline<P>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LockButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tooltip_not_authorized_trampoline<P>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LockButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tooltip_unlock_trampoline<P>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LockButton::from_glib_borrow(this).downcast_unchecked())
}
