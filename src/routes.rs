use dioxus::prelude::*;

use crate::{landing::Landing, navbar::Navbar};

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
    let nav = use_navigator();

    rsx! {
        div { class: "w-full flex justify-center my-32",
            div { class: "flex bg-slate-800 flex-col md:flex-row justify-center items-center gap-2 border-1 border-gray-700 rounded-xl p-4 lg:p-8 w-fit shadow-xl shadow-slate-950",
                PageNotFoundSVG { size_class: "size-64 lg:size-[300px] xl:size-96" }
                div { class: "flex flex-col gap-8 items-center",
                    h1 { class: "h1 text-6xl text-gray-400", "Oops!" }
                    p { class: "p text-gray-500 text-2xl whitespace-pre-line text-center",
                        "We couldn't find the page
                        you were looking for"
                    }
                    Link {
                        class: "mx-auto cursor-pointer hover:underline",
                        to: Route::Landing {},
                        "Go back home"
                    }
                }
            }
        }
    }
}

#[component]
fn PageNotFoundSVG(size_class: String) -> Element {
    rsx! {
        svg {
            class: size_class,
            "version": "1.1",
            "xmlns:svg": "http://www.w3.org/2000/svg",
            "xml:space": "preserve",
            "viewBox": "0 0 64 64",
            "fill": "#000000",
            "xmlns": "http://www.w3.org/2000/svg",
            id: "svg5",
            g { "stroke-width": "0", id: "SVGRepo_bgCarrier" }
            g {
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                id: "SVGRepo_tracerCarrier",
            }
            g { id: "SVGRepo_iconCarrier",
                defs { id: "defs2" }
                g { "transform": "translate(-384,-96)", id: "layer1",
                    path {
                        "d": "m 393.99999,105 h 49 v 6 h -49 z",
                        style: "fill:#333333;fill-opacity:1;fill-rule:evenodd;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1",
                        id: "path27804",
                    }
                    path {
                        "d": "m 393.99999,111 h 49 v 40 h -49 z",
                        style: "fill:#acbec2;fill-opacity:1;fill-rule:evenodd;stroke-width:2.00001;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1",
                        id: "path27806",
                    }
                    path {
                        "d": "m 393.99999,111 v 40 h 29.76954 a 28.484051,41.392605 35.599482 0 0 18.625,-40 z",
                        style: "fill:#e8edee;fill-opacity:1;fill-rule:evenodd;stroke:none;stroke-width:2.00002;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1",
                        id: "path27808",
                    }
                    path {
                        "d": "m 395.99999,104 c -1.64501,0 -3,1.355 -3,3 v 40 c 0,0.55229 0.44772,1 1,1 0.55229,0 1,-0.44771 1,-1 v -40 c 0,-0.56413 0.43587,-1 1,-1 h 45 c 0.56414,0 1,0.43587 1,1 v 3 h -42 c -0.55228,0 -1,0.44772 -1,1 0,0.55229 0.44772,1 1,1 h 42 v 37 c 0,0.56413 -0.43586,1 -1,1 h -49 c -0.55228,0 -1,0.44772 -1,1 0,0.55229 0.44772,1 1,1 h 49 c 1.64501,0 3,-1.35499 3,-3 0,-14 0,-28 0,-42 0,-1.645 -1.35499,-3 -3,-3 z",
                        style: "color:#000000;fill:#000000;fill-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1",
                        id: "path27810",
                    }
                    path {
                        style: "color:#000000;fill:#ed7161;fill-opacity:1;fill-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1;-inkscape-stroke:none",
                        "d": "m 438.99999,107 c -0.55228,0 -1,0.44772 -1,1 0,0.55229 0.44772,1 1,1 0.55229,0 1,-0.44771 1,-1 0,-0.55228 -0.44771,-1 -1,-1 z",
                        id: "path27812",
                    }
                    path {
                        "d": "m 434.99999,107 c -0.55228,0 -1,0.44772 -1,1 0,0.55229 0.44772,1 1,1 0.55229,0 1,-0.44771 1,-1 0,-0.55228 -0.44771,-1 -1,-1 z",
                        style: "color:#000000;fill:#ecba16;fill-opacity:1;fill-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1;-inkscape-stroke:none",
                        id: "path27814",
                    }
                    path {
                        "d": "m 430.99999,107 c -0.55228,0 -1,0.44772 -1,1 0,0.55229 0.44772,1 1,1 0.55229,0 1,-0.44771 1,-1 0,-0.55228 -0.44771,-1 -1,-1 z",
                        style: "color:#000000;fill:#42b05c;fill-opacity:1;fill-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1;-inkscape-stroke:none",
                        id: "path27816",
                    }
                    path {
                        style: "color:#000000;fill:#000000;fill-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1;-inkscape-stroke:none",
                        "d": "m 388.99999,150 a 1,1 0 0 0 -1,1 1,1 0 0 0 1,1 1,1 0 0 0 1,-1 1,1 0 0 0 -1,-1 z",
                        id: "path27818",
                    }
                    path {
                        "d": "m 396.99999,110 c -0.55228,0 -1,0.44772 -1,1 0,0.55229 0.44772,1 1,1 0.55229,0 1,-0.44771 1,-1 0,-0.55228 -0.44771,-1 -1,-1 z",
                        style: "color:#000000;fill:#000000;fill-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1;-inkscape-stroke:none",
                        id: "path27820",
                    }
                    rect {
                        "y": "120",
                        "rx": "2",
                        width: "29",
                        style: "fill:#256ada;fill-opacity:1;fill-rule:evenodd;stroke:none;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1",
                        height: "22",
                        "ry": "2",
                        "x": "404",
                        id: "rect4427",
                    }
                    path {
                        "d": "m 406,120 c -1.108,0 -2,0.892 -2,2 v 18 c 0,1.108 0.892,2 2,2 h 19.58398 A 19.317461,16.374676 0 0 0 430.2207,131.36719 19.317461,16.374676 0 0 0 424.80273,120 Z",
                        style: "fill:#6b9ae6;fill-opacity:1;fill-rule:evenodd;stroke:none;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1",
                        id: "path27648",
                    }
                    rect {
                        height: "6",
                        style: "fill:#50a824;fill-opacity:1;fill-rule:evenodd;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1",
                        width: "29",
                        "y": "120",
                        "x": "404",
                        id: "rect8552",
                    }
                    path {
                        "d": "m 404,120 v 6 h 24.58984 a 14,8.5 0 0 0 0.10938,-1 14,8.5 0 0 0 -2.67969,-5 z",
                        style: "fill:#83db57;fill-opacity:1;fill-rule:evenodd;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1",
                        id: "path8626",
                    }
                    g { "transform": "translate(0,-4)", id: "path4429",
                        path {
                            "d": "m 404,130 h 29",
                            style: "color:#000000;fill:#918383;fill-rule:evenodd;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1;-inkscape-stroke:none",
                            id: "path7162",
                        }
                        path {
                            style: "color:#000000;fill:#000000;fill-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1;-inkscape-stroke:none",
                            "d": "m 406,123 c -1.6447,0 -3,1.3553 -3,3 0,1.97201 0,3.94401 0,5.91602 0,0.55228 0.44772,1 1,1 0.55228,0 1,-0.44772 1,-1 V 131 h 27 v 6 c 0,0.55228 0.44772,1 1,1 0.55228,0 1,-0.44772 1,-1 0,-3.66667 0,-7.33333 0,-11 0,-1.6447 -1.3553,-3 -3,-3 z m 0,2 h 25 c 0.5713,0 1,0.4287 1,1 v 3 h -27 v -3 c 0,-0.5713 0.4287,-1 1,-1 z m -2,10 c -0.55228,0 -1,0.44772 -1,1 v 8 c 0,1.6447 1.3553,3 3,3 h 25 c 1.6447,0 3,-1.3553 3,-3 v -3 c 0,-0.55228 -0.44772,-1 -1,-1 -0.55228,0 -1,0.44772 -1,1 v 3 c 0,0.5713 -0.4287,1 -1,1 h -25 c -0.5713,0 -1,-0.4287 -1,-1 v -8 c 0,-0.55228 -0.44772,-1 -1,-1 z",
                            id: "path7164",
                        }
                    }
                    path {
                        "d": "m 409.93555,129.00195 c -0.45187,0.0293 -0.82765,0.35863 -0.91602,0.80274 l -1,5 C 407.89645,135.42313 408.36944,135.99975 409,136 h 3 v 2 c 0,0.55228 0.44772,1 1,1 0.55228,0 1,-0.44772 1,-1 0,-1.66667 0,-3.33333 0,-5 0,-0.55228 -0.44772,-1 -1,-1 -0.55228,0 -1,0.44772 -1,1 v 1 h -1.78125 l 0.76172,-3.80469 c 0.10771,-0.54147 -0.24375,-1.06778 -0.78516,-1.17578 -0.0854,-0.0172 -0.17278,-0.0231 -0.25976,-0.0176 z",
                        style: "color:#000000;fill:#000000;fill-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1;-inkscape-stroke:none",
                        id: "path8873",
                    }
                    path {
                        style: "fill:#ffc343;fill-opacity:1;fill-rule:evenodd;stroke:none;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1",
                        "d": "m 418.99999,130 c 1.10801,0 2.00002,0.89201 2.00002,2.00002 v 2.99996 c 0,1.10801 -0.89201,2.00002 -2.00002,2.00002 -1.10801,0 -2.00002,-0.89201 -2.00002,-2.00002 v -2.99996 c 0,-1.10801 0.89201,-2.00002 2.00002,-2.00002 z",
                        id: "rect5745",
                    }
                    path {
                        "d": "m 419,129 c -1.64471,0 -3,1.35529 -3,3 v 3 c 0,1.64471 1.35529,3 3,3 1.64471,0 3,-1.35529 3,-3 v -3 a 1,1 0 0 0 -1,-1 1,1 0 0 0 -1,1 v 3 c 0,0.57131 -0.42869,1 -1,1 -0.57131,0 -1,-0.42869 -1,-1 v -3 c 0,-0.57131 0.42869,-1 1,-1 a 1,1 0 0 0 1,-1 1,1 0 0 0 -1,-1 z",
                        style: "color:#000000;fill:#000000;fill-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1;-inkscape-stroke:none",
                        id: "path7169",
                    }
                    path {
                        style: "color:#000000;fill:#000000;fill-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4.1;-inkscape-stroke:none",
                        "d": "m 425.93555,129.00195 c -0.45187,0.0293 -0.82765,0.35863 -0.91602,0.80274 l -1,5 C 423.89645,135.42313 424.36944,135.99975 425,136 h 3 v 2 c 0,0.55228 0.44772,1 1,1 0.55228,0 1,-0.44772 1,-1 0,-1.66667 0,-3.33333 0,-5 0,-0.55228 -0.44772,-1 -1,-1 -0.55228,0 -1,0.44772 -1,1 v 1 h -1.78125 l 0.76172,-3.80469 c 0.10771,-0.54147 -0.24375,-1.06778 -0.78516,-1.17578 -0.0854,-0.0172 -0.17278,-0.0231 -0.25976,-0.0176 z",
                        id: "path69785",
                    }
                }
            }
        }
    }
}
