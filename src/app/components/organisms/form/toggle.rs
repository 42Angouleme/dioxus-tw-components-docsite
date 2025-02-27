use dioxus::prelude::*;
use dioxus_tw_components::form::toggle::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn TogglePage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(PreviewFull::<ToggleProps> {})
}

impl DemoComponent for ToggleProps {
    fn comp_introduction() -> &'static str {
        "A customizable and interactive toggle component that allows users to switch between two states"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(Toggle {
            class: state.read()[&0].get_class(),
            color: state.read()[&0].get_color(),
            size: state.read()[&0].get_size(),
            animation: state.read()[&0].get_animation(),
        })
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(CompPreviewSelector::<ToggleProps> {
            index: 0,
            state,
            comp_props: ToggleProps::default()
        })
    }
}
