use dioxus::prelude::*;
use dioxus_router::Link;
use dioxus_material_icons::MaterialIcon;

use crate::hooks::mode::{is_dark, mode, color_scheme_icon};

pub fn Footer(cx: Scope) -> Element {
    log::info!("dark mode: {:?}", is_dark(&cx));
    let mode_icon = if is_dark(&cx) {
         "light_mode"
    } else {
        "dark_mode"
    };

    cx.render(rsx! {
        div { class: "w-screen mb-2 fixed bottom-0 flex mt-6 space-x-4 justify-center",
            Link {
                class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                to: "/",
                MaterialIcon { name: "home", size: 26, color: color_scheme_icon(cx) }
            }
            a {
                class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                href: "javascript:;",
                onclick: move |_| {
                    let is_dark = mode_icon == "dark_mode";
                    mode(&cx, is_dark);
                    cx.needs_update();
                },
                MaterialIcon { name: mode_icon, size: 26, color: color_scheme_icon(cx) }
            }
            Link {
                class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                to: "/about",
                MaterialIcon { size: 26, name: "menu_book", color: color_scheme_icon(cx) }
            }
            a {
                class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                href: "https://github.com/yousofmersal",
                MaterialIcon { size: 26, name: "code", color: color_scheme_icon(cx) }
            }
        }
    })
}
