// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_ffi;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum RTSPAddressPoolResult {
    Ok,
    Einval,
    Ereserved,
    Erange,
    Elast,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RTSPAddressPoolResult {
    type GlibType = ffi::GstRTSPAddressPoolResult;

    fn to_glib(&self) -> ffi::GstRTSPAddressPoolResult {
        match *self {
            RTSPAddressPoolResult::Ok => ffi::GST_RTSP_ADDRESS_POOL_OK,
            RTSPAddressPoolResult::Einval => ffi::GST_RTSP_ADDRESS_POOL_EINVAL,
            RTSPAddressPoolResult::Ereserved => ffi::GST_RTSP_ADDRESS_POOL_ERESERVED,
            RTSPAddressPoolResult::Erange => ffi::GST_RTSP_ADDRESS_POOL_ERANGE,
            RTSPAddressPoolResult::Elast => ffi::GST_RTSP_ADDRESS_POOL_ELAST,
            RTSPAddressPoolResult::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPAddressPoolResult> for RTSPAddressPoolResult {
    fn from_glib(value: ffi::GstRTSPAddressPoolResult) -> Self {
        skip_assert_initialized!();
        match value {
            0 => RTSPAddressPoolResult::Ok,
            -1 => RTSPAddressPoolResult::Einval,
            -2 => RTSPAddressPoolResult::Ereserved,
            -3 => RTSPAddressPoolResult::Erange,
            -4 => RTSPAddressPoolResult::Elast,
            value => RTSPAddressPoolResult::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum RTSPMediaStatus {
    Unprepared,
    Unpreparing,
    Preparing,
    Prepared,
    Suspended,
    Error,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RTSPMediaStatus {
    type GlibType = ffi::GstRTSPMediaStatus;

    fn to_glib(&self) -> ffi::GstRTSPMediaStatus {
        match *self {
            RTSPMediaStatus::Unprepared => ffi::GST_RTSP_MEDIA_STATUS_UNPREPARED,
            RTSPMediaStatus::Unpreparing => ffi::GST_RTSP_MEDIA_STATUS_UNPREPARING,
            RTSPMediaStatus::Preparing => ffi::GST_RTSP_MEDIA_STATUS_PREPARING,
            RTSPMediaStatus::Prepared => ffi::GST_RTSP_MEDIA_STATUS_PREPARED,
            RTSPMediaStatus::Suspended => ffi::GST_RTSP_MEDIA_STATUS_SUSPENDED,
            RTSPMediaStatus::Error => ffi::GST_RTSP_MEDIA_STATUS_ERROR,
            RTSPMediaStatus::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPMediaStatus> for RTSPMediaStatus {
    fn from_glib(value: ffi::GstRTSPMediaStatus) -> Self {
        skip_assert_initialized!();
        match value {
            0 => RTSPMediaStatus::Unprepared,
            1 => RTSPMediaStatus::Unpreparing,
            2 => RTSPMediaStatus::Preparing,
            3 => RTSPMediaStatus::Prepared,
            4 => RTSPMediaStatus::Suspended,
            5 => RTSPMediaStatus::Error,
            value => RTSPMediaStatus::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum RTSPPublishClockMode {
    None,
    Clock,
    ClockAndOffset,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RTSPPublishClockMode {
    type GlibType = ffi::GstRTSPPublishClockMode;

    fn to_glib(&self) -> ffi::GstRTSPPublishClockMode {
        match *self {
            RTSPPublishClockMode::None => ffi::GST_RTSP_PUBLISH_CLOCK_MODE_NONE,
            RTSPPublishClockMode::Clock => ffi::GST_RTSP_PUBLISH_CLOCK_MODE_CLOCK,
            RTSPPublishClockMode::ClockAndOffset => ffi::GST_RTSP_PUBLISH_CLOCK_MODE_CLOCK_AND_OFFSET,
            RTSPPublishClockMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPPublishClockMode> for RTSPPublishClockMode {
    fn from_glib(value: ffi::GstRTSPPublishClockMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => RTSPPublishClockMode::None,
            1 => RTSPPublishClockMode::Clock,
            2 => RTSPPublishClockMode::ClockAndOffset,
            value => RTSPPublishClockMode::__Unknown(value),
        }
    }
}

impl StaticType for RTSPPublishClockMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_publish_clock_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTSPPublishClockMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTSPPublishClockMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RTSPPublishClockMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum RTSPSuspendMode {
    None,
    Pause,
    Reset,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RTSPSuspendMode {
    type GlibType = ffi::GstRTSPSuspendMode;

    fn to_glib(&self) -> ffi::GstRTSPSuspendMode {
        match *self {
            RTSPSuspendMode::None => ffi::GST_RTSP_SUSPEND_MODE_NONE,
            RTSPSuspendMode::Pause => ffi::GST_RTSP_SUSPEND_MODE_PAUSE,
            RTSPSuspendMode::Reset => ffi::GST_RTSP_SUSPEND_MODE_RESET,
            RTSPSuspendMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPSuspendMode> for RTSPSuspendMode {
    fn from_glib(value: ffi::GstRTSPSuspendMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => RTSPSuspendMode::None,
            1 => RTSPSuspendMode::Pause,
            2 => RTSPSuspendMode::Reset,
            value => RTSPSuspendMode::__Unknown(value),
        }
    }
}

impl StaticType for RTSPSuspendMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_suspend_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTSPSuspendMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTSPSuspendMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RTSPSuspendMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum RTSPThreadType {
    Client,
    Media,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RTSPThreadType {
    type GlibType = ffi::GstRTSPThreadType;

    fn to_glib(&self) -> ffi::GstRTSPThreadType {
        match *self {
            RTSPThreadType::Client => ffi::GST_RTSP_THREAD_TYPE_CLIENT,
            RTSPThreadType::Media => ffi::GST_RTSP_THREAD_TYPE_MEDIA,
            RTSPThreadType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPThreadType> for RTSPThreadType {
    fn from_glib(value: ffi::GstRTSPThreadType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => RTSPThreadType::Client,
            1 => RTSPThreadType::Media,
            value => RTSPThreadType::__Unknown(value),
        }
    }
}

