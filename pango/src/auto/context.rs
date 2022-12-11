// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Direction, Font, FontDescription, FontFamily, FontMap, FontMetrics, Fontset, Gravity,
    GravityHint, Language, Matrix,
};
use glib::{prelude::*, translate::*};
use std::{fmt, mem, ptr};

glib::wrapper! {
    #[doc(alias = "PangoContext")]
    pub struct Context(Object<ffi::PangoContext, ffi::PangoContextClass>);

    match fn {
        type_ => || ffi::pango_context_get_type(),
    }
}

impl Context {
    #[doc(alias = "pango_context_new")]
    pub fn new() -> Context {
        unsafe { from_glib_full(ffi::pango_context_new()) }
    }

    #[doc(alias = "pango_context_changed")]
    pub fn changed(&self) {
        unsafe {
            ffi::pango_context_changed(self.to_glib_none().0);
        }
    }

    #[doc(alias = "pango_context_get_base_dir")]
    #[doc(alias = "get_base_dir")]
    pub fn base_dir(&self) -> Direction {
        unsafe { from_glib(ffi::pango_context_get_base_dir(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_context_get_base_gravity")]
    #[doc(alias = "get_base_gravity")]
    pub fn base_gravity(&self) -> Gravity {
        unsafe { from_glib(ffi::pango_context_get_base_gravity(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_context_get_font_description")]
    #[doc(alias = "get_font_description")]
    pub fn font_description(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_none(ffi::pango_context_get_font_description(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_context_get_font_map")]
    #[doc(alias = "get_font_map")]
    pub fn font_map(&self) -> Option<FontMap> {
        unsafe { from_glib_none(ffi::pango_context_get_font_map(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_context_get_gravity")]
    #[doc(alias = "get_gravity")]
    pub fn gravity(&self) -> Gravity {
        unsafe { from_glib(ffi::pango_context_get_gravity(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_context_get_gravity_hint")]
    #[doc(alias = "get_gravity_hint")]
    pub fn gravity_hint(&self) -> GravityHint {
        unsafe { from_glib(ffi::pango_context_get_gravity_hint(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_context_get_language")]
    #[doc(alias = "get_language")]
    pub fn language(&self) -> Option<Language> {
        unsafe { from_glib_none(ffi::pango_context_get_language(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_context_get_matrix")]
    #[doc(alias = "get_matrix")]
    pub fn matrix(&self) -> Option<Matrix> {
        unsafe { from_glib_none(ffi::pango_context_get_matrix(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_context_get_metrics")]
    #[doc(alias = "get_metrics")]
    pub fn metrics(
        &self,
        desc: Option<&FontDescription>,
        language: Option<&Language>,
    ) -> FontMetrics {
        unsafe {
            from_glib_full(ffi::pango_context_get_metrics(
                self.to_glib_none().0,
                desc.to_glib_none().0,
                mut_override(language.to_glib_none().0),
            ))
        }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_context_get_round_glyph_positions")]
    #[doc(alias = "get_round_glyph_positions")]
    pub fn is_round_glyph_positions(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_context_get_round_glyph_positions(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_context_get_serial")]
    #[doc(alias = "get_serial")]
    pub fn serial(&self) -> u32 {
        unsafe { ffi::pango_context_get_serial(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_context_list_families")]
    pub fn list_families(&self) -> Vec<FontFamily> {
        unsafe {
            let mut families = ptr::null_mut();
            let mut n_families = mem::MaybeUninit::uninit();
            ffi::pango_context_list_families(
                self.to_glib_none().0,
                &mut families,
                n_families.as_mut_ptr(),
            );
            FromGlibContainer::from_glib_container_num(families, n_families.assume_init() as _)
        }
    }

    #[doc(alias = "pango_context_load_font")]
    pub fn load_font(&self, desc: &FontDescription) -> Option<Font> {
        unsafe {
            from_glib_full(ffi::pango_context_load_font(
                self.to_glib_none().0,
                desc.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_context_load_fontset")]
    pub fn load_fontset(&self, desc: &FontDescription, language: &Language) -> Option<Fontset> {
        unsafe {
            from_glib_full(ffi::pango_context_load_fontset(
                self.to_glib_none().0,
                desc.to_glib_none().0,
                mut_override(language.to_glib_none().0),
            ))
        }
    }

    #[doc(alias = "pango_context_set_base_dir")]
    pub fn set_base_dir(&self, direction: Direction) {
        unsafe {
            ffi::pango_context_set_base_dir(self.to_glib_none().0, direction.into_glib());
        }
    }

    #[doc(alias = "pango_context_set_base_gravity")]
    pub fn set_base_gravity(&self, gravity: Gravity) {
        unsafe {
            ffi::pango_context_set_base_gravity(self.to_glib_none().0, gravity.into_glib());
        }
    }

    #[doc(alias = "pango_context_set_font_description")]
    pub fn set_font_description(&self, desc: Option<&FontDescription>) {
        unsafe {
            ffi::pango_context_set_font_description(self.to_glib_none().0, desc.to_glib_none().0);
        }
    }

    #[doc(alias = "pango_context_set_font_map")]
    pub fn set_font_map(&self, font_map: Option<&impl IsA<FontMap>>) {
        unsafe {
            ffi::pango_context_set_font_map(
                self.to_glib_none().0,
                font_map.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "pango_context_set_gravity_hint")]
    pub fn set_gravity_hint(&self, hint: GravityHint) {
        unsafe {
            ffi::pango_context_set_gravity_hint(self.to_glib_none().0, hint.into_glib());
        }
    }

    #[doc(alias = "pango_context_set_language")]
    pub fn set_language(&self, language: Option<&Language>) {
        unsafe {
            ffi::pango_context_set_language(
                self.to_glib_none().0,
                mut_override(language.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "pango_context_set_matrix")]
    pub fn set_matrix(&self, matrix: Option<&Matrix>) {
        unsafe {
            ffi::pango_context_set_matrix(self.to_glib_none().0, matrix.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_context_set_round_glyph_positions")]
    pub fn set_round_glyph_positions(&self, round_positions: bool) {
        unsafe {
            ffi::pango_context_set_round_glyph_positions(
                self.to_glib_none().0,
                round_positions.into_glib(),
            );
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Context")
    }
}
