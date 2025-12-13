#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::time::Duration;

mod components;
mod landing;
mod loader;
mod navbar;
mod routes;

use crate::routes::Route;

use dioxus_primitives::toast::{ToastOptions, ToastProvider, use_toast};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const COMPONENTS_CSS: Asset = asset!("/assets/dx-components-theme.css");
const RESULT: Asset = asset!(
    "/assets/results.csv",
    AssetOptions::builder().with_hash_suffix(false)
);

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: COMPONENTS_CSS }
        document::Style {
            "
            :root {{
                --primary-color: #000;
                --primary-color-1: #020618;
                --primary-color-2: #0a0a0a;
                --primary-color-3: #020618;
                --primary-color-4: #1a1a1a;
                --primary-color-5: #02081e;
                --primary-color-6: #232323;
                --primary-color-7: #1e2939;

                --secondary-color: #fff;
                --secondary-color-1: #fafafa;
                --secondary-color-2: #e6e6e6;
                --secondary-color-3: #dcdcdc;
                --secondary-color-4: #d4d4d4;
                --secondary-color-5: #ddd;
                --secondary-color-6: #5d5d5d;

                --focused-border-color: #2b7fff;
                --primary-success-color: #1A7D35;
                --secondary-success-color: #b6fae3;
                --primary-warning-color: #342203;
                --secondary-warning-color: #feeac7;
                --primary-error-color: #a22e2e;
                --secondary-error-color: #9b1c1c;
                --contrast-error-color: var(--secondary-color-3);
                --primary-info-color: var(--primary-color-5);
                --secondary-info-color: var(--primary-color-7);
            }}

            body {{
                background-color: #000;
            }}
        "
        }
        ToastProvider { default_duration: Duration::from_secs(4), max_toasts: 1_usize,
            div {
                class: "text-white min-h-screen",
                style: "background: radial-gradient(circle at top, #1e293b 0, #020617 45%, #000 100%);",
                Router::<Route> {}
            }
        }
    }
}
