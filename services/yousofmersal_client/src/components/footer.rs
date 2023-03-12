use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons, icons::fa_brands_icons};
use dioxus_router::Link;

use crate::hooks::mode::{is_dark, mode};

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
                Icon { height: 26, width:26, icon: fa_solid_icons::FaHouse }
            }
            a {
                class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                href: "javascript:;",
                onclick: move |_| {
                    let is_dark = mode_icon == "dark_mode";
                    mode(&cx, is_dark);
                    cx.needs_update();
                },
            
                if is_dark(&cx) {
                     rsx! { Icon { height: 26, width:26, icon: fa_solid_icons::FaSun } }
                 } else {
                     rsx! { Icon { height: 26, width:26, icon: fa_solid_icons::FaMoon } }
                 }
            }
            Link {
                class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                to: "/about",
                Icon { height: 26, width:26, icon: fa_solid_icons::FaBook }
            }
            a {
                class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                href: "https://github.com/yousofmersal",
                // MaterialIcon { size: 26, name: "code", color: color_scheme_icon(cx) }
                Icon {height: 26, width: 26, icon: fa_brands_icons::FaGithub }
            }
        }
    })
}
