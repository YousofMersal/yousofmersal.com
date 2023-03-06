#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_material_icons::MaterialIconStylesheet;
use dioxus_toast::{ToastFrame, ToastManager};

mod components;
mod hooks;
mod pages;

use hooks::mode::init_mode_info;
use pages::starter::{About, HelloDioxus, SayHi};
use dioxus_router::{Route, Router};
use fermi::{use_atom_ref,  use_init_atom_root};

use crate::components::navbar::Navbar;

static TOAST_MANAGER: fermi::AtomRef<ToastManager> = |_| ToastManager::default();

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Dioxus Starter: https://github.com/mrxiaozhuox/dioxus-starter");
    dioxus_web::launch(App)
}

fn App(cx: Scope) -> Element {
    // init mode information
    init_mode_info(&cx);
    use_init_atom_root(cx);

    render!{
        // dioxus toast manager init
        ToastFrame { manager: use_atom_ref(cx, TOAST_MANAGER), maximum: 6 }
        // to use material icons
        MaterialIconStylesheet {}
        // dioxus router info
        Router { 
            Navbar {}
            Route { to: "/", HelloDioxus {} }
            Route { to: "/hi/:name", SayHi {} }
            Route { to: "/about", About {} }
            // // 404 page
            Route { to: "", pages::_404::NotFound {} }
        }
    }
}
