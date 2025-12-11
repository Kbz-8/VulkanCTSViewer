use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "mx-auto container mb-12 py-2 px-6 sm:px-0 flex flex-row justify-between",
            Link { class: "flex flex-row h-16 text-4xl md:text-5xl select-none cursor-pointer",
                to: Route::Landing {},
                VulkanVSvg {}
                p { class: "hidden md:block mt-auto font-bold -ml-3.5 text-[#9d1b1f]",
                    "ulkan"
                }
                p { class: "mt-auto ml-2 font-bold text-gray-300",
                    "CTS Report"
                }
            }
        }

        main { class: "mx-auto container mb-24",
            Outlet::<Route> {}
        }
        footer { class: "w-screen h-11 flex flex-row justify-start space-x-1 px-6 text-sm text-gray-400",
            p { "Made by"}
            a { class: "hover:underline text-white",
                href: "https://portfolio.kbz8.me/",
                "kbz_8"
            }
            p { "with" }
            a { class: "hover:underline text-white",
                href: "https://dioxuslabs.com/",
                "Dioxus"
            }
        }
    }
}

#[component]
fn VulkanVSvg() -> Element {
    rsx! {
        svg {
            view_box: "0 0 192 192",
            g {
                transform: "translate(0.0, 192.0) scale(0.1, -0.1)",
                fill: "#9d1b1f",
                stroke: "none",
                path {
                    d: "M320 1703 c1 -10 18 -70 38 -133 35 -106 40 -115 63 -114 41 2 311 33 315 36 1 2 -8 35 -21 73 -14 39 -29 89 -35 113 l-11 42 -174 0 c-160 0 -175 -1 -175 -17z",
                }
                path {
                    d: "M1336 1678 c-38 -104 -77 -240 -73 -251 3 -7 31 -21 63 -31 33 -10 97 -34 143 -52 46 -19 88 -33 93 -32 8 3 138 380 138 401 0 4 -78 7 -174 7 l-174 0 -16 -42z",
                }
                path {
                    d: "M550 1369 c-235 -26 -412 -107 -509 -232 l-41 -54 0 -91 c0 -89 1 -92 39 -149 50 -75 117 -142 206 -205 124 -87 339 -197 350 -178 3 6 -35 50 -84 98 -160 157 -189 276 -96 402 35 47 116 105 183 130 285 108 789 43 1208 -155 107 -50 114 -52 113 -32 0 14 -26 44 -74 86 -218 191 -465 307 -765 361 -134 24 -401 34 -530 19z",
                }
                path {
                    d: "M739 1025 c-75 -15 -166 -51 -172 -68 -4 -13 221 -717 242 -755 12 -22 14 -23 192 -20 98 1 184 5 189 8 9 6 81 213 205 589 31 95 53 177 48 182 -10 10 -294 67 -308 62 -10 -3 -83 -220 -106 -312 -9 -38 -12 -42 -25 -29 -8 8 -39 91 -68 184 l-53 169 -39 2 c-21 1 -69 -4 -105 -12z",
                }
            }
        }
    }
}
