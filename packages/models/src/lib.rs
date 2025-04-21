mod attribute;
mod auth;
pub mod error;
mod group;
mod groups;
mod info;
mod inquiries;
mod metadata;
mod organizations;
mod pagination;
mod panel;
mod panel_count;
mod panel_survey;
mod public_opinion;
mod resource;
pub mod tab;
// mod public_survey;

pub mod areas;
pub mod deliberation_basic_info_members;
pub mod deliberation_basic_info_resources;
pub mod deliberation_basic_info_surveys;
pub mod deliberation_basic_infos;

pub mod deliberation_content_members;
pub mod deliberation_contents;

pub mod deliberation_sample_survey_members;
pub mod deliberation_sample_survey_surveys;
pub mod deliberation_sample_surveys;

pub mod deliberation_discussion_members;
pub mod deliberation_discussion_resources;
pub mod deliberation_discussions;

pub mod deliberation_draft_members;
pub mod deliberation_draft_resources;
pub mod deliberation_draft_surveys;
pub mod deliberation_drafts;

pub mod deliberation_final_survey_members;
pub mod deliberation_final_survey_surveys;
pub mod deliberation_final_surveys;

pub mod deliberation_areas;

pub mod elearnings;

pub mod deliberation_comments;
pub mod deliberation_comments_likes;
pub mod deliberation_reports;
pub mod deliberation_resources;
pub mod deliberation_responses;

pub mod deliberation_roles;

pub mod deliberation_surveys;
pub mod deliberation_users;
pub mod deliberation_votes;
pub mod deliberations;
pub mod discussion_groups;
pub mod discussion_resources;
pub mod discussions;
pub mod dto;
mod group_members;
pub mod invitations;
mod organization_members;
pub mod panel_deliberations;
mod parsed_question;
mod projects;
mod reviews;
mod search;
pub mod steps;
mod strings;
mod survey;
mod update_field;
mod users;
mod verifications;

pub use crate::prelude::*;
pub use by_types::QueryResponse;

pub mod prelude {
    pub use crate::attribute::*;
    pub use crate::auth::*;
    pub use crate::deliberation_basic_info_members::*;
    pub use crate::deliberation_basic_info_resources::*;
    pub use crate::deliberation_basic_info_surveys::*;
    pub use crate::deliberation_basic_infos::*;
    pub use crate::deliberation_comments::*;
    pub use crate::deliberation_comments_likes::*;
    pub use crate::deliberation_content_members::*;
    pub use crate::deliberation_contents::*;
    pub use crate::deliberation_discussions::*;
    pub use crate::deliberation_draft_members::*;
    pub use crate::deliberation_draft_resources::*;
    pub use crate::deliberation_draft_surveys::*;
    pub use crate::deliberation_drafts::*;
    pub use crate::deliberation_final_survey_members::*;
    pub use crate::deliberation_final_survey_surveys::*;
    pub use crate::deliberation_final_surveys::*;
    pub use crate::deliberation_reports::*;
    pub use crate::deliberation_resources::*;
    pub use crate::deliberation_responses::*;
    pub use crate::deliberation_roles::*;
    pub use crate::deliberation_sample_survey_members::*;
    pub use crate::deliberation_sample_survey_surveys::*;
    pub use crate::deliberation_sample_surveys::*;
    pub use crate::deliberation_surveys::*;
    pub use crate::deliberation_users::*;
    pub use crate::deliberation_votes::*;
    pub use crate::deliberations::*;
    pub use crate::discussion_groups::*;
    pub use crate::discussion_resources::*;
    pub use crate::discussions::*;
    pub use crate::elearnings::*;
    pub use crate::error::*;
    pub use crate::group::*;
    pub use crate::group_members::*;
    pub use crate::groups::*;
    pub use crate::info::*;
    pub use crate::inquiries::*;
    pub use crate::invitations::*;
    pub use crate::metadata::*;
    pub use crate::organization_members::*;
    pub use crate::organizations::*;
    pub use crate::pagination::*;
    pub use crate::panel::*;
    pub use crate::panel_count::*;
    pub use crate::panel_survey::*;
    pub use crate::parsed_question::*;
    pub use crate::projects::*;
    pub use crate::public_opinion::*;
    pub use crate::resource::*;
    pub use crate::reviews::*;
    pub use crate::search::*;
    pub use crate::steps::*;
    pub use crate::strings::*;
    pub use crate::survey::*;
    pub use crate::tab::*;
    pub use crate::update_field::*;
    pub use crate::users::*;
    pub use crate::verifications::*;
}

pub type Result<T> = std::result::Result<T, crate::error::ApiError>;
