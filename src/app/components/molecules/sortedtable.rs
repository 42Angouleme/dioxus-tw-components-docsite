use dioxus::prelude::*;
use dioxus_tw_components::molecules::sorttable::{SortTable, SortableCell, SortableRow};

#[component]
pub fn SortedTablePage() -> Element {
    rsx! {
        h4 { class: "h4", "Sorted Table" }
        div { class: "flex flex-col gap-8",
            SortTable {
                headers: vec![
                    "Photo".to_string(),
                    "Login".to_string(),
                    "Exam01".to_string(),
                    "User".to_string(),
                ],
                data: get_data_user(),
                header_class: "bg-blue-300 text-white",
                row_class: "bg-red-100",
                cell_class: "border border-green-300 w-[50px] h-[50px]",
            }
        }
    }
}

fn get_data_user() -> Vec<SortableRow> {
    let user = TestUser {
        name: "John".to_string(),
        age: 25,
    };
    let user2 = TestUser {
        name: "Pierre".to_string(),
        age: 19,
    };
    vec![
        SortableRow::new(vec![
            SortableCell::new(
                rsx! {img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" }},
            ),
            SortableCell::new(rsx! {"user1"}).sort_by("user1".into()),
            SortableCell::new(rsx! {"5"}).sort_by(5.into()),
            SortableCell::new(rsx! {{format!("{}({})", user.name, user.age)}})
                .sort_by(user.age.into()),
        ]),
        SortableRow::new(vec![
            SortableCell::new(
                rsx! {img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" }},
            ),
            SortableCell::new(rsx! {"user2"}).sort_by("user2".into()),
            SortableCell::new(rsx! {"75"}).sort_by(75.into()),
            SortableCell::new(rsx! {{format!("{}({})", user2.name, user2.age)}})
                .sort_by(user2.age.into()),
        ]),
    ]
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct TestUser {
    name: String,
    age: u32,
}
