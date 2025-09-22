use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn ButtonGroupPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for i in 0..3 {
            hash.insert(i, FieldPreview::default());
        }
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<ButtonGroupProps> {}
    }
}

impl DemoComponent for ButtonGroupProps {
    fn comp_introduction() -> &'static str {
        "A simple customizable group of buttons"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            ButtonGroup {
                for i in 0..3 {
                    Button {
                        class: state.read()[&i].get_class(),
                        "data-style": state.read()[&i].get_color().to_string(),
                        "data-size": state.read()[&i].get_size().to_string(),
                        "data-animation": state.read()[&i].get_animation().to_string(),
                        "data-variant":  state.read()[&i].get_variant().to_string(),
                        "Button"
                    }
                }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            for i in 0..3 {
                CompPreviewSelector {
                    index: i,
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
}
