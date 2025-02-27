use dioxus::prelude::*;
use dioxus_tw_components::molecules::sidepanel::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn SidePanelPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..3 {
            hash.insert(index, FieldPreview::default());
        }
        Signal::new(hash)
    });

    rsx!(PreviewFull::<SidePanelProps> {})
}

impl DemoComponent for SidePanelProps {
    fn comp_introduction() -> &'static str {
        "A simple side panel"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            SidePanel {
                SidePanelTrigger { class: state.read()[&0].get_class(), "OpenSidePanel" }
                SidePanelBackground {
                    class: state.read()[&1].get_class(),
                    color: state.read()[&1].get_color(),
                    animation: state.read()[&1].get_animation(),
                }
                SidePanelContent {
                    class: state.read()[&2].get_class(),
                    animation: state.read()[&2].get_animation(),
                    side: state.read()[&2].get_side(),
                    div { SidePanelClose {} }
                    div { class: "h4", "TITLE" }
                    div { class: "paragraph", "CONTENT" }
                }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<SidePanelTriggerProps> {
                index: 0,
                state,
                comp_props: SidePanelTriggerProps::default(),
            }
            CompPreviewSelector::<SidePanelBackgroundProps> {
                index: 1,
                state,
                comp_props: SidePanelBackgroundProps::default(),
            }
            CompPreviewSelector::<SidePanelContentProps> {
                index: 2,
                state,
                comp_props: SidePanelContentProps::default(),
            }
        )
    }
}
