use bdk::prelude::*;
use validator::Validate;

use crate::discussion_conversations::discussion_conversation::DiscussionConversation;
use crate::discussion_participants::DiscussionParticipant;

use crate::{ResourceFile, User};

// TODO(web): using resource for discussion tab on a project
// TODO(api): implement action_by_id action(start_meeting) of POST /v2/deliberations/:deliberation-id/discussions/:id

// TODO: Add Activities in Discussion.
#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/discussions", table = discussions, action = [create(resources = Vec<i64>, users = Vec<i64>)], action_by_id = [start_meeting, participant_meeting, exit_meeting, start_recording, end_recording, delete])]
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

    #[api_model(summary, action_by_id = update, version = v0.4)]
    pub pipeline_id: String,

    #[api_model(summary, one_to_many = discussion_conversations, foreign_key = discussion_id)]
    #[serde(default)]
    pub conversations: Vec<DiscussionConversation>,

    #[api_model(summary, one_to_many = discussion_participants, foreign_key = discussion_id)]
    #[serde(default)]
    pub participants: Vec<DiscussionParticipant>,

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

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct DiscussionComment {
    created_at: i64,
    user_id: i64,
    comment: String,
}
