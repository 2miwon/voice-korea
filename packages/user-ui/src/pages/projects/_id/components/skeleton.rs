use bdk::prelude::*;

use crate::components::icons::triangle::TriangleUp;

#[component]
pub fn Skeleton() -> Element {
    rsx! {
        div { class: "rounded-xl shadow animate-pulse space-y-6 w-full p-20",
            //  title
            div { class: "w-full flex flex-coljustify-start items-center py-14",
                div { class: "w-full flex justify-between items-center gap-10",
                    div { class: "h-30 flex-1 max-w-180 rounded-md bg-gray-300" }
                    TriangleUp {}
                }
            }
            hr { class: "w-full h-1 mt-12 mb-12 border-line-gray" }

            div { class: "w-full flex flex-col gap-50",
                div { class: "w-full flex flex-col gap-20",
                    div { class: "h-24 max-w-100 rounded-sm bg-gray-300" }
                    div { class: "flex flex-col gap-5 [&>div]:h-20",
                        div { class: "rounded-sm bg-gray-200 w-11/12" }
                        div { class: "rounded-sm bg-gray-200 w-9/12" }
                        div { class: "rounded-sm bg-gray-200 w-10/12" }
                        div { class: "rounded-sm bg-gray-200 w-7/12" }
                    }
                
                }
                div { class: "flex gap-10",
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
