// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PlayerStreamInfo(Object<ffi::GstPlayerStreamInfo, ffi::GstPlayerStreamInfoClass>);

    match fn {
        get_type => || ffi::gst_player_stream_info_get_type(),
    }
}

unsafe impl Send for PlayerStreamInfo {}
unsafe impl Sync for PlayerStreamInfo {}

pub trait PlayerStreamInfoExt {
    fn get_caps(&self) -> Option<gst::Caps>;

    fn get_codec(&self) -> Option<String>;

    fn get_index(&self) -> i32;

    fn get_stream_type(&self) -> String;

    fn get_tags(&self) -> Option<gst::TagList>;
}

impl<O: IsA<PlayerStreamInfo>> PlayerStreamInfoExt for O {
    fn get_caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_none(ffi::gst_player_stream_info_get_caps(const_override(self.to_glib_none().0)))
        }
    }

    fn get_codec(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_player_stream_info_get_codec(const_override(self.to_glib_none().0)))
        }
    }

    fn get_index(&self) -> i32 {
        unsafe {
            ffi::gst_player_stream_info_get_index(const_override(self.to_glib_none().0))
        }
    }

    fn get_stream_type(&self) -> String {
        unsafe {
            from_glib_none(ffi::gst_player_stream_info_get_stream_type(const_override(self.to_glib_none().0)))
        }
    }

    fn get_tags(&self) -> Option<gst::TagList> {
        unsafe {
            from_glib_none(ffi::gst_player_stream_info_get_tags(const_override(self.to_glib_none().0)))
        }
    }
}
