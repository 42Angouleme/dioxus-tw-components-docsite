use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn MarkdownPage() -> Element {
    rsx! {
        PreviewFull::<MarkdownProps> {}
    }
}

impl DemoComponent for MarkdownProps {
    fn comp_introduction() -> &'static str {
        "A markdown renderer"
    }

    fn BuildCompPreview() -> Element {
        let theme_manager = use_context::<Signal<ThemeManager>>();
        let current_theme = theme_manager.read().current_theme;

        let content = use_signal(|| {
            String::from(
                r#"# Example
This is an [example](https://www.youtube.com/watch?v=dQw4w9WgXcQ) markdown [rendered by dioxus components](https://portfolio.kbz8.me/)"#,
            )
        });

        rsx! {
            div { class: "flex flex-col sm:space-x-4 space-y-4",
                Markdown {
                    class: format!("prose {} max-w-none bg-background rounded-radius p-4 border-border border-1 mx-auto", if current_theme != 0 { "prose-invert" } else { "" }),
                    markdown: "{content}",
                }
                TextArea {
                    class: "w-full min-h-[200px]",
                    placeholder: "Type markdown here",
                    value: content,
                }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        rsx! {}
    }
}
