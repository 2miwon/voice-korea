use crate::areas::area::Area;
use crate::deliberation_basic_infos::deliberation_basic_info::DeliberationBasicInfo;
use crate::deliberation_basic_infos::deliberation_basic_info::DeliberationBasicInfoCreateRequest;
use crate::deliberation_comment::DeliberationComment;
use crate::deliberation_contents::deliberation_content::DeliberationContent;
use crate::deliberation_contents::deliberation_content::DeliberationContentCreateRequest;
use crate::deliberation_discussions::deliberation_discussion::DeliberationDiscussion;
use crate::deliberation_discussions::deliberation_discussion::DeliberationDiscussionCreateRequest;
use crate::deliberation_draft::DeliberationDraft;
use crate::deliberation_drafts::deliberation_draft::DeliberationDraftCreateRequest;
use crate::deliberation_final_surveys::deliberation_final_survey::DeliberationFinalSurvey;
use crate::deliberation_final_surveys::deliberation_final_survey::DeliberationFinalSurveyCreateRequest;
use crate::deliberation_response::DeliberationResponse;
use crate::deliberation_sample_surveys::deliberation_sample_survey::DeliberationSampleSurvey;
use crate::deliberation_sample_surveys::deliberation_sample_survey::DeliberationSampleSurveyCreateRequest;

use crate::deliberation_role::{DeliberationRole, DeliberationRoleCreateRequest};
use crate::deliberation_user::DeliberationUser;

// #[cfg(feature = "server")]
// use crate::deliberation_user::DeliberationUserRepositoryQueryBuilder;

use bdk::prelude::*;
use validator::Validate;

use crate::deliberation_report::DeliberationReport;
use crate::deliberation_vote::DeliberationVote;
use crate::discussions::*;
use crate::step::*;
use crate::{PanelV2, ProjectArea, ResourceFile, SurveyV2};

#[cfg(feature = "server")]
use crate::{
    DeliberationBasicInfoRepositoryQueryBuilder, DeliberationContentRepositoryQueryBuilder,
    DeliberationDiscussionRepositoryQueryBuilder, DeliberationFinalSurveyRepositoryQueryBuilder,
    DeliberationSampleSurveyRepositoryQueryBuilder,
};

#[derive(Validate)]
#[api_model(base = "/v2/organizations/:org-id/deliberations", action = [create(project_areas = Vec<ProjectArea>, resource_ids = Vec<i64>, survey_ids = Vec<i64>, roles = Vec<DeliberationRoleCreateRequest>, panel_ids = Vec<i64>, steps = Vec<StepCreateRequest>, elearning = Vec<i64>, basic_infos = Vec<DeliberationBasicInfoCreateRequest>, sample_surveys = Vec<DeliberationSampleSurveyCreateRequest>, contents = Vec<DeliberationContentCreateRequest>, deliberation_discussions = Vec<DeliberationDiscussionCreateRequest>, final_surveys = Vec<DeliberationFinalSurveyCreateRequest>, drafts = Vec<DeliberationDraftCreateRequest>)], action_by_id = [update(req = DeliberationCreateRequest), start_deliberation, remove_deliberation], table = deliberations)]
pub struct Deliberation {
    #[api_model(summary, primary_key, read_action = get_draft)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,

    // First page of creating a deliberation
    // started_at indicates the start time of the deliberation.
    #[api_model(summary, action = create)]
    pub started_at: i64,
    // ended_at indicates the end time of the deliberation.
    #[api_model(summary, action = create)]
    pub ended_at: i64,
    #[api_model(summary, one_to_many = deliberations_steps, foreign_key = deliberation_id)]
    #[serde(default)]
    pub steps: Vec<Step>,

    #[api_model(summary, many_to_many = deliberation_areas, table_name = areas, foreign_primary_key = area_id, foreign_reference_key = deliberation_id)]
    #[serde(default)]
    pub project_areas: Vec<Area>,
    #[api_model(summary, action = create, version = v0.5)]
    #[serde(default)]
    pub thumbnail_image: String,

    #[api_model(summary, action = create, query_action = search_by)]
    pub title: String,
    #[api_model(action = create)]
    pub description: String,

    // this field will be used instead of members field
    #[api_model(one_to_many = deliberation_roles, foreign_key = deliberation_id)]
    pub roles: Vec<DeliberationRole>,
    // FIXME: this field will be deplicated.
    #[api_model(one_to_many = deliberation_users, foreign_key = deliberation_id)]
    #[serde(default)]
    pub members: Vec<DeliberationUser>,
    #[api_model(one_to_many = deliberation_reports, foreign_key = deliberation_id)]
    #[serde(default)]
    pub reports: Vec<DeliberationReport>,
    #[api_model(one_to_many = deliberation_votes, foreign_key = deliberation_id)]
    #[serde(default)]
    pub votes: Vec<DeliberationVote>,

