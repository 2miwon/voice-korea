#![allow(unused_variables, unused)]
use bdk::prelude::*;
use validator::Validate;

use crate::deliberation_role::DeliberationRole;
use crate::deliberation_user::DeliberationUser;
use crate::discussions::Discussion;
use crate::discussions::DiscussionCreateRequest;
use crate::ResourceFile;
use crate::User;

//FIXME: fix to wording when discussion function is implemented
#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/ideas", table = deliberation_discussions, action = [create(users = Vec<i64>, resources = Vec<i64>, discussions = Vec<DiscussionCreateRequest>)])]
pub struct DeliberationDiscussion {
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

    #[api_model(summary, many_to_many = deliberation_discussion_roles, foreign_table_name = deliberation_roles, foreign_primary_key = role_id, foreign_reference_key = discussion_id)]
    #[serde(default)]
    pub roles: Vec<DeliberationRole>,

    //FIXME: this field will be deprecated. use roles field instead.
    #[api_model(summary, many_to_many = deliberation_discussion_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = discussion_id)]
    #[serde(default)]
    pub members: Vec<User>,

    #[api_model(summary, many_to_many = deliberation_discussion_resources, foreign_table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = discussion_id)]
    #[serde(default)]
    pub resources: Vec<ResourceFile>,

    #[api_model(summary, one_to_many = discussions, foreign_key = deliberation_id, reference_key = deliberation_id)]
    #[serde(default)]
    pub discussions: Vec<Discussion>,
}

impl Into<DeliberationDiscussionCreateRequest> for DeliberationDiscussion {
    fn into(self) -> DeliberationDiscussionCreateRequest {
        DeliberationDiscussionCreateRequest {
            users: self.members.into_iter().map(|u| u.id).collect(),
            resources: self.resources.into_iter().map(|r| r.id).collect(),
            discussions: self.discussions.into_iter().map(|d| d.into()).collect(),
            started_at: self.started_at,
            ended_at: self.ended_at,
            title: self.title,
            description: self.description,
        }
    }
}
