use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::*};

pub fn ToastPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<Toast> {}
    }
}

impl From<Color> for ToastColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Default => ToastColor::Default,
            Color::Primary => ToastColor::Primary,
            Color::Secondary => ToastColor::Secondary,
            Color::Destructive => ToastColor::Destructive,
            Color::Success => ToastColor::Success,
            _ => ToastColor::Default,
        }
    }
}


impl From<Animation> for ToastAnimation {
    fn from(animation: Animation) -> Self {
        match animation {
            Animation::Full => ToastAnimation::Full,
            Animation::Light => ToastAnimation::Light,
            _ => ToastAnimation::None,
        }
    }
}

impl DemoComponent for Toast {
    fn comp_introduction() -> &'static str {
        "A short, temporary message that pops up on the user interface to provide feedback or information."
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        let mut toast = use_toast();

        rsx! {
            Button { class: "px-2 py-1",
                onclick: move |_| {
                    toast
                        .title("Title")
                        .description(rsx! {
                            div { "Content" }
                        })
                        .color(ToastColor::from(state.read()[&0].get_color()))
                        .animation(ToastAnimation::from(state.read()[&0].get_animation()))
                        .render();
                },
                "Toasting"
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            CompPreviewSelector {
                index: 0,
                title: None,
                state,
                comp_props: CompProps {
                    has_color: true,
                    has_size: false,
                    has_side: false,
                    has_variant: false,
                    has_animation: true,
                    has_orientation: false,
                },
            }
        }
    }
}
