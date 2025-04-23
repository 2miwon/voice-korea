use bdk::prelude::*;

use crate::pages::projects::_id::components::{
    accordion::Accordion, section::Section, tab_title::TabTitle,
};

#[component]
pub fn Skeleton() -> Element {
    rsx! {
        Section { id: "skeleton", class: "shadow animate-pulse w-full",
            TabTitle {
                title: "",
                class: "[&>span]:h-24 [&>span]:w-200 [&>span]:rounded-md [&>span]:bg-gray-300",
                p { class: "h-20 max-w-100 flex-1 rounded-sm bg-gray-200" }
            }
            Accordion {
                title: "",
                class: "w-full flex-1 [&>div>span]:h-24 [&>div>span]:w-200 [&>div>span]:rounded-md [&>div>span]:bg-gray-300",
                default_open: true,
                div { class: "w-full flex flex-col gap-20 ",
                    div { class: "h-24 max-w-100 rounded-sm bg-gray-300" }
                    div { class: "flex flex-col gap-5 [&>div]:h-20",
                        div { class: "rounded-sm bg-gray-200 w-11/12" }
                        div { class: "rounded-sm bg-gray-200 w-9/12" }
                        div { class: "rounded-sm bg-gray-200 w-10/12" }
                        div { class: "rounded-sm bg-gray-200 w-7/12" }
                    }
                }
            }
            div { class: "p-20",
                div { class: "grid grid-cols-6 gap-10 w-full",
                    Committee {}
                    Committee {}
                    Committee {}
                }
            }
        }
    }
}

#[component]
fn Committee() -> Element {
    rsx! {
        div { class: "flex flex-row w-140 gap-10",
            div { class: "size-48 rounded-full bg-gray-200" }
            div { class: "flex flex-col flex-1 justify-center gap-5",
                div { class: "h-19 bg-gray-300 rounded-sm mr-20" }
                div { class: "h-15 bg-gray-200 rounded-sm" }
            }
        }
    }
}
