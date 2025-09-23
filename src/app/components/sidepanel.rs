use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn SidePanelPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default().class("p-2".to_string()));
        hash.insert(1, FieldPreview::default());
        hash.insert(2, FieldPreview::default());
        hash.insert(3, FieldPreview::default().class("!w-96".to_string()));
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<SidePanelProps> {}
    }
}

impl DemoComponent for SidePanelProps {
    fn comp_introduction() -> &'static str {
        "A simple side panel"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            SidePanel {
                SidePanelTrigger { class: state.read()[&0].get_class(), "OpenSidePanel" }
                SidePanelBackground {
                    class: state.read()[&1].get_class(),
                    "data-animation": state.read()[&1].get_animation().to_string(),
                }
                SidePanelContent {
                    class: state.read()[&3].get_class(),
                    "data-animation": state.read()[&3].get_animation().to_string(),
                    "data-side": state.read()[&3].get_side().to_string(),
                    div {
                        SidePanelClose {
                            class: state.read()[&2].get_class(),
                        }
                    }
                    div { class: "h4 mb-6", "Title" }
                    div {
                        class: "paragraph",
                        r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt
                        ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
                        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
                        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                        "#
                    }
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
                    has_size: false,
                    has_side: true,
                    has_variant: false,
                    has_animation: true,
                    has_orientation: false,
                },
            }
        }
    }
}
