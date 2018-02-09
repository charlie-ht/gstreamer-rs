// This file was generated by gir (https://github.com/gtk-rs/gir @ 47eb915)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use RTSPMedia;
use RTSPSessionMedia;
use ffi;
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
    pub struct RTSPSession(Object<ffi::GstRTSPSession, ffi::GstRTSPSessionClass>);

    match fn {
        get_type => || ffi::gst_rtsp_session_get_type(),
    }
}

impl RTSPSession {
    pub fn new(sessionid: &str) -> RTSPSession {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_new(sessionid.to_glib_none().0))
        }
    }
}

unsafe impl Send for RTSPSession {}
unsafe impl Sync for RTSPSession {}

pub trait RTSPSessionExt {
    fn allow_expire(&self);

    //fn filter<'a, P: Into<Option<&'a /*Unimplemented*/RTSPSessionFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: P, user_data: Q) -> Vec<RTSPSessionMedia>;

    fn get_header(&self) -> Option<String>;

    fn get_media(&self, path: &str) -> (Option<RTSPSessionMedia>, i32);

    fn get_sessionid(&self) -> Option<String>;

    fn get_timeout(&self) -> u32;

    //fn is_expired(&self, now: /*Ignored*/&mut glib::TimeVal) -> bool;

    fn is_expired_usec(&self, now: i64) -> bool;

    fn manage_media(&self, path: &str, media: &RTSPMedia) -> Option<RTSPSessionMedia>;

    //fn next_timeout(&self, now: /*Ignored*/&mut glib::TimeVal) -> i32;

    fn next_timeout_usec(&self, now: i64) -> i32;

    fn prevent_expire(&self);

    fn release_media(&self, media: &RTSPSessionMedia) -> bool;

    fn set_timeout(&self, timeout: u32);

    fn touch(&self);

    fn get_property_timeout_always_visible(&self) -> bool;

    fn set_property_timeout_always_visible(&self, timeout_always_visible: bool);

    fn connect_property_sessionid_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timeout_always_visible_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPSession> + IsA<glib::object::Object>> RTSPSessionExt for O {
    fn allow_expire(&self) {
        unsafe {
            ffi::gst_rtsp_session_allow_expire(self.to_glib_none().0);
        }
    }

    //fn filter<'a, P: Into<Option<&'a /*Unimplemented*/RTSPSessionFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: P, user_data: Q) -> Vec<RTSPSessionMedia> {
    //    unsafe { TODO: call ffi::gst_rtsp_session_filter() }
    //}

    fn get_header(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_get_header(self.to_glib_none().0))
        }
    }

    fn get_media(&self, path: &str) -> (Option<RTSPSessionMedia>, i32) {
        unsafe {
            let mut matched = mem::uninitialized();
            let ret = from_glib_none(ffi::gst_rtsp_session_get_media(self.to_glib_none().0, path.to_glib_none().0, &mut matched));
            (ret, matched)
        }
    }

    fn get_sessionid(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_session_get_sessionid(self.to_glib_none().0))
        }
    }

    fn get_timeout(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_session_get_timeout(self.to_glib_none().0)
        }
    }

    //fn is_expired(&self, now: /*Ignored*/&mut glib::TimeVal) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_session_is_expired() }
    //}

    fn is_expired_usec(&self, now: i64) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_session_is_expired_usec(self.to_glib_none().0, now))
        }
    }

    fn manage_media(&self, path: &str, media: &RTSPMedia) -> Option<RTSPSessionMedia> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_session_manage_media(self.to_glib_none().0, path.to_glib_none().0, media.to_glib_full()))
        }
    }

    //fn next_timeout(&self, now: /*Ignored*/&mut glib::TimeVal) -> i32 {
    //    unsafe { TODO: call ffi::gst_rtsp_session_next_timeout() }
    //}

    fn next_timeout_usec(&self, now: i64) -> i32 {
        unsafe {
            ffi::gst_rtsp_session_next_timeout_usec(self.to_glib_none().0, now)
        }
    }

    fn prevent_expire(&self) {
        unsafe {
            ffi::gst_rtsp_session_prevent_expire(self.to_glib_none().0);
        }
    }

    fn release_media(&self, media: &RTSPSessionMedia) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_session_release_media(self.to_glib_none().0, media.to_glib_none().0))
        }
    }

    fn set_timeout(&self, timeout: u32) {
        unsafe {
            ffi::gst_rtsp_session_set_timeout(self.to_glib_none().0, timeout);
        }
    }

    fn touch(&self) {
        unsafe {
            ffi::gst_rtsp_session_touch(self.to_glib_none().0);
        }
    }

    fn get_property_timeout_always_visible(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "timeout-always-visible".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_timeout_always_visible(&self, timeout_always_visible: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "timeout-always-visible".to_glib_none().0, Value::from(&timeout_always_visible).to_glib_none().0);
        }
    }

    fn connect_property_sessionid_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::sessionid",
                transmute(notify_sessionid_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::timeout",
                transmute(notify_timeout_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_timeout_always_visible_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::timeout-always-visible",
                transmute(notify_timeout_always_visible_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_sessionid_trampoline<P>(this: *mut ffi::GstRTSPSession, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPSession> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPSession::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_timeout_trampoline<P>(this: *mut ffi::GstRTSPSession, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPSession> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPSession::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_timeout_always_visible_trampoline<P>(this: *mut ffi::GstRTSPSession, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPSession> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPSession::from_glib_borrow(this).downcast_unchecked())
}
