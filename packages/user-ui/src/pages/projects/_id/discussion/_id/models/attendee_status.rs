use serde::Deserialize;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AttendeeStatus {
    pub video_on: bool,
    pub audio_muted: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReceivedAttendeeStatus {
    pub attendee_id: String,
    pub video_on: bool,
    pub audio_muted: bool,
}
