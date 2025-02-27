use dioxus::prelude::*;
use dioxus_tw_components::molecules::breadcrumb::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn BreadcrumbPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..6 {
            hash.insert(index, FieldPreview::default());
        }
        Signal::new(hash)
    });

    rsx!(PreviewFull::<BreadcrumbProps> {})
}

impl DemoComponent for BreadcrumbProps {
    fn comp_introduction() -> &'static str {
        "You left some on your way here"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            Breadcrumb { class: state.read()[&0].get_class(),
                BreadcrumbItem { class: state.read()[&1].get_class(), "Home" }
                BreadcrumbSeparator { class: state.read()[&2].get_class() }
                BreadcrumbItem { class: state.read()[&3].get_class(), "Library" }
                BreadcrumbSeparator { class: state.read()[&4].get_class() }
                BreadcrumbItem { class: state.read()[&5].get_class(), "Data" }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        return rsx! {}; // Empty element
    }
}
