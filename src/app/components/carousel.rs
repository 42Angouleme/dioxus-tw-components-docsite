use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn CarouselPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default().class("max-w-96".to_string()));
        hash.insert(
            1,
            FieldPreview::default().class("bg-primary/60".to_string()),
        );
        hash.insert(
            2,
            FieldPreview::default()
                .class("bg-gradient-to-r from-primary/60 to-destructive/60".to_string()),
        );
        hash.insert(
            3,
            FieldPreview::default().class("bg-destructive/60".to_string()),
        );
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<CarouselProps> {}
    }
}

impl DemoComponent for CarouselProps {
    fn comp_introduction() -> &'static str {
        "A carousel to display multiple items and navigate through them"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            Carousel {
                class: state.read()[&0].get_class(),
                is_circular: true,
                autoscroll_duration: 1000,
                CarouselTrigger { next: false }
                CarouselWindow {
                    CarouselContent { id: "carousel-prev", class: "h-32",
                        CarouselItem { item_key: 0, class: state.read()[&1].get_class(),
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 1"
                            }
                        }
                        CarouselItem { item_key: 1, class: state.read()[&2].get_class(),
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 2"
                            }
                        }
                        CarouselItem { item_key: 2, class: state.read()[&3].get_class(),
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 3"
                            }
                        }
                    }
                }
                CarouselTrigger { next: true }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            CompPreviewSelector {
                index: 0,
                title: "Carousel".to_string(),
                state,
                comp_props: CompProps::default(),
            }
            CompPreviewSelector {
                index: 1,
                title: "First Item".to_string(),
                state,
                comp_props: CompProps::default(),
            }
            CompPreviewSelector {
                index: 2,
                title: "Second Item".to_string(),
                state,
                comp_props: CompProps::default(),
            }
            CompPreviewSelector {
                index: 3,
                title: "Third Item".to_string(),
                state,
                comp_props: CompProps::default(),
            }
        }
    }
}
