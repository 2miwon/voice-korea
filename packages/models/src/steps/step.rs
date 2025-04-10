use super::step_type::StepType;
use bdk::prelude::*;

#[api_model(base = "/organizations/v2/:org-id/deliberations", table = deliberations_steps)]
pub struct Step {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(summary, many_to_one = deliberations)]
    pub deliberation_id: i64,
    #[api_model(summary, type = INTEGER, action = create)]
    #[serde(default)]
    pub step_type: StepType,
    #[api_model(summary, action = create)]
    pub name: String,
    #[api_model(summary, action = create)]
    pub started_at: i64,
    #[api_model(summary, action = create)]
    pub ended_at: i64,
}

impl Into<StepCreateRequest> for Step {
    fn into(self) -> StepCreateRequest {
        StepCreateRequest {
            step_type: self.step_type,
            name: self.name,
            started_at: self.started_at,
            ended_at: self.ended_at,
        }
    }
}
