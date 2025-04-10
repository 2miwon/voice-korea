use bdk::prelude::*;
use by_components::rich_texts::RichText;
use models::DeliberationFinalSurveyCreateRequest;

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::{
        components::calendar_dropdown::CalendarDropdown,
        details::votes::i18n::IntroductionTranslate,
    },
};

#[component]
pub fn Introduction(
    lang: Language,
    final_survey: DeliberationFinalSurveyCreateRequest,
    set_final_survey: EventHandler<DeliberationFinalSurveyCreateRequest>,
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
                            value: final_survey.clone().title,
                            oninput: {
                                let mut final_survey = final_survey.clone();
                                move |e: Event<FormData>| {
                                    final_survey.title = e.value();
                                    set_final_survey.call(final_survey.clone());
                                }
                            },
                        }
                    }

                    div { class: "flex flex-row w-fit justify-start items-center gap-10",
                        CalendarDropdown {
                            id: "sample_survey_start_date",
                            date: final_survey.started_at,
                            onchange: {
                                let mut final_survey = final_survey.clone();
                                move |e| {
                                    final_survey.started_at = e;
                                    set_final_survey.call(final_survey.clone());
                                }
                            },
                        }

                        div { class: "flex flex-row w-16 h-2 bg-label-border-gray" }

                        CalendarDropdown {
                            id: "sample_survey_end_date",
                            date: final_survey.ended_at,
                            onchange: {
                                let mut final_survey = final_survey.clone();
                                move |e| {
                                    final_survey.ended_at = e;
                                    set_final_survey.call(final_survey.clone());
                                }
                            },
                        }
                    }
                }

                div { class: "flex flex-row w-full h-1 bg-period-border-gray" }

                RichText {
                    id: "sample-survey-rich-text",
                    content: final_survey.clone().description,
                    onchange: {
                        let mut final_survey = final_survey.clone();
                        move |e| {
                            final_survey.description = e;
                            set_final_survey.call(final_survey.clone());
                        }
                    },
                }
            }
        }
    }
}
