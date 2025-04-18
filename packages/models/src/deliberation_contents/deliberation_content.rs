#![allow(unused_variables, unused)]
use bdk::prelude::*;
use validator::Validate;

use crate::deliberation_user::DeliberationUser;
use crate::elearnings::elearning::ElearningCreateRequest;
use crate::elearnings::elearning::{self, Elearning};
use crate::{Question, ResourceFile, User};

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/contents", table = deliberation_contents, action = [create(users = Vec<i64>, elearnings = Vec<ElearningCreateRequest>)])]
pub struct DeliberationContent {
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

    #[api_model(summary, many_to_many = deliberation_content_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = content_id)]
    #[serde(default)]
    pub members: Vec<User>,

    #[api_model(one_to_many = elearnings, foreign_key = content_id)]
    #[serde(default)]
    pub elearnings: Vec<Elearning>,

    #[api_model(summary, action = create, type = JSONB, version = v0.2, action_by_id = update)]
    #[serde(default)]
    pub questions: Vec<Option<Question>>,

    #[api_model(summary, many_to_many = deliberation_study_materials, table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = deliberation_id)]
    #[serde(default)]
    pub resources: Vec<ResourceFile>,
}

impl Into<DeliberationContentCreateRequest> for DeliberationContent {
    fn into(self) -> DeliberationContentCreateRequest {
        DeliberationContentCreateRequest {
            users: self.members.into_iter().map(|u| u.id).collect(),
            elearnings: self.elearnings.into_iter().map(|e| e.into()).collect(),
            started_at: self.started_at,
            ended_at: self.ended_at,
            title: self.title,
            description: self.description,
            questions: self.questions,
        }
    }
}
