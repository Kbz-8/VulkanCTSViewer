use dioxus::prelude::*;

use crate::{
    landing::Landing,
    navbar::Navbar,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Landing {},

    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        "test"
    }
}
