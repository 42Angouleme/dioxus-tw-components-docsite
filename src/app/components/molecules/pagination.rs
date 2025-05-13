use dioxus::prelude::*;
use dioxus_tw_components::molecules::pagination::*;
use dioxus_tw_components::atoms::{Button, ButtonVariant, button::ButtonProps};

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
        let page_number_outlined = use_signal(|| 1_usize);
        let page_number_ghost = use_signal(|| 1_usize);
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            div { class: "flex flex-col space-y-8 w-full h-fit",
                Pagination {
                    class: state.read()[&0].get_class(),
                    color: state.read()[&0].get_color(),
                    size: state.read()[&0].get_size(),
                    data_size: 1000_usize,
                    page_size: 25_usize,
                    page_number,
                }

                Pagination {
                    class: state.read()[&0].get_class(),
                    color: state.read()[&0].get_color(),
                    size: state.read()[&0].get_size(),
                    variant: ButtonVariant::Outline,
                    data_size: 1000_usize,
                    page_size: 25_usize,
                    page_number: page_number_outlined,
                }

                Pagination {
                    class: state.read()[&0].get_class(),
                    color: state.read()[&0].get_color(),
                    size: state.read()[&0].get_size(),
                    variant: ButtonVariant::Ghost,
                    data_size: 1000_usize,
                    page_size: 25_usize,
                    page_number: page_number_ghost,
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
