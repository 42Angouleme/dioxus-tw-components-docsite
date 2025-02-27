use dioxus::prelude::*;
use dioxus_tw_components::{
    molecules::sorttable::{KeyType, SortTable, Sortable, SortableCell, SortableRow, ToTableData},
    Row,
};

#[component]
pub fn SortedTablePage() -> Element {
    //let vec_user = UserTab::get_10_users();
    //let vec_rows = RowTest::get_4_rows();

    rsx! {
        h4 { class: "h4", "Sorted Table" }
        div { class: "flex flex-col gap-8",
            //SortTable::<UserTab> {
            //    data: vec_user,
            //    header_class: "bg-blue-300 text-white",
            //    row_class: "bg-red-100",
            //    cell_class: "border border-green-300",
            //}
            //SortTable::<RowTest> {
            //    data: vec_rows,
            //    header_class: "bg-blue-300 text-white",
            //    row_class: "bg-red-100",
            //    cell_class: "border border-green-300",
            //}
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

//impl std::fmt::Display for TestUser {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "{} ({})", self.name, self.age)
//    }
//}
//
//#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
//struct TestProject {
//    slug: String,
//    id: u64,
//}
//
//impl std::fmt::Display for TestProject {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "{} ({})", self.slug, self.id)
//    }
//}
//
//#[derive(PartialEq, Clone, Debug, Row)]
//struct UserTab {
//    #[row(header = "pp")]
//    photo: Element,
//    #[row(header = "username")]
//    login: &'static str,
//    #[row(nosort, header = "user_status")]
//    status: &'static str,
//    exam2: u64,
//    exam3: u64,
//    exam4: u64,
//    exam5: u64,
//    exam6: u64,
//    libft: u64,
//    gnl: u64,
//    printf: u64,
//    #[row(sort = "age")]
//    user: TestUser,
//    #[row(header = "uid", sort = "id")]
//    project: TestProject,
//}
//
//impl UserTab {
//    fn get_10_users() -> Vec<UserTab> {
//        vec![
//            UserTab {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                login: "user1",
//                status: "active",
//                exam2: 85,
//                exam3: 90,
//                exam4: 88,
//                exam5: 92,
//                exam6: 89,
//                libft: 100,
//                gnl: 95,
//                printf: 98,
//                user: TestUser {
//                    name: "John".to_string(),
//                    age: 25,
//                },
//                project: TestProject {
//                    slug: "project1".to_string(),
//                    id: 1,
//                },
//            },
//            UserTab {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                login: "user2",
//                status: "inactive",
//                exam2: 78,
//                exam3: 82,
//                exam4: 79,
//                exam5: 85,
//                exam6: 80,
//                libft: 90,
//                gnl: 88,
//                printf: 92,
//                user: TestUser {
//                    name: "Jane".to_string(),
//                    age: 23,
//                },
//                project: TestProject {
//                    slug: "project2".to_string(),
//                    id: 2,
//                },
//            },
//            UserTab {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                login: "user3",
//                status: "active",
//                exam2: 95,
//                exam3: 98,
//                exam4: 93,
//                exam5: 97,
//                exam6: 94,
//                libft: 100,
//                gnl: 100,
//                printf: 100,
//                user: TestUser {
//                    name: "Alice".to_string(),
//                    age: 27,
//                },
//                project: TestProject {
//                    slug: "project3".to_string(),
//                    id: 3,
//                },
//            },
//            UserTab {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                login: "user4",
//                status: "active",
//                exam2: 88,
//                exam3: 91,
//                exam4: 89,
//                exam5: 93,
//                exam6: 90,
//                libft: 95,
//                gnl: 98,
//                printf: 96,
//                user: TestUser {
//                    name: "Bob".to_string(),
//                    age: 26,
//                },
//                project: TestProject {
//                    slug: "project4".to_string(),
//                    id: 4,
//                },
//            },
//            UserTab {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                login: "user5",
//                status: "inactive",
//                exam2: 75,
//                exam3: 78,
//                exam4: 76,
//                exam5: 80,
//                exam6: 77,
//                libft: 85,
//                gnl: 82,
//                printf: 88,
//                user: TestUser {
//                    name: "Eve".to_string(),
//                    age: 24,
//                },
//                project: TestProject {
//                    slug: "project5".to_string(),
//                    id: 5,
//                },
//            },
//            UserTab {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                login: "user6",
//                status: "active",
//                exam2: 92,
//                exam3: 96,
//                exam4: 94,
//                exam5: 98,
//                exam6: 95,
//                libft: 100,
//                gnl: 99,
//                printf: 100,
//                user: TestUser {
//                    name: "Charlie".to_string(),
//                    age: 28,
//                },
//                project: TestProject {
//                    slug: "project6".to_string(),
//                    id: 6,
//                },
//            },
//            UserTab {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                login: "user7",
//                status: "active",
//                exam2: 89,
//                exam3: 93,
//                exam4: 90,
//                exam5: 95,
//                exam6: 91,
//                libft: 98,
//                gnl: 97,
//                printf: 99,
//                user: TestUser {
//                    name: "David".to_string(),
//                    age: 29,
//                },
//                project: TestProject {
//                    slug: "project7".to_string(),
//                    id: 7,
//                },
//            },
//            UserTab {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                login: "user8",
//                status: "inactive",
//                exam2: 72,
//                exam3: 75,
//                exam4: 73,
//                exam5: 77,
//                exam6: 74,
//                libft: 80,
//                gnl: 78,
//                printf: 82,
//                user: TestUser {
//                    name: "Frank".to_string(),
//                    age: 30,
//                },
//                project: TestProject {
//                    slug: "project8".to_string(),
//                    id: 8,
//                },
//            },
//            UserTab {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                login: "user9",
//                status: "active",
//                exam2: 97,
//                exam3: 100,
//                exam4: 98,
//                exam5: 100,
//                exam6: 99,
//                libft: 100,
//                gnl: 100,
//                printf: 100,
//                user: TestUser {
//                    name: "Grace".to_string(),
//                    age: 31,
//                },
//                project: TestProject {
//                    slug: "project9".to_string(),
//                    id: 9,
//                },
//            },
//            UserTab {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                login: "user10",
//                status: "active",
//                exam2: 87,
//                exam3: 90,
//                exam4: 88,
//                exam5: 92,
//                exam6: 89,
//                libft: 96,
//                gnl: 94,
//                printf: 97,
//                user: TestUser {
//                    name: "Helen".to_string(),
//                    age: 32,
//                },
//                project: TestProject {
//                    slug: "project10".to_string(),
//                    id: 10,
//                },
//            },
//        ]
//    }
//}
//
//#[derive(PartialEq, Clone, Debug)]
//struct RowTest {
//    photo: Element,
//    user: TestUser,
//    project: TestProject,
//}
//impl ToTableData for RowTest {
//    fn headers_to_strings() -> Vec<impl ToString> {
//        vec!["pp", "user", "uid"]
//    }
//    fn to_keytype(&self) -> Vec<KeyType> {
//        vec![
//            KeyType::Element(self.photo.clone(), false),
//            KeyType::String(self.user.to_string(), true),
//            KeyType::String(self.project.to_string(), true),
//        ]
//    }
//}
//impl RowTest {
//    pub fn get_4_rows() -> Vec<RowTest> {
//        vec![
//            RowTest {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                user: TestUser {
//                    name: "John".to_string(),
//                    age: 25,
//                },
//                project: TestProject {
//                    slug: "project1".to_string(),
//                    id: 1,
//                },
//            },
//            RowTest {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                user: TestUser {
//                    name: "John".to_string(),
//                    age: 25,
//                },
//                project: TestProject {
//                    slug: "project1".to_string(),
//                    id: 1,
//                },
//            },
//            RowTest {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                user: TestUser {
//                    name: "John".to_string(),
//                    age: 25,
//                },
//                project: TestProject {
//                    slug: "project1".to_string(),
//                    id: 1,
//                },
//            },
//            RowTest {
//                photo: rsx! { img { src: "https://imgs.search.brave.com/qBizHqeYVCfOzw9Jv60VJJAYxYvMaTfWIACzH-D0UJA/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvNjE1/NTgyNDAyL2ZyL3Bo/b3RvL21lZXJrYXQt/Y2F0LmpwZz9zPTYx/Mng2MTImdz0wJms9/MjAmYz1admJYbVp4/dUE5c3Nyc3JfT2JT/Vko2TWtVS0IySkg1/ZmRGRlFUdmc5bFMw/PQ" } },
//                user: TestUser {
//                    name: "John".to_string(),
//                    age: 25,
//                },
//                project: TestProject {
//                    slug: "project1".to_string(),
//                    id: 1,
//                },
//            },
//        ]
//    }
//}
