use dioxus::prelude::*;

use csv::{ReaderBuilder, StringRecord};

mod landing;
mod navbar;
mod routes;
mod loader;

use crate::routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // TODO: get results from request
    let result_data = include_str!("../assets/results.csv");
    let mut reader = ReaderBuilder::new().from_reader(result_data.as_bytes());

    let records = reader.into_records().collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    let result = use_context_provider(|| records);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "text-white min-h-screen",
            style: "background: radial-gradient(circle at top, #1e293b 0, #020617 45%, #000 100%);",
            Router::<Route> {}
        }
    }
}
