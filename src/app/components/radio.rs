use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn RadioPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        hash.insert(1, FieldPreview::default());
        hash.insert(2, FieldPreview::default());
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<RadioGroupProps> {}
    }
}

impl DemoComponent for RadioGroupProps {
    fn comp_introduction() -> &'static str {
        "A customizable and interactive radio component"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            RadioGroup {
                for i in 0..3 {
                    RadioItem { value: i.to_string(),
                        class: state.read()[&i].get_class(),
                        "data-style": state.read()[&i].get_color().to_string(),
                    }
                }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            for index in 0..3 {
                CompPreviewSelector {
                    index,
                    title: format!("Radio {index}"),
                    state,
                    comp_props: CompProps {
                        has_color: true,
                        has_size: false,
                        has_side: false,
                        has_variant: false,
                        has_animation: false,
                        has_orientation: false,
                    },
                }
            }
        }
    }
}
