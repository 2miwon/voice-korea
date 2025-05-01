#![allow(unused_variables, unused)]
use bdk::prelude::*;
use validator::Validate;

use crate::deliberation_panel_email::DeliberationPanelEmail;
use crate::deliberation_response::*;
use crate::deliberation_role::DeliberationRole;
use crate::deliberation_user::DeliberationUser;
use crate::Question;
use crate::SurveyV2;
use crate::User;

//FIXME: read_action = get_by_id rename action
#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/final-surveys", table = deliberation_final_surveys, read_action = get_by_id , action = [create(users = Vec<String>, surveys = Vec<Question>)])]
pub struct DeliberationFinalSurvey {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    // started_at indicates the start time of the deliberation.
    #[api_model(summary, action = create)]
    pub started_at: i64,
    // ended_at indicates the end time of the deliberation.
    #[api_model(summary, action = create)]
    pub ended_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,

    #[api_model(summary, many_to_one = deliberations)]
    pub deliberation_id: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    #[serde(default)]
    pub estimate_time: i64,
    #[api_model(summary, action = create, action_by_id = update)]
    #[serde(default)]
    pub point: i64,

    #[api_model(summary, one_to_many = deliberation_panel_emails, foreign_key = deliberation_id, reference_key = deliberation_id)]
    #[serde(default)]
    pub emails: Vec<DeliberationPanelEmail>,
    #[api_model(summary, many_to_many = deliberation_final_survey_roles, foreign_table_name = deliberation_roles, foreign_primary_key = role_id, foreign_reference_key = final_survey_id)]
    #[serde(default)]
    pub roles: Vec<DeliberationRole>,

    //FIXME: this field will be deprecated. use roles field instead.
    #[api_model(skip)]
    #[serde(default)]
    pub is_member: bool,

    #[api_model(summary, many_to_many = deliberation_final_survey_surveys, foreign_table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = final_survey_id)]
    #[serde(default)]
    pub surveys: Vec<SurveyV2>,

    #[api_model(skip)]
    #[serde(default)]
    pub responses: Vec<DeliberationResponse>,

    #[api_model(skip)]
    #[serde(default)]
    pub user_response: Vec<DeliberationResponse>,
}

impl Into<DeliberationFinalSurveyCreateRequest> for DeliberationFinalSurvey {
    fn into(self) -> DeliberationFinalSurveyCreateRequest {
        DeliberationFinalSurveyCreateRequest {
            users: self.roles.into_iter().map(|u| u.email).collect(),
            surveys: self
                .surveys
                .into_iter()
                .map(|s| s.questions)
                .flatten()
                .collect(),
            started_at: self.started_at,
            ended_at: self.ended_at,
            title: self.title,
            description: self.description,
            estimate_time: self.estimate_time,
            point: self.point,
        }
    }
}
