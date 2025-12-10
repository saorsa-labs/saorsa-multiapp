//! Toast notification component.

use dioxus::prelude::*;

/// Toast notification container.
#[component]
pub fn Toast() -> Element {
    rsx! {
        div { class: "toast-container",
            // Toasts will be rendered here
        }
    }
}
