pub mod components;
pub mod doctrait;
pub mod home;
pub mod layout;
pub mod router;

use dioxus::prelude::*;
use dioxus_tw_components::prelude::*;

use crate::app::router::Route;

const TAILWIND_CSS: Asset = asset!(
    "/assets/tailwind.css",
    CssAssetOptions::new().with_preload(true).with_minify(true)
);

const FAVICON: Asset = asset!("/assets/favicon.png");

pub fn App() -> Element {
    let theme_manager = use_context_provider(|| Signal::new(ThemeManager::default()));

    rsx! {
        DioxusTwComponentsBootstrap { icon: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        Toaster {
            div {
                class: "relative bg-background text-foreground",
                style: theme_manager.read().to_style(),
                Router::<Route> {}
            }
        }
    }
}
