use bdk::prelude::*;
use dioxus_popup::PopupService;
use indexmap::IndexMap;
use models::{
    deliberation_response::{DeliberationResponse, DeliberationType},
    response::Answer,
    DeliberationSampleSurvey, DeliberationSampleSurveyQuery, DeliberationSampleSurveySummary,
    ParsedQuestion, Question,
};

use crate::{
    pages::projects::_id::components::sample_survey::remove_survey_modal::RemoveSurveyModal,
    service::user_service::UserService,
};

use super::{i18n::RemoveSurveyModalTranslate, sample_survey::SurveyStep};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SampleSurveyResponses {
    pub answers: IndexMap<i64, (String, ParsedQuestion)>, // question_id, (title, response_count, <panel_id, answer>)
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    deliberation_id: ReadOnlySignal<i64>,

    pub sample_survey: Resource<DeliberationSampleSurveySummary>,
    answers: Signal<Vec<Answer>>,

    survey_completed: Signal<bool>,
    response_id: Signal<i64>,

    pub user: UserService,
    pub survey_responses: Signal<SampleSurveyResponses>,
    popup_service: PopupService,
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

        let mut ctrl = Self {
            lang,
            deliberation_id,
            sample_survey,
            answers: use_signal(|| vec![]),
            survey_completed: use_signal(|| false),
            response_id: use_signal(|| 0),

            user,
            survey_responses: use_signal(|| SampleSurveyResponses::default()),
            popup_service: use_context(),
            survey_step: use_signal(|| SurveyStep::Display),
        };

        use_effect(move || {
            let mut answers = vec![];
            let mut completed = false;
            let mut response_id = 0;

            let user_id = (ctrl.user.user_id)();

            let questions = if (ctrl.sample_survey)()
                .unwrap_or_default()
                .surveys
                .is_empty()
            {
                vec![]
            } else {
                (ctrl.sample_survey)().unwrap_or_default().surveys[0]
                    .clone()
                    .questions
            };
            let responses = (ctrl.sample_survey)().unwrap_or_default().responses;

            let survey_responses = SampleSurveyResponses {
                answers: ctrl
                    .clone()
                    .parsing_sample_answers(questions.clone(), responses.clone()),
            };

            for response in (ctrl.sample_survey)().unwrap_or_default().responses {
                if response.deliberation_type == DeliberationType::Sample
                    && response.user_id == user_id
                {
                    answers = response.answers;
                    completed = true;
                    response_id = response.id;
                }
            }

            // if answers.len() == 0 {
            //     answers = survey
            //         .questions
            //         .iter()
            //         .map(|question| match question {
            //             Question::SingleChoice(_) => Answer::SingleChoice { answer: 0 },
            //             Question::MultipleChoice(_) => Answer::MultipleChoice { answer: vec![] },
            //             Question::ShortAnswer(_) => Answer::ShortAnswer {
            //                 answer: "".to_string(),
            //             },
            //             Question::Subjective(_) => Answer::Subjective {
            //                 answer: "".to_string(),
            //             },
            //         })
            //         .collect::<Vec<_>>();
            // }

            ctrl.survey_responses.set(survey_responses);
            ctrl.answers.set(answers);
            ctrl.survey_completed.set(completed);
            ctrl.response_id.set(response_id);
        });

        Ok(ctrl)
    }

    pub fn parsing_sample_answers(
        &self,
        questions: Vec<Question>,
        responses: Vec<DeliberationResponse>,
    ) -> IndexMap<i64, (String, ParsedQuestion)> {
        let mut survey_maps: IndexMap<i64, (String, ParsedQuestion)> = IndexMap::new();

        for response in responses {
            if response.deliberation_type == DeliberationType::Survey {
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

    pub fn set_step(&mut self, step: SurveyStep) {
        self.survey_step.set(step);
    }

    pub fn change_answer(&mut self, index: usize, answer: Answer) {
        let mut answers = self.answers();
        answers[index] = answer;
        self.answers.set(answers.clone());
    }

    pub async fn remove_sample_response(&mut self) {
        let user_id = (self.user.user_id)();
        let deliberation_id = (self.deliberation_id)();
        let response_id = (self.response_id)();

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }

        match DeliberationResponse::get_client(&crate::config::get().api_url)
            .remove_respond_answer(deliberation_id, response_id)
            .await
        {
            Ok(_) => {
                self.sample_survey.restart();
                self.set_step(SurveyStep::Display);
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

    pub async fn update_sample_response(&mut self) {
        let user_id = (self.user.user_id)();
        let deliberation_id = (self.deliberation_id)();
        let response_id = (self.response_id)();

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }

        let answers = self.answers();

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

    pub async fn send_sample_response(&mut self) {
        let user_id = (self.user.user_id)();
        let deliberation_id = (self.deliberation_id)();

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }

        let answers = self.answers();

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
            }
            Err(e) => {
                btracing::error!("send response failed with error: {:?}", e);
            }
        };
    }
}
