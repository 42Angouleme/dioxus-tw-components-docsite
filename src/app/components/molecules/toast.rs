use dioxus::prelude::*;
use dioxus_tw_components::{
    atoms::Button,
    molecules::{toast::use_toast, Toast, ToastRenderer},
};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn ToastPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(PreviewFull::<Toast> {})
}

impl DemoComponent for Toast {
    fn comp_introduction() -> &'static str {
        "A short, temporary message that pops up on the user interface to provide feedback or information."
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        let mut toast = use_toast();

        rsx!(
            Button {
                onclick: move |_| {
                    toast
                        .title("Title")
                        .description(rsx! {
                            div { "Content" }
                        })
                        .color(state.read()[&0].get_color())
                        .animation(state.read()[&0].get_animation())
                        .render();
                },
                "Toasting"
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(CompPreviewSelector::<Toast> {
            index: 0,
            state,
            comp_props: Toast::default()
        })
    }
}
