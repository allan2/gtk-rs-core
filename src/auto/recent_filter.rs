// This file was generated by gir (https://github.com/gtk-rs/gir @ ea993ed)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Buildable;
use RecentFilterFlags;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct RecentFilter(Object<ffi::GtkRecentFilter>): Buildable;

    match fn {
        get_type => || ffi::gtk_recent_filter_get_type(),
    }
}

impl RecentFilter {
    pub fn new() -> RecentFilter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_recent_filter_new())
        }
    }
}

impl Default for RecentFilter {
    fn default() -> Self {
        Self::new()
    }
}

pub trait RecentFilterExt {
    fn add_age(&self, days: i32);

    fn add_application(&self, application: &str);

    //fn add_custom<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, needed: RecentFilterFlags, func: /*Unknown conversion*//*Unimplemented*/RecentFilterFunc, data: P, data_destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn add_group(&self, group: &str);

    fn add_mime_type(&self, mime_type: &str);

    fn add_pattern(&self, pattern: &str);

    fn add_pixbuf_formats(&self);

    //fn filter(&self, filter_info: /*Ignored*/&RecentFilterInfo) -> bool;

    fn get_name(&self) -> Option<String>;

    fn get_needed(&self) -> RecentFilterFlags;

    fn set_name(&self, name: &str);
}

impl<O: IsA<RecentFilter>> RecentFilterExt for O {
    fn add_age(&self, days: i32) {
        unsafe {
            ffi::gtk_recent_filter_add_age(self.to_glib_none().0, days);
        }
    }

    fn add_application(&self, application: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_application(self.to_glib_none().0, application.to_glib_none().0);
        }
    }

    //fn add_custom<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, needed: RecentFilterFlags, func: /*Unknown conversion*//*Unimplemented*/RecentFilterFunc, data: P, data_destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_recent_filter_add_custom() }
    //}

    fn add_group(&self, group: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_group(self.to_glib_none().0, group.to_glib_none().0);
        }
    }

    fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_mime_type(self.to_glib_none().0, mime_type.to_glib_none().0);
        }
    }

    fn add_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    fn add_pixbuf_formats(&self) {
        unsafe {
            ffi::gtk_recent_filter_add_pixbuf_formats(self.to_glib_none().0);
        }
    }

    //fn filter(&self, filter_info: /*Ignored*/&RecentFilterInfo) -> bool {
    //    unsafe { TODO: call ffi::gtk_recent_filter_filter() }
    //}

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_filter_get_name(self.to_glib_none().0))
        }
    }

    fn get_needed(&self) -> RecentFilterFlags {
        unsafe {
            from_glib(ffi::gtk_recent_filter_get_needed(self.to_glib_none().0))
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            ffi::gtk_recent_filter_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }
}
