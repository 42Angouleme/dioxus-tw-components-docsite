use dioxus::prelude::*;
use dioxus_tw_components::form::input::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn InputPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(PreviewFull::<InputProps> {})
}

impl DemoComponent for InputProps {
    fn comp_introduction() -> &'static str {
        "A customizable and interactive input component"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(Input {
            class: state.read()[&0].get_class(),
            color: state.read()[&0].get_color(),
            size: state.read()[&0].get_size(),
            placeholder: "Input",
        })
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(CompPreviewSelector::<InputProps> {
            index: 0,
            state,
            comp_props: InputProps::default()
        })
    }
}
