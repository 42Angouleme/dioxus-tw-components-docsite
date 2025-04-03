use dioxus::prelude::*;
use dioxus_tw_components::molecules::pagination::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn PaginationPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..2 {
            hash.insert(index, FieldPreview::default());
        }
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

        rsx! {
            div { class: "border rounded-global-radius w-full h-fit",
                Pagination {
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
            CompPreviewSelector::<PaginationProps> {
                index: 0,
                title: "Pagination component".to_string(),
                state,
                comp_props: PaginationProps::default(),
            }
        }
    }
}
