use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::router::Route;

#[component]
pub fn SideBarComponent() -> Element {
    let components = vec![
        "Accordion",
        "Button",
        "Buttongroup",
        "Callout",
        "Carousel",
        "Checkbox",
        "Dropdown",
        "FormList",
        "HoverCard",
        "Icon",
        "Input",
        "LightSwitch",
        "Modal",
        "Pagination",
        "Placeholder",
        "ProgressBar",
        "Radio",
        "Scrollable",
        "Select",
        "SidePanel",
        "Slider",
        "SortedTable",
        "Table",
        "Tabs",
        "Textarea",
        "Toast",
        "Toggle",
    ];

    rsx! {
        div {
            id: "component-div",
            class: "container w-full flex flex-col md:flex-row md:space-x-10 mb-12",
            aside {
                id: "components-list-link",
                class: "hidden md:block bg-background/50 backdrop-blur-sm border-border border-1 rounded-radius p-4 w-48 space-y-2",
                for component in components.clone() {
                    p { class: "anchor",
                        Link { to: format!("/components/{}", component.to_lowercase()),
                            {component}
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
                    class: "md:hidden !h-3/4 !w-3/4 p-0 overflow-y-auto",
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
