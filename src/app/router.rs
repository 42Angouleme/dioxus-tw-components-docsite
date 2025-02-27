use super::components::*;
use super::home::HomePage;
use super::layout::Layout;
use dioxus::prelude::*;
use dioxus_tw_components::atoms::Separator;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        HomePage {},
        #[nest("/components")]
            #[layout(SideBarComponent)]
                #[route("/:name")]
                ComponentPage {name: String},
            #[end_layout]
        #[end_nest]

        #[route("/:..route")]
        NotFound {route: Vec<String>}
}

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx!(
        div { class: "w-[70%]",
            h1 { class: "h1", "Page not found" }
            Separator { class: "mt-5" }
            div { class: "flex mt-20 items-center justify-center",
                div {
                    h2 { class: "h2 mb-3", "Something went wrong :(" }
                    p {
                        strong { "Houston we have a problem. " }
                        "We couldn't find that page."
                    }
                    p {
                        "Please check the URL or go back to "
                        Link { to: Route::HomePage {},
                            strong { "Home Page." }
                        }
                    }
                }
            }
        }
    )
}
