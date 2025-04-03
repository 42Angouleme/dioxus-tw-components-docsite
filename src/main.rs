#![allow(non_snake_case)]

mod app;

fn main() {
    dioxus::launch(crate::app::App);
}
