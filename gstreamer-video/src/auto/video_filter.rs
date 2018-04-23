// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_base;
use gst_base_ffi;
use gst_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct VideoFilter(Object<ffi::GstVideoFilter, ffi::GstVideoFilterClass>): [
        gst_base::BaseTransform => gst_base_ffi::GstBaseTransform,
        gst::Element => gst_ffi::GstElement,
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_video_filter_get_type(),
    }
}

impl VideoFilter {}

unsafe impl Send for VideoFilter {}
unsafe impl Sync for VideoFilter {}
