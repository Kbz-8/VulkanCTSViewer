use dioxus::prelude::*;

use csv::{ReaderBuilder, StringRecord};

mod components;
mod landing;
mod navbar;
mod routes;

use crate::routes::Route;

use dioxus_primitives::toast::{ToastOptions, ToastProvider, use_toast};
use std::time::Duration;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const RESULT: Asset = asset!(
    "/assets/results.csv",
    AssetOptions::builder().with_hash_suffix(false)
);

fn main() {
    dioxus::launch(AppWrapper);
}

#[component]
fn AppWrapper() -> Element {
    rsx! {
        ToastProvider { App{} }
    }
}

#[component]
fn App() -> Element {
    let toast = use_toast();

    let resource = use_resource(move || get_results());
    let mut result = use_context_provider(|| Signal::new(Vec::<StringRecord>::new()));

    use_effect(move || match &*resource.read() {
        Some(Ok(results)) => {
            let mut reader = ReaderBuilder::new().from_reader(results.as_bytes());
            let records = match reader
                .into_records()
                .collect::<Result<Vec<StringRecord>, csv::Error>>()
            {
                Ok(res) => res,
                Err(e) => {
                    error!("Failed to parse results: {e}");
                    toast.error(
                        "Error".to_string(),
                        ToastOptions::new()
                            .description(format!("Failed to parse CTS results data: {e}"))
                            .duration(Duration::from_secs(2))
                            .permanent(false),
                    );
                    Vec::new()
                }
            };
            result.set(records);
        }
        Some(Err(e)) => {
            error!("Failed to fetch results: {e}");
            toast.error(
                "Error".to_string(),
                ToastOptions::new()
                    .description(format!("Failed to fetch CTS results data: {e}"))
                    .duration(Duration::from_secs(2))
                    .permanent(false),
            );
        }
        None => {}
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Style { "
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
                --primary-success-color: #02271c;
                --secondary-success-color: #b6fae3;
                --primary-warning-color: #342203;
                --secondary-warning-color: #feeac7;
                --primary-error-color: #a22e2e;
                --secondary-error-color: #9b1c1c;
                --contrast-error-color: var(--secondary-color-3));
                --primary-info-color: var(--primary-color-5));
                --secondary-info-color: var(--primary-color-7));
            }}
        "}
        div {
            class: "text-white min-h-screen",
            style: "background: radial-gradient(circle at top, #1e293b 0, #020617 45%, #000 100%);",
            Router::<Route> {}
        }
    }
}

async fn get_results() -> Result<String> {
    Ok(
        reqwest::get(format!("{}/assets/results.csv", std::env!("URL")))
            .await?
            .text()
            .await?,
    )
}
