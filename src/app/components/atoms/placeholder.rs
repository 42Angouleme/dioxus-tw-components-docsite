use dioxus::prelude::*;
use dioxus_tw_components::atoms::{placeholder::PlaceholderProps, Placeholder};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn PlaceholderPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(PreviewFull::<PlaceholderProps> {})
}

impl DemoComponent for PlaceholderProps {
    fn comp_introduction() -> &'static str {
        "A customizable and versatile placeholder for text, images, or other content"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(Placeholder {
            class: state.read()[&0].get_class(),
            color: state.read()[&0].get_color(),
            animation: state.read()[&0].get_animation(),
        })
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(CompPreviewSelector::<PlaceholderProps> {
            index: 0,
            state,
            comp_props: PlaceholderProps::default(),
        })
    }
}
