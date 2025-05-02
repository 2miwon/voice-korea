use bdk::prelude::*;

#[component]
pub fn Video(video: bool) -> Element {
    rsx! {
        div { class: "flex-1 flex flex-col w-full justify-center items-center bg-black",
            div {
                id: "video-grid",
                class: "flex flex-col w-full h-[80vh] justify-end items-center aria-active:!bg-text-black",
                "aria-active": !video,
            }
        }
    }
}
