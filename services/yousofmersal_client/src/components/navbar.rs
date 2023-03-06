use dioxus::prelude::*;
use dioxus_router::Link;
use crate::components::icon::Logo_c;

pub fn Navbar(cx: Scope) -> Element {
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
                div { class: "flex items-center md-order-2",
                    button {
                        class: "flex mr-3 text-sm bg-gray-800 rounded-full md:mr-0 focus:ring-4 focus:ring-gray-300 dark:focus:ring-gray-600",
                        r#type: "button",
                        id: "user-menu-button",
                        aria_expanded: "false",
                        span { class: "sr-only", "Open user menu" }
                        img {
                            class: "w-8 h-8 rounded-full",
                            src: "https://designshack.net/wp-content/uploads/placeholder-image.png",
                            alt: "user photo"
                        }
                    }
                    div {
                        class: "z-50 hidden my-4 text-base list-none bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700 dark:divide-gray-600",
                        id: "user-dropdown",
                        div { class: "px-4 py-3",
                            span { class: "block text-sm text-gray-900 dark:text-white", "Bonnie Green" }
                            span { class: "block text-sm font-medium text-gray-500 truncate dark:text-gray-400",
                                "name@yousofmersal.com"
                            }
                        }
                        ul { class: "py-2", aria_labelledby: "user-menu-button",
                            li { class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600",
                                a { class: "w-full text-left", href: "#", "Your Profile" }
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex items-center p-2 ml-1 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600",
                        aria_controls: "mobile-menu-2",
                        aria_expanded: "false",
                        span { class: "sr-only", "Open main menu" }
                        svg { class: "w-6 h-6", fill: "currentColor" }
                    }
                }
            }
        }
    })
}
