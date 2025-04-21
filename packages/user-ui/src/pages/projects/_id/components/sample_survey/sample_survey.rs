use bdk::prelude::*;
use dioxus_translate::translate;
use models::SurveyV2;

use crate::pages::projects::_id::components::sample_survey::{
    i18n::SampleSurveyTranslate, statistics::Statistics, survey::SurveyComponent,
};

use super::{
    controllers::{Controller, SurveyStep},
    info::Info,
    my_response::MyResponse,
    submitted::Submitted,
};

#[component]
pub fn SampleSurvey(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut ctrl = Controller::new(lang, project_id)?;
    let survey = ctrl.sample_survey()?;
    let step = ctrl.survey_step();
    let tr: SampleSurveyTranslate = translate(&lang);
    rsx! {
        div {
            id: "sample-survey",
            class: "max-w-desktop:px-30 flex flex-col w-full h-fit justify-center items-center",
            ..attributes,
            if step == SurveyStep::NotParticipated {
                Info {
                    lang,
                    is_login: ctrl.user.is_login(),
                    sample_survey: survey.clone(),
                    start_date: survey.started_at,
                    end_date: survey.ended_at,
                    survey_status: ctrl.survey_status(),
                    on_process_survey: move |_| {
                        ctrl.set_step(SurveyStep::InProgress);
                    },
                }
            } else if step == SurveyStep::InProgress {
                SurveyComponent {
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
                Submitted {
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
                MyResponse {
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
                Statistics {
                    lang,
                    grouped_answers: ctrl.get_grouped_answers(),
                    onprev: move |_| {
                        ctrl.set_step(SurveyStep::Submitted);
                    },
                }
            }
        }
    }
}
