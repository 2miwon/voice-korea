use dioxus::prelude::*;
use models::ChoiceQuestion;

use super::checkbox::Checkbox;

#[component]
pub fn MultipleObjective(
    #[props(default = None)] id: Option<String>,
    question: ChoiceQuestion,
    answer: Vec<i32>,
    onchange: EventHandler<Vec<i32>>,
    #[props(default = false)] disabled: bool,
) -> Element {
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
                        onclick: {
                            let answer = answer.clone();
                            move |e: MouseEvent| {
                                e.stop_propagation();
                                if !disabled {
                                    let mut new_answer = answer.clone();
                                    if new_answer.contains(&(i as i32)) {
                                        new_answer.retain(|&x| x != (i as i32));
                                    } else {
                                        new_answer.push(i as i32);
                                    };
                                    onchange.call(new_answer);
                                }
                            }
                        },
                        Checkbox {
                            disabled,
                            checked: answer.contains(&(i as i32)),
                        }
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
