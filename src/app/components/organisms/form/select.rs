use dioxus::prelude::*;
use dioxus_tw_components::form::select::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn SelectPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(PreviewFull::<SelectGroupProps> {})
}

impl DemoComponent for SelectGroupProps {
    fn comp_introduction() -> &'static str {
        "Interactive dropdown list that allows users to select an option. This component provides a user-friendly way to choose from a list of options."
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            SelectGroup { class: state.read()[&0].get_class(),
                SelectPlaceholder { "Select an option" }
                SelectLabel { label: "Label 1" }
                SelectItem { "Option 1" }
                SelectItem { "Option 2" }
                SelectItem { "Option 3" }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(CompPreviewSelector::<SelectGroupProps> {
            index: 0,
            state,
            comp_props: SelectGroupProps::default(),
        })
    }
}
