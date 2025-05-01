use bdk::prelude::*;
use dioxus_translate::translate;
use models::SurveyV2;

use crate::pages::projects::_id::components::{
    sample_survey::i18n::SampleSurveyTranslate,
    section::Section,
    survey::{MySurveyResponse, SurveyInfo, SurveyProgress, SurveyResult, SurveyStatistics},
};

use super::controllers::{Controller, SurveyStep};

#[component]
pub fn SampleSurvey(lang: Language, project_id: ReadOnlySignal<i64>) -> Element {
    let mut ctrl = Controller::new(lang, project_id)?;
    let survey = ctrl.sample_survey()?;
    let step = ctrl.survey_step();
    let tr: SampleSurveyTranslate = translate(&lang);
    rsx! {
        Section { id: "sample-survey",
            if step == SurveyStep::NotParticipated {
                SurveyInfo {
                    lang,
                    title: tr.title,
                    data: survey.clone().into(),
                    is_login: ctrl.user.is_login(),
                    is_member: survey.is_member,
                    on_process_survey: move |_| {
                        ctrl.set_step(SurveyStep::InProgress);
                    },
                }
            } else if step == SurveyStep::InProgress {
                SurveyProgress {
                    lang,
                    survey: ctrl.get_survey(),
                    answers: ctrl.answers(),
                    onprev: move |_| {
                        ctrl.set_step(SurveyStep::NotParticipated);
                    },
                    onsend: move |_| async move {
                        ctrl.submit_survey_answers().await;
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                }
            } else if step == SurveyStep::Submitted {
                SurveyResult {
                    lang,
                    start_date: survey.started_at,
                    end_date: survey.ended_at,
                    on_response_click: move |_| {
                        ctrl.set_step(SurveyStep::MyResponse);
                    },
                    on_statistic_click: move |_| {
                        ctrl.set_step(SurveyStep::Statistics);
                    },
                }
            } else if step == SurveyStep::MyResponse {
                MySurveyResponse {
                    lang,
                    title: tr.my_answer,
                    start_date: survey.started_at,
                    end_date: survey.ended_at,
                    survey: if survey.surveys.len() != 0 { survey.surveys[0].clone() } else { SurveyV2::default() },
                    answers: ctrl.answers,
                    onprev: move |_| {
                        ctrl.set_step(SurveyStep::Submitted);
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                    onupdate: move |_| async move {
                        ctrl.update_survey_answers().await;
                    },
                    onremove: move |_| async move {
                        ctrl.open_remove_sample_modal();
                    },
                }
            } else if step == SurveyStep::Statistics {
                SurveyStatistics {
                    lang,
                    grouped_answers: ctrl.get_grouped_responses(),
                    onprev: move |_| {
                        ctrl.set_step(SurveyStep::Submitted);
                    },
                }
            }
        }
    }
}
