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
pub enum WebRTCDTLSSetup {
    None,
    Actpass,
    Active,
    Passive,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WebRTCDTLSSetup {
    type GlibType = ffi::GstWebRTCDTLSSetup;

    fn to_glib(&self) -> ffi::GstWebRTCDTLSSetup {
        match *self {
            WebRTCDTLSSetup::None => ffi::GST_WEBRTC_DTLS_SETUP_NONE,
            WebRTCDTLSSetup::Actpass => ffi::GST_WEBRTC_DTLS_SETUP_ACTPASS,
            WebRTCDTLSSetup::Active => ffi::GST_WEBRTC_DTLS_SETUP_ACTIVE,
            WebRTCDTLSSetup::Passive => ffi::GST_WEBRTC_DTLS_SETUP_PASSIVE,
            WebRTCDTLSSetup::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstWebRTCDTLSSetup> for WebRTCDTLSSetup {
    fn from_glib(value: ffi::GstWebRTCDTLSSetup) -> Self {
        skip_assert_initialized!();
        match value {
            0 => WebRTCDTLSSetup::None,
            1 => WebRTCDTLSSetup::Actpass,
            2 => WebRTCDTLSSetup::Active,
            3 => WebRTCDTLSSetup::Passive,
            value => WebRTCDTLSSetup::__Unknown(value),
        }
    }
}

impl StaticType for WebRTCDTLSSetup {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_webrtc_dtls_setup_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WebRTCDTLSSetup {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WebRTCDTLSSetup {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WebRTCDTLSSetup {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum WebRTCDTLSTransportState {
    New,
    Closed,
    Failed,
    Connecting,
    Connected,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WebRTCDTLSTransportState {
    type GlibType = ffi::GstWebRTCDTLSTransportState;

    fn to_glib(&self) -> ffi::GstWebRTCDTLSTransportState {
        match *self {
            WebRTCDTLSTransportState::New => ffi::GST_WEBRTC_DTLS_TRANSPORT_STATE_NEW,
            WebRTCDTLSTransportState::Closed => ffi::GST_WEBRTC_DTLS_TRANSPORT_STATE_CLOSED,
            WebRTCDTLSTransportState::Failed => ffi::GST_WEBRTC_DTLS_TRANSPORT_STATE_FAILED,
            WebRTCDTLSTransportState::Connecting => ffi::GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTING,
            WebRTCDTLSTransportState::Connected => ffi::GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTED,
            WebRTCDTLSTransportState::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstWebRTCDTLSTransportState> for WebRTCDTLSTransportState {
    fn from_glib(value: ffi::GstWebRTCDTLSTransportState) -> Self {
        skip_assert_initialized!();
        match value {
            0 => WebRTCDTLSTransportState::New,
            1 => WebRTCDTLSTransportState::Closed,
            2 => WebRTCDTLSTransportState::Failed,
            3 => WebRTCDTLSTransportState::Connecting,
            4 => WebRTCDTLSTransportState::Connected,
            value => WebRTCDTLSTransportState::__Unknown(value),
        }
    }
}

impl StaticType for WebRTCDTLSTransportState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_webrtc_dtls_transport_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WebRTCDTLSTransportState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WebRTCDTLSTransportState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WebRTCDTLSTransportState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum WebRTCICEComponent {
    Rtp,
    Rtcp,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WebRTCICEComponent {
    type GlibType = ffi::GstWebRTCICEComponent;

    fn to_glib(&self) -> ffi::GstWebRTCICEComponent {
        match *self {
            WebRTCICEComponent::Rtp => ffi::GST_WEBRTC_ICE_COMPONENT_RTP,
            WebRTCICEComponent::Rtcp => ffi::GST_WEBRTC_ICE_COMPONENT_RTCP,
            WebRTCICEComponent::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstWebRTCICEComponent> for WebRTCICEComponent {
    fn from_glib(value: ffi::GstWebRTCICEComponent) -> Self {
        skip_assert_initialized!();
        match value {
            0 => WebRTCICEComponent::Rtp,
            1 => WebRTCICEComponent::Rtcp,
            value => WebRTCICEComponent::__Unknown(value),
        }
    }
}

impl StaticType for WebRTCICEComponent {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_webrtc_ice_component_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WebRTCICEComponent {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WebRTCICEComponent {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WebRTCICEComponent {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum WebRTCICEConnectionState {
    New,
    Checking,
    Connected,
    Completed,
    Failed,
    Disconnected,
    Closed,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WebRTCICEConnectionState {
    type GlibType = ffi::GstWebRTCICEConnectionState;

    fn to_glib(&self) -> ffi::GstWebRTCICEConnectionState {
        match *self {
            WebRTCICEConnectionState::New => ffi::GST_WEBRTC_ICE_CONNECTION_STATE_NEW,
            WebRTCICEConnectionState::Checking => ffi::GST_WEBRTC_ICE_CONNECTION_STATE_CHECKING,
            WebRTCICEConnectionState::Connected => ffi::GST_WEBRTC_ICE_CONNECTION_STATE_CONNECTED,
            WebRTCICEConnectionState::Completed => ffi::GST_WEBRTC_ICE_CONNECTION_STATE_COMPLETED,
            WebRTCICEConnectionState::Failed => ffi::GST_WEBRTC_ICE_CONNECTION_STATE_FAILED,
            WebRTCICEConnectionState::Disconnected => ffi::GST_WEBRTC_ICE_CONNECTION_STATE_DISCONNECTED,
            WebRTCICEConnectionState::Closed => ffi::GST_WEBRTC_ICE_CONNECTION_STATE_CLOSED,
            WebRTCICEConnectionState::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstWebRTCICEConnectionState> for WebRTCICEConnectionState {
    fn from_glib(value: ffi::GstWebRTCICEConnectionState) -> Self {
        skip_assert_initialized!();
        match value {
            0 => WebRTCICEConnectionState::New,
            1 => WebRTCICEConnectionState::Checking,
            2 => WebRTCICEConnectionState::Connected,
            3 => WebRTCICEConnectionState::Completed,
            4 => WebRTCICEConnectionState::Failed,
            5 => WebRTCICEConnectionState::Disconnected,
            6 => WebRTCICEConnectionState::Closed,
            value => WebRTCICEConnectionState::__Unknown(value),
        }
    }
}

impl StaticType for WebRTCICEConnectionState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_webrtc_ice_connection_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WebRTCICEConnectionState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WebRTCICEConnectionState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WebRTCICEConnectionState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum WebRTCICEGatheringState {
    New,
    Gathering,
    Complete,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WebRTCICEGatheringState {
    type GlibType = ffi::GstWebRTCICEGatheringState;

    fn to_glib(&self) -> ffi::GstWebRTCICEGatheringState {
        match *self {
            WebRTCICEGatheringState::New => ffi::GST_WEBRTC_ICE_GATHERING_STATE_NEW,
            WebRTCICEGatheringState::Gathering => ffi::GST_WEBRTC_ICE_GATHERING_STATE_GATHERING,
            WebRTCICEGatheringState::Complete => ffi::GST_WEBRTC_ICE_GATHERING_STATE_COMPLETE,
            WebRTCICEGatheringState::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstWebRTCICEGatheringState> for WebRTCICEGatheringState {
    fn from_glib(value: ffi::GstWebRTCICEGatheringState) -> Self {
        skip_assert_initialized!();
        match value {
            0 => WebRTCICEGatheringState::New,
            1 => WebRTCICEGatheringState::Gathering,
            2 => WebRTCICEGatheringState::Complete,
            value => WebRTCICEGatheringState::__Unknown(value),
        }
    }
}

impl StaticType for WebRTCICEGatheringState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_webrtc_ice_gathering_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WebRTCICEGatheringState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WebRTCICEGatheringState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WebRTCICEGatheringState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum WebRTCICERole {
    Controlled,
    Controlling,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WebRTCICERole {
    type GlibType = ffi::GstWebRTCICERole;

    fn to_glib(&self) -> ffi::GstWebRTCICERole {
        match *self {
            WebRTCICERole::Controlled => ffi::GST_WEBRTC_ICE_ROLE_CONTROLLED,
            WebRTCICERole::Controlling => ffi::GST_WEBRTC_ICE_ROLE_CONTROLLING,
            WebRTCICERole::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstWebRTCICERole> for WebRTCICERole {
    fn from_glib(value: ffi::GstWebRTCICERole) -> Self {
        skip_assert_initialized!();
        match value {
            0 => WebRTCICERole::Controlled,
            1 => WebRTCICERole::Controlling,
            value => WebRTCICERole::__Unknown(value),
        }
    }
}

impl StaticType for WebRTCICERole {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_webrtc_ice_role_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WebRTCICERole {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WebRTCICERole {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WebRTCICERole {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum WebRTCPeerConnectionState {
    New,
    Connecting,
    Connected,
    Disconnected,
    Failed,
    Closed,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WebRTCPeerConnectionState {
    type GlibType = ffi::GstWebRTCPeerConnectionState;

    fn to_glib(&self) -> ffi::GstWebRTCPeerConnectionState {
        match *self {
            WebRTCPeerConnectionState::New => ffi::GST_WEBRTC_PEER_CONNECTION_STATE_NEW,
            WebRTCPeerConnectionState::Connecting => ffi::GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTING,
            WebRTCPeerConnectionState::Connected => ffi::GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTED,
            WebRTCPeerConnectionState::Disconnected => ffi::GST_WEBRTC_PEER_CONNECTION_STATE_DISCONNECTED,
            WebRTCPeerConnectionState::Failed => ffi::GST_WEBRTC_PEER_CONNECTION_STATE_FAILED,
            WebRTCPeerConnectionState::Closed => ffi::GST_WEBRTC_PEER_CONNECTION_STATE_CLOSED,
            WebRTCPeerConnectionState::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstWebRTCPeerConnectionState> for WebRTCPeerConnectionState {
    fn from_glib(value: ffi::GstWebRTCPeerConnectionState) -> Self {
        skip_assert_initialized!();
        match value {
            0 => WebRTCPeerConnectionState::New,
            1 => WebRTCPeerConnectionState::Connecting,
            2 => WebRTCPeerConnectionState::Connected,
            3 => WebRTCPeerConnectionState::Disconnected,
            4 => WebRTCPeerConnectionState::Failed,
            5 => WebRTCPeerConnectionState::Closed,
            value => WebRTCPeerConnectionState::__Unknown(value),
        }
    }
}

impl StaticType for WebRTCPeerConnectionState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_webrtc_peer_connection_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WebRTCPeerConnectionState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WebRTCPeerConnectionState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WebRTCPeerConnectionState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum WebRTCRTPTransceiverDirection {
    None,
    Inactive,
    Sendonly,
    Recvonly,
    Sendrecv,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WebRTCRTPTransceiverDirection {
    type GlibType = ffi::GstWebRTCRTPTransceiverDirection;

    fn to_glib(&self) -> ffi::GstWebRTCRTPTransceiverDirection {
        match *self {
            WebRTCRTPTransceiverDirection::None => ffi::GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_NONE,
            WebRTCRTPTransceiverDirection::Inactive => ffi::GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_INACTIVE,
            WebRTCRTPTransceiverDirection::Sendonly => ffi::GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_SENDONLY,
            WebRTCRTPTransceiverDirection::Recvonly => ffi::GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_RECVONLY,
            WebRTCRTPTransceiverDirection::Sendrecv => ffi::GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_SENDRECV,
            WebRTCRTPTransceiverDirection::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstWebRTCRTPTransceiverDirection> for WebRTCRTPTransceiverDirection {
    fn from_glib(value: ffi::GstWebRTCRTPTransceiverDirection) -> Self {
        skip_assert_initialized!();
        match value {
            0 => WebRTCRTPTransceiverDirection::None,
            1 => WebRTCRTPTransceiverDirection::Inactive,
            2 => WebRTCRTPTransceiverDirection::Sendonly,
            3 => WebRTCRTPTransceiverDirection::Recvonly,
            4 => WebRTCRTPTransceiverDirection::Sendrecv,
            value => WebRTCRTPTransceiverDirection::__Unknown(value),
        }
    }
}

impl StaticType for WebRTCRTPTransceiverDirection {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_webrtc_rtp_transceiver_direction_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WebRTCRTPTransceiverDirection {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WebRTCRTPTransceiverDirection {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WebRTCRTPTransceiverDirection {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum WebRTCSDPType {
    Offer,
    Pranswer,
    Answer,
    Rollback,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WebRTCSDPType {
    type GlibType = ffi::GstWebRTCSDPType;

    fn to_glib(&self) -> ffi::GstWebRTCSDPType {
        match *self {
            WebRTCSDPType::Offer => ffi::GST_WEBRTC_SDP_TYPE_OFFER,
            WebRTCSDPType::Pranswer => ffi::GST_WEBRTC_SDP_TYPE_PRANSWER,
            WebRTCSDPType::Answer => ffi::GST_WEBRTC_SDP_TYPE_ANSWER,
            WebRTCSDPType::Rollback => ffi::GST_WEBRTC_SDP_TYPE_ROLLBACK,
            WebRTCSDPType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstWebRTCSDPType> for WebRTCSDPType {
    fn from_glib(value: ffi::GstWebRTCSDPType) -> Self {
        skip_assert_initialized!();
        match value {
            1 => WebRTCSDPType::Offer,
            2 => WebRTCSDPType::Pranswer,
            3 => WebRTCSDPType::Answer,
            4 => WebRTCSDPType::Rollback,
            value => WebRTCSDPType::__Unknown(value),
        }
    }
}

impl StaticType for WebRTCSDPType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_webrtc_sdp_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WebRTCSDPType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WebRTCSDPType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WebRTCSDPType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum WebRTCSignalingState {
    Stable,
    Closed,
    HaveLocalOffer,
    HaveRemoteOffer,
    HaveLocalPranswer,
    HaveRemotePranswer,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WebRTCSignalingState {
    type GlibType = ffi::GstWebRTCSignalingState;

    fn to_glib(&self) -> ffi::GstWebRTCSignalingState {
        match *self {
            WebRTCSignalingState::Stable => ffi::GST_WEBRTC_SIGNALING_STATE_STABLE,
            WebRTCSignalingState::Closed => ffi::GST_WEBRTC_SIGNALING_STATE_CLOSED,
            WebRTCSignalingState::HaveLocalOffer => ffi::GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_OFFER,
            WebRTCSignalingState::HaveRemoteOffer => ffi::GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_OFFER,
            WebRTCSignalingState::HaveLocalPranswer => ffi::GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_PRANSWER,
            WebRTCSignalingState::HaveRemotePranswer => ffi::GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_PRANSWER,
            WebRTCSignalingState::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstWebRTCSignalingState> for WebRTCSignalingState {
    fn from_glib(value: ffi::GstWebRTCSignalingState) -> Self {
        skip_assert_initialized!();
        match value {
            0 => WebRTCSignalingState::Stable,
            1 => WebRTCSignalingState::Closed,
            2 => WebRTCSignalingState::HaveLocalOffer,
            3 => WebRTCSignalingState::HaveRemoteOffer,
            4 => WebRTCSignalingState::HaveLocalPranswer,
            5 => WebRTCSignalingState::HaveRemotePranswer,
            value => WebRTCSignalingState::__Unknown(value),
        }
    }
}

impl StaticType for WebRTCSignalingState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_webrtc_signaling_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WebRTCSignalingState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WebRTCSignalingState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WebRTCSignalingState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum WebRTCStatsType {
    Codec,
    InboundRtp,
    OutboundRtp,
    RemoteInboundRtp,
    RemoteOutboundRtp,
    Csrc,
    PeerConnection,
    DataChannel,
    Stream,
    Transport,
    CandidatePair,
    LocalCandidate,
    RemoteCandidate,
    Certificate,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WebRTCStatsType {
    type GlibType = ffi::GstWebRTCStatsType;

    fn to_glib(&self) -> ffi::GstWebRTCStatsType {
        match *self {
            WebRTCStatsType::Codec => ffi::GST_WEBRTC_STATS_CODEC,
            WebRTCStatsType::InboundRtp => ffi::GST_WEBRTC_STATS_INBOUND_RTP,
            WebRTCStatsType::OutboundRtp => ffi::GST_WEBRTC_STATS_OUTBOUND_RTP,
            WebRTCStatsType::RemoteInboundRtp => ffi::GST_WEBRTC_STATS_REMOTE_INBOUND_RTP,
            WebRTCStatsType::RemoteOutboundRtp => ffi::GST_WEBRTC_STATS_REMOTE_OUTBOUND_RTP,
            WebRTCStatsType::Csrc => ffi::GST_WEBRTC_STATS_CSRC,
            WebRTCStatsType::PeerConnection => ffi::GST_WEBRTC_STATS_PEER_CONNECTION,
            WebRTCStatsType::DataChannel => ffi::GST_WEBRTC_STATS_DATA_CHANNEL,
            WebRTCStatsType::Stream => ffi::GST_WEBRTC_STATS_STREAM,
            WebRTCStatsType::Transport => ffi::GST_WEBRTC_STATS_TRANSPORT,
            WebRTCStatsType::CandidatePair => ffi::GST_WEBRTC_STATS_CANDIDATE_PAIR,
            WebRTCStatsType::LocalCandidate => ffi::GST_WEBRTC_STATS_LOCAL_CANDIDATE,
            WebRTCStatsType::RemoteCandidate => ffi::GST_WEBRTC_STATS_REMOTE_CANDIDATE,
            WebRTCStatsType::Certificate => ffi::GST_WEBRTC_STATS_CERTIFICATE,
            WebRTCStatsType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstWebRTCStatsType> for WebRTCStatsType {
    fn from_glib(value: ffi::GstWebRTCStatsType) -> Self {
        skip_assert_initialized!();
        match value {
            1 => WebRTCStatsType::Codec,
            2 => WebRTCStatsType::InboundRtp,
            3 => WebRTCStatsType::OutboundRtp,
            4 => WebRTCStatsType::RemoteInboundRtp,
            5 => WebRTCStatsType::RemoteOutboundRtp,
            6 => WebRTCStatsType::Csrc,
            7 => WebRTCStatsType::PeerConnection,
            8 => WebRTCStatsType::DataChannel,
            9 => WebRTCStatsType::Stream,
            10 => WebRTCStatsType::Transport,
            11 => WebRTCStatsType::CandidatePair,
            12 => WebRTCStatsType::LocalCandidate,
            13 => WebRTCStatsType::RemoteCandidate,
            14 => WebRTCStatsType::Certificate,
            value => WebRTCStatsType::__Unknown(value),
        }
    }
}

impl StaticType for WebRTCStatsType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_webrtc_stats_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WebRTCStatsType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WebRTCStatsType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for WebRTCStatsType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

