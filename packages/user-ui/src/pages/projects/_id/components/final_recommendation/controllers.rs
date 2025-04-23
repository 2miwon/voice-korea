use bdk::prelude::*;
use indexmap::IndexMap;

use models::{deliberation_draft::DeliberationDraft, ParsedQuestion};

use super::super::super::utils::group_responses_by_question;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    project_id: ReadOnlySignal<i64>,

    recommendation: Resource<DeliberationDraft>,
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
        let recommendation = use_server_future(move || {
            let deliberation_id = project_id();
            async move {
                DeliberationDraft::get_client(&crate::config::get().api_url)
                    .read(deliberation_id)
                    .await
                    .unwrap_or_default()
            }
        })?;

        let ctrl = Self {
            lang,
            project_id,
            recommendation,
        };

        Ok(ctrl)
    }

    pub fn get_grouped_responses(&self) -> Vec<(String, ParsedQuestion)> {
        let Some(recommendation) = self.recommendation().ok() else {
            return vec![];
        };

        let Some(questions) = recommendation.surveys.get(0).map(|s| &s.questions) else {
            return vec![];
        };

        group_responses_by_question(&questions, &recommendation.responses)
    }

    // pub async fn update_draft(&self, title: String, description: String) {
    //     tracing::debug!("title: {:?} description: {:?}", title, description);
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
    // }
}
