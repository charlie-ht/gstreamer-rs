// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EncodingProfile;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct EncodingTarget(Object<ffi::GstEncodingTarget>);

    match fn {
        get_type => || ffi::gst_encoding_target_get_type(),
    }
}

impl EncodingTarget {
    pub fn new(name: &str, category: &str, description: &str, profiles: &[EncodingProfile]) -> EncodingTarget {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_encoding_target_new(name.to_glib_none().0, category.to_glib_none().0, description.to_glib_none().0, profiles.to_glib_none().0))
        }
    }

    pub fn load<'a, P: Into<Option<&'a str>>>(name: &str, category: P) -> Result<EncodingTarget, Error> {
        assert_initialized_main_thread!();
        let category = category.into();
        let category = category.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_encoding_target_load(name.to_glib_none().0, category.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn load_from_file<P: AsRef<std::path::Path>>(filepath: P) -> Result<EncodingTarget, Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_encoding_target_load_from_file(filepath.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

unsafe impl Send for EncodingTarget {}
unsafe impl Sync for EncodingTarget {}

pub trait EncodingTargetExt {
    fn get_category(&self) -> String;

    fn get_description(&self) -> String;

    fn get_name(&self) -> String;

    fn get_profile(&self, name: &str) -> Option<EncodingProfile>;

    fn get_profiles(&self) -> Vec<EncodingProfile>;

    fn save(&self) -> Result<(), Error>;

    fn save_to_file<P: AsRef<std::path::Path>>(&self, filepath: P) -> Result<(), Error>;
}

impl<O: IsA<EncodingTarget>> EncodingTargetExt for O {
    fn get_category(&self) -> String {
        unsafe {
            from_glib_none(ffi::gst_encoding_target_get_category(self.to_glib_none().0))
        }
    }

    fn get_description(&self) -> String {
        unsafe {
            from_glib_none(ffi::gst_encoding_target_get_description(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> String {
        unsafe {
            from_glib_none(ffi::gst_encoding_target_get_name(self.to_glib_none().0))
        }
    }

    fn get_profile(&self, name: &str) -> Option<EncodingProfile> {
        unsafe {
            from_glib_full(ffi::gst_encoding_target_get_profile(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_profiles(&self) -> Vec<EncodingProfile> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_encoding_target_get_profiles(self.to_glib_none().0))
        }
    }

    fn save(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gst_encoding_target_save(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn save_to_file<P: AsRef<std::path::Path>>(&self, filepath: P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gst_encoding_target_save_to_file(self.to_glib_none().0, filepath.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}
