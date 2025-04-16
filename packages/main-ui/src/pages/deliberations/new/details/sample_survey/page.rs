use bdk::prelude::*;

use crate::pages::deliberations::new::details::sample_survey::components::{
    introduction::Introduction, member::Member, question::QuestionList, reward::Reward,
};

use models::Question;

use super::*;
use controller::*;
use i18n::*;

#[component]
pub fn DeliberationSampleSurveySettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: SampleSurveyTranslate = translate(&lang);
    let sample_survey = ctrl.get_sample_survey();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start gap-10",
                div { class: "font-medium text-base text-text-black", {tr.input_introduction} }
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    Introduction {
                        lang,
                        sample_survey: sample_survey.clone(),
                        start_date_id: "sample_survey_start_date",
                        end_date_id: "sample_survey_end_date",
                        set_title: move |title: String| {
                            ctrl.set_title(title);
                        },
                        set_description: move |description: String| {
                            ctrl.set_description(description);
                        },
                        set_start_date: move |start_date: i64| {
                            ctrl.set_start_date(start_date);
                        },
                        set_end_date: move |end_date: i64| {
                            ctrl.set_end_date(end_date);
                        },
                    }

                    Reward {
                        lang,
                        sample_survey: sample_survey.clone(),
                        set_estimate_time: move |estimate_time: i64| {
                            ctrl.set_estimate_time(estimate_time);
                        },
                        set_point: move |point: i64| {
                            ctrl.set_point(point);
                        },
                    }

                    Member {
                        lang,
                        total_committees: ctrl.get_committees(),
                        selected_committees: ctrl.get_selected_committee(),
                        add_committee: move |id: i64| {
                            ctrl.add_committee(id);
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

                        add_question: move |_| {
                            ctrl.add_question();
                        },
                        remove_question: move |index: usize| {
                            ctrl.remove_question(index);
                        },
                        update_question: move |(index, question): (usize, Question)| {
                            ctrl.update_question(index, question);
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
