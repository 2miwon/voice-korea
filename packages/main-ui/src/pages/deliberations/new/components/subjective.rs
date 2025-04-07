use crate::components::icons::Trash;
use bdk::prelude::*;
use models::Question;

#[component]
pub fn Subjective(
    lang: Language,
    onchange: EventHandler<Question>,
    onremove: EventHandler<MouseEvent>,
    question: Question,
) -> Element {
    let tr: SubjectiveTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full h-1 bg-period-border-gray my-10" }

            input {
                class: "flex flex-row w-full h-55 justify-start items-center bg-white focus:outline-none border-b-1 border-label-border-gray px-15 py-15 font-medium text-hint-gray text-[15px] leading-22 mb-20",
                r#type: "text",
                placeholder: tr.input_description_hint,
                value: question.description(),
                oninput: move |e: Event<FormData>| {
                    question.set_description(&e.value());
                    onchange.call(question.clone());
                },
            }

            div { class: "flex flex-row w-full justify-end items-center gap-5",
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
    SubjectiveTranslate;

    input_title_hint: {
        ko: "제목을 입력해주세요",
        en: "Please enter a title"
    }
    input_description_hint: {
        ko: "내용을 입력해주세요",
        en: "Please enter a description"
    }
    remove: {
        ko: "삭제",
        en: "Remove"
    }
}
