// This file was generated by gir (https://github.com/gtk-rs/gir @ d1e0127)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

mod discoverer;
pub use self::discoverer::Discoverer;

mod discoverer_audio_info;
pub use self::discoverer_audio_info::DiscovererAudioInfo;

mod discoverer_container_info;
pub use self::discoverer_container_info::DiscovererContainerInfo;

mod discoverer_info;
pub use self::discoverer_info::DiscovererInfo;
pub use self::discoverer_info::DiscovererInfoExt;

mod discoverer_stream_info;
pub use self::discoverer_stream_info::DiscovererStreamInfo;
pub use self::discoverer_stream_info::DiscovererStreamInfoExt;

mod discoverer_subtitle_info;
pub use self::discoverer_subtitle_info::DiscovererSubtitleInfo;

mod discoverer_video_info;
pub use self::discoverer_video_info::DiscovererVideoInfo;

mod enums;
pub use self::enums::DiscovererResult;

mod flags;
pub use self::flags::DiscovererSerializeFlags;

#[doc(hidden)]
pub mod traits {
    pub use super::DiscovererInfoExt;
    pub use super::DiscovererStreamInfoExt;
}
