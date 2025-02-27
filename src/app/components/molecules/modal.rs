use dioxus::prelude::*;
use dioxus_tw_components::molecules::modal::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn ModalPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..4 {
            hash.insert(index, FieldPreview::default());
        }
        Signal::new(hash)
    });

    rsx!(PreviewFull::<ModalProps> {})
}

impl DemoComponent for ModalProps {
    fn comp_introduction() -> &'static str {
        "A simple modal"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            Modal {
                ModalTrigger { class: state.read()[&0].get_class(), "OpenModal" }
                ModalBackground {
                    class: state.read()[&1].get_class(),
                    color: state.read()[&1].get_color(),
                    animation: state.read()[&1].get_animation(),
                }
                ModalContent { class: state.read()[&2].get_class(),
                    div { ModalClose {} }
                    div { class: "h4", "TITLE" }
                    div { class: "paragraph", "CONTENT" }
                }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<ModalTriggerProps> {
                index: 0,
                state,
                comp_props: ModalTriggerProps::default(),
            }
            CompPreviewSelector::<ModalBackgroundProps> {
                index: 1,
                state,
                comp_props: ModalBackgroundProps::default(),
            }
            CompPreviewSelector::<ModalCloseProps> { index: 2, state, comp_props: ModalCloseProps::default() }
            CompPreviewSelector::<ModalContentProps> {
                index: 3,
                state,
                comp_props: ModalContentProps::default(),
            }
        )
    }
}
