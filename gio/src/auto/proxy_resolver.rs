// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{AsyncResult, Cancellable};
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, fmt, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GProxyResolver")]
    pub struct ProxyResolver(Interface<ffi::GProxyResolver, ffi::GProxyResolverInterface>);

    match fn {
        type_ => || ffi::g_proxy_resolver_get_type(),
    }
}

impl ProxyResolver {
    pub const NONE: Option<&'static ProxyResolver> = None;

    #[doc(alias = "g_proxy_resolver_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> ProxyResolver {
        unsafe { from_glib_none(ffi::g_proxy_resolver_get_default()) }
    }
}

pub trait ProxyResolverExt: 'static {
    #[doc(alias = "g_proxy_resolver_is_supported")]
    fn is_supported(&self) -> bool;

    #[doc(alias = "g_proxy_resolver_lookup")]
    fn lookup(
        &self,
        uri: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Vec<glib::GString>, glib::Error>;

    #[doc(alias = "g_proxy_resolver_lookup_async")]
    fn lookup_async<P: FnOnce(Result<Vec<glib::GString>, glib::Error>) + 'static>(
        &self,
        uri: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn lookup_future(
        &self,
        uri: &str,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<glib::GString>, glib::Error>> + 'static>,
    >;
}

impl<O: IsA<ProxyResolver>> ProxyResolverExt for O {
    fn is_supported(&self) -> bool {
        unsafe {
            from_glib(ffi::g_proxy_resolver_is_supported(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn lookup(
        &self,
        uri: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Vec<glib::GString>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_proxy_resolver_lookup(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn lookup_async<P: FnOnce(Result<Vec<glib::GString>, glib::Error>) + 'static>(
        &self,
        uri: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn lookup_async_trampoline<
            P: FnOnce(Result<Vec<glib::GString>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_proxy_resolver_lookup_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = lookup_async_trampoline::<P>;
        unsafe {
            ffi::g_proxy_resolver_lookup_async(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn lookup_future(
        &self,
        uri: &str,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<glib::GString>, glib::Error>> + 'static>,
    > {
        let uri = String::from(uri);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.lookup_async(&uri, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }
}

impl fmt::Display for ProxyResolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ProxyResolver")
    }
}
