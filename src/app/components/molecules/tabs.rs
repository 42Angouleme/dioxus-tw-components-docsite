use dioxus::prelude::*;
use dioxus_tw_components::molecules::tabs::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn TabsPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default().class("w-96".to_string()).clone());
        Signal::new(hash)
    });

    rsx! {
        PreviewFull::<TabsProps> {}
    }
}

impl DemoComponent for TabsProps {
    fn comp_introduction() -> &'static str {
        "A customizable and user-friendly navigation component that allows users to switch between different sections"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            div { class: "min-h-64 items-start",
                Tabs {
                    default_tab: "tabs-0",
                    class: state.read()[&0].get_class(),
                    TabsList {
                        TabsTrigger { id: "tabs-0", "Home" }
                        TabsTrigger { id: "tabs-1", "About" }
                        TabsTrigger { id: "tabs-2", "Contact" }
                    }
                    TabsContent { id: "tabs-0", class: "space-y-4",
                        h4 { class: "h4 text-foreground", "Welcome to our home page!" }
                        p { class: "paragraph text-foreground",
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl. Nulla facilisi."
                        }
                    }
                    TabsContent { id: "tabs-1", class: "space-y-4",
                        h4 { class: "h4 text-foreground", "Learn more about us here." }
                        p { class: "paragraph text-foreground",
                            "Vivamus eget nisl velit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl. Nulla facilisi."
                        }
                    }
                    TabsContent { id: "tabs-2", class: "space-y-4",
                        h4 { class: "h4 text-foreground", "Get in touch with us using the form below." }
                        p { class: "paragraph text-foreground",
                            "Praesent eget nisl velit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl. Nulla facilisi."
                        }
                    }
                }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx! {
            CompPreviewSelector::<TabsProps> { index: 0, state, comp_props: TabsProps::default() }
        }
    }
}
