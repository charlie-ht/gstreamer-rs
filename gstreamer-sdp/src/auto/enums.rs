// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYCacheType {
    None,
    Always,
    ForCsb,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYCacheType {
    type GlibType = ffi::GstMIKEYCacheType;

    fn to_glib(&self) -> ffi::GstMIKEYCacheType {
        match *self {
            MIKEYCacheType::None => ffi::GST_MIKEY_CACHE_NONE,
            MIKEYCacheType::Always => ffi::GST_MIKEY_CACHE_ALWAYS,
            MIKEYCacheType::ForCsb => ffi::GST_MIKEY_CACHE_FOR_CSB,
            MIKEYCacheType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYCacheType> for MIKEYCacheType {
    fn from_glib(value: ffi::GstMIKEYCacheType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => MIKEYCacheType::None,
            1 => MIKEYCacheType::Always,
            2 => MIKEYCacheType::ForCsb,
            value => MIKEYCacheType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYEncAlg {
    Null,
    AesCm128,
    AesKw128,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYEncAlg {
    type GlibType = ffi::GstMIKEYEncAlg;

    fn to_glib(&self) -> ffi::GstMIKEYEncAlg {
        match *self {
            MIKEYEncAlg::Null => ffi::GST_MIKEY_ENC_NULL,
            MIKEYEncAlg::AesCm128 => ffi::GST_MIKEY_ENC_AES_CM_128,
            MIKEYEncAlg::AesKw128 => ffi::GST_MIKEY_ENC_AES_KW_128,
            MIKEYEncAlg::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYEncAlg> for MIKEYEncAlg {
    fn from_glib(value: ffi::GstMIKEYEncAlg) -> Self {
        skip_assert_initialized!();
        match value {
            0 => MIKEYEncAlg::Null,
            1 => MIKEYEncAlg::AesCm128,
            2 => MIKEYEncAlg::AesKw128,
            value => MIKEYEncAlg::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYKVType {
    Null,
    Spi,
    Interval,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYKVType {
    type GlibType = ffi::GstMIKEYKVType;

    fn to_glib(&self) -> ffi::GstMIKEYKVType {
        match *self {
            MIKEYKVType::Null => ffi::GST_MIKEY_KV_NULL,
            MIKEYKVType::Spi => ffi::GST_MIKEY_KV_SPI,
            MIKEYKVType::Interval => ffi::GST_MIKEY_KV_INTERVAL,
            MIKEYKVType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYKVType> for MIKEYKVType {
    fn from_glib(value: ffi::GstMIKEYKVType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => MIKEYKVType::Null,
            1 => MIKEYKVType::Spi,
            2 => MIKEYKVType::Interval,
            value => MIKEYKVType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYKeyDataType {
    Tgk,
    Tek,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYKeyDataType {
    type GlibType = ffi::GstMIKEYKeyDataType;

    fn to_glib(&self) -> ffi::GstMIKEYKeyDataType {
        match *self {
            MIKEYKeyDataType::Tgk => ffi::GST_MIKEY_KD_TGK,
            MIKEYKeyDataType::Tek => ffi::GST_MIKEY_KD_TEK,
            MIKEYKeyDataType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYKeyDataType> for MIKEYKeyDataType {
    fn from_glib(value: ffi::GstMIKEYKeyDataType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => MIKEYKeyDataType::Tgk,
            2 => MIKEYKeyDataType::Tek,
            value => MIKEYKeyDataType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYMacAlg {
    Null,
    HmacSha1160,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYMacAlg {
    type GlibType = ffi::GstMIKEYMacAlg;

    fn to_glib(&self) -> ffi::GstMIKEYMacAlg {
        match *self {
            MIKEYMacAlg::Null => ffi::GST_MIKEY_MAC_NULL,
            MIKEYMacAlg::HmacSha1160 => ffi::GST_MIKEY_MAC_HMAC_SHA_1_160,
            MIKEYMacAlg::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYMacAlg> for MIKEYMacAlg {
    fn from_glib(value: ffi::GstMIKEYMacAlg) -> Self {
        skip_assert_initialized!();
        match value {
            0 => MIKEYMacAlg::Null,
            1 => MIKEYMacAlg::HmacSha1160,
            value => MIKEYMacAlg::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYMapType {
    MikeyMapTypeSrtp,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYMapType {
    type GlibType = ffi::GstMIKEYMapType;

    fn to_glib(&self) -> ffi::GstMIKEYMapType {
        match *self {
            MIKEYMapType::MikeyMapTypeSrtp => ffi::GST_MIKEY_MAP_TYPE_SRTP,
            MIKEYMapType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYMapType> for MIKEYMapType {
    fn from_glib(value: ffi::GstMIKEYMapType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => MIKEYMapType::MikeyMapTypeSrtp,
            value => MIKEYMapType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYPRFFunc {
    MikeyPrfMikey1,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYPRFFunc {
    type GlibType = ffi::GstMIKEYPRFFunc;

    fn to_glib(&self) -> ffi::GstMIKEYPRFFunc {
        match *self {
            MIKEYPRFFunc::MikeyPrfMikey1 => ffi::GST_MIKEY_PRF_MIKEY_1,
            MIKEYPRFFunc::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYPRFFunc> for MIKEYPRFFunc {
    fn from_glib(value: ffi::GstMIKEYPRFFunc) -> Self {
        skip_assert_initialized!();
        match value {
            0 => MIKEYPRFFunc::MikeyPrfMikey1,
            value => MIKEYPRFFunc::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYPayloadType {
    Last,
    Kemac,
    Pke,
    Dh,
    Sign,
    T,
    Id,
    Cert,
    Chash,
    V,
    Sp,
    Rand,
    Err,
    KeyData,
    GenExt,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYPayloadType {
    type GlibType = ffi::GstMIKEYPayloadType;

    fn to_glib(&self) -> ffi::GstMIKEYPayloadType {
        match *self {
            MIKEYPayloadType::Last => ffi::GST_MIKEY_PT_LAST,
            MIKEYPayloadType::Kemac => ffi::GST_MIKEY_PT_KEMAC,
            MIKEYPayloadType::Pke => ffi::GST_MIKEY_PT_PKE,
            MIKEYPayloadType::Dh => ffi::GST_MIKEY_PT_DH,
            MIKEYPayloadType::Sign => ffi::GST_MIKEY_PT_SIGN,
            MIKEYPayloadType::T => ffi::GST_MIKEY_PT_T,
            MIKEYPayloadType::Id => ffi::GST_MIKEY_PT_ID,
            MIKEYPayloadType::Cert => ffi::GST_MIKEY_PT_CERT,
            MIKEYPayloadType::Chash => ffi::GST_MIKEY_PT_CHASH,
            MIKEYPayloadType::V => ffi::GST_MIKEY_PT_V,
            MIKEYPayloadType::Sp => ffi::GST_MIKEY_PT_SP,
            MIKEYPayloadType::Rand => ffi::GST_MIKEY_PT_RAND,
            MIKEYPayloadType::Err => ffi::GST_MIKEY_PT_ERR,
            MIKEYPayloadType::KeyData => ffi::GST_MIKEY_PT_KEY_DATA,
            MIKEYPayloadType::GenExt => ffi::GST_MIKEY_PT_GEN_EXT,
            MIKEYPayloadType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYPayloadType> for MIKEYPayloadType {
    fn from_glib(value: ffi::GstMIKEYPayloadType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => MIKEYPayloadType::Last,
            1 => MIKEYPayloadType::Kemac,
            2 => MIKEYPayloadType::Pke,
            3 => MIKEYPayloadType::Dh,
            4 => MIKEYPayloadType::Sign,
            5 => MIKEYPayloadType::T,
            6 => MIKEYPayloadType::Id,
            7 => MIKEYPayloadType::Cert,
            8 => MIKEYPayloadType::Chash,
            9 => MIKEYPayloadType::V,
            10 => MIKEYPayloadType::Sp,
            11 => MIKEYPayloadType::Rand,
            12 => MIKEYPayloadType::Err,
            20 => MIKEYPayloadType::KeyData,
            21 => MIKEYPayloadType::GenExt,
            value => MIKEYPayloadType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYSecProto {
    MikeySecProtoSrtp,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYSecProto {
    type GlibType = ffi::GstMIKEYSecProto;

    fn to_glib(&self) -> ffi::GstMIKEYSecProto {
        match *self {
            MIKEYSecProto::MikeySecProtoSrtp => ffi::GST_MIKEY_SEC_PROTO_SRTP,
            MIKEYSecProto::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYSecProto> for MIKEYSecProto {
    fn from_glib(value: ffi::GstMIKEYSecProto) -> Self {
        skip_assert_initialized!();
        match value {
            0 => MIKEYSecProto::MikeySecProtoSrtp,
            value => MIKEYSecProto::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYSecSRTP {
    EncAlg,
    EncKeyLen,
    AuthAlg,
    AuthKeyLen,
    SaltKeyLen,
    Prf,
    KeyDerivRate,
    SrtpEnc,
    SrtcpEnc,
    FecOrder,
    SrtpAuth,
    AuthTagLen,
    SrtpPrefixLen,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYSecSRTP {
    type GlibType = ffi::GstMIKEYSecSRTP;

    fn to_glib(&self) -> ffi::GstMIKEYSecSRTP {
        match *self {
            MIKEYSecSRTP::EncAlg => ffi::GST_MIKEY_SP_SRTP_ENC_ALG,
            MIKEYSecSRTP::EncKeyLen => ffi::GST_MIKEY_SP_SRTP_ENC_KEY_LEN,
            MIKEYSecSRTP::AuthAlg => ffi::GST_MIKEY_SP_SRTP_AUTH_ALG,
            MIKEYSecSRTP::AuthKeyLen => ffi::GST_MIKEY_SP_SRTP_AUTH_KEY_LEN,
            MIKEYSecSRTP::SaltKeyLen => ffi::GST_MIKEY_SP_SRTP_SALT_KEY_LEN,
            MIKEYSecSRTP::Prf => ffi::GST_MIKEY_SP_SRTP_PRF,
            MIKEYSecSRTP::KeyDerivRate => ffi::GST_MIKEY_SP_SRTP_KEY_DERIV_RATE,
            MIKEYSecSRTP::SrtpEnc => ffi::GST_MIKEY_SP_SRTP_SRTP_ENC,
            MIKEYSecSRTP::SrtcpEnc => ffi::GST_MIKEY_SP_SRTP_SRTCP_ENC,
            MIKEYSecSRTP::FecOrder => ffi::GST_MIKEY_SP_SRTP_FEC_ORDER,
            MIKEYSecSRTP::SrtpAuth => ffi::GST_MIKEY_SP_SRTP_SRTP_AUTH,
            MIKEYSecSRTP::AuthTagLen => ffi::GST_MIKEY_SP_SRTP_AUTH_TAG_LEN,
            MIKEYSecSRTP::SrtpPrefixLen => ffi::GST_MIKEY_SP_SRTP_SRTP_PREFIX_LEN,
            MIKEYSecSRTP::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYSecSRTP> for MIKEYSecSRTP {
    fn from_glib(value: ffi::GstMIKEYSecSRTP) -> Self {
        skip_assert_initialized!();
        match value {
            0 => MIKEYSecSRTP::EncAlg,
            1 => MIKEYSecSRTP::EncKeyLen,
            2 => MIKEYSecSRTP::AuthAlg,
            3 => MIKEYSecSRTP::AuthKeyLen,
            4 => MIKEYSecSRTP::SaltKeyLen,
            5 => MIKEYSecSRTP::Prf,
            6 => MIKEYSecSRTP::KeyDerivRate,
            7 => MIKEYSecSRTP::SrtpEnc,
            8 => MIKEYSecSRTP::SrtcpEnc,
            9 => MIKEYSecSRTP::FecOrder,
            10 => MIKEYSecSRTP::SrtpAuth,
            11 => MIKEYSecSRTP::AuthTagLen,
            12 => MIKEYSecSRTP::SrtpPrefixLen,
            value => MIKEYSecSRTP::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYTSType {
    NtpUtc,
    Ntp,
    Counter,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYTSType {
    type GlibType = ffi::GstMIKEYTSType;

    fn to_glib(&self) -> ffi::GstMIKEYTSType {
        match *self {
            MIKEYTSType::NtpUtc => ffi::GST_MIKEY_TS_TYPE_NTP_UTC,
            MIKEYTSType::Ntp => ffi::GST_MIKEY_TS_TYPE_NTP,
            MIKEYTSType::Counter => ffi::GST_MIKEY_TS_TYPE_COUNTER,
            MIKEYTSType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYTSType> for MIKEYTSType {
    fn from_glib(value: ffi::GstMIKEYTSType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => MIKEYTSType::NtpUtc,
            1 => MIKEYTSType::Ntp,
            2 => MIKEYTSType::Counter,
            value => MIKEYTSType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MIKEYType {
    Invalid,
    PskInit,
    PskVerify,
    PkInit,
    PkVerify,
    DhInit,
    DhResp,
    Error,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MIKEYType {
    type GlibType = ffi::GstMIKEYType;

    fn to_glib(&self) -> ffi::GstMIKEYType {
        match *self {
            MIKEYType::Invalid => ffi::GST_MIKEY_TYPE_INVALID,
            MIKEYType::PskInit => ffi::GST_MIKEY_TYPE_PSK_INIT,
            MIKEYType::PskVerify => ffi::GST_MIKEY_TYPE_PSK_VERIFY,
            MIKEYType::PkInit => ffi::GST_MIKEY_TYPE_PK_INIT,
            MIKEYType::PkVerify => ffi::GST_MIKEY_TYPE_PK_VERIFY,
            MIKEYType::DhInit => ffi::GST_MIKEY_TYPE_DH_INIT,
            MIKEYType::DhResp => ffi::GST_MIKEY_TYPE_DH_RESP,
            MIKEYType::Error => ffi::GST_MIKEY_TYPE_ERROR,
            MIKEYType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMIKEYType> for MIKEYType {
    fn from_glib(value: ffi::GstMIKEYType) -> Self {
        skip_assert_initialized!();
        match value {
            -1 => MIKEYType::Invalid,
            0 => MIKEYType::PskInit,
            1 => MIKEYType::PskVerify,
            2 => MIKEYType::PkInit,
            3 => MIKEYType::PkVerify,
            4 => MIKEYType::DhInit,
            5 => MIKEYType::DhResp,
            6 => MIKEYType::Error,
            value => MIKEYType::__Unknown(value),
        }
    }
}

