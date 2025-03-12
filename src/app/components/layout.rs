use dioxus::prelude::*;
use dioxus_tw_components::{atoms::Separator, molecules::sidepanel::*, prelude::Side};

use crate::app::router::Route;

#[component]
pub fn SideBarComponent() -> Element {
    // Empty strings are there to render a Separator
    let components = vec![
        "Button",
        "Buttongroup",
        "Icon",
        "Placeholder",
        "",
        "Accordion",
        "Breadcrumb",
        "Callout",
        "Carousel",
        "Dropdown",
        "HoverCard",
        "LightSwitch",
        "Modal",
        "ProgressBar",
        "Scrollable",
        "SidePanel",
        "SortedTable",
        "Table",
        "Tabs",
        "Toast",
        "",
        "Checkbox",
        "FormList",
        "Input",
        "Radio",
        "Select",
        "Slider",
        "Textarea",
        "Toggle",
    ];

    rsx! {
        div {
            id: "component-div",
            class: "container w-full flex flex-col md:flex-row md:space-x-10 mb-12",
            aside {
                id: "components-list-link",
                class: "hidden md:block bg-background/50 backdrop-blur-sm border-border border-1 rounded-global-radius p-4 w-48 space-y-2",
                for component in components.clone() {
                    if component.is_empty() {
                        Separator {}
                    } else {
                        p { class: "anchor",
                            Link { to: format!("/components/{}", component.to_lowercase()),
                                {component}
                            }
                        }
                    }
                }
            }
            SidePanel {
                div { class: "flex justify-center mb-12",
                    SidePanelTrigger { class: "md:hidden w-fit", "Select component" }
                }
                SidePanelBackground { class: "md:hidden opacity-15" }
                SidePanelContent {
                    class: "md:hidden h-full w-full p-0",
                    side: Side::Right,
                    SidePanelClose {}
                    div { class: "overflow-y-auto mt-8 py-4 space-y-2 ",
                        for component in components.clone() {
                            if component.is_empty() {
                                Separator { class: "mx-auto w-64" }
                            } else {
                                p { class: "anchor text-center",
                                    Link {
                                        to: format!("/components/{}", component.to_lowercase()),
                                        onclick: move |_| {
                                            let mut state = use_context::<Signal<SidePanelState>>();
                                            state.write().toggle();
                                        },
                                        {component}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { id: "component-main", class: "px-8 w-full", Outlet::<Route> {} }
        }
    }
}
