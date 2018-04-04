// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use AppStreamType;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_base;
use gst_base_ffi;
use gst_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct AppSrc(Object<ffi::GstAppSrc, ffi::GstAppSrcClass>): [
        gst_base::BaseSrc => gst_base_ffi::GstBaseSrc,
        gst::Element => gst_ffi::GstElement,
        gst::Object => gst_ffi::GstObject,
        gst::URIHandler => gst_ffi::GstURIHandler,
    ];

    match fn {
        get_type => || ffi::gst_app_src_get_type(),
    }
}

impl AppSrc {
    pub fn end_of_stream(&self) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_app_src_end_of_stream(self.to_glib_none().0))
        }
    }

    pub fn get_caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_app_src_get_caps(self.to_glib_none().0))
        }
    }

    pub fn get_current_level_bytes(&self) -> u64 {
        unsafe {
            ffi::gst_app_src_get_current_level_bytes(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_duration(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_app_src_get_duration(self.to_glib_none().0))
        }
    }

    pub fn get_emit_signals(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_app_src_get_emit_signals(self.to_glib_none().0))
        }
    }

    pub fn get_max_bytes(&self) -> u64 {
        unsafe {
            ffi::gst_app_src_get_max_bytes(self.to_glib_none().0)
        }
    }

    pub fn get_size(&self) -> i64 {
        unsafe {
            ffi::gst_app_src_get_size(self.to_glib_none().0)
        }
    }

    pub fn get_stream_type(&self) -> AppStreamType {
        unsafe {
            from_glib(ffi::gst_app_src_get_stream_type(self.to_glib_none().0))
        }
    }

    pub fn push_sample(&self, sample: &gst::Sample) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_app_src_push_sample(self.to_glib_none().0, sample.to_glib_none().0))
        }
    }

    //pub fn set_callbacks<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callbacks: /*Ignored*/&mut AppSrcCallbacks, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_app_src_set_callbacks() }
    //}

    pub fn set_caps<'a, P: Into<Option<&'a gst::Caps>>>(&self, caps: P) {
        let caps = caps.into();
        let caps = caps.to_glib_none();
        unsafe {
            ffi::gst_app_src_set_caps(self.to_glib_none().0, caps.0);
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn set_duration(&self, duration: gst::ClockTime) {
        unsafe {
            ffi::gst_app_src_set_duration(self.to_glib_none().0, duration.to_glib());
        }
    }

    pub fn set_emit_signals(&self, emit: bool) {
        unsafe {
            ffi::gst_app_src_set_emit_signals(self.to_glib_none().0, emit.to_glib());
        }
    }

    pub fn set_max_bytes(&self, max: u64) {
        unsafe {
            ffi::gst_app_src_set_max_bytes(self.to_glib_none().0, max);
        }
    }

    pub fn set_size(&self, size: i64) {
        unsafe {
            ffi::gst_app_src_set_size(self.to_glib_none().0, size);
        }
    }

    pub fn set_stream_type(&self, type_: AppStreamType) {
        unsafe {
            ffi::gst_app_src_set_stream_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    pub fn get_property_block(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "block".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_block(&self, block: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "block".to_glib_none().0, Value::from(&block).to_glib_none().0);
        }
    }

    pub fn get_property_duration(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "duration".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_duration(&self, duration: u64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "duration".to_glib_none().0, Value::from(&duration).to_glib_none().0);
        }
    }

    pub fn get_property_format(&self) -> gst::Format {
        unsafe {
            let mut value = Value::from_type(<gst::Format as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "format".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_format(&self, format: gst::Format) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "format".to_glib_none().0, Value::from(&format).to_glib_none().0);
        }
    }

    pub fn get_property_is_live(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-live".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_is_live(&self, is_live: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "is-live".to_glib_none().0, Value::from(&is_live).to_glib_none().0);
        }
    }

    pub fn get_property_max_latency(&self) -> i64 {
        unsafe {
            let mut value = Value::from_type(<i64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "max-latency".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_max_latency(&self, max_latency: i64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "max-latency".to_glib_none().0, Value::from(&max_latency).to_glib_none().0);
        }
    }

    pub fn get_property_min_latency(&self) -> i64 {
        unsafe {
            let mut value = Value::from_type(<i64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "min-latency".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_min_latency(&self, min_latency: i64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "min-latency".to_glib_none().0, Value::from(&min_latency).to_glib_none().0);
        }
    }

    pub fn get_property_min_percent(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "min-percent".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_min_percent(&self, min_percent: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "min-percent".to_glib_none().0, Value::from(&min_percent).to_glib_none().0);
        }
    }

    pub fn connect_enough_data<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "enough-data",
                transmute(enough_data_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_need_data<F: Fn(&AppSrc, u32) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc, u32) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "need-data",
                transmute(need_data_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_seek_data<F: Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "seek-data",
                transmute(seek_data_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_block_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::block",
                transmute(notify_block_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_caps_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::caps",
                transmute(notify_caps_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_current_level_bytes_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::current-level-bytes",
                transmute(notify_current_level_bytes_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_duration_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::duration",
                transmute(notify_duration_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_emit_signals_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::emit-signals",
                transmute(notify_emit_signals_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_format_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::format",
                transmute(notify_format_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_is_live_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::is-live",
                transmute(notify_is_live_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_max_bytes_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::max-bytes",
                transmute(notify_max_bytes_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_max_latency_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::max-latency",
                transmute(notify_max_latency_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_min_latency_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::min-latency",
                transmute(notify_min_latency_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_min_percent_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::min-percent",
                transmute(notify_min_percent_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_size_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::size",
                transmute(notify_size_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_stream_type_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSrc) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stream-type",
                transmute(notify_stream_type_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe impl Send for AppSrc {}
unsafe impl Sync for AppSrc {}

unsafe extern "C" fn enough_data_trampoline(this: *mut ffi::GstAppSrc, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn need_data_trampoline(this: *mut ffi::GstAppSrc, length: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc, u32) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this), length)
}

unsafe extern "C" fn seek_data_trampoline(this: *mut ffi::GstAppSrc, offset: u64, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &&(Fn(&AppSrc, u64) -> bool + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this), offset).to_glib()
}

unsafe extern "C" fn notify_block_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_caps_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_current_level_bytes_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_duration_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_emit_signals_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_format_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_is_live_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_max_bytes_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_max_latency_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_min_latency_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_min_percent_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_size_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_stream_type_trampoline(this: *mut ffi::GstAppSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSrc) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}
