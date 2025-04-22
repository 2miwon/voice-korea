use dioxus::prelude::*;
use models::ChoiceQuestion;

use super::checkbox::Checkbox;
#[component]
pub fn SingleObjective(
    #[props(default = None)] id: Option<String>,
    question: ChoiceQuestion,
    answer: i32,
    onchange: EventHandler<i32>,
    #[props(default = false)] disabled: bool,
) -> Element {
    let handle_select = move |index: i32| {
        onchange.call(index);
    };

    rsx! {
        div {
            id,
            class: "flex flex-col w-full justify-start items-start bg-white rounded-[8px] px-[20px] py-[24px] gap-[15px]",
            div { class: "font-semibold text-[16px] text-[#2D2D2D]", {question.title} }
            div { class: "flex flex-row w-full h-[1px] bg-[#eeeeee]" }
            div { class: "flex flex-col gap-[10px]",
                for (i , option) in question.options.into_iter().enumerate() {
                    div {
                        class: "flex flex-row gap-[10px]",
                        onclick: move |e| {
                            e.stop_propagation();
                            if !disabled {
                                if answer == (i + 1) as i32 {
                                    handle_select(0);
                                } else {
                                    handle_select((i + 1) as i32);
                                }
                            }
                        },
                        Checkbox { checked: (i + 1) as i32 == answer, disabled }
                        div { class: "flex flex-row gap-[10px]",
                            div { class: "font-semibold text-[#2D2D2D] text-[15px]",
                                {option}
                            }
                        }
                    }
                }
            }
        }
    }
}
