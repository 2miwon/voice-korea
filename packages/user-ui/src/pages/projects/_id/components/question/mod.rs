mod multiple_objective;
mod single_objective;
mod subjective;

use multiple_objective::MultipleObjective;
use single_objective::SingleObjective;
use subjective::Subjective;

mod checkbox;

use bdk::prelude::*;
use models::{response::Answer, Question};

#[component]
pub fn QuestionComponent(
    question: Question,
    answer: Option<Answer>,
    onchange: EventHandler<Answer>,
    #[props(default = "".to_string())] placeholder: String,
    #[props(default = false)] disabled: bool,
) -> Element {
    match question {
        Question::SingleChoice(v) => {
            let answer = if let Some(Answer::SingleChoice { answer }) = answer {
                answer + 1
            } else {
                0
            };
            rsx! {
                SingleObjective {
                    question: v,
                    answer,
                    disabled,
                    onchange: move |e| {
                        onchange
                            .call(Answer::SingleChoice {
                                answer: e - 1,
                            });
                    },
                }
            }
        }
        Question::MultipleChoice(v) => {
            let answer = if let Some(Answer::MultipleChoice { answer }) = answer {
                answer
            } else {
                vec![]
            };
            rsx! {
                MultipleObjective {
                    question: v,
                    answer,
                    disabled,
                    onchange: move |e| {
                        onchange
                            .call(Answer::MultipleChoice {
                                answer: e,
                            });
                    },
                }
            }
        }
        Question::ShortAnswer(v) => {
            let answer = if let Some(Answer::ShortAnswer { answer }) = answer {
                answer
            } else {
                "".to_string()
            };
            rsx! {
                Subjective {
                    placeholder,
                    id: None,
                    question: v,
                    answer,
                    disabled,
                    onchange: move |e| {
                        onchange.call(Answer::ShortAnswer { answer: e });
                    },
                }
            }
        }
        Question::Subjective(v) => {
            let answer = if let Some(Answer::Subjective { answer }) = answer {
                answer
            } else {
                "".to_string()
            };
            rsx! {
                Subjective {
                    placeholder,
                    id: None,
                    question: v,
                    answer,
                    disabled,
                    onchange: move |e| {
                        onchange.call(Answer::Subjective { answer: e });
                    },
                }
            }
        }
    }
}
