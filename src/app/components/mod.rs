pub mod atoms;
pub use atoms::{button::ButtonPage, buttongroup::ButtonGroupPage, icon::IconPage, placeholder::PlaceholderPage};
pub mod layout;
pub use layout::SideBarComponent;

pub mod molecules;
use molecules::{
    accordion::AccordionPage, breadcrumb::BreadcrumbPage, carousel::CarouselPage,
    dropdown::DropdownPage, hovercard::HoverCardPage, modal::ModalPage,
    progressbar::ProgressBarPage, scrollable::ScrollablePage, sidepanel::SidePanelPage,
    table::TablePage, toast::ToastPage,
};
pub use molecules::{lightswitch::LightSwitchPage, sortedtable::SortedTablePage, tabs::TabsPage};

pub mod organisms;
use organisms::form::formlist::FormListPage;
pub use organisms::form::{
    checkbox::CheckboxPage, input::InputPage, radio::RadioPage, select::SelectPage,
    slider::SliderPage, textarea::TextAreaPage, toggle::TogglePage,
};

pub mod preview;

use crate::app::router::Route;
use dioxus::prelude::*;

#[component]
pub fn ComponentPage(name: String) -> Element {
    let route: Route = use_route();

    match route {
        Route::ComponentPage { name } => match name.as_str() {
            "accordion" => rsx!(AccordionPage {}),
            "button" => rsx!(ButtonPage {}),
            "buttongroup" => rsx!(ButtonGroupPage {}),
            "icon" => rsx!(IconPage {}),
            "breadcrumb" => rsx!(BreadcrumbPage {}),
            "carousel" => rsx!(CarouselPage {}),
            "dropdown" => rsx!(DropdownPage {}),
            "formlist" => rsx!(FormListPage {}),
            "hovercard" => rsx!(HoverCardPage {}),
            "placeholder" => rsx!(PlaceholderPage {}),
            "modal" => rsx!(ModalPage {}),
            "sidepanel" => rsx!(SidePanelPage {}),
            "progressbar" => rsx!(ProgressBarPage {}),
            "lightswitch" => rsx!(LightSwitchPage {}),
            "scrollable" => rsx!(ScrollablePage {}),
            "tabs" => rsx!(TabsPage {}),
            "checkbox" => rsx!(CheckboxPage {}),
            "input" => rsx!(InputPage {}),
            "radio" => rsx!(RadioPage {}),
            "select" => rsx!(SelectPage {}),
            "sortedtable" => rsx!(SortedTablePage {}),
            "table" => rsx!(TablePage {}),
            "slider" => rsx!(SliderPage {}),
            "textarea" => rsx!(TextAreaPage {}),
            "toggle" => rsx!(TogglePage {}),
            "toast" => rsx!(ToastPage {}),
            _ => {
                rsx!("Component not found")
            }
        },
        _ => {
            rsx!("How did you got there ?")
        }
    }
}
