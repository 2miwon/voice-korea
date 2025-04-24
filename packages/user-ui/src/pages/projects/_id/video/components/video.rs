use bdk::prelude::*;

#[component]
pub fn Video() -> Element {
    rsx! {
        div { class: "flex-1 flex flex-col w-full justify-center items-center bg-black",
            div { class: "flex flex-col w-full h-[80%] justify-end bg-[#222222]",
                div { class: "font-normal text-xs text-white", "nickname" }
            }
        }
    }
}
