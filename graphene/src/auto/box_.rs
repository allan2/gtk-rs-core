// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Point3D, Sphere, Vec3};
use glib::translate::*;

glib::wrapper! {
    pub struct Box(BoxedInline<ffi::graphene_box_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_box_get_type(), ptr as *mut _) as *mut ffi::graphene_box_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_box_get_type(), ptr as *mut _),
        type_ => || ffi::graphene_box_get_type(),
    }
}

impl Box {
    #[doc(alias = "graphene_box_contains_box")]
    pub fn contains_box(&self, b: &Box) -> bool {
        unsafe { ffi::graphene_box_contains_box(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_box_contains_point")]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        unsafe { ffi::graphene_box_contains_point(self.to_glib_none().0, point.to_glib_none().0) }
    }

    #[doc(alias = "graphene_box_equal")]
    fn equal(&self, b: &Box) -> bool {
        unsafe { ffi::graphene_box_equal(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_box_expand")]
    #[must_use]
    pub fn expand(&self, point: &Point3D) -> Box {
        unsafe {
            let mut res = Box::uninitialized();
            ffi::graphene_box_expand(
                self.to_glib_none().0,
                point.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_box_expand_scalar")]
    #[must_use]
    pub fn expand_scalar(&self, scalar: f32) -> Box {
        unsafe {
            let mut res = Box::uninitialized();
            ffi::graphene_box_expand_scalar(
                self.to_glib_none().0,
                scalar,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_box_expand_vec3")]
    #[must_use]
    pub fn expand_vec3(&self, vec: &Vec3) -> Box {
        unsafe {
            let mut res = Box::uninitialized();
            ffi::graphene_box_expand_vec3(
                self.to_glib_none().0,
                vec.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_box_get_bounding_sphere")]
    #[doc(alias = "get_bounding_sphere")]
    pub fn bounding_sphere(&self) -> Sphere {
        unsafe {
            let mut sphere = Sphere::uninitialized();
            ffi::graphene_box_get_bounding_sphere(
                self.to_glib_none().0,
                sphere.to_glib_none_mut().0,
            );
            sphere
        }
    }

    #[doc(alias = "graphene_box_get_center")]
    #[doc(alias = "get_center")]
    pub fn center(&self) -> Point3D {
        unsafe {
            let mut center = Point3D::uninitialized();
            ffi::graphene_box_get_center(self.to_glib_none().0, center.to_glib_none_mut().0);
            center
        }
    }

    #[doc(alias = "graphene_box_get_depth")]
    #[doc(alias = "get_depth")]
    pub fn depth(&self) -> f32 {
        unsafe { ffi::graphene_box_get_depth(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_box_get_height")]
    #[doc(alias = "get_height")]
    pub fn height(&self) -> f32 {
        unsafe { ffi::graphene_box_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_box_get_max")]
    #[doc(alias = "get_max")]
    pub fn max(&self) -> Point3D {
        unsafe {
            let mut max = Point3D::uninitialized();
            ffi::graphene_box_get_max(self.to_glib_none().0, max.to_glib_none_mut().0);
            max
        }
    }

    #[doc(alias = "graphene_box_get_min")]
    #[doc(alias = "get_min")]
    pub fn min(&self) -> Point3D {
        unsafe {
            let mut min = Point3D::uninitialized();
            ffi::graphene_box_get_min(self.to_glib_none().0, min.to_glib_none_mut().0);
            min
        }
    }

    #[doc(alias = "graphene_box_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&self) -> Vec3 {
        unsafe {
            let mut size = Vec3::uninitialized();
            ffi::graphene_box_get_size(self.to_glib_none().0, size.to_glib_none_mut().0);
            size
        }
    }

    #[doc(alias = "graphene_box_get_width")]
    #[doc(alias = "get_width")]
    pub fn width(&self) -> f32 {
        unsafe { ffi::graphene_box_get_width(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_box_intersection")]
    pub fn intersection(&self, b: &Box) -> Option<Box> {
        unsafe {
            let mut res = Box::uninitialized();
            let ret = ffi::graphene_box_intersection(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            if ret {
                Some(res)
            } else {
                None
            }
        }
    }

    #[doc(alias = "graphene_box_union")]
    #[must_use]
    pub fn union(&self, b: &Box) -> Box {
        unsafe {
            let mut res = Box::uninitialized();
            ffi::graphene_box_union(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_box_empty")]
    pub fn empty() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_empty()) }
    }

    #[doc(alias = "graphene_box_infinite")]
    pub fn infinite() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_infinite()) }
    }

    #[doc(alias = "graphene_box_minus_one")]
    pub fn minus_one() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_minus_one()) }
    }

    #[doc(alias = "graphene_box_one")]
    pub fn one() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_one()) }
    }

    #[doc(alias = "graphene_box_one_minus_one")]
    pub fn one_minus_one() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_one_minus_one()) }
    }

    #[doc(alias = "graphene_box_zero")]
    pub fn zero() -> Box {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_box_zero()) }
    }
}

impl PartialEq for Box {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Box {}
