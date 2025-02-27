use dioxus::prelude::*;
use dioxus_tw_components::{atoms::Separator, molecules::dropdown::*};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn DropdownPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..3 {
            hash.insert(index, FieldPreview::default());
        }
        Signal::new(hash)
    });

    rsx!(PreviewFull::<DropdownProps> {})
}

impl DemoComponent for DropdownProps {
    fn comp_introduction() -> &'static str {
        "A dropdown to display a list of items"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            Dropdown { class: state.read()[&0].get_class(), id: "dropdown-demo",
                DropdownToggle {
                    class: state.read()[&1].get_class(),
                    id: "dropdown-toggle-demo",
                    "Dropdown"
                }
                DropdownContent {
                    id: "dropdown-content-demo",
                    class: state.read()[&2].get_class(),
                    animation: state.read()[&2].get_animation(),
                    div { "Content" }
                    div { "Content" }
                    Separator {}
                    div { "Content" }
                    div { "Content" }
                }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<DropdownProps> { index: 0, state, comp_props: DropdownProps::default() }
            CompPreviewSelector::<DropdownToggleProps> {
                index: 1,
                state,
                comp_props: DropdownToggleProps::default(),
            }
            CompPreviewSelector::<DropdownContentProps> {
                index: 2,
                state,
                comp_props: DropdownContentProps::default(),
            }
        )
    }
}
