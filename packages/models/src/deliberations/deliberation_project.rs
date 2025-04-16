use bdk::prelude::*;

use crate::areas::area::Area;

use super::DeliberationStatus;

// TODO(web): using resource for project.
#[api_model(base = "/v2/projects", custom_query_type = ProjectQueryBy, table = deliberations)]
pub struct DeliberationProject {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    pub started_at: i64,
    pub ended_at: i64,

    #[api_model(summary, query_action = search)]
    pub title: String,
    #[api_model(summary)]
    pub description: String,
    #[api_model(summary, many_to_many = deliberation_areas, table_name = areas, foreign_primary_key = area_id, foreign_reference_key = deliberation_id)]
    pub project_areas: Vec<Area>,

    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
    #[api_model(summary, one_to_many = deliberation_users, foreign_key = deliberation_id, aggregator = count)]
    pub participants: i64,
    #[api_model(summary, one_to_many = deliberation_votes, foreign_key = deliberation_id, aggregator = count)]
    pub votes: i64,

    #[api_model(summary, type = INTEGER)]
    #[serde(default)]
    pub status: DeliberationStatus,

    #[api_model(summary)]
    #[serde(default)]
    pub thumbnail_image: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ProjectQueryBy {
    pub sorter: ProjectSorter,
    #[serde(flatten)]
    pub status: Option<ProjectStatusCondition>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ProjectStatusCondition {
    pub status: ProjectStatusValue,
    pub op: ProjectStatusOp,
}
//FIXME:
// Using an enum structure would improve readability.
// pub enum ProjectStatusCondition {
//     Equal(DeliberationStatus),
//     NotEqual(DeliberationStatus),
// }
// However, due to serialization and deserialization issues with enums,
// we’ve implemented this using a struct for now.

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectStatusOp {
    Equal,
    NotEqual,
}
// FIXME:

// FIXME: Use DeliberationStatus instead of this enum.
// For now, `DeliberationStatus` cannot deserialize in axum backend.
// So, we need to use this enum for now.

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectStatusValue {
    Draft,
    Ready,
    InProgress,
    Finish,
}

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize, Translate, Default,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectSorter {
    #[default]
    #[translate(ko => "오래된순")]
    Oldest,
    #[translate(ko => "최신순")]
    Newest,
}

impl DeliberationProject {
    pub fn period(&self) -> String {
        // TODO(web): returns Feb. 12, 2025 - Mar. 15, 2025

        todo!()
    }
}
