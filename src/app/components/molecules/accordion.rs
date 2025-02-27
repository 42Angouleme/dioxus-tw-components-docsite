use dioxus::prelude::*;
use dioxus_tw_components::molecules::accordion::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn AccordionPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default().class("w-96".to_string()));
        for index in 1..4 {
            hash.insert(index, FieldPreview::default());
        }
        Signal::new(hash)
    });

    rsx!(PreviewFull::<AccordionProps> {})
}

impl DemoComponent for AccordionProps {
    fn comp_introduction() -> &'static str {
        "Up and down !"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            Accordion { class: state.read()[&0].get_class(),
                AccordionItem {
                    AccordionTrigger { id: "accordion-1", "Products" }
                    AccordionContent {
                        class: state.read()[&1].get_class(),
                        animation: state.read()[&1].get_animation(),
                        id: "accordion-1",
                        p { "Check out our latest products!" }
                    }
                }
                AccordionItem {
                    AccordionTrigger { id: "accordion-2", "Services" }
                    AccordionContent {
                        class: state.read()[&2].get_class(),
                        animation: state.read()[&2].get_animation(),
                        id: "accordion-2",
                        p { "Discover our range of services." }
                    }
                }
                AccordionItem {
                    AccordionTrigger { id: "accordion-3", "Testimonials" }
                    AccordionContent {
                        class: state.read()[&3].get_class(),
                        animation: state.read()[&3].get_animation(),
                        id: "accordion-3",
                        p {
                            "I've been a customer for over a year now and I'm extremely satisfied with their services. The team is always responsive and goes above and beyond to ensure my needs are met. Their attention to detail is impressive and I highly recommend them to anyone looking for top-notch service."
                        }
                    }
                }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<AccordionProps> { index: 0, state, comp_props: AccordionProps::default() }
            CompPreviewSelector::<AccordionContentProps> {
                index: 1,
                state,
                comp_props: AccordionContentProps::default(),
            }
            CompPreviewSelector::<AccordionContentProps> {
                index: 2,
                state,
                comp_props: AccordionContentProps::default(),
            }
            CompPreviewSelector::<AccordionContentProps> {
                index: 3,
                state,
                comp_props: AccordionContentProps::default(),
            }
        )
    }
}
