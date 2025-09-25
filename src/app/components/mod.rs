pub mod accordion;
pub mod button;
pub mod buttongroup;
pub mod callout;
pub mod carousel;
pub mod checkbox;
pub mod dropdown;
pub mod formlist;
pub mod hovercard;
pub mod icon;
pub mod input;
pub mod lightswitch;
pub mod markdown;
pub mod modal;
pub mod pagination;
pub mod placeholder;
pub mod progressbar;
pub mod radio;
pub mod scrollable;
pub mod select;
pub mod sidepanel;
pub mod slider;
pub mod sortedtable;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod toast;
pub mod toggle;

use accordion::*;
use button::*;
use buttongroup::*;
use callout::*;
use carousel::*;
use checkbox::*;
use dropdown::*;
use formlist::*;
use hovercard::*;
use icon::*;
use input::*;
use lightswitch::*;
use markdown::*;
use modal::*;
use pagination::*;
use placeholder::*;
use progressbar::*;
use radio::*;
use scrollable::*;
use select::*;
use sidepanel::*;
use slider::*;
use sortedtable::*;
use table::*;
use tabs::*;
use textarea::*;
use toast::*;
use toggle::*;

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
            "placeholder" => rsx! {
                PlaceholderPage {}
            },
            "markdown" => rsx! {
                MarkdownPage {}
            },
            "modal" => rsx! {
                ModalPage {}
            },
            "sidepanel" => rsx! {
                SidePanelPage {}
            },
            "pagination" => rsx! {
                PaginationPage {}
            },
            "progressbar" => rsx! {
                ProgressBarPage {}
            },
            "lightswitch" => rsx! {
                LightSwitchPage {}
            },
            "scrollable" => rsx! {
                ScrollablePage {}
            },
            "tabs" => rsx! {
                TabsPage {}
            },
            "checkbox" => rsx! {
                CheckboxPage {}
            },
            "input" => rsx! {
                InputPage {}
            },
            "radio" => rsx! {
                RadioPage {}
            },
            "select" => rsx! {
                SelectPage {}
            },
            "sortedtable" => rsx! {
                SortedTablePage {}
            },
            "table" => rsx! {
                TablePage {}
            },
            "slider" => rsx! {
                SliderPage {}
            },
            "textarea" => rsx! {
                TextAreaPage {}
            },
            "toggle" => rsx! {
                TogglePage {}
            },
            "toast" => rsx! {
                ToastPage {}
            },
            _ => {
                rsx! { "Component not found" }
            }
        },
        _ => {
            rsx! { "How did you got there ?" }
        }
    }
}
