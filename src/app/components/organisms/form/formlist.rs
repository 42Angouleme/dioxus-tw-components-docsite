use dioxus::prelude::*;
use dioxus_tw_components::{
    atoms::{Button, Separator},
    form::*,
};
use formlist::FormListProps;

use crate::app::{components::preview::*, doctrait::DemoComponent};

#[component]
pub fn FormListPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(PreviewFull::<FormListProps> {})
}

impl DemoComponent for FormListProps {
    fn comp_introduction() -> &'static str {
        "WIP"
    }

    fn BuildCompPreview() -> Element {
        let elem_to_repeat = rsx!(
            div { class: "space-y-2 mx-1",
                "Field"
                Input {}
                Input {}
                Separator { class: "" }
            }
        );

        let list_fields = (0..6)
            .map(|_| elem_to_repeat.clone())
            .collect::<Vec<Element>>();

        let button_class = "w-10";

        rsx!(
            FormList {
                FormListTriggerMinus {
                    Button { class: button_class, "-" }
                }
                FormListTriggerPlus {
                    Button { class: button_class, "+" }
                }
                FormListContent { list_fields }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        rsx!()
    }
}
