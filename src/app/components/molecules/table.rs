use dioxus::prelude::*;
use dioxus_tw_components::molecules::table::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn TablePage() -> Element {
    let _state = use_context_provider(|| Signal::new(HashPreview::new()));

    rsx! {
        PreviewFull::<TableProps> {}
    }
}

impl DemoComponent for TableProps {
    fn comp_introduction() -> &'static str {
        "Nice looking table"
    }

    fn BuildCompPreview() -> Element {
        let _state = use_context::<Signal<HashPreview>>();

        rsx! {
            div { class: "max-w-96 border rounded-global-radius bg-background p-1",
                Table {
                    TableCaption { "Product Inventory" }
                    TableHeader {
                        TableRow {
                            TableHead { "Product Name" }
                            TableHead { "Quantity" }
                            TableHead { "Price" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { "Widget A" }
                            TableCell { "100" }
                            TableCell { "$1.99" }
                        }
                        TableRow {
                            TableCell { "Widget B" }
                            TableCell { "50" }
                            TableCell { "$2.99" }
                        }
                        TableRow {
                            TableCell { "Widget C" }
                            TableCell { "25" }
                            TableCell { "$3.99" }
                        }
                        TableRow {
                            TableCell { "Widget D" }
                            TableCell { "75" }
                            TableCell { "$4.99" }
                        }
                        TableRow {
                            TableCell { "Widget E" }
                            TableCell { "125" }
                            TableCell { "$5.99" }
                        }
                    }
                    TableFooter {
                        TableRow {
                            TableCell { "Total" }
                            TableCell { "350" }
                            TableCell { "$27.91" }
                        }
                    }
                }
            }
        }
    }

    fn BuildCompSelectors() -> Element {
        let _state = use_context::<Signal<HashPreview>>();

        rsx! {}
    }
}
