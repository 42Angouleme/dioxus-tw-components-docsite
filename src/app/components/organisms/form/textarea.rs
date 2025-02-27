use dioxus::prelude::*;
use dioxus_tw_components::form::textarea::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn TextAreaPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(PreviewFull::<TextAreaProps> {})
}

impl DemoComponent for TextAreaProps {
    fn comp_introduction() -> &'static str {
        "A customizable and interactive textarea component"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(TextArea {
            class: state.read()[&0].get_class(),
            color: state.read()[&0].get_color(),
        })
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(CompPreviewSelector::<TextAreaProps> {
            index: 0,
            state,
            comp_props: TextAreaProps::default()
        })
    }
}
