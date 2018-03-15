// This file was generated by gir (https://github.com/gtk-rs/gir @ d1e0127)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Caps;
use Object;
use PluginFeature;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TypeFindFactory(Object<ffi::GstTypeFindFactory, ffi::GstTypeFindFactoryClass>): PluginFeature, Object;

    match fn {
        get_type => || ffi::gst_type_find_factory_get_type(),
    }
}

impl TypeFindFactory {
    //pub fn call_function(&self, find: /*Ignored*/&mut TypeFind) {
    //    unsafe { TODO: call ffi::gst_type_find_factory_call_function() }
    //}

    pub fn get_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_none(ffi::gst_type_find_factory_get_caps(self.to_glib_none().0))
        }
    }

    pub fn get_extensions(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_type_find_factory_get_extensions(self.to_glib_none().0))
        }
    }

    pub fn has_function(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_type_find_factory_has_function(self.to_glib_none().0))
        }
    }

    pub fn get_list() -> Vec<TypeFindFactory> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_type_find_factory_get_list())
        }
    }
}

unsafe impl Send for TypeFindFactory {}
unsafe impl Sync for TypeFindFactory {}
