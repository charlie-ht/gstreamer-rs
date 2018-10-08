// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DiscovererStreamInfo;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    /// `DiscovererStreamInfo` specific to subtitle streams (this includes text and
    /// image based ones).
    ///
    /// # Implements
    ///
    /// [`DiscovererStreamInfoExt`](trait.DiscovererStreamInfoExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
    pub struct DiscovererSubtitleInfo(Object<ffi::GstDiscovererSubtitleInfo>): DiscovererStreamInfo;

    match fn {
        get_type => || ffi::gst_discoverer_subtitle_info_get_type(),
    }
}

impl DiscovererSubtitleInfo {
    ///
    /// # Returns
    ///
    /// the language of the stream, or NULL if unknown.
    pub fn get_language(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_subtitle_info_get_language(self.to_glib_none().0))
        }
    }
}

unsafe impl Send for DiscovererSubtitleInfo {}
unsafe impl Sync for DiscovererSubtitleInfo {}
