use crate::components::block_header::BlockHeader;
use bdk::prelude::*;

#[component]
pub fn SubSection(required: bool, title: String, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center",
            div { class: "flex flex-row justify-start items-center w-150",
                if required {
                    div { class: "text-base font-bold text-necessary mr-2", "*" }
                }
                div { class: "text-[15px] font-medium text-text-black", {title} }
            }
            {children}
        }
    }
}

#[component]
pub fn MainSection(
    required: bool,
    header: String,
    description: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: format!("flex flex-col w-full justify-start items-start"),
            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-20 mb-20 gap-10",
                BlockHeader { required, header, description }
                {children}
            }
        }
    }
}
