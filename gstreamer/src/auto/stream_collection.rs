// This file was generated by gir (f00d658) from gir-files (???)
// DO NOT EDIT

use Object;
#[cfg(feature = "v1_10")]
use Stream;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct StreamCollection(Object<ffi::GstStreamCollection>): Object;

    match fn {
        get_type => || ffi::gst_stream_collection_get_type(),
    }
}

impl StreamCollection {
    #[cfg(feature = "v1_10")]
    pub fn new<'a, P: Into<Option<&'a str>>>(upstream_id: P) -> StreamCollection {
        assert_initialized_main_thread!();
        let upstream_id = upstream_id.into();
        let upstream_id = upstream_id.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_stream_collection_new(upstream_id.0))
        }
    }
}

unsafe impl Send for StreamCollection {}
unsafe impl Sync for StreamCollection {}

pub trait StreamCollectionExt {
    #[cfg(feature = "v1_10")]
    fn add_stream(&self, stream: &Stream) -> bool;

    #[cfg(feature = "v1_10")]
    fn get_size(&self) -> u32;

    #[cfg(feature = "v1_10")]
    fn get_stream(&self, index: u32) -> Option<Stream>;

    #[cfg(feature = "v1_10")]
    fn get_upstream_id(&self) -> Option<String>;

    //fn connect_stream_notify<Unsupported or ignored types>(&self, f: F) -> u64;
}

impl<O: IsA<StreamCollection>> StreamCollectionExt for O {
    #[cfg(feature = "v1_10")]
    fn add_stream(&self, stream: &Stream) -> bool {
        unsafe {
            from_glib(ffi::gst_stream_collection_add_stream(self.to_glib_none().0, stream.to_glib_full()))
        }
    }

    #[cfg(feature = "v1_10")]
    fn get_size(&self) -> u32 {
        unsafe {
            ffi::gst_stream_collection_get_size(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v1_10")]
    fn get_stream(&self, index: u32) -> Option<Stream> {
        unsafe {
            from_glib_none(ffi::gst_stream_collection_get_stream(self.to_glib_none().0, index))
        }
    }

    #[cfg(feature = "v1_10")]
    fn get_upstream_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_stream_collection_get_upstream_id(self.to_glib_none().0))
        }
    }

    //fn connect_stream_notify<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored p0: GObject.ParamSpec
    //}
}
