// This file was generated by gir (https://github.com/gtk-rs/gir @ bd67955)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use DiscovererResult;
use DiscovererSerializeFlags;
use DiscovererStreamInfo;
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
    pub struct DiscovererInfo(Object<ffi::GstDiscovererInfo>);

    match fn {
        get_type => || ffi::gst_discoverer_info_get_type(),
    }
}

impl DiscovererInfo {
    pub fn from_variant(variant: &glib::Variant) -> Option<DiscovererInfo> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_discoverer_info_from_variant(variant.to_glib_none().0))
        }
    }
}

unsafe impl Send for DiscovererInfo {}
unsafe impl Sync for DiscovererInfo {}

pub trait DiscovererInfoExt {
    fn copy(&self) -> DiscovererInfo;

    fn get_audio_streams(&self) -> Vec<DiscovererStreamInfo>;

    fn get_container_streams(&self) -> Vec<DiscovererStreamInfo>;

    fn get_duration(&self) -> gst::ClockTime;

    fn get_misc(&self) -> Option<gst::Structure>;

    fn get_missing_elements_installer_details(&self) -> Vec<String>;

    fn get_result(&self) -> DiscovererResult;

    fn get_seekable(&self) -> bool;

    fn get_stream_info(&self) -> Option<DiscovererStreamInfo>;

    fn get_stream_list(&self) -> Vec<DiscovererStreamInfo>;

    fn get_streams(&self, streamtype: glib::types::Type) -> Vec<DiscovererStreamInfo>;

    fn get_subtitle_streams(&self) -> Vec<DiscovererStreamInfo>;

    fn get_tags(&self) -> Option<gst::TagList>;

    fn get_toc(&self) -> Option<gst::Toc>;

    fn get_uri(&self) -> Option<String>;

    fn get_video_streams(&self) -> Vec<DiscovererStreamInfo>;

    fn to_variant(&self, flags: DiscovererSerializeFlags) -> Option<glib::Variant>;
}

impl<O: IsA<DiscovererInfo>> DiscovererInfoExt for O {
    fn copy(&self) -> DiscovererInfo {
        unsafe {
            from_glib_full(ffi::gst_discoverer_info_copy(self.to_glib_none().0))
        }
    }

    fn get_audio_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_audio_streams(self.to_glib_none().0))
        }
    }

    fn get_container_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_container_streams(self.to_glib_none().0))
        }
    }

    fn get_duration(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_discoverer_info_get_duration(const_override(self.to_glib_none().0)))
        }
    }

    fn get_misc(&self) -> Option<gst::Structure> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_info_get_misc(const_override(self.to_glib_none().0)))
        }
    }

    fn get_missing_elements_installer_details(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_discoverer_info_get_missing_elements_installer_details(const_override(self.to_glib_none().0)))
        }
    }

    fn get_result(&self) -> DiscovererResult {
        unsafe {
            from_glib(ffi::gst_discoverer_info_get_result(const_override(self.to_glib_none().0)))
        }
    }

    fn get_seekable(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_discoverer_info_get_seekable(const_override(self.to_glib_none().0)))
        }
    }

    fn get_stream_info(&self) -> Option<DiscovererStreamInfo> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_info_get_stream_info(self.to_glib_none().0))
        }
    }

    fn get_stream_list(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_stream_list(self.to_glib_none().0))
        }
    }

    fn get_streams(&self, streamtype: glib::types::Type) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_streams(self.to_glib_none().0, streamtype.to_glib()))
        }
    }

    fn get_subtitle_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_subtitle_streams(self.to_glib_none().0))
        }
    }

    fn get_tags(&self) -> Option<gst::TagList> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_info_get_tags(const_override(self.to_glib_none().0)))
        }
    }

    fn get_toc(&self) -> Option<gst::Toc> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_info_get_toc(const_override(self.to_glib_none().0)))
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_info_get_uri(const_override(self.to_glib_none().0)))
        }
    }

    fn get_video_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_video_streams(self.to_glib_none().0))
        }
    }

    fn to_variant(&self, flags: DiscovererSerializeFlags) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_info_to_variant(self.to_glib_none().0, flags.to_glib()))
        }
    }
}
