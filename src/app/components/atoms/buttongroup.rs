use dioxus::prelude::*;
use dioxus_tw_components::atoms::buttongroup::{
    ButtonGroup, ButtonGroupItem, ButtonGroupItemProps, ButtonGroupProps,
};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn ButtonGroupPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(PreviewFull::<ButtonGroupProps> {})
}

impl DemoComponent for ButtonGroupProps {
    fn comp_introduction() -> &'static str {
        "A simple customizable group of buttons"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            ButtonGroup {
                class: state.read()[&0].get_class(),
                color: state.read()[&0].get_color(),
                size: state.read()[&0].get_size(),
                animation: state.read()[&0].get_animation(),
                ButtonGroupItem { "Button 1" }
                ButtonGroupItem { "Button 2" }
                ButtonGroupItem { "Button 3" }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(CompPreviewSelector::<ButtonGroupProps> {
            index: 0,
            state,
            comp_props: ButtonGroupProps::default(),
        })
    }
}
