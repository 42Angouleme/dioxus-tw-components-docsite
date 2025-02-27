use dioxus::prelude::*;
use dioxus_tw_components::atoms::icon::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn IconPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!(PreviewFull::<IconProps> {})
}

impl DemoComponent for IconProps {
    fn comp_introduction() -> &'static str {
        "A simple icon based on Google's Material font"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        let icons = vec![
            Icons::Dashboard,
            Icons::Menu,
            Icons::OpenInNew,
            Icons::People,
            Icons::Image,
            Icons::Tune,
            Icons::DisplaySettings,
            Icons::Discord,
            Icons::BlurOn,
            Icons::Person,
            Icons::CropFree,
            Icons::Edit,
            Icons::FileOpen,
            Icons::Cloud,
        ];

        rsx! {
            div { class: "flex flex-row md:flex-col space-x-4 md:space-y-4 md:space-x-0",
                div { class: "flex flex-col md:flex-row md:space-x-4",
                    for i in 0..(icons.len() / 2) {
                        Icon {
                            class: state.read()[&0].get_class(),
                            size: state.read()[&0].get_size(),
                            icon: icons[i]
                        }
                    }
                }
                div { class: "flex flex-col md:flex-row md:space-x-4",
                    for i in (icons.len() / 2)..icons.len() {
                        Icon {
                            class: state.read()[&0].get_class(),
                            size: state.read()[&0].get_size(),
                            icon: icons[i]
                        }
                    }
                }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            CompPreviewSelector::<IconProps> {
                index: 0,
                state,
                comp_props: IconProps::default(),
            }
        }
    }
}
