use bdk::prelude::*;
use dioxus_translate::{translate, Language};
use models::{prelude::Question, ProjectArea};

use crate::pages::surveys::{
    components::{introduction::InputIntroduction, reward::SurveyReward, survey::QuestionListView},
    new::i18n::CreateSurveyTranslate,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CreateSurveyResponse {
    pub title: String,
    pub description: String,
    pub start_date: i64,
    pub end_date: i64,
    pub area: ProjectArea,
    pub questions: Vec<Question>,

    pub point: i64,
    pub estimate_time: i64,
}

#[component]
pub fn CreateSurvey(
    lang: Language,
    #[props(default = false)] visibility: bool,
    #[props(default = CreateSurveyResponse::default())] value: CreateSurveyResponse,
    onnext: EventHandler<CreateSurveyResponse>,
    onchange: EventHandler<CreateSurveyResponse>,
) -> Element {
    let CreateSurveyResponse {
        title,
        description,
        start_date,
        end_date,
        area,
        questions,

        point,
        estimate_time,
    } = value.clone();

    let timestamp = chrono::Local::now().timestamp();
    let translates: CreateSurveyTranslate = translate(&lang);
    let mut point = use_signal(move || point);
    let mut estimate_time = use_signal(move || estimate_time);
    let mut title = use_signal(move || title);
    let mut description = use_signal(move || description);
    let mut start_date = use_signal(move || {
        if start_date > 0 {
            start_date
        } else {
            timestamp
        }
    });
    let mut end_date = use_signal(move || if end_date > 0 { end_date } else { timestamp });
    let mut area = use_signal(move || area);
    let mut questions = use_signal(move || questions);
    let nav = use_navigator();

    use_effect(use_reactive(&value.clone(), move |value| {
        title.set(value.title.clone());
        description.set(value.description.clone());
        questions.set(value.questions.clone());
        point.set(value.point);
        estimate_time.set(value.estimate_time);

        if value.start_date > 0 {
            start_date.set(value.start_date);
        } else {
            start_date.set(timestamp);
        }

        if value.end_date > 0 {
            if value.end_date < start_date() {
                end_date.set(start_date() + 86400);
            } else {
                end_date.set(value.end_date);
            }
        } else {
            end_date.set(start_date() + 86400);
        }

        area.set(value.area.clone());
    }));

    rsx! {
        div {
            class: format!(
                "flex flex-col w-full h-full justify-start items-start {}",
                if !visibility { "hidden" } else { "" },
            ),
            width: if !visibility { "0px" },
            height: if !visibility { "0px" },
            div { class: "flex flex-col w-full gap-20",
                InputIntroduction {
                    lang,
                    ti: title(),
                    desc: description(),
                    sd: start_date(),
                    ed: end_date(),
                    area: area(),
                    onchange_area: {
                        let value = value.clone();
                        move |field: ProjectArea| {
                            area.set(field);
                            onchange
                                .call(CreateSurveyResponse {
                                    area: field,
                                    ..value.clone()
                                })
                        }
                    },

                    onchange_title: {
                        let value = value.clone();
                        move |v: String| {
                            title.set(v.clone());
                            onchange
                                .call(CreateSurveyResponse {
                                    title: v.clone(),
                                    ..value.clone()
                                })
                        }
                    },

                    onchange_start_date: {
                        let value = value.clone();
                        move |v: i64| {
                            start_date.set(v);
                            onchange
                                .call(CreateSurveyResponse {
                                    start_date: v,
                                    ..value.clone()
                                })
                        }
                    },

                    onchange_end_date: {
                        let value = value.clone();
                        move |v: i64| {
                            end_date.set(v);
                            onchange
                                .call(CreateSurveyResponse {
                                    end_date: v,
                                    ..value.clone()
                                })
                        }
                    },

                    onchange_description: {
                        let value = value.clone();
                        move |v: String| {
                            description.set(v.clone());
                            onchange
                                .call(CreateSurveyResponse {
                                    description: v.clone(),
                                    ..value.clone()
                                })
                        }
                    },
                }

                SurveyReward {
                    lang,
                    point: point(),
                    estimate_time: estimate_time(),

                    onchange_point: {
                        let value = value.clone();
                        move |v: i64| {
                            point.set(v);
                            onchange
                                .call(CreateSurveyResponse {
                                    point: v,
                                    ..value.clone()
                                })
                        }
                    },

                    onchange_estimate_time: {
                        let value = value.clone();
                        move |v: i64| {
                            estimate_time.set(v);
                            onchange
                                .call(CreateSurveyResponse {
                                    estimate_time: v,
                                    ..value.clone()
                                })
                        }
                    },
                }

                QuestionListView {
                    lang,
                    questions,

                    onchange: move |v: Vec<Question>| {
                        onchange
                            .call(CreateSurveyResponse {
                                questions: v.clone(),
                                ..value.clone()
                            });
                    },
                }
            }

            div { class: "flex flex-row w-full justify-end items-center gap-20 text-white mt-40",
                button {
                    class: "cursor-pointer px-20 py-10 border-label-border-gray bg-white border !text-davy-gray font-semibold text-sm rounded-sm",
                    onclick: move |_| {
                        nav.go_back();
                    },
                    "{translates.btn_back}"
                }

                button {
                    class: "cursor-pointer px-20 py-10 bg-hover font-semibold text-sm rounded-sm",
                    onclick: move |_| async move {
                        onnext(CreateSurveyResponse {
                            title: title(),
                            description: description(),
                            start_date: start_date(),
                            end_date: end_date(),
                            area: area(),
                            questions: questions(),
                            point: point(),
                            estimate_time: estimate_time(),
                        });
                    },
                    "{translates.btn_next}"
                }
            }
        }
    }
}
