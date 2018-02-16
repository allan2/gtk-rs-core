// This file was generated by gir (https://github.com/gtk-rs/gir @ ea993ed)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use ToggleButton;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct CheckButton(Object<ffi::GtkCheckButton, ffi::GtkCheckButtonClass>): ToggleButton, Button, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_check_button_get_type(),
    }
}

impl CheckButton {
    pub fn new() -> CheckButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_button_new()).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> CheckButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_button_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> CheckButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_button_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl Default for CheckButton {
    fn default() -> Self {
        Self::new()
    }
}
