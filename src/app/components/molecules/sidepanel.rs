use dioxus::prelude::*;
use dioxus_tw_components::molecules::sidepanel::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn SidePanelPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..3 {
            hash.insert(index, FieldPreview::default());
        }
        hash.insert(
            3,
            FieldPreview::default().class("max-w-full h-96".to_string()),
        );
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
                    color: state.read()[&1].get_color(),
                    animation: state.read()[&1].get_animation(),
                }
                SidePanelContent {
                    class: state.read()[&3].get_class(),
                    animation: state.read()[&3].get_animation(),
                    side: state.read()[&3].get_side(),
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
            CompPreviewSelector::<SidePanelTriggerProps> {
                index: 0,
                title: "Trigger".to_string(),
                state,
                comp_props: SidePanelTriggerProps::default(),
            }
            CompPreviewSelector::<SidePanelBackgroundProps> {
                index: 1,
                title: "Background".to_string(),
                state,
                comp_props: SidePanelBackgroundProps::default(),
            }
            CompPreviewSelector::<SidePanelCloseProps> {
                index: 2,
                title: "Close button".to_string(),
                state,
                comp_props: SidePanelCloseProps::default(),
            }
            CompPreviewSelector::<SidePanelContentProps> {
                index: 3,
                title: "Panel".to_string(),
                state,
                comp_props: SidePanelContentProps::default(),
            }
        }
    }
}
