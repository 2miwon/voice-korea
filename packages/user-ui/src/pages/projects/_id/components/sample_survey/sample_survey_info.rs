#![allow(non_snake_case, dead_code, unused_variables)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{DeliberationSampleSurveySummary, Tab};

use crate::{
    components::icons::right_arrow::RightArrow,
    pages::projects::_id::components::{accordion::Accordion, rich_text_viewer::RichTextViewer},
    utils::time::{current_timestamp, formatted_timestamp},
};

use super::{
    i18n::SampleSurveyTranslate,
    sample_survey::{SurveyStatus, SurveyStep},
};

#[component]
pub fn SampleSurveyInfo(
    lang: Language,
    sample_survey: DeliberationSampleSurveySummary,
    survey_completed: bool,
    start_date: i64,
    end_date: i64,
    onchange: EventHandler<SurveyStep>,
) -> Element {
    let tab_title: &str = Tab::SampleSurvey.translate(&lang);
    let status = get_survey_status(start_date, end_date);
    let tr: SampleSurveyTranslate = translate(&lang);

    let title = if sample_survey.surveys.is_empty() {
        "".to_string()
    } else {
        sample_survey.surveys[0].name.clone()
    };

    let description = if sample_survey.surveys.is_empty() {
        "".to_string()
    } else {
        sample_survey.surveys[0].description.clone()
    };

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-10",
            div { class: "flex flex-col w-full h-fit bg-box-gray gap-20",

                // header
                div { class: "w-full flex max-[500px]:flex-col max-[500px]:items-start max-[500px]:justify-start max-[500px]:gap-5 flex-row justify-between items-center mt-28",
                    div { class: " font-semibold text-[20px]", "{tab_title}" }
                    div { class: "font-medium text-[15px] text-black",
                        {
                            format!(
                                "{} ~ {}",
                                formatted_timestamp(lang, start_date),
                                formatted_timestamp(lang, end_date),
                            )
                        }
                    }
                }

                // information section
                div {
                    style: if survey_completed { "display: none;" } else { "" },
                    class: "flex flex-col gap-10",
                    // introduction section

                    Accordion { title: tr.title, default_open: true,
                        div { class: "w-full justify-start mt-15 mb-20 font-bold text-lg",
                            "{title}"
                        }
                        RichTextViewer {
                            class: "w-full flex justify-start text-[15px]",
                            contenteditable: false,
                            html: description,
                        }
                                        // FIXME: fix to query by members field
                    // div { class: "w-full mt-20 flex flex-row justify-start gap-40",
                    //     for member in sample_survey.members {
                    //         div { class: "flex flex-row justify-start gap-8",
                    //             img { class: "w-40 h-40 bg-profile-gray rounded-full" }
                    //             div { class: "flex flex-col justify-start",
                    //                 p { class: "font-semibold text-[15px] justify-start",
                    //                     {member.role.translate(&lang)}
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
                    }
                }

                // information section when survey completed
                div {
                    class: "flex flex-col w-full gap-10",
                    style: if survey_completed { "" } else { "display: none;" },
                    SampleLinkComponent {
                        lang,
                        title: tr.my_answer,
                        onclick: move |_| {
                            onchange.call(SurveyStep::MySurvey);
                        },
                    }
                    SampleLinkComponent {
                        lang,
                        title: tr.response_per_question,
                        onclick: move |_| {
                            onchange.call(SurveyStep::Statistics);
                        },
                    }
                }
            }

            div { class: "flex flex-row w-full justify-center mb-40",
                div {
                    style: if sample_survey.surveys.is_empty() || survey_completed { "display: none;" } else { "" },
                    class: format!(
                        "flex flex-row px-15 py-13 {} rounded-lg font-bold text-white text-base",
                        if status == SurveyStatus::InProgress {
                            "bg-button-primary cursor-pointer"
                        } else {
                            "bg-hint-gray cursor-not-allowed"
                        },
                    ),
                    onclick: move |_| {
                        if status == SurveyStatus::InProgress {
                            onchange.call(SurveyStep::WriteSurvey);
                        }
                    },
                    {status.translate(&lang)}
                }
            }
        }
    }
}

#[component]
pub fn SampleLinkComponent(
    lang: Language,
    title: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let tr: SampleSurveyTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center px-20 py-9 bg-white rounded-lg",
            div { class: "font-bold text-base text-text-black", "{title}" }
            div { class: "flex flex-row justify-start items-center gap-5",
                div {
                    class: "cursor-pointer font-semibold text-optional-blue text-sm underline",
                    onclick: move |e: Event<MouseData>| {
                        onclick.call(e);
                    },
                    "{tr.see_detail}"
                }
                RightArrow { color: "#555462" }
            }
        }
    }
}

pub fn get_survey_status(started_at: i64, ended_at: i64) -> SurveyStatus {
    let current = current_timestamp();

    if started_at > 10000000000 {
        tracing::error!("time parsing failed");
        return SurveyStatus::default();
    }

    if started_at > current {
        SurveyStatus::Ready
    } else if ended_at < current {
        SurveyStatus::Finish
    } else {
        SurveyStatus::InProgress
    }
}
