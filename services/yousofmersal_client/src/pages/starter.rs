use dioxus::prelude::*;
use dioxus_router::{use_route, use_router};
use dioxus_toast::ToastInfo;
use fermi::use_atom_ref;
use gloo::console;

use crate::{
    components::{
        content::Markdown,
        footer::Footer,
    },
    TOAST_MANAGER,
};

pub fn HelloDioxus(cx: Scope) -> Element {
    let input_name = use_state(&cx, String::new);
    let router = use_router(&cx);
    let toast = use_atom_ref(&cx, TOAST_MANAGER);

    cx.render(rsx! {
        section { class: "h-screen bg-cover",
            div { class: "flex h-full w-full items-center justify-center container mx-auto px-8",
                div { class: "max-w-2xl text-center",
                    h1 { class: "text-3xl sm:text-5xl capitalize tracking-widest text-gray-600 dark:text-white lg:text-6xl",
                        "Yousof site"
                    }
                    p { class: "mt-6 lg:text-lg text-gray-600 dark:text-white",
                        "Opinionated Dioxus Starter Template"
                    }
                    div { class: "mt-8 flex flex-col space-y-3 sm:-mx-2 sm:flex-row sm:justify-center sm:space-y-0",
                        input {
                            r#type: "text",
                            class: "input input-primary px-4 py-2 text-white placeholder-white backdrop-blur-sm focus:border-blue-400 focus:outline-none focus:ring focus:ring-blue-300 focus:ring-opacity-40 sm:mx-2",
                            placeholder: "What's your name ?",
                            oninput: move |e| input_name.set(e.value.clone()),
                            value: "{input_name}"
                        }
                        button {
                            id: "submit-button",
                            class: "btn btn-active btn-primary",
                            onclick: move |_| {
                                let name = input_name.get();
                                if !name.is_empty() {
                                    router.navigate_to(&format!("/hi/{name}"));
                                } else {
                                    toast.write().popup(ToastInfo::error("Empty input box", "Dioxus Toast"));
                                }
                            },
                            "Go"
                        }
                    }
                }
            }
            Footer {}
        }
    })
}

pub fn SayHi(cx: Scope) -> Element {
    let route = use_route(&cx);
    let name = route.segment("name").expect("Could not read name segment");
    let name = urlencoding::decode(name).expect("UTF-8").to_string();
    cx.render(rsx! {
        section { class: "h-screen bg-cover dark:bg-gray-600",
            div { class: "flex h-full w-full items-center justify-center container mx-auto px-8",
                div { class: "max-w-2xl text-center",
                    h1 { class: "text-3xl sm:text-5xl capitalize tracking-widest dark:text-white lg:text-6xl",
                        "Hi, "
                        name
                    }
                }
            }
            Footer {}
        }
    })
}

pub fn About(cx: Scope) -> Element {
    let content = include_str!("../markdown/readme.md");
    cx.render(rsx! {
        div { class: "h-screen dark:bg-gray-600",
            br {}
            div { class: "md:flex md:justify-center",
                div { class: "block p-8 rounded-lg shadow-2xl bg-white dark:bg-gray-700",
                    Markdown { class: "", content: content.to_string() }
                }
            }
            Footer {}
            div {class: "hidden toast toast-start flex flex-col-reverse",
                div {class: "alert alert-error alert-warning alert-info alert-success"}
            
    }
        }
    })
}
