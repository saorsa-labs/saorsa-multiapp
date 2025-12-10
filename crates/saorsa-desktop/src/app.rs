//! Root application component.

use dioxus::prelude::*;
use dioxus_router::prelude::Router;
use saorsa_desktop_core::{AppState, create_event_bus};

use crate::components::{Navbar, Sidebar, Toast};
use crate::router::Route;

/// Root application component.
#[component]
pub fn App() -> Element {
    // Initialize global state
    let events = create_event_bus();
    let app_state = use_signal(|| AppState::new(events));

    // Provide state to all children via context
    use_context_provider(|| app_state);

    rsx! {
        div { class: "app-container",
            // Navigation bar
            Navbar {}

            div { class: "app-content",
                // Sidebar navigation
                Sidebar {}

                // Main content area with router
                main { class: "main-content",
                    Router::<Route> {}
                }
            }

            // Toast notifications
            Toast {}
        }

        // Global styles
        style { {include_str!("../../../assets/styles.css")} }
    }
}
