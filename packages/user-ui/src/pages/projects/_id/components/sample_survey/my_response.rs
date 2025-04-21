use std::collections::BTreeMap;

use bdk::prelude::*;
use by_components::icons::validations::Extra;
use models::{response::Answer, ProjectStatus, SurveyV2};

use crate::{
    components::icons::left_arrow::LeftArrow,
    pages::projects::_id::components::question::QuestionComponent,
};

#[component]
pub fn MyResponse(
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
    rsx! {
        div { class: "flex flex-col w-full gap-10 mb-40 mt-28",
            div { class: "flex flex-row w-full justify-between items-center mb-10",
                div { class: "flex flex-row justify-start items-center gap-8",
                    div {
                        class: "cursor-pointer w-24 h-24",
                        onclick: move |e: Event<MouseData>| {
                            onprev.call(e);
                        },
                        LeftArrow { stroke: "black" }
                    }
                    div { class: "font-semibold text-text-black text-20", {title} }
                }

                if survey.status == ProjectStatus::InProgress {
                    div { class: "group relative",
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                            button { class: "cursor-pointer", Extra {} }
                            nav { class: "border-2 bg-white invisible border-none shadow-lg rounded w-180 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                ul { class: "py-1",
                                    li {
                                        class: "px-20 py-15 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                        onclick: move |e: Event<MouseData>| {
                                            onupdate.call(e);
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

            for (i , question) in survey.questions.iter().enumerate() {
                QuestionComponent {
                    placeholder: tr.input_placehoder,
                    question: question.clone(),
                    answer: answers().get(&i).cloned(),
                    onchange: move |e: Answer| {
                        onchange.call((i, e));
                    },
                    disabled: survey.status != ProjectStatus::InProgress,
                }
            }
        }
    }
}

translate! {
    MyResponseTranslate;

    update: {
        ko: "수정하기",
        en: "Update"
    }
    remove: {
        ko: "삭제하기",
        en: "Remove"
    }

    input_placehoder: {
        ko: "내용을 입력해주세요",
        en: "Please Enter Details"
    }
}
