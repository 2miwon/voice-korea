use bdk::prelude::*;
use models::SurveyV2;

use crate::pages::projects::_id::components::final_survey::controllers::FinalSurveyStep;

use super::{
    controllers::Controller, final_statistics::FinalStatistics, final_survey_info::FinalSurveyInfo,
    final_survey_question::FinalSurveyQuestion, my_final_survey::MyFinalSurvey,
};

#[component]
pub fn FinalSurvey(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut ctrl = Controller::new(lang, project_id)?;
    let survey = ctrl.survey()?;

    let step = ctrl.get_step();

    rsx! {
        div { id: "final-survey", ..attributes,
            if step == FinalSurveyStep::Display {
                FinalSurveyInfo {
                    lang,
                    survey: ctrl.survey()?,
                    start_date: survey.started_at,
                    end_date: survey.ended_at,
                    survey_completed: ctrl.survey_completed(),
                    onchange: move |step| {
                        ctrl.set_step(step);
                    },
                }
            } else if step == FinalSurveyStep::WriteSurvey {
                FinalSurveyQuestion {
                    lang,
                    survey: if survey.surveys.len() != 0 { survey.surveys[0].clone() } else { SurveyV2::default() },
                    answers: ctrl.answers(),
                    onprev: move |_| {
                        ctrl.set_step(FinalSurveyStep::Display);
                    },
                    onsend: move |_| async move {
                        ctrl.open_send_survey_modal();
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                }
            } else if step == FinalSurveyStep::MySurvey {
                MyFinalSurvey {
                    lang,
                    survey: if survey.surveys.len() != 0 { survey.surveys[0].clone() } else { SurveyV2::default() },
                    answers: ctrl.answers(),
                    onprev: move |_| {
                        ctrl.set_step(FinalSurveyStep::Display);
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                }
            } else {
                FinalStatistics {
                    lang,
                    responses: ctrl.survey_responses(),
                    onprev: move |_| {
                        ctrl.set_step(FinalSurveyStep::Display);
                    },
                }
            }
        }
    }
}

// pub fn get_survey_status(started_at: i64, ended_at: i64) -> FinalSurveyStatus {
//     let current = current_timestamp();

//     if started_at > 10000000000 {
//         tracing::error!("time parsing failed");
//         return FinalSurveyStatus::default();
//     }

//     if started_at > current {
//         FinalSurveyStatus::Ready
//     } else if ended_at < current {
//         FinalSurveyStatus::Finish
//     } else {
//         FinalSurveyStatus::InProgress
//     }
// }
