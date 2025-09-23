use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn InputPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<InputProps> {}
    }
}

impl DemoComponent for InputProps {
    fn comp_introduction() -> &'static str {
        "A customizable and interactive input component"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            Input {
                class: state.read()[&0].get_class(),
                "data-style": state.read()[&0].get_color().to_string(),
                "data-size": state.read()[&0].get_size().to_string(),
                placeholder: "Input",
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            CompPreviewSelector {
                index: 0,
                title: None,
                state,
                comp_props: CompProps {
                    has_color: true,
                    has_size: true,
                    has_side: false,
                    has_variant: false,
                    has_animation: false,
                    has_orientation: false,
                },
            }
        }
    }
}
