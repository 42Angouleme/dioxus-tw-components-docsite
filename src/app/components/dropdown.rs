use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn DropdownPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        hash.insert(1, FieldPreview::default().class("p-2".to_string()));
        hash.insert(2, FieldPreview::default());
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<DropdownProps> {}
    }
}

impl DemoComponent for DropdownProps {
    fn comp_introduction() -> &'static str {
        "A dropdown to display a list of items"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            Dropdown { class: state.read()[&0].get_class(), id: "dropdown-demo",
                DropdownToggle {
                    class: state.read()[&1].get_class(),
                    id: "dropdown-toggle-demo",
                    "Dropdown"
                }
                DropdownContent {
                    id: "dropdown-content-demo",
                    class: state.read()[&2].get_class(),
                    "data-animation": state.read()[&2].get_animation().to_string(),
                    div { "Content" }
                    div { "Content" }
                    Separator {}
                    div { "Content" }
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
                title: "Dropdown".to_string(),
                state,
                comp_props: CompProps::default(),
            }
            CompPreviewSelector {
                index: 1,
                title: "Toggle".to_string(),
                state,
                comp_props: CompProps{
                    has_color: false,
                    has_size: false,
                    has_side: false,
                    has_variant: false,
                    has_animation: true,
                    has_orientation: false,
                },
            }
            CompPreviewSelector {
                index: 2,
                title: "Content".to_string(),
                state,
                comp_props: CompProps{
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
