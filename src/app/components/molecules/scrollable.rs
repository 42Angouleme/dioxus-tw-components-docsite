use dioxus::prelude::*;
use dioxus_tw_components::{atoms::Separator, attributes::Orientation, molecules::scrollable::*};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn ScrollablePage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(
            0,
            FieldPreview::default()
                .class("h-72".to_string())
                .orientation(Orientation::Vertical),
        );
        Signal::new(hash)
    });

    rsx!(PreviewFull::<ScrollableProps> {})
}

impl DemoComponent for ScrollableProps {
    fn comp_introduction() -> &'static str {
        "WIP"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            Scrollable {
                class: state.read()[&0].get_class(),
                orientation: state.read()[&0].get_orientation(),
                div { class: "flex flex-col space-y-2 text-sm font-medium",
                    p { class: "paragraph", "About Us" }
                    p { class: "paragraph", "Our Mission" }
                    p { class: "paragraph", "Our Team" }
                    Separator {}
                    p { class: "paragraph", "Services" }
                    p { class: "paragraph", "Service 1" }
                    p { class: "paragraph", "Service 2" }
                    p { class: "paragraph", "Service 3" }
                    p { class: "paragraph", "Products" }
                    p { class: "paragraph", "Product 1" }
                    p { class: "paragraph", "Product 2" }
                    p { class: "paragraph", "Product 3" }
                    Separator {}
                    p { class: "paragraph", "Testimonials" }
                    p { class: "paragraph", "Testimonial 1" }
                    p { class: "paragraph", "Testimonial 2" }
                    p { class: "paragraph", "Testimonial 3" }
                    Separator {}
                    p { class: "paragraph", "Contact Us" }
                    p { class: "paragraph", "Email: info@example.com" }
                    p { class: "paragraph", "Phone: +1234567890" }
                    p { class: "paragraph", "Address: 123 Main St, City, Country" }
                }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(CompPreviewSelector::<ScrollableProps> {
            index: 0,
            state,
            comp_props: ScrollableProps::default()
        })
    }
}
