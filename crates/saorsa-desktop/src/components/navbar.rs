//! Navigation bar component.

use dioxus::prelude::*;

/// Top navigation bar.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar",
            div { class: "navbar-brand",
                span { class: "navbar-logo", "S" }
                span { class: "navbar-title", "Saorsa" }
            }

            div { class: "navbar-status",
                // Node status indicator
                div { class: "status-indicator status-connected",
                    span { class: "status-dot" }
                    span { "Connected" }
                }
            }

            div { class: "navbar-actions",
                button { class: "btn btn-icon", title: "Settings",
                    "⚙"
                }
            }
        }
    }
}
