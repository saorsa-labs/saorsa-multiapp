//! Migration page.

use dioxus::prelude::*;

/// Migration page component.
#[component]
pub fn Migration() -> Element {
    rsx! {
        div { class: "page page-migration",
            h1 { "📦 Data Migration" }

            p { class: "lead",
                "Migrate your data from Autonomi network to quantum-resistant Saorsa network."
            }

            div { class: "migration-steps",
                div { class: "step",
                    span { class: "step-number", "1" }
                    div { class: "step-content",
                        h3 { "Discover" }
                        p { "Scan for existing data on Autonomi network and local ant-node storage." }
                        button { class: "btn btn-primary", "Start Scan" }
                    }
                }

                div { class: "step",
                    span { class: "step-number", "2" }
                    div { class: "step-content",
                        h3 { "Review" }
                        p { "Review discovered data and create migration plan." }
                    }
                }

                div { class: "step",
                    span { class: "step-number", "3" }
                    div { class: "step-content",
                        h3 { "Migrate" }
                        p { "Execute migration with progress tracking." }
                    }
                }
            }
        }
    }
}
