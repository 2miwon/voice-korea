use std::collections::BTreeMap;

use bdk::prelude::*;
use dioxus_popup::PopupService;
use models::{
    deliberation_response::DeliberationResponse, response::Answer, DeliberationSampleSurvey,
    DeliberationSampleSurveyQuery, DeliberationSampleSurveySummary, ParsedQuestion, ProjectStatus,
    Question, SurveyV2,
};

use crate::{
    pages::projects::_id::components::{
        sample_survey::remove_survey_modal::RemoveSurveyModal, survey::SurveyData,
    },
    service::user_service::UserService,
};

use super::i18n::RemoveSurveyModalTranslate;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SurveyStep {
    NotParticipated,
    InProgress,
    Submitted,
    MyResponse,
    Statistics,
}

impl From<DeliberationSampleSurveySummary> for SurveyData {
    fn from(survey: DeliberationSampleSurveySummary) -> Self {
        Self {
            title: survey.title,
            description: survey.description,
            start_date: survey.started_at,
            end_date: survey.ended_at,
            status: survey
                .surveys
                .get(0)
                .map_or(ProjectStatus::Ready, |s| s.status),
            roles: survey.roles.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    deliberation_id: ReadOnlySignal<i64>,
    pub user: UserService,

    popup_service: PopupService,

    pub sample_survey: Resource<DeliberationSampleSurveySummary>,
    pub answers: Signal<BTreeMap<usize, Answer>>,

    survey_step: Signal<SurveyStep>,
}

impl Controller {
    pub fn new(
        lang: Language,
        deliberation_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let user: UserService = use_context();
        let sample_survey = use_server_future(move || {
            let deliberation_id = deliberation_id();
            async move {
                let v = DeliberationSampleSurvey::get_client(&crate::config::get().api_url)
                    .query(deliberation_id, DeliberationSampleSurveyQuery::new(1))
                    .await
                    .unwrap_or_default();
                if v.total_count == 1 {
                    v.items[0].clone()
                } else {
                    DeliberationSampleSurveySummary::default()
                }
            }
        })?;
        // let initial_answers = sample_survey()
        //     .and_then(|survey| survey.user_response.get(0).cloned())
        //     .map(|response| response.answers.into_iter().enumerate().collect())
        //     .unwrap_or_else(BTreeMap::new);

        // let initial_step = if sample_survey()
        //     .is_some_and(|sample_survey| sample_survey.user_response.get(0).is_some())
        // {
        //     SurveyStep::Submitted
        // } else {
        //     SurveyStep::NotParticipated
        // };
        let prev_answers = sample_survey()
            .and_then(|survey| survey.user_response.get(0).cloned())
            .map(|response| response.answers.into_iter().enumerate().collect())
            .unwrap_or_else(BTreeMap::new);
        tracing::debug!("prev_answers : {prev_answers:?}");

        let _step = if !prev_answers.is_empty() {
            SurveyStep::Submitted
        } else {
            SurveyStep::NotParticipated
        };
        let ctrl = Self {
            lang,
            deliberation_id,
            sample_survey,
            answers: use_signal(|| prev_answers),
            user,
            popup_service: use_context(),
            // survey_step: use_signal(|| step),
            survey_step: use_signal(|| SurveyStep::Statistics),
        };

        Ok(ctrl)
    }

    pub fn get_grouped_answers(&self) -> Vec<(String, ParsedQuestion)> {
        let Some(deliberation_survey) = self.sample_survey().ok() else {
            return vec![];
        };

        let Some(questions) = deliberation_survey.surveys.get(0).map(|s| &s.questions) else {
            return vec![];
        };

        let mut parsed_questions: Vec<(String, ParsedQuestion)> = questions
            .into_iter()
            .map(|question| match question.clone() {
                Question::MultipleChoice(inner) => {
                    let title = inner.title;
                    let options = inner.options;
                    let count = vec![0; options.len()];
                    (
                        title,
                        ParsedQuestion::MultipleChoice {
                            answers: options,
                            response_count: count,
                        },
                    )
                }
                Question::SingleChoice(inner) => {
                    let title = inner.title;
                    let options = inner.options;
                    let count = vec![0; options.len()];
                    (
                        title,
                        ParsedQuestion::SingleChoice {
                            answers: options,
                            response_count: count,
                        },
                    )
                }
                Question::ShortAnswer(inner) => {
                    (inner.title, ParsedQuestion::ShortAnswer { answers: vec![] })
                }
                Question::Subjective(inner) => {
                    (inner.title, ParsedQuestion::Subjective { answers: vec![] })
                }
            })
            .collect();

        for responses in deliberation_survey.responses.into_iter() {
            for (i, answer) in responses.answers.into_iter().enumerate() {
                parsed_questions.get_mut(i).map(|(_, parsed_question)| {
                    match parsed_question {
                        ParsedQuestion::SingleChoice { response_count, .. } => {
                            if let Answer::SingleChoice { answer } = answer {
                                response_count[answer as usize] += 1;
                            }
                        }
                        ParsedQuestion::MultipleChoice { response_count, .. } => {
                            if let Answer::MultipleChoice { answer } = answer {
                                for ans in answer {
                                    response_count[ans as usize] += 1;
                                }
                            }
                        }
                        ParsedQuestion::ShortAnswer { answers } => {
                            if let Answer::ShortAnswer { answer } = answer {
                                answers.push(answer.clone());
                            }
                        }
                        ParsedQuestion::Subjective { answers } => {
                            if let Answer::Subjective { answer } = answer {
                                answers.push(answer.clone());
                            }
                        }
                    };
                });
            }
        }
        parsed_questions
    }

    pub fn get_survey(&self) -> SurveyV2 {
        if let Ok(survey) = self.sample_survey() {
            if let Some(survey) = survey.surveys.get(0) {
                return survey.clone();
            }
        }
        SurveyV2::default()
    }
    pub fn set_step(&mut self, step: SurveyStep) {
        self.survey_step.set(step);
    }

    pub fn change_answer(&mut self, index: usize, answer: Answer) {
        tracing::debug!("change_answer : {index} {answer:?}");
        self.answers.with_mut(|v| {
            v.insert(index, answer);
        });
    }

    pub async fn remove_sample_response(&mut self) {
        let user_id = (self.user.user_id)();
        let deliberation_id = (self.deliberation_id)();

        let response_id = self
            .sample_survey()
            .ok()
            .and_then(|survey| survey.user_response.get(0).map(|r| r.id))
            .unwrap_or(0);

        if user_id == 0 || response_id == 0 {
            btracing::error!("login is required");
            return;
        }
        match DeliberationResponse::get_client(&crate::config::get().api_url)
            .remove_respond_answer(deliberation_id, response_id)
            .await
        {
            Ok(_) => {
                self.sample_survey.restart();
                self.set_step(SurveyStep::NotParticipated);
            }
            Err(e) => {
                btracing::error!("update response failed with error: {:?}", e);
            }
        }
    }

    pub fn open_remove_sample_modal(&mut self) {
        let mut popup_service = self.popup_service;
        let mut ctrl = self.clone();
        let lang = self.lang;
        let tr: RemoveSurveyModalTranslate = translate(&lang);

        popup_service
            .open(rsx! {
                RemoveSurveyModal {
                    lang,
                    onclose: move |_| {
                        popup_service.close();
                    },
                    onremove: move |_| async move {
                        ctrl.remove_sample_response().await;
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_sample")
            .with_title(tr.title);
    }

    pub async fn update_survey_answers(&mut self) {
        let user_id = (self.user.user_id)();
        let deliberation_id = (self.deliberation_id)();
        let response_id = self
            .sample_survey()
            .ok()
            .and_then(|survey| survey.user_response.get(0).map(|r| r.id))
            .unwrap_or(0);

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }

        let answers: Vec<Answer> = self
            .answers()
            .iter()
            .map(|(_, answer)| answer.clone())
            .collect();

        match DeliberationResponse::get_client(&crate::config::get().api_url)
            .update_respond_answer(deliberation_id, response_id, answers)
            .await
        {
            Ok(_) => {
                self.sample_survey.restart();
            }
            Err(e) => {
                btracing::error!("update response failed with error: {:?}", e);
            }
        };
    }

    pub async fn submit_survey_answers(&mut self) {
        tracing::debug!("submit_survey_answers : {:?}", self.answers());
        let user_id = (self.user.user_id)();
        let deliberation_id = (self.deliberation_id)();

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }
        let answers: Vec<Answer> = self
            .answers()
            .iter()
            .map(|(_, answer)| answer.clone())
            .collect();
        match DeliberationResponse::get_client(&crate::config::get().api_url)
            .respond_answer(
                deliberation_id,
                answers,
                models::deliberation_response::DeliberationType::Sample,
            )
            .await
        {
            Ok(_) => {
                self.sample_survey.restart();
                self.set_step(SurveyStep::Submitted);
            }
            Err(e) => {
                btracing::error!("send response failed with error: {:?}", e);
            }
        };
    }
}
