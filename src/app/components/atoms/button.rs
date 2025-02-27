use dioxus::prelude::*;
use dioxus_tw_components::atoms::{button::ButtonProps, Button};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn ButtonPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(PreviewFull::<ButtonProps> {})
}

impl DemoComponent for ButtonProps {
    fn comp_introduction() -> &'static str {
        "A simple yet customizable button"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            Button {
                class: state.read()[&0].get_class(),
                color: state.read()[&0].get_color(),
                size: state.read()[&0].get_size(),
                animation: state.read()[&0].get_animation(),
                "Button"
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(CompPreviewSelector::<ButtonProps> {
            index: 0,
            state,
            comp_props: ButtonProps::default()
        })
    }
}
