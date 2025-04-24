use bdk::prelude::*;

use crate::components::icons::{Minus, Trash};
use models::Question;

#[component]
pub fn Objective(
    lang: Language,
    onchange: EventHandler<Question>,
    onremove: EventHandler<MouseEvent>,
    question: Question,
) -> Element {
    let tr: ObjectiveTranslate = translate(&lang);
    let options = question.options();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",

            // input {
            //     class: format!(
            //         "flex flex-row w-full h-55 justify-start items-center bg-background-gray focus:outline-none px-15 py-10 font-medium text-hint-gray text-[15px] leading-22 rounded-sm mt-10",
            //     ),
            //     r#type: "text",
            //     placeholder: tr.input_description,
            //     value: question.description(),
            //     oninput: {
            //         let mut question = question.clone();
            //         move |e: Event<FormData>| {
            //             question.set_description(&e.value());
            //             onchange.call(question.clone());
            //         }
            //     },
            // }

            div { class: "flex flex-row w-full h-[1px] bg-period-border-gray my-10" }

            for (index , option) in options.iter().enumerate() {
                div { class: "flex flex-row w-full justify-start items-center mt-10",
                    div { class: "flex flex-row w-20 h-20 border-3 bg-white border-header-gray mr-10 rounded-[100px]" }
                    input {
                        class: "flex flex-row max-w-888 w-full h-55 justify-start items-center bg-white focus:outline-none border-b-1 border-header-gray px-15 py-15 font-medium text-[#9f9f9f] text-[15px] leading-22 mr-10",
                        r#type: "text",
                        placeholder: format!("{} {}", tr.option, index + 1),
                        value: option.clone(),
                        oninput: {
                            let mut question = question.clone();
                            move |e: Event<FormData>| {
                                question.change_option(index, &e.value());
                                onchange.call(question.clone());
                            }
                        },
                    }
                    button {
                        onclick: {
                            let mut question = question.clone();
                            move |_| {
                                question.remove_option(index);
                                onchange.call(question.clone());
                            }
                        },
                        Minus { width: "20", height: "20" }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-start items-center mt-30",
                div { class: "flex flex-row w-20 h-20 border-3 bg-white border-[#9f9f9f] mr-10 rounded-[100px]" }
                button {
                    class: "cursor-pointer font-medium text-base text-[#3a94ff]",
                    onclick: {
                        let mut question = question.clone();
                        move |_| {
                            question.add_option("");
                            onchange.call(question.clone());
                        }
                    },
                    {tr.add_option}
                }
            }

            div { class: "flex flex-row w-full justify-end items-center gap-5 mt-10",
                button {
                    class: "cursor-pointer flex flex-row w-80 items-center justify-end",
                    onclick: move |e: Event<MouseData>| {
                        onremove.call(e);
                    },
                    div { class: "font-medium text-text-black text-[15px]", {tr.remove} }
                    Trash { width: "18", height: "18" }
                }
            }
        }
    }
}

translate! {
    ObjectiveTranslate;

    option: {
        ko: "옵션",
        en: "Option"
    }
    add_option: {
        ko: "옵션 추가하기",
        en: "Add Option"
    }
    remove: {
        ko: "삭제",
        en: "Remove"
    }
    input_description: {
        ko: "설명을 입력해주세요.",
        en: "Please Input Description"
    }
}
