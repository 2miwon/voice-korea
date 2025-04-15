use bdk::prelude::*;

use indexmap::IndexMap;
use models::{
    deliberation_response::{DeliberationResponse, DeliberationType},
    response::Answer,
    DeliberationFinalSurvey, DeliberationFinalSurveyQuery, DeliberationFinalSurveySummary,
    ParsedQuestion, Question, SurveyV2,
};

use crate::{
    pages::projects::_id::components::final_survey::final_vote_modal::FinalVoteModal,
    service::user_service::UserService,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FinalSurveyStep {
    Display,
    WriteSurvey,
    MySurvey,
    Statistics,
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,

    survey: Resource<DeliberationFinalSurveySummary>,
    answers: Signal<Vec<Answer>>,

    survey_completed: Signal<bool>,
    response_id: Signal<i64>,

    pub user: UserService,
    pub survey_responses: Signal<FinalSurveyResponses>,
    popup_service: PopupService,
    survey_step: Signal<FinalSurveyStep>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FinalSurveyResponses {
    pub answers: IndexMap<i64, (String, ParsedQuestion)>, // question_id, (title, response_count, <panel_id, answer>)
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let user: UserService = use_context();

        let survey = use_server_future(move || async move {
            let res = DeliberationFinalSurvey::get_client(&crate::config::get().api_url)
                .query(project_id(), DeliberationFinalSurveyQuery::new(1))
                .await
                .unwrap_or_default();
            if res.items.is_empty() {
                DeliberationFinalSurveySummary::default()
            } else {
                res.items[0].clone()
            }
        })?;

        let mut ctrl = Self {
            lang,
            project_id,
            survey,

            answers: use_signal(|| vec![]),
            survey_completed: use_signal(|| false),
            response_id: use_signal(|| 0),

            user,
            survey_responses: use_signal(|| FinalSurveyResponses::default()),
            popup_service: use_context(),
            survey_step: use_signal(|| FinalSurveyStep::Display),
        };

        use_effect(move || {
            let surveys = (ctrl.survey)().unwrap_or_default().surveys;
            let survey = if surveys.len() == 0 {
                SurveyV2::default()
            } else {
                surveys[0].clone()
            };

            let mut answers = vec![];
            let mut completed = false;
            let mut response_id = 0;

            let user_id = (ctrl.user.user_id)();

            let questions = if (ctrl.survey)().unwrap_or_default().surveys.is_empty() {
                vec![]
            } else {
                (ctrl.survey)().unwrap_or_default().surveys[0]
                    .clone()
                    .questions
            };
            let responses = (ctrl.survey)().unwrap_or_default().responses;

            let survey_responses = FinalSurveyResponses {
                answers: ctrl
                    .clone()
                    .parsing_final_answers(questions.clone(), responses.clone()),
            };

            for response in (ctrl.survey)().unwrap_or_default().responses {
                if response.deliberation_type == DeliberationType::Survey
                    && response.user_id == user_id
                {
                    answers = response.answers;
                    completed = true;
                    response_id = response.id;
                }
            }

            if answers.len() == 0 {
                answers = survey
                    .questions
                    .iter()
                    .map(|question| match question {
                        Question::SingleChoice(_) => Answer::SingleChoice { answer: 0 },
                        Question::MultipleChoice(_) => Answer::MultipleChoice { answer: vec![] },
                        Question::ShortAnswer(_) => Answer::ShortAnswer {
                            answer: "".to_string(),
                        },
                        Question::Subjective(_) => Answer::Subjective {
                            answer: "".to_string(),
                        },
                    })
                    .collect::<Vec<_>>();
            }

            ctrl.survey_responses.set(survey_responses);
            ctrl.answers.set(answers);
            ctrl.survey_completed.set(completed);
            ctrl.response_id.set(response_id);
        });

        Ok(ctrl)
    }

    pub fn parsing_final_answers(
        &self,
        questions: Vec<Question>,
        responses: Vec<DeliberationResponse>,
    ) -> IndexMap<i64, (String, ParsedQuestion)> {
        let mut survey_maps: IndexMap<i64, (String, ParsedQuestion)> = IndexMap::new();

        for response in responses {
            if response.deliberation_type == DeliberationType::Sample {
                continue;
            }

            for (i, answer) in response.answers.iter().enumerate() {
                let questions = questions.clone();
                let question = &questions[i];
                let title = question.title();

                let parsed_question: ParsedQuestion = (question, answer).into();

                survey_maps
                    .entry(i as i64)
                    .and_modify(|survey_data| match &mut survey_data.1 {
                        ParsedQuestion::SingleChoice { response_count, .. } => {
                            if let Answer::SingleChoice { answer } = answer {
                                response_count[(answer - 1) as usize] += 1;
                            }
                        }
                        ParsedQuestion::MultipleChoice { response_count, .. } => {
                            if let Answer::MultipleChoice { answer } = answer {
                                for ans in answer {
                                    response_count[(ans - 1) as usize] += 1;
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
                    })
                    .or_insert_with(|| (title, parsed_question.clone()));
            }
        }

        survey_maps
    }

    pub fn set_step(&mut self, step: FinalSurveyStep) {
        self.survey_step.set(step);
    }

    pub fn get_step(&mut self) -> FinalSurveyStep {
        (self.survey_step)()
    }

    pub fn change_answer(&mut self, index: usize, answer: Answer) {
        let mut answers = self.answers();
        answers[index] = answer;
        self.answers.set(answers.clone());
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
                        ctrl.send_final_response().await;
                        popup_service.close();
                    },
                }
            })
            .with_id("send_survey")
            .with_title(tr.title);
    }

    pub async fn send_final_response(&mut self) {
        let user_id = (self.user.user_id)();
        let deliberation_id = (self.project_id)();

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }

        let answers = self.answers();

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
                self.set_step(FinalSurveyStep::Display);
            }
            Err(e) => {
                btracing::error!("send response failed with error: {:?}", e);
            }
        };
    }
}
