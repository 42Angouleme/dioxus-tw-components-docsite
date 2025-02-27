#![allow(non_snake_case)]

use dioxus::prelude::*;

mod app;

fn main() {
    dioxus::launch(crate::app::App);
}
