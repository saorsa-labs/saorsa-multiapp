//! Sync management page.

use dioxus::prelude::*;

/// Sync page component.
#[component]
pub fn Sync() -> Element {
    rsx! {
        div { class: "page page-sync",
            h1 { "🔄 File Sync" }

            div { class: "sync-actions",
                button { class: "btn btn-primary", "Add Folder" }
                button { class: "btn btn-secondary", "Sync Now" }
            }

            div { class: "sync-folders",
                p { class: "text-muted", "No folders synced yet. Add a folder to start syncing." }
            }

            div { class: "sync-activity",
                h3 { "Recent Activity" }
                p { class: "text-muted", "No recent activity." }
            }
        }
    }
}
