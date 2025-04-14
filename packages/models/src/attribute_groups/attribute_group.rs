#![allow(unused_variables, unused)]
use crate::response::Attribute;
use crate::ProjectArea;
use bdk::prelude::*;
use by_types::QueryResponse;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v2/attribute_groups", table = attribute_groups)]
pub struct AttributeGroup {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [create], action_by_id = update, type = JSONB, version = v0.1, nullable)]
    #[serde(default)]
    pub attribute: Attribute,
    #[api_model(summary, action = [create], action_by_id = update)]
    pub rate: i64,
}
