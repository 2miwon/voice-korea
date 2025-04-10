use bdk::prelude::*;
use indexmap::IndexMap;

use models::{
    deliberation_draft::DeliberationDraft,
    deliberation_response::{DeliberationResponse, DeliberationType},
    response::Answer,
    DeliberationDraftQuery, DeliberationDraftSummary, ParsedQuestion, Question,
};

use crate::service::user_service::UserService;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(unused)]
    project_id: ReadOnlySignal<i64>,
    user_id: Signal<i64>,

    draft: Resource<DeliberationDraftSummary>,
    pub survey_responses: Signal<FinalSurveyResponses>,
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

        let draft = use_server_future(move || async move {
            let res = DeliberationDraft::get_client(&crate::config::get().api_url)
                .query(project_id(), DeliberationDraftQuery::new(1).with_page(1))
                .await
                .unwrap_or_default();
            if res.items.is_empty() {
                DeliberationDraftSummary::default()
            } else {
                res.items[0].clone()
            }
        })?;

        let mut ctrl = Self {
            lang,
            project_id,
            user_id: use_signal(|| (user.user_id)()),
            draft,
            survey_responses: use_signal(|| FinalSurveyResponses::default()),
        };

        use_effect(move || {
            let questions = if (ctrl.draft)().unwrap_or_default().surveys.is_empty() {
                vec![]
            } else {
                (ctrl.draft)().unwrap_or_default().surveys[0]
                    .clone()
                    .questions
            };
            let responses = (ctrl.draft)().unwrap_or_default().responses;

            let survey_responses = FinalSurveyResponses {
                answers: ctrl
                    .clone()
                    .parsing_final_answers(questions.clone(), responses.clone()),
            };

            ctrl.survey_responses.set(survey_responses);
        });

        Ok(ctrl)
    }

    pub async fn update_draft(&self, title: String, description: String) {
        tracing::debug!("title: {:?} description: {:?}", title, description);
        // let draft = self.draft().unwrap_or_default();
        // let deliberation_id = self.project_id();

        // let reports = draft.reports;

        // if reports.is_empty() {
        //     match DeliberationReport::get_client(&crate::config::get().api_url)
        //         .create(
        //             draft.org_id,
        //             deliberation_id,
        //             title,
        //             description,
        //             DeliberationReportStatus::Draft,
        //         )
        //         .await
        //     {
        //         Ok(_) => {
        //             self.draft.restart();
        //         }
        //         Err(e) => {
        //             btracing::error!("change report failed with error: {:?}", e);
        //         }
        //     };
        // } else {
        //     let id = reports[0].id;

        //     match DeliberationReport::get_client(&crate::config::get().api_url)
        //         .update(draft.org_id, id, title, description)
        //         .await
        //     {
        //         Ok(_) => {
        //             self.draft.restart();
        //         }
        //         Err(e) => {
        //             btracing::error!("change report failed with error: {:?}", e);
        //         }
        //     };
        // }
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
}
