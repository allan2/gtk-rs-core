// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{translate::*, Bytes, Error, UriFlags, UriHideFlags};
use std::{fmt, mem, ptr};

crate::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Uri(Shared<ffi::GUri>);

    match fn {
        ref => |ptr| ffi::g_uri_ref(ptr),
        unref => |ptr| ffi::g_uri_unref(ptr),
        type_ => || ffi::g_uri_get_type(),
    }
}

impl Uri {
    #[doc(alias = "g_uri_get_auth_params")]
    #[doc(alias = "get_auth_params")]
    pub fn auth_params(&self) -> Option<crate::GString> {
        unsafe { from_glib_none(ffi::g_uri_get_auth_params(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_get_flags")]
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> UriFlags {
        unsafe { from_glib(ffi::g_uri_get_flags(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_get_fragment")]
    #[doc(alias = "get_fragment")]
    pub fn fragment(&self) -> Option<crate::GString> {
        unsafe { from_glib_none(ffi::g_uri_get_fragment(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_get_host")]
    #[doc(alias = "get_host")]
    pub fn host(&self) -> Option<crate::GString> {
        unsafe { from_glib_none(ffi::g_uri_get_host(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_get_password")]
    #[doc(alias = "get_password")]
    pub fn password(&self) -> Option<crate::GString> {
        unsafe { from_glib_none(ffi::g_uri_get_password(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_get_path")]
    #[doc(alias = "get_path")]
    pub fn path(&self) -> crate::GString {
        unsafe { from_glib_none(ffi::g_uri_get_path(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_get_port")]
    #[doc(alias = "get_port")]
    pub fn port(&self) -> i32 {
        unsafe { ffi::g_uri_get_port(self.to_glib_none().0) }
    }

    #[doc(alias = "g_uri_get_query")]
    #[doc(alias = "get_query")]
    pub fn query(&self) -> Option<crate::GString> {
        unsafe { from_glib_none(ffi::g_uri_get_query(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_get_scheme")]
    #[doc(alias = "get_scheme")]
    pub fn scheme(&self) -> crate::GString {
        unsafe { from_glib_none(ffi::g_uri_get_scheme(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_get_user")]
    #[doc(alias = "get_user")]
    pub fn user(&self) -> Option<crate::GString> {
        unsafe { from_glib_none(ffi::g_uri_get_user(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_get_userinfo")]
    #[doc(alias = "get_userinfo")]
    pub fn userinfo(&self) -> Option<crate::GString> {
        unsafe { from_glib_none(ffi::g_uri_get_userinfo(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_parse_relative")]
    pub fn parse_relative(&self, uri_ref: &str, flags: UriFlags) -> Result<Uri, crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_uri_parse_relative(
                self.to_glib_none().0,
                uri_ref.to_glib_none().0,
                flags.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_uri_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> crate::GString {
        unsafe { from_glib_full(ffi::g_uri_to_string(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_to_string_partial")]
    pub fn to_string_partial(&self, flags: UriHideFlags) -> crate::GString {
        unsafe {
            from_glib_full(ffi::g_uri_to_string_partial(
                self.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_uri_build")]
    pub fn build(
        flags: UriFlags,
        scheme: &str,
        userinfo: Option<&str>,
        host: Option<&str>,
        port: i32,
        path: &str,
        query: Option<&str>,
        fragment: Option<&str>,
    ) -> Uri {
        unsafe {
            from_glib_full(ffi::g_uri_build(
                flags.into_glib(),
                scheme.to_glib_none().0,
                userinfo.to_glib_none().0,
                host.to_glib_none().0,
                port,
                path.to_glib_none().0,
                query.to_glib_none().0,
                fragment.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_uri_build_with_user")]
    pub fn build_with_user(
        flags: UriFlags,
        scheme: &str,
        user: Option<&str>,
        password: Option<&str>,
        auth_params: Option<&str>,
        host: Option<&str>,
        port: i32,
        path: &str,
        query: Option<&str>,
        fragment: Option<&str>,
    ) -> Uri {
        unsafe {
            from_glib_full(ffi::g_uri_build_with_user(
                flags.into_glib(),
                scheme.to_glib_none().0,
                user.to_glib_none().0,
                password.to_glib_none().0,
                auth_params.to_glib_none().0,
                host.to_glib_none().0,
                port,
                path.to_glib_none().0,
                query.to_glib_none().0,
                fragment.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_uri_escape_bytes")]
    pub fn escape_bytes(unescaped: &[u8], reserved_chars_allowed: Option<&str>) -> crate::GString {
        let length = unescaped.len() as _;
        unsafe {
            from_glib_full(ffi::g_uri_escape_bytes(
                unescaped.to_glib_none().0,
                length,
                reserved_chars_allowed.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_uri_escape_string")]
    pub fn escape_string(
        unescaped: &str,
        reserved_chars_allowed: Option<&str>,
        allow_utf8: bool,
    ) -> crate::GString {
        unsafe {
            from_glib_full(ffi::g_uri_escape_string(
                unescaped.to_glib_none().0,
                reserved_chars_allowed.to_glib_none().0,
                allow_utf8.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_uri_is_valid")]
    pub fn is_valid(uri_string: &str, flags: UriFlags) -> Result<(), crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok =
                ffi::g_uri_is_valid(uri_string.to_glib_none().0, flags.into_glib(), &mut error);
            assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_uri_join")]
    pub fn join(
        flags: UriFlags,
        scheme: Option<&str>,
        userinfo: Option<&str>,
        host: Option<&str>,
        port: i32,
        path: &str,
        query: Option<&str>,
        fragment: Option<&str>,
    ) -> crate::GString {
        unsafe {
            from_glib_full(ffi::g_uri_join(
                flags.into_glib(),
                scheme.to_glib_none().0,
                userinfo.to_glib_none().0,
                host.to_glib_none().0,
                port,
                path.to_glib_none().0,
                query.to_glib_none().0,
                fragment.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_uri_join_with_user")]
    pub fn join_with_user(
        flags: UriFlags,
        scheme: Option<&str>,
        user: Option<&str>,
        password: Option<&str>,
        auth_params: Option<&str>,
        host: Option<&str>,
        port: i32,
        path: &str,
        query: Option<&str>,
        fragment: Option<&str>,
    ) -> crate::GString {
        unsafe {
            from_glib_full(ffi::g_uri_join_with_user(
                flags.into_glib(),
                scheme.to_glib_none().0,
                user.to_glib_none().0,
                password.to_glib_none().0,
                auth_params.to_glib_none().0,
                host.to_glib_none().0,
                port,
                path.to_glib_none().0,
                query.to_glib_none().0,
                fragment.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_uri_list_extract_uris")]
    pub fn list_extract_uris(uri_list: &str) -> Vec<crate::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_uri_list_extract_uris(
                uri_list.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_uri_parse")]
    pub fn parse(uri_string: &str, flags: UriFlags) -> Result<Uri, crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_uri_parse(uri_string.to_glib_none().0, flags.into_glib(), &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[doc(alias = "g_uri_parse_params")]
    //pub fn parse_params(params: &str, separators: &str, flags: UriParamsFlags) -> Result</*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, crate::Error> {
    //    unsafe { TODO: call ffi:g_uri_parse_params() }
    //}

    #[doc(alias = "g_uri_parse_scheme")]
    pub fn parse_scheme(uri: &str) -> Option<crate::GString> {
        unsafe { from_glib_full(ffi::g_uri_parse_scheme(uri.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_peek_scheme")]
    pub fn peek_scheme(uri: &str) -> Option<crate::GString> {
        unsafe { from_glib_none(ffi::g_uri_peek_scheme(uri.to_glib_none().0)) }
    }

    #[doc(alias = "g_uri_resolve_relative")]
    pub fn resolve_relative(
        base_uri_string: Option<&str>,
        uri_ref: &str,
        flags: UriFlags,
    ) -> Result<crate::GString, crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_uri_resolve_relative(
                base_uri_string.to_glib_none().0,
                uri_ref.to_glib_none().0,
                flags.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_uri_split")]
    pub fn split(
        uri_ref: &str,
        flags: UriFlags,
    ) -> Result<
        (
            Option<crate::GString>,
            Option<crate::GString>,
            Option<crate::GString>,
            i32,
            crate::GString,
            Option<crate::GString>,
            Option<crate::GString>,
        ),
        crate::Error,
    > {
        unsafe {
            let mut scheme = ptr::null_mut();
            let mut userinfo = ptr::null_mut();
            let mut host = ptr::null_mut();
            let mut port = mem::MaybeUninit::uninit();
            let mut path = ptr::null_mut();
            let mut query = ptr::null_mut();
            let mut fragment = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_uri_split(
                uri_ref.to_glib_none().0,
                flags.into_glib(),
                &mut scheme,
                &mut userinfo,
                &mut host,
                port.as_mut_ptr(),
                &mut path,
                &mut query,
                &mut fragment,
                &mut error,
            );
            assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((
                    from_glib_full(scheme),
                    from_glib_full(userinfo),
                    from_glib_full(host),
                    port.assume_init(),
                    from_glib_full(path),
                    from_glib_full(query),
                    from_glib_full(fragment),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_uri_split_network")]
    pub fn split_network(
        uri_string: &str,
        flags: UriFlags,
    ) -> Result<(Option<crate::GString>, Option<crate::GString>, i32), crate::Error> {
        unsafe {
            let mut scheme = ptr::null_mut();
            let mut host = ptr::null_mut();
            let mut port = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_uri_split_network(
                uri_string.to_glib_none().0,
                flags.into_glib(),
                &mut scheme,
                &mut host,
                port.as_mut_ptr(),
                &mut error,
            );
            assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((
                    from_glib_full(scheme),
                    from_glib_full(host),
                    port.assume_init(),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_uri_split_with_user")]
    pub fn split_with_user(
        uri_ref: &str,
        flags: UriFlags,
    ) -> Result<
        (
            Option<crate::GString>,
            Option<crate::GString>,
            Option<crate::GString>,
            Option<crate::GString>,
            Option<crate::GString>,
            i32,
            crate::GString,
            Option<crate::GString>,
            Option<crate::GString>,
        ),
        crate::Error,
    > {
        unsafe {
            let mut scheme = ptr::null_mut();
            let mut user = ptr::null_mut();
            let mut password = ptr::null_mut();
            let mut auth_params = ptr::null_mut();
            let mut host = ptr::null_mut();
            let mut port = mem::MaybeUninit::uninit();
            let mut path = ptr::null_mut();
            let mut query = ptr::null_mut();
            let mut fragment = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_uri_split_with_user(
                uri_ref.to_glib_none().0,
                flags.into_glib(),
                &mut scheme,
                &mut user,
                &mut password,
                &mut auth_params,
                &mut host,
                port.as_mut_ptr(),
                &mut path,
                &mut query,
                &mut fragment,
                &mut error,
            );
            assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((
                    from_glib_full(scheme),
                    from_glib_full(user),
                    from_glib_full(password),
                    from_glib_full(auth_params),
                    from_glib_full(host),
                    port.assume_init(),
                    from_glib_full(path),
                    from_glib_full(query),
                    from_glib_full(fragment),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_uri_unescape_bytes")]
    pub fn unescape_bytes(
        escaped_string: &str,
        illegal_characters: Option<&str>,
    ) -> Result<Bytes, crate::Error> {
        let length = escaped_string.len() as _;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_uri_unescape_bytes(
                escaped_string.to_glib_none().0,
                length,
                illegal_characters.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_uri_unescape_segment")]
    pub fn unescape_segment(
        escaped_string: Option<&str>,
        escaped_string_end: Option<&str>,
        illegal_characters: Option<&str>,
    ) -> Option<crate::GString> {
        unsafe {
            from_glib_full(ffi::g_uri_unescape_segment(
                escaped_string.to_glib_none().0,
                escaped_string_end.to_glib_none().0,
                illegal_characters.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_uri_unescape_string")]
    pub fn unescape_string(
        escaped_string: &str,
        illegal_characters: Option<&str>,
    ) -> Option<crate::GString> {
        unsafe {
            from_glib_full(ffi::g_uri_unescape_string(
                escaped_string.to_glib_none().0,
                illegal_characters.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Uri {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

unsafe impl Send for Uri {}
unsafe impl Sync for Uri {}
