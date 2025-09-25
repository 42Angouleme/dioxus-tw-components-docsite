use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn PaginationPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default().class("px-2 py-1".to_string()));
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<PaginationProps> {}
    }
}

impl DemoComponent for PaginationProps {
    fn comp_introduction() -> &'static str {
        "Don't miss anything"
    }

    fn BuildCompPreview() -> Element {
        let page_number = use_signal(|| 1_usize);
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            div { class: "flex flex-col space-y-8 w-full h-fit",
                Pagination {
                    class: state.read()[&0].get_class(),
                    "data-style": state.read()[&0].get_color().to_string(),
                    "data-variant": state.read()[&0].get_variant().to_string(),
                    data_size: 1000_usize,
                    page_size: 25_usize,
                    page_number,
                }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();
        rsx! {
            CompPreviewSelector {
                index: 0,
                title: "Pagination component".to_string(),
                state,
                comp_props: CompProps {
                    has_color: true,
                    has_size: false,
                    has_side: false,
                    has_variant: true,
                    has_animation: false,
                    has_orientation: false,
                },
            }
        }
    }
}
