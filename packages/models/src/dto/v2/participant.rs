use bdk::prelude::*;

use crate::{discussion_participants::DiscussionParticipant, UserSummary};

#[api_model(base = "/v2/contents/:content-id", database = skip, read_action = find)]
pub struct ParticipantData {
    pub content_id: i64,
    pub participants: Vec<DiscussionParticipant>,
    pub users: Vec<UserSummary>,
}
