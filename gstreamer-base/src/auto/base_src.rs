// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct BaseSrc(Object<ffi::GstBaseSrc, ffi::GstBaseSrcClass>): [
        gst::Element => gst_ffi::GstElement,
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_base_src_get_type(),
    }
}

unsafe impl Send for BaseSrc {}
unsafe impl Sync for BaseSrc {}

pub trait BaseSrcExt {
    //fn get_allocator(&self, allocator: /*Ignored*/gst::Allocator, params: /*Ignored*/gst::AllocationParams);

    fn get_blocksize(&self) -> u32;

    //fn get_buffer_pool(&self) -> /*Ignored*/Option<gst::BufferPool>;

    fn get_do_timestamp(&self) -> bool;

    fn is_async(&self) -> bool;

    fn is_live(&self) -> bool;

    fn new_seamless_segment(&self, start: i64, stop: i64, time: i64) -> bool;

    fn query_latency(&self) -> Option<(bool, gst::ClockTime, gst::ClockTime)>;

    fn set_async(&self, async: bool);

    fn set_automatic_eos(&self, automatic_eos: bool);

    fn set_blocksize(&self, blocksize: u32);

    fn set_caps(&self, caps: &gst::Caps) -> bool;

    fn set_do_timestamp(&self, timestamp: bool);

    fn set_dynamic_size(&self, dynamic: bool);

    fn set_format(&self, format: gst::Format);

    fn set_live(&self, live: bool);

    fn start_complete(&self, ret: gst::FlowReturn);

    fn start_wait(&self) -> gst::FlowReturn;

    fn wait_playing(&self) -> gst::FlowReturn;

    fn get_property_num_buffers(&self) -> i32;

    fn set_property_num_buffers(&self, num_buffers: i32);

    fn get_property_typefind(&self) -> bool;

    fn set_property_typefind(&self, typefind: bool);

    fn connect_property_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_do_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_num_buffers_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_typefind_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BaseSrc> + IsA<glib::object::Object>> BaseSrcExt for O {
    //fn get_allocator(&self, allocator: /*Ignored*/gst::Allocator, params: /*Ignored*/gst::AllocationParams) {
    //    unsafe { TODO: call ffi::gst_base_src_get_allocator() }
    //}

    fn get_blocksize(&self) -> u32 {
        unsafe {
            ffi::gst_base_src_get_blocksize(self.to_glib_none().0)
        }
    }

    //fn get_buffer_pool(&self) -> /*Ignored*/Option<gst::BufferPool> {
    //    unsafe { TODO: call ffi::gst_base_src_get_buffer_pool() }
    //}

    fn get_do_timestamp(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_src_get_do_timestamp(self.to_glib_none().0))
        }
    }

    fn is_async(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_src_is_async(self.to_glib_none().0))
        }
    }

    fn is_live(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_src_is_live(self.to_glib_none().0))
        }
    }

    fn new_seamless_segment(&self, start: i64, stop: i64, time: i64) -> bool {
        unsafe {
            from_glib(ffi::gst_base_src_new_seamless_segment(self.to_glib_none().0, start, stop, time))
        }
    }

    fn query_latency(&self) -> Option<(bool, gst::ClockTime, gst::ClockTime)> {
        unsafe {
            let mut live = mem::uninitialized();
            let mut min_latency = mem::uninitialized();
            let mut max_latency = mem::uninitialized();
            let ret = from_glib(ffi::gst_base_src_query_latency(self.to_glib_none().0, &mut live, &mut min_latency, &mut max_latency));
            if ret { Some((from_glib(live), from_glib(min_latency), from_glib(max_latency))) } else { None }
        }
    }

    fn set_async(&self, async: bool) {
        unsafe {
            ffi::gst_base_src_set_async(self.to_glib_none().0, async.to_glib());
        }
    }

    fn set_automatic_eos(&self, automatic_eos: bool) {
        unsafe {
            ffi::gst_base_src_set_automatic_eos(self.to_glib_none().0, automatic_eos.to_glib());
        }
    }

    fn set_blocksize(&self, blocksize: u32) {
        unsafe {
            ffi::gst_base_src_set_blocksize(self.to_glib_none().0, blocksize);
        }
    }

    fn set_caps(&self, caps: &gst::Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_base_src_set_caps(self.to_glib_none().0, caps.to_glib_none().0))
        }
    }

    fn set_do_timestamp(&self, timestamp: bool) {
        unsafe {
            ffi::gst_base_src_set_do_timestamp(self.to_glib_none().0, timestamp.to_glib());
        }
    }

    fn set_dynamic_size(&self, dynamic: bool) {
        unsafe {
            ffi::gst_base_src_set_dynamic_size(self.to_glib_none().0, dynamic.to_glib());
        }
    }

    fn set_format(&self, format: gst::Format) {
        unsafe {
            ffi::gst_base_src_set_format(self.to_glib_none().0, format.to_glib());
        }
    }

    fn set_live(&self, live: bool) {
        unsafe {
            ffi::gst_base_src_set_live(self.to_glib_none().0, live.to_glib());
        }
    }

    fn start_complete(&self, ret: gst::FlowReturn) {
        unsafe {
            ffi::gst_base_src_start_complete(self.to_glib_none().0, ret.to_glib());
        }
    }

    fn start_wait(&self) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_base_src_start_wait(self.to_glib_none().0))
        }
    }

    fn wait_playing(&self) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_base_src_wait_playing(self.to_glib_none().0))
        }
    }

    fn get_property_num_buffers(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "num-buffers".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_num_buffers(&self, num_buffers: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "num-buffers".to_glib_none().0, Value::from(&num_buffers).to_glib_none().0);
        }
    }

    fn get_property_typefind(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "typefind".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_typefind(&self, typefind: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "typefind".to_glib_none().0, Value::from(&typefind).to_glib_none().0);
        }
    }

    fn connect_property_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::blocksize",
                transmute(notify_blocksize_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_do_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::do-timestamp",
                transmute(notify_do_timestamp_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_num_buffers_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::num-buffers",
                transmute(notify_num_buffers_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_typefind_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::typefind",
                transmute(notify_typefind_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_blocksize_trampoline<P>(this: *mut ffi::GstBaseSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSrc> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSrc::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_do_timestamp_trampoline<P>(this: *mut ffi::GstBaseSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSrc> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSrc::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_num_buffers_trampoline<P>(this: *mut ffi::GstBaseSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSrc> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSrc::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_typefind_trampoline<P>(this: *mut ffi::GstBaseSrc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseSrc> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseSrc::from_glib_borrow(this).downcast_unchecked())
}
