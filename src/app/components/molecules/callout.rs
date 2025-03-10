use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;
use dioxus_tw_components::molecules::callout::{CalloutProps, Callout, CalloutVariant};

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn CalloutPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default());
        Signal::new(hash)
    });

    rsx!{
        PreviewFull::<CalloutProps> {}
    }
}

impl DemoComponent for CalloutProps {
    fn comp_introduction() -> &'static str {
        "A callout system"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!{
            div { class: "flex flex-col space-y-4 w-2/3 mt-6",
                Callout {
                    class: state.read()[&0].get_class(),
                    title: "Note",
                    variant: CalloutVariant::Note,
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl"
                }

                Callout {
                    class: state.read()[&0].get_class(),
                    title: "Tip",
                    variant: CalloutVariant::Tip,
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl"
                }

                Callout {
                    class: state.read()[&0].get_class(),
                    title: "Warning",
                    variant: CalloutVariant::Warning,
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl"
                }

                Callout {
                    class: state.read()[&0].get_class(),
                    title: "Caution",
                    variant: CalloutVariant::Caution,
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl"
                }

                Callout {
                    class: state.read()[&0].get_class(),
                    title: "Custom callout with custom icon",
                    icon: Icons::LunchDining,
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl"
                }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!{
            CompPreviewSelector::<CalloutProps> {
                index: 0,
                state,
                comp_props: CalloutProps::default()
            }
        }
    }
}
