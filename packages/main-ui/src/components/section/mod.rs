use crate::components::{block_header::BlockHeader, icons::Trash};
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
    lang: Language,
    #[props(default = false)] required: bool,
    header: Option<String>,
    description: Option<String>,
    children: Element,
    #[props(default = None)] open: Option<bool>,
    // onmultiselect: Option<EventHandler<bool>>, TODO: "복수 선택 가능" toggle in figma
    // onrequire: Option<EventHandler<bool>>, TODO: "필수 입력" toggle in figma
    ondelete: Option<EventHandler<MouseEvent>>,
) -> Element {
    let tr: MainSectionTranslate = translate(&lang);
    let mut opened = use_signal(|| open.unwrap_or(true));
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-20 gap-10",
                if let (Some(header), Some(description)) = (header, description) {
                    BlockHeader {
                        required,
                        header,
                        description,
                        onopen: Some(
                            EventHandler::new(move |is_open| {
                                opened.set(is_open);
                            }),
                        ),
                    }
                }
                if opened() {
                    {children}
                }
                if let Some(onclick) = ondelete {
                    div { class: "flex flex-row w-full justify-end items-center gap-5 mt-10",
                        button {
                            class: "cursor-pointer flex flex-row w-80 items-center justify-end",
                            onclick,
                            div { class: "font-medium text-text-black text-[15px]",
                                {tr.remove}
                            }
                            Trash { width: "18", height: "18" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AddSection(lang: Language, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "relative w-full flex items-center justify-center",
            div { class: "absolute w-full h-1 border border-dashed border-hint-gray" }
            button {
                class: "cursor-pointer z-10 bg-white border border-hint-gray rounded-full w-45 h-45 flex items-center justify-center hover:shadow-md",
                onclick: move |e| {
                    onclick.call(e);
                },
                "+"
            }
        }
    }
}

translate! {
    MainSectionTranslate;

    remove: {
        ko: "삭제",
        en: "Remove"
    }
}
