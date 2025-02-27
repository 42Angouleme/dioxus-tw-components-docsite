use dioxus::prelude::*;
use dioxus_tw_components::form::radio::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn RadioPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        hash.insert(1, FieldPreview::default());
        hash.insert(2, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(PreviewFull::<RadioProps> {})
}

impl DemoComponent for RadioProps {
    fn comp_introduction() -> &'static str {
        "A customizable and interactive radio component"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            div { class: "flex space-x-2",
                for i in 0..3 {
                    div { class: "flex flex-col items-center space-y-2",
                        p { class: "text-foreground font-bold", "{i}" }
                        Radio {
                            class: state.read()[&i].get_class(),
                            color: state.read()[&i].get_color(),
                            size: state.read()[&i].get_size(),
                        }
                    }
                }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(for index in 0..3 {
            CompPreviewSelector::<RadioProps> {
                index,
                state,
                comp_props: RadioProps::default(),
            }
        })
    }
}
