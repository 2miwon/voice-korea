use bdk::prelude::*;
use validator::Validate;

use crate::{ResourceFile, User};

// TODO(web): using resource for discussion tab on a project
// TODO(api): implement action_by_id action(start_meeting) of POST /v2/deliberations/:deliberation-id/discussions/:id
#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/discussions", table = discussions, action = [create(resources = Vec<i64>, users = Vec<i64>)], action_by_id = [start_meeting, delete])]
pub struct Discussion {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = deliberations)]
    pub deliberation_id: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub started_at: i64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub ended_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub name: String,

    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,

    #[api_model(summary, action = create, action_by_id = update)]
    #[serde(default)]
    pub maximum_count: i64,

    #[api_model(summary, action_by_id = update, version = v0.3)]
    // FIXME: action_by_id = update is anti-pattern
    pub meeting_id: Option<String>,

    #[api_model(summary, many_to_many = discussion_groups, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = discussion_id)]
    #[serde(default)]
    pub user_id: Vec<User>,

    #[api_model(summary, many_to_many = discussion_resources, table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = discussion_id)]
    #[serde(default)]
    pub resources: Vec<ResourceFile>,
}

impl Into<DiscussionCreateRequest> for Discussion {
    fn into(self) -> DiscussionCreateRequest {
        DiscussionCreateRequest {
            resources: self.resources.into_iter().map(|r| r.id).collect(),
            users: self.user_id.into_iter().map(|u| u.id).collect(),
            started_at: self.started_at,
            ended_at: self.ended_at,
            name: self.name,
            description: self.description,
            maximum_count: self.maximum_count,
        }
    }
}
