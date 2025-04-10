use bdk::prelude::*;
use models::SurveyV2;

use super::{
    controllers::Controller, my_sample_survey::MySampleSurvey, sample_statistics::SampleStatistics,
    sample_survey_info::SampleSurveyInfo, sample_survey_question::SampleSurveyQuestion,
};

use crate::utils::time::current_timestamp;

#[derive(Translate, PartialEq, Default, Debug)]
pub enum SurveyStatus {
    #[default]
    #[translate(ko = "조사가 준비중입니다.", en = "The investigation is underway.")]
    Ready,
    #[translate(ko = "조사 참여하기", en = "Take part in the survey")]
    InProgress,
    #[translate(
        ko = "조사가 마감되었습니다.",
        en = "The investigation has been closed."
    )]
    Finish,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SurveyStep {
    Display,
    WriteSurvey,
    MySurvey,
    Statistics,
}

#[component]
pub fn SampleSurvey(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut ctrl = Controller::new(lang, project_id)?;
    let survey = ctrl.sample_survey()?;

    let step = ctrl.survey_step();

    rsx! {
        div {
            id: "sample-survey",
            class: "max-[1000px]:px-30 flex flex-col w-full h-fit justify-center items-center",
            ..attributes,

            if step == SurveyStep::Display {
                SampleSurveyInfo {
                    lang,
                    sample_survey: survey.clone(),
                    start_date: survey.started_at,
                    end_date: survey.ended_at,
                    survey_completed: ctrl.survey_completed(),
                    onchange: move |step| {
                        ctrl.set_step(step);
                    },
                }
            } else if step == SurveyStep::WriteSurvey {
                SampleSurveyQuestion {
                    lang,
                    survey: if survey.surveys.len() != 0 { survey.surveys[0].clone() } else { SurveyV2::default() },
                    answers: ctrl.answers(),
                    onprev: move |_| {
                        ctrl.set_step(SurveyStep::Display);
                    },
                    onsend: move |_| async move {
                        ctrl.send_sample_response().await;
                        ctrl.set_step(SurveyStep::Display);
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                }
            } else if step == SurveyStep::MySurvey {
                MySampleSurvey {
                    lang,
                    start_date: survey.started_at,
                    end_date: survey.ended_at,
                    survey: if survey.surveys.len() != 0 { survey.surveys[0].clone() } else { SurveyV2::default() },
                    answers: ctrl.answers(),
                    onprev: move |_| {
                        ctrl.set_step(SurveyStep::Display);
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                    onupdate: move |_| async move {
                        ctrl.update_sample_response().await;
                        ctrl.set_step(SurveyStep::Display);
                    },
                    onremove: move |_| async move {
                        ctrl.open_remove_sample_modal();
                    },
                }
            } else {
                SampleStatistics {
                    lang,
                    responses: ctrl.survey_responses(),
                    onprev: move |_| {
                        ctrl.set_step(SurveyStep::Display);
                    },
                }
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
