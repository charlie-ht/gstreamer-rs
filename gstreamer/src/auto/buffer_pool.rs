// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Object;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct BufferPool(Object<ffi::GstBufferPool, ffi::GstBufferPoolClass>): Object;

    match fn {
        get_type => || ffi::gst_buffer_pool_get_type(),
    }
}

impl BufferPool {
    pub fn new() -> BufferPool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_buffer_pool_new())
        }
    }
}

impl Default for BufferPool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for BufferPool {}
unsafe impl Sync for BufferPool {}

pub trait BufferPoolExt {
    fn get_options(&self) -> Vec<String>;

    fn has_option(&self, option: &str) -> bool;

    fn is_active(&self) -> bool;

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError>;

    fn set_flushing(&self, flushing: bool);
}

impl<O: IsA<BufferPool>> BufferPoolExt for O {
    fn get_options(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_buffer_pool_get_options(self.to_glib_none().0))
        }
    }

    fn has_option(&self, option: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_buffer_pool_has_option(self.to_glib_none().0, option.to_glib_none().0))
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_buffer_pool_is_active(self.to_glib_none().0))
        }
    }

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_buffer_pool_set_active(self.to_glib_none().0, active.to_glib()), "Failed to activate buffer pool")
        }
    }

    fn set_flushing(&self, flushing: bool) {
        unsafe {
            ffi::gst_buffer_pool_set_flushing(self.to_glib_none().0, flushing.to_glib());
        }
    }
}
