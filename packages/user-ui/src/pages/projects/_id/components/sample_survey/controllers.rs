use std::collections::BTreeMap;

use bdk::prelude::*;
use dioxus_popup::PopupService;
use models::{
    deliberation_response::DeliberationResponse, response::Answer, DeliberationSampleSurvey,
    ParsedQuestion, ProjectStatus, SurveyV2,
};

use super::super::super::utils::group_responses_by_question;
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

impl From<DeliberationSampleSurvey> for SurveyData {
    fn from(survey: DeliberationSampleSurvey) -> Self {
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

    pub sample_survey: Resource<DeliberationSampleSurvey>,
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
                DeliberationSampleSurvey::get_client(&crate::config::get().api_url)
                    .get_by_id(deliberation_id)
                    .await
                    .unwrap_or_default()
            }
        })?;

        let prev_answers = sample_survey()
            .and_then(|survey| survey.user_response.get(0).cloned())
            .map(|response| response.answers.into_iter().enumerate().collect())
            .unwrap_or_else(BTreeMap::new);
        tracing::debug!("prev_answers : {prev_answers:?}");

        let step = if !prev_answers.is_empty() {
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
            survey_step: use_signal(|| step),
        };

        Ok(ctrl)
    }

    pub fn get_grouped_responses(&self) -> Vec<(String, ParsedQuestion)> {
        let Some(deliberation_survey) = self.sample_survey().ok() else {
            return vec![];
        };

        let Some(questions) = deliberation_survey.surveys.get(0).map(|s| &s.questions) else {
            return vec![];
        };

        group_responses_by_question(&questions, &deliberation_survey.responses)
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

        if user_id == 0 || response_id == 0 {
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
