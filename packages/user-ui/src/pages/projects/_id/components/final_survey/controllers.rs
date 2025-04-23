use std::collections::BTreeMap;

use bdk::prelude::*;

use models::{
    deliberation_response::DeliberationResponse, response::Answer, DeliberationFinalSurvey,
    ParsedQuestion, ProjectStatus, SurveyV2,
};

use super::super::super::utils::group_responses_by_question;
use super::final_vote_modal::FinalVoteModal;
use crate::{
    pages::projects::_id::components::survey::SurveyData, service::user_service::UserService,
};

use dioxus_popup::PopupService;

use super::i18n::FinalVoteModalTranslate;

#[derive(Translate, PartialEq, Default, Debug)]
pub enum FinalSurveyStatus {
    #[default]
    #[translate(ko = "투표가 준비중입니다.", en = "Voting is in preparation.")]
    Ready,
    #[translate(ko = "투표 참여하기", en = "Take part in the vote.")]
    InProgress,
    #[translate(ko = "투표가 마감되었습니다.", en = "Voting is now closed.")]
    Finish,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SurveyStep {
    NotParticipated,
    InProgress,
    Submitted,
    MyResponse,
    Statistics,
}

impl From<DeliberationFinalSurvey> for SurveyData {
    fn from(survey: DeliberationFinalSurvey) -> Self {
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
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    pub user: UserService,

    popup_service: PopupService,

    survey: Resource<DeliberationFinalSurvey>,
    pub answers: Signal<BTreeMap<usize, Answer>>,

    survey_step: Signal<SurveyStep>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let user: UserService = use_context();

        let survey = use_server_future(move || {
            let deliberation_id = project_id();
            async move {
                DeliberationFinalSurvey::get_client(&crate::config::get().api_url)
                    .get_by_id(deliberation_id)
                    .await
                    .unwrap_or_default()
            }
        })?;

        let prev_answers = survey()
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
            project_id,
            survey,
            answers: use_signal(|| prev_answers),
            user,
            popup_service: use_context(),
            survey_step: use_signal(|| step),
        };

        Ok(ctrl)
    }

    pub fn get_grouped_responses(&self) -> Vec<(String, ParsedQuestion)> {
        let Some(deliberation_survey) = self.survey().ok() else {
            return vec![];
        };

        let Some(questions) = deliberation_survey.surveys.get(0).map(|s| &s.questions) else {
            return vec![];
        };

        group_responses_by_question(&questions, &deliberation_survey.responses)
    }

    pub fn get_survey(&self) -> SurveyV2 {
        if let Ok(survey) = self.survey() {
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

    pub fn open_send_survey_modal(&mut self) {
        let mut popup_service = self.popup_service;
        let mut ctrl = self.clone();
        let lang = self.lang;
        let tr: FinalVoteModalTranslate = translate(&lang);

        popup_service
            .open(rsx! {
                FinalVoteModal {
                    lang,
                    oncancel: move |_| {
                        popup_service.close();
                    },
                    onsend: move |_| async move {
                        ctrl.submit_survey_answers().await;
                        popup_service.close();
                    },
                }
            })
            .with_id("send_survey")
            .with_title(tr.title);
    }

    pub async fn submit_survey_answers(&mut self) {
        let user_id = (self.user.user_id)();
        let deliberation_id = (self.project_id)();

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
                models::deliberation_response::DeliberationType::Survey,
            )
            .await
        {
            Ok(_) => {
                self.survey.restart();
                self.set_step(SurveyStep::Submitted);
            }
            Err(e) => {
                btracing::error!("send response failed with error: {:?}", e);
            }
        };
    }
}
