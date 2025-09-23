use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn ModalPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default().class("p-2".to_string()));
        hash.insert(1, FieldPreview::default());
        hash.insert(2, FieldPreview::default());
        hash.insert(3, FieldPreview::default().class("!w-64".to_string()));
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<ModalProps> {}
    }
}

impl DemoComponent for ModalProps {
    fn comp_introduction() -> &'static str {
        "A simple modal"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            Modal {
                ModalTrigger { class: state.read()[&0].get_class(), "OpenModal" }
                ModalBackground {
                    class: state.read()[&1].get_class(),
                    "data-style": state.read()[&1].get_color().to_string(),
                    "data-animation": state.read()[&1].get_animation().to_string(),
                }
                ModalContent {
                    class: state.read()[&3].get_class(),
                    "data-animation": state.read()[&3].get_animation().to_string(),
                    div {
                        ModalClose {
                            class: state.read()[&2].get_class(),
                        }
                    }
                    div { class: "h4", "TITLE" }
                    div { class: "paragraph", "CONTENT" }
                }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            CompPreviewSelector {
                index: 0,
                title: "Trigger".to_string(),
                state,
                comp_props: CompProps::default(),
            }
            CompPreviewSelector {
                index: 1,
                title: "Background".to_string(),
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
            CompPreviewSelector {
                index: 2,
                title: "Close button".to_string(),
                state,
                comp_props: CompProps::default(),
            }
            CompPreviewSelector {
                index: 3,
                title: "Content".to_string(),
                state,
                comp_props: CompProps {
                    has_color: false,
                    has_size: true,
                    has_side: false,
                    has_variant: false,
                    has_animation: true,
                    has_orientation: false,
                },
            }
        }
    }
}
