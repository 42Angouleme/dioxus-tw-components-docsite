use dioxus::prelude::*;
use dioxus_tw_components::molecules::progressbar::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn ProgressBarPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        for index in 0..3 {
            hash.insert(index, FieldPreview::default());
        }
        Signal::new(hash)
    });

    rsx!(PreviewFull::<ProgressBarProps> {})
}

impl DemoComponent for ProgressBarProps {
    fn comp_introduction() -> &'static str {
        "Don't go so fast"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            div { class: "bg-muted w-96 h-fit p-4",
                ProgressBar {
                    class: state.read()[&0].get_class(),
                    color: state.read()[&0].get_color(),
                    size: state.read()[&0].get_size(),
                    ProgressBarInner {
                        class: state.read()[&1].get_class(),
                        color: state.read()[&1].get_color(),
                    }
                }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<ProgressBarProps> {
                index: 0,
                state,
                comp_props: ProgressBarProps::default(),
            }
            CompPreviewSelector::<ProgressBarInnerProps> {
                index: 1,
                state,
                comp_props: ProgressBarInnerProps::default(),
            }
        )
    }
}
