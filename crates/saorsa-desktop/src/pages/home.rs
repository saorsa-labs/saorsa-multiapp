//! Home page.

use dioxus::prelude::*;

/// Home page component.
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "page page-home",
            h1 { "Welcome to Saorsa" }
            p { class: "lead",
                "Your quantum-resistant gateway to the decentralized web."
            }

            div { class: "dashboard-grid",
                // Node status card
                div { class: "card",
                    h3 { "🌐 Network Status" }
                    p { "Connected to Saorsa network" }
                    div { class: "stat",
                        span { class: "stat-value", "42" }
                        span { class: "stat-label", "peers" }
                    }
                }

                // Wallet card
                div { class: "card",
                    h3 { "💼 Wallet" }
                    p { "Secure quantum-resistant wallet" }
                    div { class: "stat",
                        span { class: "stat-value", "1" }
                        span { class: "stat-label", "wallets" }
                    }
                }

                // Sync card
                div { class: "card",
                    h3 { "🔄 Sync Status" }
                    p { "All files synchronized" }
                    div { class: "stat",
                        span { class: "stat-value", "0" }
                        span { class: "stat-label", "pending" }
                    }
                }

                // Media card
                div { class: "card",
                    h3 { "🎵 Media Library" }
                    p { "Music and video streaming" }
                    div { class: "stat",
                        span { class: "stat-value", "0" }
                        span { class: "stat-label", "tracks" }
                    }
                }
            }
        }
    }
}
