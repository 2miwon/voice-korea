use bdk::prelude::*;
use models::{DeliberationSampleSurveyCreateRequest, Question};

use crate::{
    components::icons::{Plus, RowMenuDial},
    pages::deliberations::new::{
        components::{objective::Objective, subjective::Subjective},
        details::sample_survey::i18n::QuestionListTranslate,
    },
};

#[component]
pub fn QuestionList(
    lang: Language,
    sample_survey: DeliberationSampleSurveyCreateRequest,

    add_question: EventHandler<MouseEvent>,
    remove_question: EventHandler<usize>,
    update_question: EventHandler<(usize, Question)>,
) -> Element {
    let tr: QuestionListTranslate = translate(&lang);
    let questions = sample_survey.surveys.clone();

    rsx! {
        for index in 0..questions.len() {
            div {
                class: "flex flex-col w-full justify-start items-start pt-5 px-40 pb-25 bg-white rounded-lg",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-full justify-center items-center mb-[10px]",
                    RowMenuDial { width: "24", height: "24" }
                }

                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row w-full justify-start items-center",
                        QuestionTypeSelector {
                            selected: questions[index].to_type(&lang),
                            lang,
                            onchange: move |qtype: String| {
                                update_question.call((index, Question::new(&qtype)));
                            },
                        }

                        input {
                            class: format!(
                                "flex flex-row flex-1 h-55 justify-start items-center bg-background-gray focus:outline-none px-15 py-10 font-medium text-hint-gray text-[15px] leading-22 rounded-sm",
                            ),
                            r#type: "text",
                            placeholder: tr.input_title_hint,
                            value: questions[index].title(),
                            oninput: {
                                let mut questions = questions.clone();
                                move |e: Event<FormData>| {
                                    questions[index].set_title(&e.value());
                                    let question = questions[index].clone();
                                    update_question.call((index, question));
                                }
                            },
                        }
                    }

                    if matches!(questions[index], Question::ShortAnswer(_) | Question::Subjective(_)) {
                        Subjective {
                            lang,
                            onchange: move |v: Question| {
                                update_question.call((index, v));
                            },
                            onremove: move |_| {
                                remove_question.call(index);
                            },
                            question: questions[index].clone(),
                        }
                    } else {
                        Objective {
                            lang,
                            onchange: move |v: Question| {
                                update_question.call((index, v));
                            },
                            onremove: move |_| {
                                remove_question.call(index);
                            },
                            question: questions[index].clone(),
                        }
                    }
                }
            }
        }

        button {
            class: "flex flex-row w-full",
            onclick: {
                move |e| {
                    add_question.call(e);
                }
            },
            AddQuestion { lang }
        }
    }
}

#[component]
pub fn AddQuestion(lang: Language) -> Element {
    let tr: QuestionListTranslate = translate(&lang);
    rsx! {
        div { class: "cursor-pointer flex flex-col w-full h-200 rounded-lg justify-center items-center border border-dashed border-hint-gray",
            div { class: "flex flex-row w-45 h-45 justify-center items-center rounded-[100px] border border-hint-gray",
                Plus { width: "12", height: "12", color: "#b4b4b4" }
            }
            div { class: "mt-10 font-medium text-[15px] text-hint-gray leading-22",
                {tr.add_question}
            }
        }
    }
}

#[component]
pub fn QuestionTypeSelector(
    lang: Language,
    selected: String,
    onchange: EventHandler<String>,
) -> Element {
    let mut selected_type = use_signal({
        let selected = selected.clone();
        move || selected.clone()
    });

    use_effect(use_reactive(&selected.clone(), move |selected| {
        selected_type.set(selected);
    }));

    rsx! {
        select {
            class: "focus:outline-none w-215 h-55 justify-start items-start p-[15px] bg-background-gray rounded-sm mr-20 font-medium text-[15px] text-hint-gray",
            value: "{selected_type}",
            onchange: move |e: Event<FormData>| {
                selected_type.set(e.value());
                onchange.call(e.value());
            },
            for question_type in Question::types(&lang) {
                option {
                    value: question_type.clone(),
                    selected: selected_type() == question_type,
                    "{question_type}"
                }
            }
        }
    }
}
