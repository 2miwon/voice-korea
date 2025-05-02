#![allow(unused_variables, unused)]
use crate::deliberation_report::DeliberationReport;
use crate::deliberation_response::DeliberationResponse;
use crate::deliberation_user::DeliberationUser;
use crate::DeliberationFinalSurvey;

use crate::ResourceFile;
use crate::SurveyV2;
use bdk::prelude::*;
use validator::Validate;

#[cfg(feature = "server")]
use crate::DeliberationFinalSurveyRepositoryQueryBuilder;

//FIXME: ADD ROLES
#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/drafts", table = deliberation_drafts, read_action = read)]
pub struct DeliberationDraft {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,

    #[api_model(summary, many_to_one = deliberations)]
    pub deliberation_id: i64,

    #[api_model(skip)]
    #[serde(default)]
    pub is_member: bool,

    #[api_model(skip)]
    #[serde(default)]
    pub final_surveys: Vec<DeliberationFinalSurvey>,

    #[api_model(skip)]
    #[serde(default)]
    pub responses: Vec<DeliberationResponse>,
}

impl Into<DeliberationDraftCreateRequest> for DeliberationDraft {
    fn into(self) -> DeliberationDraftCreateRequest {
        DeliberationDraftCreateRequest {
            title: self.title,
            description: self.description,
        }
    }
}
