//! Media player page.

use dioxus::prelude::*;

/// Media page component.
#[component]
pub fn Media() -> Element {
    rsx! {
        div { class: "page page-media",
            h1 { "🎵 Media Player" }

            div { class: "media-tabs",
                button { class: "tab active", "Music" }
                button { class: "tab", "Video" }
                button { class: "tab", "Playlists" }
            }

            div { class: "media-library",
                p { class: "text-muted", "Your media library is empty. Add music or videos to get started." }
            }

            div { class: "media-player",
                div { class: "player-controls",
                    button { class: "btn btn-icon", "⏮" }
                    button { class: "btn btn-icon btn-play", "▶" }
                    button { class: "btn btn-icon", "⏭" }
                }
                div { class: "player-progress",
                    span { "0:00" }
                    div { class: "progress-bar" }
                    span { "0:00" }
                }
            }
        }
    }
}
