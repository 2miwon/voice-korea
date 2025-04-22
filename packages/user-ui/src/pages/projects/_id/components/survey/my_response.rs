use std::collections::BTreeMap;

use bdk::prelude::*;
use by_components::icons::validations::Extra;
use models::{response::Answer, ProjectStatus, SurveyV2};

use crate::{
    components::button::Button,
    pages::projects::_id::components::{question::QuestionComponent, tab_title::TabTitleWithPrev},
};

use super::i18n::MyResponseTranslate;

#[component]
pub fn MySurveyResponse(
    lang: Language,
    title: String,
    start_date: i64,
    end_date: i64,
    survey: SurveyV2,
    answers: ReadOnlySignal<BTreeMap<usize, Answer>>,
    onprev: EventHandler<MouseEvent>,
    onchange: EventHandler<(usize, Answer)>,
    onupdate: EventHandler<MouseEvent>,
    onremove: EventHandler<MouseEvent>,
) -> Element {
    let tr: MyResponseTranslate = translate(&lang);
    let mut editable = use_signal(|| false);
    rsx! {
        TabTitleWithPrev {
            title,
            onprev: move |e: Event<MouseData>| {
                onprev.call(e);
            },
            if survey.status == ProjectStatus::InProgress {
                div { class: "flex flex-row gap-20",
                    if editable() {
                        Button {
                            class: "text-white py-8 px-10 rounded-lg",
                            onclick: move |v| {
                                editable.set(false);
                                onupdate.call(v)
                            },
                            {tr.save}
                        }
                    }
                    div { class: "group relative",
                        div { class: "flex flex-row h-full justify-center items-center",
                            button { class: "cursor-pointer",

                                ToggableExtra { active: editable() }
                            }
                            nav { class: "bg-white invisible border-none shadow-[0px_8px_20px_0px_#141a3e40] rounded-lg w-180 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                ul {
                                    li {
                                        class: "px-20 py-15 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                        onclick: move |_e: Event<MouseData>| {
                                            editable.set(true);
                                        },
                                        {tr.update}
                                    }
                                    li {
                                        class: "px-20 py-15 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                        onclick: move |e: Event<MouseData>| {
                                            onremove.call(e);
                                        },
                                        {tr.remove}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        div { class: "flex flex-col w-full gap-10",
            for (i , question) in survey.questions.iter().enumerate() {
                QuestionComponent {
                    placeholder: tr.survey_input_placehoder,
                    question: question.clone(),
                    answer: answers().get(&i).cloned(),
                    onchange: move |e: Answer| {
                        onchange.call((i, e));
                    },
                    disabled: !editable(),
                }
            }
        }
    }
}

#[component]
pub fn ToggableExtra(active: bool) -> Element {
    if active {
        rsx! {
            Extra { class: "[&>circle]:fill-white bg-primary rounded-sm" }
        }
    } else {
        rsx! {
            Extra { class: "group-focus-within:[&>circle]:fill-white group-focus-within:bg-primary rounded-sm" }
        }
    }
}
