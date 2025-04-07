use bdk::prelude::*;
use by_components::rich_texts::RichText;
use models::DeliberationSampleSurveyCreateRequest;

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::{
        components::calendar_dropdown::CalendarDropdown,
        details::sample_survey::i18n::IntroductionTranslate,
    },
};

#[component]
pub fn Introduction(
    lang: Language,

    sample_survey: DeliberationSampleSurveyCreateRequest,
    set_sample_survey: EventHandler<DeliberationSampleSurveyCreateRequest>,
) -> Element {
    let tr: IntroductionTranslate = translate(&lang);

    rsx! {
        ExpandableCard {
            required: true,
            header: tr.input_introduction_title,
            description: tr.input_introduction_description,
            div { class: "flex flex-col w-full justify-start items-start gap-10",
                div { class: "flex flex-row w-full gap-20",
                    div { class: "flex flex-row gap-20 px-15 w-full h-54 bg-background-gray rounded-sm justify-center items-center",
                        input {
                            class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: tr.input_title_hint,
                            value: sample_survey.clone().title,
                            oninput: {
                                let mut survey = sample_survey.clone();
                                move |e: Event<FormData>| {
                                    survey.title = e.value();
                                    set_sample_survey.call(survey.clone());
                                }
                            },
                        }
                    }

                    div { class: "flex flex-row w-fit justify-start items-center gap-10",
                        CalendarDropdown {
                            id: "sample_survey_start_date",
                            date: sample_survey.started_at,
                            onchange: {
                                let mut survey = sample_survey.clone();
                                move |e| {
                                    survey.started_at = e;
                                    set_sample_survey.call(survey.clone());
                                }
                            },
                        }

                        div { class: "flex flex-row w-16 h-2 bg-label-border-gray" }

                        CalendarDropdown {
                            id: "sample_survey_end_date",
                            date: sample_survey.ended_at,
                            onchange: {
                                let mut survey = sample_survey.clone();
                                move |e| {
                                    survey.ended_at = e;
                                    set_sample_survey.call(survey.clone());
                                }
                            },
                        }
                    }
                }

                div { class: "flex flex-row w-full h-1 bg-period-border-gray" }

                RichText {
                    id: "sample-survey-rich-text",
                    content: sample_survey.clone().description,
                    onchange: {
                        let mut survey = sample_survey.clone();
                        move |e| {
                            survey.description = e;
                            set_sample_survey.call(survey.clone());
                        }
                    },
                }
            }
        }
    }
}
