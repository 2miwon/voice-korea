use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMessage {
    pub topic: String,
    pub sender_attendee_id: String,
    pub sender_external_user_id: String,
    pub text: String,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Chat {
    pub text: String,
    pub user_id: i64,
    pub email: String,
    pub timestamp_ms: u64,
}
