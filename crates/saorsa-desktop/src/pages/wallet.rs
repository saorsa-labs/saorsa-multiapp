//! Wallet management page.

use dioxus::prelude::*;

/// Wallet page component.
#[component]
pub fn Wallet() -> Element {
    rsx! {
        div { class: "page page-wallet",
            h1 { "💼 Wallet" }

            div { class: "wallet-actions",
                button { class: "btn btn-primary", "Create New Wallet" }
                button { class: "btn btn-secondary", "Import Wallet" }
            }

            div { class: "wallet-list",
                p { class: "text-muted", "No wallets yet. Create or import a wallet to get started." }
            }
        }
    }
}
