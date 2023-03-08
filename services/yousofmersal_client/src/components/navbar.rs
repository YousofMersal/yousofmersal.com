use dioxus::prelude::*;
use dioxus_router::Link;
use crate::components::icon::Logo_c;

pub fn Navbar(cx: Scope) -> Element {
    // let is_hidden = use_state(cx , || "hidden");

    cx.render(rsx! {
        nav { class: "bg-gray-200 border-gray-200 sm:px-2 py-2.5 rounded dark:bg-gray-700",
            div { class: "container flex flex-wrap items-center justify-between mx-auto",
                Link { to: "/", class: "",
                    a { class: "flex items-center",
                        Logo_c { width: 32, height: 32, class: "dark:ICON h-6 mr-3 sm:h-9" }
                        span { class: "self-center text-xl font-light whitespace-nowrap dark:text-white",
                            "YM"
                        }
                    }
                }
                div { class: "flex items-center md:order-2",
                    button {
                        class: "flex mr-3 text-sm bg-gray-800 rounded-full md:mr-0 focus:ring-4 focus:ring-gray-300 dark:focus:ring-gray-600",
                        r#type: "button",
                        aria_expanded: "false",
                        "data-dropdown-toggle": "user-dropdown",
                        "data-dropdown-placement": "bottom",
                        id: "user-menu-button",
                        // onclick: move |_| {
                        //     if is_hidden.get() == &"hidden" {
                        //         is_hidden.set("block");
                        //     } else {
                        //         is_hidden.set("hidden");
                        //     }
                        // },
                        span { class: "sr-only", "Open user menu" },
                        img {
                            class: "w-8 h-8 rounded-full",
                            src: "https://t3.ftcdn.net/jpg/02/22/85/16/360_F_222851624_jfoMGbJxwRi5AWGdPgXKSABMnzCQo9RN.jpg",
                            alt: "user photo"
                        }
                    }
                    div {
                        class: "z-50 hidden block my-4 text-base list-none bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-600 dark:divide-gray-500",
                        id: "user-dropdown",
                        "data-popper-placement": "bottom",
                        div { class: "px-4 py-3",
                            span { class: "block text-sm text-gray-900 dark:text-white", "Yousof Mersal" }
                            span { class: "block text-sm font-medium text-gray-500 truncate dark:text-gray-400", "name@yousofmersal.com"}
                        }
                        ul { class: "py-2", aria_labelledby: "user-menu-button",
                            li {
                                a { class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white", href: "#", "Your Profile" }
                            }
                            li {
                                a { class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white", href: "#", "Your Profile" }
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        "data-collapse-toggle": "mobile-menu-2",
                        class: "inline-flex items-center p-2 ml-1 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600",
                        aria_controls: "mobile-menu-2",
                        aria_expanded: "false",
                        span { class: "sr-only", "Open main menu" }
                        svg { class: "w-6 h-6", "aria-hidden": "true", fill: "currentColor", view_box:"0 0 20 20", xmlns: "http://www.w3.org/2000/svg", 
                            path {
                                "fill-rule": "evenodd",
                                d: "M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z",
                                "clip-rule": "evenodd"
                            }
                        }
                    }
                }
                // div {
                //     class: "items-center justify-between hidden w-full md:flex md:w-auto md:order-1", id: "mobile-menu-2",
                //     ul {
                //         class: "flex flex-col p-4 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700",
                //         li {
                //             a {
                //                 href: "#",
                //                 class: "block py-2 pl-3 pr-4 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white",
                //                 "aria-current": "page",
                //                 "Home"
                //             }
                //         }
                //     }
                // }
            }
        }
    })
}
