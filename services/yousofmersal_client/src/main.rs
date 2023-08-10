#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_toast::{ToastFrame, ToastManager};

mod components;
mod hooks;
mod pages;

use dioxus_router::prelude::*;
use fermi::{use_atom_ref, use_init_atom_root, AtomRef};
use hooks::mode::init_mode_info;
use pages::{
    starter::{About, HelloDioxus, SayHi},
    _404::NotFound,
};

use crate::components::{footer::Footer, navbar::Navbar};

static TOAST_MANAGER: fermi::AtomRef<ToastManager> = AtomRef(|_| ToastManager::default());

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[layout(NavFooter)]
        #[route("/")]
        HelloDioxus {},
        #[route("/hi/:name")]
        SayHi {name: String},
        #[route("/about")]
        About {},
    #[end_layout]
    #[route("/:..segments")]
    NotFound {segments: Vec<String>}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App)
}

fn NavFooter(cx: Scope) -> Element {
    render! {
        Navbar {}
        Outlet::<Route> {}
        Footer {}
    }
}

fn App(cx: Scope) -> Element {
    // init mode information
    init_mode_info(&cx);
    use_init_atom_root(cx);

    // TODO: fix toast manager
    render! {
    //     ToastFrame { manager: use_atom_ref(cx, TOAST_MANAGER) }
        Router::<Route> {}
    }

    // render! {
    //     // dioxus toast manager init
    //     ToastFrame { manager: use_atom_ref(cx, TOAST_MANAGER) }
    //     // dioxus router info
    // }
}
