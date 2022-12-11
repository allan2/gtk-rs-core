// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{AttrIterator, Attribute};
use glib::translate::*;
#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
use std::fmt;

glib::wrapper! {
    #[derive(Debug)]
    pub struct AttrList(Shared<ffi::PangoAttrList>);

    match fn {
        ref => |ptr| ffi::pango_attr_list_ref(ptr),
        unref => |ptr| ffi::pango_attr_list_unref(ptr),
        type_ => || ffi::pango_attr_list_get_type(),
    }
}

impl AttrList {
    #[doc(alias = "pango_attr_list_new")]
    pub fn new() -> AttrList {
        unsafe { from_glib_full(ffi::pango_attr_list_new()) }
    }

    #[doc(alias = "pango_attr_list_copy")]
    #[must_use]
    pub fn copy(&self) -> Option<AttrList> {
        unsafe { from_glib_full(ffi::pango_attr_list_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_attr_list_filter")]
    #[must_use]
    pub fn filter<P: FnMut(&Attribute) -> bool>(&self, func: P) -> Option<AttrList> {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Attribute) -> bool>(
            attribute: *mut ffi::PangoAttribute,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let attribute = from_glib_borrow(attribute);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&attribute);
            res.into_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            from_glib_full(ffi::pango_attr_list_filter(
                self.to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_list_get_attributes")]
    #[doc(alias = "get_attributes")]
    pub fn attributes(&self) -> Vec<Attribute> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::pango_attr_list_get_attributes(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_attr_list_get_iterator")]
    #[doc(alias = "get_iterator")]
    pub fn iterator(&self) -> AttrIterator {
        unsafe { from_glib_full(ffi::pango_attr_list_get_iterator(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_attr_list_splice")]
    pub fn splice(&self, other: &AttrList, pos: i32, len: i32) {
        unsafe {
            ffi::pango_attr_list_splice(self.to_glib_none().0, other.to_glib_none().0, pos, len);
        }
    }

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_attr_list_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::pango_attr_list_to_string(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_list_update")]
    pub fn update(&self, pos: i32, remove: i32, add: i32) {
        unsafe {
            ffi::pango_attr_list_update(self.to_glib_none().0, pos, remove, add);
        }
    }

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_attr_list_from_string")]
    pub fn from_string(text: &str) -> Result<AttrList, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::pango_attr_list_from_string(text.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Can't parse AttrList"))
        }
    }
}

impl Default for AttrList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
impl fmt::Display for AttrList {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}
