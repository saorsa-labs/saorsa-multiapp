//! 404 Not Found page.

use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::router::Route;

/// Not Found page component.
#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "page page-not-found",
            h1 { "404" }
            p { "Page not found: /{route.join(\"/\")}" }
            Link { to: Route::Home {}, "Go Home" }
        }
    }
}
