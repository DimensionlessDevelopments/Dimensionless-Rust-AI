// Dimensionless Developments Rust Ai
// # AI Research Agent - Frontend Library
// This module contains the Leptos frontend for the AI research agent.
// It only compiles for WASM targets.

pub mod app;
pub mod components;
pub mod model;

use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| {
        view! { <app::App/> }
    });
}