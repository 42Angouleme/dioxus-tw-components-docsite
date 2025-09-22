use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn HoverCardPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        hash.insert(1, FieldPreview::default());
        hash.insert(2, FieldPreview::default());
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<HoverCardProps> {}
    }
}

impl DemoComponent for HoverCardProps {
    fn comp_introduction() -> &'static str {
        "A card that appears when hovering over an element"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            HoverCard { id: "hover-card-demo", class: state.read()[&0].get_class(),
                HoverCardTrigger {
                    id: "hover-card-trigger-demo",
                    class: state.read()[&1].get_class(),
                    "Hover me"
                }
                HoverCardContent {
                    id: "hover-card-content-demo",
                    class: state.read()[&2].get_class(),
                    "data-animation": state.read()[&2].get_animation().to_string(),
                    div { "Content" }
                }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            CompPreviewSelector {
                index: 0,
                title: "Hover Card".to_string(),
                state,
                comp_props: CompProps::default(),
            }
            CompPreviewSelector {
                index: 1,
                title: "Trigger".to_string(),
                state,
                comp_props: CompProps::default(),
            }
            CompPreviewSelector {
                index: 2,
                title: "Content".to_string(),
                state,
                comp_props: CompProps {
                    has_color: false,
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
