use bdk::prelude::*;

use crate::pages::deliberations::new::details::sample_survey::components::{
    introduction::Introduction, member::SampleSurveyMember, reward::SampleSurveyReward,
};

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
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", {tr.input_introduction} }
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    Introduction {
                        lang,
                        sample_survey: sample_survey.clone(),
                        set_sample_survey: move |survey| {
                            ctrl.set_sample_survey(survey);
                        },
                    }

                    SampleSurveyReward {
                        lang,
                        sample_survey: sample_survey.clone(),
                        set_sample_survey: move |survey| {
                            ctrl.set_sample_survey(survey);
                        },
                    }

                    SampleSurveyMember {
                        lang,
                        total_committees: ctrl.get_committees(),
                        selected_committees: ctrl.get_selected_committee(),
                        sample_survey: sample_survey.clone(),
                        set_sample_survey: move |survey| {
                            ctrl.set_sample_survey(survey);
                        },
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
