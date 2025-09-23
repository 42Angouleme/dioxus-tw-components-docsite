use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn PlaceholderPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<PlaceholderProps> {}
    }
}

impl DemoComponent for PlaceholderProps {
    fn comp_introduction() -> &'static str {
        "A customizable and versatile placeholder for text, images, or other content"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            Placeholder {
                class: state.read()[&0].get_class(),
                "data-style": state.read()[&0].get_color().to_string(),
                "data-animation": state.read()[&0].get_animation().to_string(),
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
                    has_size: false,
                    has_side: false,
                    has_variant: false,
                    has_animation: true,
                    has_orientation: false,
                },
            }
        }
    }
}
