// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::source::{Continue, Priority, SourceId};
use glib::translate::*;
use glib_ffi;
use glib_ffi::{gboolean, gpointer};
use std::cell::RefCell;
use std::mem::transmute;
use std::ptr;

use Bus;
use BusSyncReply;
use Message;

unsafe extern "C" fn trampoline_watch(
    bus: *mut ffi::GstBus,
    msg: *mut ffi::GstMessage,
    func: gpointer,
) -> gboolean {
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &RefCell<Box<FnMut(&Bus, &Message) -> Continue + Send + 'static>> = transmute(func);
    (&mut *func.borrow_mut())(&from_glib_borrow(bus), &Message::from_glib_borrow(msg)).to_glib()
}

unsafe extern "C" fn destroy_closure_watch(ptr: gpointer) {
    Box::<RefCell<Box<FnMut(&Bus, &Message) -> Continue + Send + 'static>>>::from_raw(
        ptr as *mut _,
    );
}

fn into_raw_watch<F: FnMut(&Bus, &Message) -> Continue + Send + 'static>(func: F) -> gpointer {
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    let func: Box<RefCell<Box<FnMut(&Bus, &Message) -> Continue + Send + 'static>>> =
        Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as gpointer
}

unsafe extern "C" fn trampoline_sync(
    bus: *mut ffi::GstBus,
    msg: *mut ffi::GstMessage,
    func: gpointer,
) -> ffi::GstBusSyncReply {
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let f: &&(Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static) = transmute(func);
    let res = f(&from_glib_borrow(bus), &Message::from_glib_borrow(msg)).to_glib();

    if res == ffi::GST_BUS_DROP {
        ffi::gst_mini_object_unref(msg as *mut _);
    }

    res
}

unsafe extern "C" fn destroy_closure_sync(ptr: gpointer) {
    Box::<Box<Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static>>::from_raw(ptr as *mut _);
}

fn into_raw_sync<F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static>(
    func: F,
) -> gpointer {
    let func: Box<Box<Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static>> =
        Box::new(Box::new(func));
    Box::into_raw(func) as gpointer
}

impl Bus {
    pub fn add_signal_watch_full(&self, priority: Priority) {
        unsafe {
            ffi::gst_bus_add_signal_watch_full(self.to_glib_none().0, priority.to_glib());
        }
    }

    pub fn create_watch<'a, N: Into<Option<&'a str>>, F>(
        &self,
        name: N,
        priority: Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&Bus, &Message) -> Continue + Send + 'static,
    {
        skip_assert_initialized!();
        unsafe {
            let source = ffi::gst_bus_create_watch(self.to_glib_none().0);
            let trampoline = trampoline_watch as gpointer;
            glib_ffi::g_source_set_callback(
                source,
                Some(transmute(trampoline)),
                into_raw_watch(func),
                Some(destroy_closure_watch),
            );
            glib_ffi::g_source_set_priority(source, priority.to_glib());

            let name = name.into();
            if let Some(name) = name {
                glib_ffi::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }

    pub fn add_watch<F>(&self, func: F) -> SourceId
    where
        F: FnMut(&Bus, &Message) -> Continue + Send + 'static,
    {
        unsafe {
            from_glib(ffi::gst_bus_add_watch_full(
                self.to_glib_none().0,
                glib_ffi::G_PRIORITY_DEFAULT,
                Some(trampoline_watch),
                into_raw_watch(func),
                Some(destroy_closure_watch),
            ))
        }
    }

    pub fn set_sync_handler<F>(&self, func: F)
    where
        F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static,
    {
        unsafe {
            ffi::gst_bus_set_sync_handler(
                self.to_glib_none().0,
                Some(trampoline_sync),
                into_raw_sync(func),
                Some(destroy_closure_sync),
            )
        }
    }

    pub fn unset_sync_handler(&self) {
        unsafe { ffi::gst_bus_set_sync_handler(self.to_glib_none().0, None, ptr::null_mut(), None) }
    }
}

#[cfg(any(feature = "futures", feature = "dox"))]
mod futures {
    use super::*;
    use futures_core::stream::Stream;
    use futures_core::task::{Context, Waker};
    use futures_core::{Async, Poll};
    use std::sync::{Arc, Mutex};

    pub struct BusStream(Bus, Arc<Mutex<Option<Waker>>>);

    impl BusStream {
        pub fn new(bus: &Bus) -> Self {
            skip_assert_initialized!();
            let waker = Arc::new(Mutex::new(None));
            let waker_clone = Arc::clone(&waker);

            bus.set_sync_handler(move |_, _| {
                let mut waker = waker_clone.lock().unwrap();
                if let Some(waker) = waker.take() {
                    // FIXME: Force type...
                    let waker: Waker = waker;
                    waker.wake();
                }

                BusSyncReply::Pass
            });

            BusStream(bus.clone(), waker)
        }
    }

    impl Drop for BusStream {
        fn drop(&mut self) {
            self.0.unset_sync_handler();
        }
    }

    impl Stream for BusStream {
        type Item = Message;
        type Error = ();

        fn poll_next(&mut self, ctx: &mut Context) -> Poll<Option<Self::Item>, Self::Error> {
            let BusStream(ref bus, ref waker) = *self;

            let msg = bus.pop();
            if let Some(msg) = msg {
                Ok(Async::Ready(Some(msg)))
            } else {
                let mut waker = waker.lock().unwrap();
                *waker = Some(ctx.waker().clone());
                Ok(Async::Pending)
            }
        }
    }
}

#[cfg(any(feature = "futures", feature = "dox"))]
pub use bus::futures::BusStream;
