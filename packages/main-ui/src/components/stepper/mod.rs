use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StepperProps {
    current_step: usize,
    steps: Vec<String>,
}

#[component]
pub fn Stepper(props: StepperProps) -> Element {
    rsx! {
        ol { class: "flex items-center w-full",
            {
                props
                    .steps
                    .iter()
                    .enumerate()
                    .map(|(index, label)| {
                        let is_completed = index < props.current_step;
                        let is_last = index == props.steps.len() - 1;
                        rsx! {
                            div { class: if is_last { "relative flex flex-col w-[100px] justify-center items-center" } else { "relative flex flex-col flex-1 justify-center items-center" },
                                li {
                                    class: format!(
                                        "flex items-center w-full {}",
                                        if is_last {
                                            ""
                                        } else if is_completed {
                                            "text-blue-600 dark:text-blue-500 after:content-[''] after:w-full after:h-1 after:border-b after:border-[#2a60d3] after:border-4 after:inline-block dark:after:border-blue-800"
                                        } else {
                                            "text-blue-600 dark:text-blue-500 after:content-[''] after:w-full after:h-1 after:border-b after:border-[#bfc8d9] after:border-4 after:inline-block dark:after:border-blue-800"
                                        },
                                    ),
                                    span { class: get_step_circle_styles(is_completed), "{index + 1}" }
                                    label_text { label: label.clone() }
                                }
                            }
                        }
                    })
            }
        }
    }
}

fn get_step_circle_styles(is_completed: bool) -> &'static str {
    if is_completed {
        "flex items-center justify-center w-[32px] h-[32px] bg-[#2a60d3] rounded-full dark:bg-blue-800 shrink-0 text-white font-black text-[14px]"
    } else {
        "flex items-center justify-center w-[32px] h-[32px] border-[5px] border-[#bfc8d9] bg-white rounded-full shrink-0 text-[#bfc8d9] font-black text-[14px]"
    }
}

#[component]
fn label_text(label: String) -> Element {
    rsx! {
        div {
            class: format!(
                "absolute top-[38px] {} text-center text-[#222222] font-medium text-[14px]",
                if label.chars().count() > 6 { "left-[-30px]" } else { "left-[-20px]" },
            ),
            "{label}"
        }
    }
}
