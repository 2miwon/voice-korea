use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn PopupZone() -> Element {
    let mut popup: PopupService = use_context();
    rsx! {
        div {
            class: format!(
                "{}",
                match popup.is_opened() {
                    true => {
                        "fixed top-0 left-0 w-screen h-screen bg-black bg-opacity-50 flex justify-center items-center backdrop-blur-[4px] bg-black/25 z-[101]"
                    }
                    false => "hidden",
                },
            ),
            onclick: move |_| {
                popup.close();
            },
            if popup.is_opened() {
                div {
                    class: "relative bg-white rounded-[12px] border-white border-[1px] p-[25px] min-w-[350px]",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    if (popup.close)() {
                        div {
                            class: format!("absolute top-[25px] right-[25px] rounded-[4px] cursor-pointer"),
                            onclick: move |_| {
                                popup.close();
                            },
                            Close { color: "black" }
                        }
                    }
                    div {
                        id: popup.get_id(),
                        class: "flex flex-col items-center justify-center gap-[25px]",
                        match popup.get_title() {
                            Some(title) => {
                                rsx! {
                                    div { class: "text-[20px] font-bold text-[#222222]", "{title}" }
                                }
                            }
                            None => rsx! {},
                        }
                        {popup.render()}
                    }
                }
            }
        }
    }
}

#[component]
pub fn Close(#[props(default = "white".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M16.9498 7.05029L7.05029 16.9498M7.05029 7.05029L16.9498 16.9498",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
