use super::super::components::{AssignMember, IntroductionCard};
use super::*;
use crate::pages::deliberations::new::details::sample_survey::components::{
    question::QuestionList, reward::SampleSurveyReward,
};
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn DeliberationSampleSurveySettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: SampleSurveyTranslate = translate(&lang);
    let sample_survey = ctrl.get_sample_survey();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "font-medium text-base text-text-black", {tr.input_introduction} }
                    div { class: "flex flex-col w-full justify-start items-start gap-20",
                        IntroductionCard {
                            lang,
                            description: tr.introduction_description.to_string(),
                            text_value: sample_survey.clone().title,
                            started_at: sample_survey.clone().started_at,
                            ended_at: sample_survey.clone().ended_at,
                            content: sample_survey.clone().description,
                            set_title: move |title: String| {
                                ctrl.set_title(title);
                            },
                            set_description: move |description: String| {
                                ctrl.set_description(description);
                            },
                            set_start_date: move |timestamp: i64| {
                                ctrl.set_start_date(timestamp);
                            },
                            set_end_date: move |timestamp: i64| {
                                ctrl.set_end_date(timestamp);
                            },
                        }
                        SampleSurveyReward {
                            lang,
                            sample_survey: sample_survey.clone(),
                            set_sample_survey: move |survey| {
                                ctrl.set_sample_survey(survey);
                            },
                        }

                        AssignMember {
                            lang,
                            committees: ctrl.get_committees(),
                            selected_committees: ctrl.get_selected_committee(),
                            add_committee: move |user_id: i64| {
                                ctrl.add_committee(user_id);
                            },
                            remove_committee: move |id: i64| {
                                ctrl.remove_committee(id);
                            },
                            clear_committee: move |_| {
                                ctrl.clear_committee();
                            },
                        }
                    }

                    div { class: "flex flex-col w-full justify-start items-start gap-10",
                        div { class: "font-medium text-base text-text-black", {tr.voting_items} }
                        QuestionList {
                            lang,

                            sample_survey: ctrl.get_sample_survey(),
                            set_sample_survey: move |survey| {
                                ctrl.set_sample_survey(survey);
                            },
                        }
                    }
                }
                div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {
                            ctrl.back();
                        },
                        {tr.backward}
                    }
                    div {
                        class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| async move {
                            ctrl.temp_save().await;
                        },
                        {tr.temporary_save}
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                        onclick: move |_| {
                            ctrl.next();
                        },
                        {tr.next}
                    }
                }
            }
        }
    }
}
