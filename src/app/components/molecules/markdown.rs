use dioxus::prelude::*;
use dioxus_tw_components::{
    form::TextArea,
    molecules::markdown::{Markdown, MarkdownProps},
};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn MarkdownPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<MarkdownProps> {}
    }
}

impl DemoComponent for MarkdownProps {
    fn comp_introduction() -> &'static str {
        "Markdown renderer"
    }

    fn BuildCompPreview() -> Element {
        let _state = use_context::<Signal<HashPreview>>();
        let mut content = use_signal(|| String::from("# Hello World"));

        rsx! {
            div { class: "flex flex-col sm:space-x-4 space-y-4",
                h3 { class: "h3 text-destructive", "Use dangerous inner HTML" }
                Markdown {
                    content: "{content}",
                }
                TextArea {
                    class: "w-full min-h-[200px]",
                    value: "{content.read()}",
                    oninput: move |e: FormEvent| {
                        content.set(e.value());
                    },
                    placeholder: "# Hello World",
                }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            CompPreviewSelector::<MarkdownProps> {
                index: 0,
                title: None,
                state,
                comp_props: MarkdownProps::default()
            }
        }
    }
}
