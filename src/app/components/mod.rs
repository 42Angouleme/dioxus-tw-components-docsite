pub mod accordion;
pub mod button;
pub mod buttongroup;
pub mod icon;
//pub mod placeholder;
pub mod callout;
pub mod carousel;
pub mod dropdown;
pub mod hovercard;
//pub mod lightswitch;
//pub mod modal;
//pub mod pagination;
//pub mod progressbar;
//pub mod scrollable;
//pub mod sidepanel;
//pub mod sortedtable;
//pub mod table;
//pub mod tabs;
//pub mod toast;
pub mod checkbox;
//pub mod input;
//pub mod radio;
//pub mod select;
//pub mod slider;
//pub mod textarea;
//pub mod toggle;
pub mod formlist;

use accordion::*;
use button::*;
use buttongroup::*;
use icon::*;
//use placeholder::*;
use callout::*;
use carousel::*;
use dropdown::*;
use hovercard::*;
//use lightswitch::*;
//use modal::*;
//use pagination::*;
//use progressbar::*;
//use scrollable::*;
//use sidepanel::*;
//use sortedtable::*;
//use table::*;
//use tabs::*;
//use toast::*;
use checkbox::*;
//use input::*;
//use radio::*;
//use select::*;
//use slider::*;
//use textarea::*;
//use toggle::*;
use formlist::*;

pub mod layout;
pub use layout::SideBarComponent;
pub mod preview;

use crate::app::router::Route;
use dioxus::prelude::*;

#[component]
pub fn ComponentPage(name: String) -> Element {
    let route: Route = use_route();

    match route {
        Route::ComponentPage { name } => match name.as_str() {
            "accordion" => rsx! {
                AccordionPage {}
            },
            "button" => rsx! {
                ButtonPage {}
            },
            "buttongroup" => rsx! {
                ButtonGroupPage {}
            },
            "icon" => rsx! {
                IconPage {}
            },
            "callout" => rsx! {
                CalloutPage {}
            },
            "carousel" => rsx! {
                CarouselPage {}
            },
            "dropdown" => rsx! {
                DropdownPage {}
            },
            "formlist" => rsx! {
                FormListPage {}
            },
            "hovercard" => rsx! {
                HoverCardPage {}
            },
            //"placeholder" => rsx! {
            //    PlaceholderPage {}
            //},
            //"modal" => rsx! {
            //    ModalPage {}
            //},
            //"sidepanel" => rsx! {
            //    SidePanelPage {}
            //},
            //"pagination" => rsx! {
            //    PaginationPage {}
            //},
            //"progressbar" => rsx! {
            //    ProgressBarPage {}
            //},
            //"lightswitch" => rsx! {
            //    LightSwitchPage {}
            //},
            //"scrollable" => rsx! {
            //    ScrollablePage {}
            //},
            //"tabs" => rsx! {
            //    TabsPage {}
            //},
            "checkbox" => rsx! {
                CheckboxPage {}
            },
            //"input" => rsx! {
            //    InputPage {}
            //},
            //"radio" => rsx! {
            //    RadioPage {}
            //},
            //"select" => rsx! {
            //    SelectPage {}
            //},
            //"sortedtable" => rsx! {
            //    SortedTablePage {}
            //},
            //"table" => rsx! {
            //    TablePage {}
            //},
            //"slider" => rsx! {
            //    SliderPage {}
            //},
            //"textarea" => rsx! {
            //    TextAreaPage {}
            //},
            //"toggle" => rsx! {
            //    TogglePage {}
            //},
            //"toast" => rsx! {
            //    ToastPage {}
            //},
            _ => {
                rsx! { "Component not found" }
            }
        },
        _ => {
            rsx! { "How did you got there ?" }
        }
    }
}
