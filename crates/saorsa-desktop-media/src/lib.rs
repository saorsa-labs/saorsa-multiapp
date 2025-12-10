//! Music and video playback for Saorsa desktop application.
//!
//! This crate provides:
//! - Audio playback using rodio
//! - Video playback using gstreamer
//! - Network streaming with buffering
//! - Playlist management
//! - Library scanning

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod audio;
pub mod buffer;
pub mod library;
pub mod playlist;

pub use audio::AudioPlayer;
pub use buffer::StreamBuffer;
pub use library::MediaLibrary;
pub use playlist::PlaylistManager;
