#![allow(unused_variables, unused)]
use bdk::prelude::*;
use validator::Validate;

use crate::deliberation_response::DeliberationResponse;
use crate::deliberation_user::DeliberationUser;
use crate::Question;
use crate::SurveyV2;
use crate::User;

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/final-surveys", table = deliberation_final_surveys, action = [create(users = Vec<i64>, surveys = Vec<Question>)])]
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

    #[api_model(summary, many_to_many = deliberation_final_survey_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = final_survey_id)]
    #[serde(default)]
    pub members: Vec<User>,

    #[api_model(summary, many_to_many = deliberation_final_survey_surveys, foreign_table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = final_survey_id)]
    #[serde(default)]
    pub surveys: Vec<SurveyV2>,

    #[api_model(summary, one_to_many = deliberation_responses, foreign_key = deliberation_id)]
    #[serde(default)]
    pub responses: Vec<DeliberationResponse>,
    // NOTE: skipped data for chart, responses per question types
    #[api_model(summary, one_to_many = deliberation_responses, foreign_key = deliberation_id, aggregator = count)]
    pub response_count: i64,
}

impl Into<DeliberationFinalSurveyCreateRequest> for DeliberationFinalSurvey {
    fn into(self) -> DeliberationFinalSurveyCreateRequest {
        DeliberationFinalSurveyCreateRequest {
            users: self.members.into_iter().map(|u| u.id).collect(),
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
