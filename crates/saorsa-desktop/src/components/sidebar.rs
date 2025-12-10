//! Sidebar navigation component.

use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::router::Route;

/// Sidebar navigation.
#[component]
pub fn Sidebar() -> Element {
    rsx! {
        aside { class: "sidebar",
            nav { class: "sidebar-nav",
                SidebarLink { to: Route::Home {}, icon: "🏠", label: "Home" }
                SidebarLink { to: Route::Wallet {}, icon: "💼", label: "Wallet" }
                SidebarLink { to: Route::Sync {}, icon: "🔄", label: "Sync" }
                SidebarLink { to: Route::Media {}, icon: "🎵", label: "Media" }
                SidebarLink { to: Route::Migration {}, icon: "📦", label: "Migration" }
            }

            div { class: "sidebar-footer",
                SidebarLink { to: Route::Settings {}, icon: "⚙", label: "Settings" }
            }
        }
    }
}

/// Sidebar link item.
#[component]
fn SidebarLink(to: Route, icon: &'static str, label: &'static str) -> Element {
    rsx! {
        Link { to: to, class: "sidebar-link",
            span { class: "sidebar-icon", "{icon}" }
            span { class: "sidebar-label", "{label}" }
        }
    }
}
