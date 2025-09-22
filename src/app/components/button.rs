use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::*};

pub fn ButtonPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<ButtonProps> {}
    }
}

impl DemoComponent for ButtonProps {
    fn comp_introduction() -> &'static str {
        "A simple yet customizable button"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            div { class: "flex flex-col sm:flex-row sm:space-x-4 space-y-4",
                Button {
                    class: state.read()[&0].get_class(),
                    "data-style": state.read()[&0].get_color().to_string(),
                    "data-size": state.read()[&0].get_size().to_string(),
                    "data-animation": state.read()[&0].get_animation().to_string(),
                    "data-variant":  state.read()[&0].get_variant().to_string(),
                    "Button"
                }
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
                    has_variant: true,
                    has_animation: true,
                    has_orientation: false,
                },
            }
        }
    }
}