    // FIXME: this field will be deplicated. use the emails field instead.
    #[api_model(summary, many_to_many = panel_deliberations, foreign_table_name = panels, foreign_primary_key = panel_id, foreign_reference_key = deliberation_id,)]
    #[serde(default)]
    pub panels: Vec<PanelV2>,
    #[api_model(one_to_many = deliberation_comments, foreign_key = deliberation_id)]
    #[serde(default)]
    pub comments: Vec<DeliberationComment>,
    #[api_model(summary, one_to_many = deliberation_responses, foreign_key = deliberation_id)]
    #[serde(default)]
    pub responses: Vec<DeliberationResponse>,
    #[api_model(summary, one_to_many = deliberation_responses, foreign_key = deliberation_id, aggregator = count)]
    #[serde(default)]
    pub response_count: i64,

    #[api_model(one_to_many = deliberation_basic_infos, foreign_key = deliberation_id, nested)]
    #[serde(default)]
    //Note: expected to contain only one field.
    pub basic_infos: Vec<DeliberationBasicInfo>,
    #[api_model(one_to_many = deliberation_sample_surveys, foreign_key = deliberation_id, nested)]
    #[serde(default)]
    pub sample_surveys: Vec<DeliberationSampleSurvey>,
    #[api_model(one_to_many = deliberation_contents, foreign_key = deliberation_id, nested)]
    #[serde(default)]
    pub contents: Vec<DeliberationContent>,
    #[api_model(one_to_many = deliberation_discussions, foreign_key = deliberation_id, nested)]
    #[serde(default)]
    pub deliberation_discussions: Vec<DeliberationDiscussion>,
    #[api_model(one_to_many = deliberation_final_surveys, foreign_key = deliberation_id, nested)]
    #[serde(default)]
    pub final_surveys: Vec<DeliberationFinalSurvey>,
    #[api_model(one_to_many = deliberation_drafts, foreign_key = deliberation_id)]
    #[serde(default)]
    pub drafts: Vec<DeliberationDraft>,

    // FIXME: below field will be deprecated.
    // Relation fields
    #[api_model(many_to_many = deliberation_resources, table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = deliberation_id)]
    #[serde(default)]
    pub resources: Vec<ResourceFile>,
    #[api_model(many_to_many = deliberation_surveys, table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = deliberation_id)]
    #[serde(default)]
    pub surveys: Vec<SurveyV2>,
    // TODO: panel counts field is required.
    // #[api_model(summary, action = create, type = JSONB, version = v0.1, action_by_id = update)]
    // pub panel_counts: Vec<PanelCountsV2>,
    #[api_model(one_to_many = discussions, foreign_key = deliberation_id)]
    #[serde(default)]
    pub discussions: Vec<Discussion>,
    // Second page of creating a deliberation
    #[api_model(summary, type = INTEGER, action = create)]
    #[serde(default)]
    pub project_area: ProjectArea,

    #[api_model(summary, type = INTEGER, action = create, version = v0.2)]
    #[serde(default)]
    pub status: DeliberationStatus,

    #[api_model(summary, many_to_one = users, action = create)]
    pub creator_id: i64,
}

#[derive(Clone, Copy, Translate, Eq, PartialEq, Default, Debug, ApiModel)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum DeliberationStatus {
    #[default]
    #[translate(ko = "초안", en = "Draft")]
    Draft = 0,
    #[translate(ko = "준비", en = "Ready")]
    Ready = 1,
    #[translate(ko = "진행", en = "InProgress")]
    InProgress = 2,
    #[translate(ko = "마감", en = "Finish")]
    Finish = 3,
}

impl Into<DeliberationCreateRequest> for Deliberation {
    fn into(self) -> DeliberationCreateRequest {
        DeliberationCreateRequest {
            title: self.title,
            description: self.description,
            project_area: self.project_area,
            project_areas: self
                .project_areas
                .into_iter()
                .map(|area| area.project_area)
                .collect(),
            started_at: self.started_at,
            ended_at: self.ended_at,
            thumbnail_image: self.thumbnail_image,
            steps: self.steps.into_iter().map(|step| step.into()).collect(),
            resource_ids: self
                .resources
                .clone()
                .into_iter()
                .map(|resource| resource.id)
                .collect(),
            survey_ids: self.surveys.into_iter().map(|survey| survey.id).collect(),
            roles: self.roles.into_iter().map(|role| role.into()).collect(),
            panel_ids: self.panels.into_iter().map(|panel| panel.id).collect(),
            basic_infos: self
                .basic_infos
                .into_iter()
                .map(|basic_info| basic_info.into())
                .collect(),
            sample_surveys: self
                .sample_surveys
                .into_iter()
                .map(|sample_survey| sample_survey.into())
                .collect(),
            contents: self
                .contents
                .into_iter()
                .map(|content| content.into())
                .collect(),
            deliberation_discussions: self
                .deliberation_discussions
                .into_iter()
                .map(|discussion| discussion.into())
                .collect(),
            final_surveys: self
                .final_surveys
                .into_iter()
                .map(|final_survey| final_survey.into())
                .collect(),
            drafts: self.drafts.into_iter().map(|draft| draft.into()).collect(),
            // FIXME: it may be deprecated.
            //        elearning is included in content.
            elearning: self
                .resources
                .into_iter()
                .map(|resource| resource.id)
                .collect(),
            status: self.status,
            creator_id: self.creator_id,
        }
    }
}
