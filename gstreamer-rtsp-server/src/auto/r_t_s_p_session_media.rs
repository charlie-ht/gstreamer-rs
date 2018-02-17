// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use RTSPMedia;
use RTSPStreamTransport;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct RTSPSessionMedia(Object<ffi::GstRTSPSessionMedia, ffi::GstRTSPSessionMediaClass>);

    match fn {
        get_type => || ffi::gst_rtsp_session_media_get_type(),
    }
}

impl RTSPSessionMedia {
    pub fn new(path: &str, media: &RTSPMedia) -> RTSPSessionMedia {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_media_new(path.to_glib_none().0, media.to_glib_full()))
        }
    }
}

unsafe impl Send for RTSPSessionMedia {}
unsafe impl Sync for RTSPSessionMedia {}

pub trait RTSPSessionMediaExt {
    //fn alloc_channels(&self, range: /*Ignored*/gst_rtsp::RTSPRange) -> bool;

    fn get_base_time(&self) -> gst::ClockTime;

    fn get_media(&self) -> Option<RTSPMedia>;

    fn get_rtpinfo(&self) -> Option<String>;

    //fn get_rtsp_state(&self) -> /*Ignored*/gst_rtsp::RTSPState;

    fn get_transport(&self, idx: u32) -> Option<RTSPStreamTransport>;

    fn matches(&self, path: &str) -> Option<i32>;

    //fn set_rtsp_state(&self, state: /*Ignored*/gst_rtsp::RTSPState);

    fn set_state(&self, state: gst::State) -> Result<(), glib::error::BoolError>;

    //fn set_transport(&self, stream: &RTSPStream, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport) -> Option<RTSPStreamTransport>;
}

impl<O: IsA<RTSPSessionMedia>> RTSPSessionMediaExt for O {
    //fn alloc_channels(&self, range: /*Ignored*/gst_rtsp::RTSPRange) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_session_media_alloc_channels() }
    //}

    fn get_base_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_rtsp_session_media_get_base_time(self.to_glib_none().0))
        }
    }

    fn get_media(&self) -> Option<RTSPMedia> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_session_media_get_media(self.to_glib_none().0))
        }
    }

    fn get_rtpinfo(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_media_get_rtpinfo(self.to_glib_none().0))
        }
    }

    //fn get_rtsp_state(&self) -> /*Ignored*/gst_rtsp::RTSPState {
    //    unsafe { TODO: call ffi::gst_rtsp_session_media_get_rtsp_state() }
    //}

    fn get_transport(&self, idx: u32) -> Option<RTSPStreamTransport> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_session_media_get_transport(self.to_glib_none().0, idx))
        }
    }

    fn matches(&self, path: &str) -> Option<i32> {
        unsafe {
            let mut matched = mem::uninitialized();
            let ret = from_glib(ffi::gst_rtsp_session_media_matches(self.to_glib_none().0, path.to_glib_none().0, &mut matched));
            if ret { Some(matched) } else { None }
        }
    }

    //fn set_rtsp_state(&self, state: /*Ignored*/gst_rtsp::RTSPState) {
    //    unsafe { TODO: call ffi::gst_rtsp_session_media_set_rtsp_state() }
    //}

    fn set_state(&self, state: gst::State) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_rtsp_session_media_set_state(self.to_glib_none().0, state.to_glib()), "Failed to set state of session media")
        }
    }

    //fn set_transport(&self, stream: &RTSPStream, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport) -> Option<RTSPStreamTransport> {
    //    unsafe { TODO: call ffi::gst_rtsp_session_media_set_transport() }
    //}
}
