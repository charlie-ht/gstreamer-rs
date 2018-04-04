// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use RTSPToken;
use ffi;
use gio;
use gio_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use gst_rtsp;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct RTSPAuth(Object<ffi::GstRTSPAuth, ffi::GstRTSPAuthClass>);

    match fn {
        get_type => || ffi::gst_rtsp_auth_get_type(),
    }
}

impl RTSPAuth {
    pub fn new() -> RTSPAuth {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_new())
        }
    }

    pub fn check(check: &str) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gst_rtsp_auth_check(check.to_glib_none().0))
        }
    }

    pub fn make_basic(user: &str, pass: &str) -> String {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_make_basic(user.to_glib_none().0, pass.to_glib_none().0))
        }
    }
}

impl Default for RTSPAuth {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPAuth {}
unsafe impl Sync for RTSPAuth {}

pub trait RTSPAuthExt {
    fn add_basic(&self, basic: &str, token: &RTSPToken);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn add_digest(&self, user: &str, pass: &str, token: &RTSPToken);

    fn get_default_token(&self) -> Option<RTSPToken>;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_supported_methods(&self) -> gst_rtsp::RTSPAuthMethod;

    fn get_tls_authentication_mode(&self) -> gio::TlsAuthenticationMode;

    fn get_tls_certificate(&self) -> Option<gio::TlsCertificate>;

    fn get_tls_database(&self) -> Option<gio::TlsDatabase>;

    fn remove_basic(&self, basic: &str);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn remove_digest(&self, user: &str);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_supported_methods(&self, methods: gst_rtsp::RTSPAuthMethod);

    fn set_tls_authentication_mode(&self, mode: gio::TlsAuthenticationMode);

    fn set_tls_certificate<'a, P: Into<Option<&'a gio::TlsCertificate>>>(&self, cert: P);

    fn set_tls_database<'a, P: IsA<gio::TlsDatabase> + 'a, Q: Into<Option<&'a P>>>(&self, database: Q);

    fn connect_accept_certificate<F: Fn(&Self, &gio::TlsConnection, &gio::TlsCertificate, gio::TlsCertificateFlags) -> bool + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPAuth> + IsA<glib::object::Object>> RTSPAuthExt for O {
    fn add_basic(&self, basic: &str, token: &RTSPToken) {
        unsafe {
            ffi::gst_rtsp_auth_add_basic(self.to_glib_none().0, basic.to_glib_none().0, token.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn add_digest(&self, user: &str, pass: &str, token: &RTSPToken) {
        unsafe {
            ffi::gst_rtsp_auth_add_digest(self.to_glib_none().0, user.to_glib_none().0, pass.to_glib_none().0, token.to_glib_none().0);
        }
    }

    fn get_default_token(&self) -> Option<RTSPToken> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_get_default_token(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_supported_methods(&self) -> gst_rtsp::RTSPAuthMethod {
        unsafe {
            from_glib(ffi::gst_rtsp_auth_get_supported_methods(self.to_glib_none().0))
        }
    }

    fn get_tls_authentication_mode(&self) -> gio::TlsAuthenticationMode {
        unsafe {
            from_glib(ffi::gst_rtsp_auth_get_tls_authentication_mode(self.to_glib_none().0))
        }
    }

    fn get_tls_certificate(&self) -> Option<gio::TlsCertificate> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_get_tls_certificate(self.to_glib_none().0))
        }
    }

    fn get_tls_database(&self) -> Option<gio::TlsDatabase> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_get_tls_database(self.to_glib_none().0))
        }
    }

    fn remove_basic(&self, basic: &str) {
        unsafe {
            ffi::gst_rtsp_auth_remove_basic(self.to_glib_none().0, basic.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn remove_digest(&self, user: &str) {
        unsafe {
            ffi::gst_rtsp_auth_remove_digest(self.to_glib_none().0, user.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_supported_methods(&self, methods: gst_rtsp::RTSPAuthMethod) {
        unsafe {
            ffi::gst_rtsp_auth_set_supported_methods(self.to_glib_none().0, methods.to_glib());
        }
    }

    fn set_tls_authentication_mode(&self, mode: gio::TlsAuthenticationMode) {
        unsafe {
            ffi::gst_rtsp_auth_set_tls_authentication_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_tls_certificate<'a, P: Into<Option<&'a gio::TlsCertificate>>>(&self, cert: P) {
        let cert = cert.into();
        let cert = cert.to_glib_none();
        unsafe {
            ffi::gst_rtsp_auth_set_tls_certificate(self.to_glib_none().0, cert.0);
        }
    }

    fn set_tls_database<'a, P: IsA<gio::TlsDatabase> + 'a, Q: Into<Option<&'a P>>>(&self, database: Q) {
        let database = database.into();
        let database = database.to_glib_none();
        unsafe {
            ffi::gst_rtsp_auth_set_tls_database(self.to_glib_none().0, database.0);
        }
    }

    fn connect_accept_certificate<F: Fn(&Self, &gio::TlsConnection, &gio::TlsCertificate, gio::TlsCertificateFlags) -> bool + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gio::TlsConnection, &gio::TlsCertificate, gio::TlsCertificateFlags) -> bool + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "accept-certificate",
                transmute(accept_certificate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn accept_certificate_trampoline<P>(this: *mut ffi::GstRTSPAuth, connection: *mut gio_ffi::GTlsConnection, peer_cert: *mut gio_ffi::GTlsCertificate, errors: gio_ffi::GTlsCertificateFlags, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<RTSPAuth> {
    callback_guard!();
    let f: &&(Fn(&P, &gio::TlsConnection, &gio::TlsCertificate, gio::TlsCertificateFlags) -> bool + Send + Sync + 'static) = transmute(f);
    f(&RTSPAuth::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(connection), &from_glib_borrow(peer_cert), from_glib(errors)).to_glib()
}
