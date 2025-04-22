#![allow(unused_variables, unused)]
use crate::deliberation_role::DeliberationRole;
use crate::deliberation_user::DeliberationUser;
use crate::ResourceFile;
use crate::SurveyV2;
use crate::User;
use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/infos", table = deliberation_basic_infos, action = [create(users = Vec<String>, resources = Vec<i64>, surveys = Vec<i64>)])]
pub struct DeliberationBasicInfo {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    // started_at indicates the start time of the deliberation.
    #[api_model(summary, action = create, action_by_id = update)]
    pub started_at: i64,
    // ended_at indicates the end time of the deliberation.
    #[api_model(summary, action = create, action_by_id = update)]
    pub ended_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,

    #[api_model(summary, many_to_one = deliberations)]
    pub deliberation_id: i64,

    #[api_model(summary, many_to_many = deliberation_basic_info_roles, foreign_table_name = deliberation_roles, foreign_primary_key = role_id, foreign_reference_key = basic_id)]
    #[serde(default)]
    pub roles: Vec<DeliberationRole>,

    //FIXME: this field will be deprecated. use roles field instead.
    #[api_model(summary, many_to_many = deliberation_basic_info_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = basic_id)]
    #[serde(default)]
    pub members: Vec<User>,

    #[api_model(summary, many_to_many = deliberation_basic_info_resources, foreign_table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = basic_id)]
    #[serde(default)]
    pub resources: Vec<ResourceFile>,

    #[api_model(summary, many_to_many = deliberation_basic_info_surveys, foreign_table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = basic_id)]
    #[serde(default)]
    pub surveys: Vec<SurveyV2>,

    #[api_model(summary, one_to_many = deliberation_roles, reference_key = deliberation_id, foreign_key = deliberation_id)]
    #[serde(default)]
    pub roles: Vec<DeliberationRole>,
}

impl Into<DeliberationBasicInfoCreateRequest> for DeliberationBasicInfo {
    fn into(self) -> DeliberationBasicInfoCreateRequest {
        DeliberationBasicInfoCreateRequest {
            users: self.roles.into_iter().map(|u| u.email).collect(),
            resources: self.resources.into_iter().map(|r| r.id).collect(),
            surveys: self.surveys.into_iter().map(|s| s.id).collect(),
            started_at: self.started_at,
            ended_at: self.ended_at,
            title: self.title,
            description: self.description,
        }
    }
}
