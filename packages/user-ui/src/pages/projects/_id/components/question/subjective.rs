use dioxus::prelude::*;
use models::SubjectiveQuestion;

use crate::components::input::InputBox;

#[component]
pub fn Subjective(
    #[props(default = None)] id: Option<String>,
    question: SubjectiveQuestion,
    answer: String,
    onchange: EventHandler<String>,
    #[props(default = false)] disabled: bool,
    #[props(default = "".to_string())] placeholder: String,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start bg-white rounded-[8px] px-[20px] py-[24px] gap-[15px]",
            div { class: "font-semibold text-[16px] text-[#2D2D2D]", {question.title} }
            div { class: "flex flex-row w-full h-[1px] bg-[#eeeeee]" }
            div {
                class: "flex flex-row w-full",
                display: if disabled { "none" } else { "flex" },
                InputBox {
                    id,
                    placeholder,
                    value: answer.clone(),
                    onchange: move |e: String| {
                        onchange.call(e);
                    },
                }
            }
            div {
                class: "flex flex-row w-full",
                display: if disabled { "flex" } else { "none" },
                div { class: "flex flex-row w-full rounded-[10px] px-[15px] py-[10px] min-h-[45px] bg-[#f7f7f7] text-[#222222]",
                    {answer}
                }
            }
        }
    }
}
