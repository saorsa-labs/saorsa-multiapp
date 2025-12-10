//! Settings page.

use dioxus::prelude::*;

/// Settings page component.
#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "page page-settings",
            h1 { "⚙ Settings" }

            div { class: "settings-section",
                h2 { "Network" }
                div { class: "setting-item",
                    label { "Bootstrap Peers" }
                    input { r#type: "text", placeholder: "Enter peer addresses..." }
                }
            }

            div { class: "settings-section",
                h2 { "Wallet" }
                div { class: "setting-item",
                    label { "Auto-lock Timeout" }
                    select {
                        option { value: "300", "5 minutes" }
                        option { value: "900", selected: true, "15 minutes" }
                        option { value: "1800", "30 minutes" }
                        option { value: "3600", "1 hour" }
                    }
                }
            }

            div { class: "settings-section",
                h2 { "Sync" }
                div { class: "setting-item",
                    label { "Max Concurrent Uploads" }
                    input { r#type: "number", value: "8", min: "1", max: "32" }
                }
            }

            div { class: "settings-section",
                h2 { "Appearance" }
                div { class: "setting-item",
                    label { "Theme" }
                    select {
                        option { value: "system", "System" }
                        option { value: "light", "Light" }
                        option { value: "dark", "Dark" }
                    }
                }
            }
        }
    }
}
