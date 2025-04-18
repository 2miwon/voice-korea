use crate::Role;
use validator::Validate;

use bdk::prelude::*;

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/roles", table = deliberation_roles)]
pub struct DeliberationRole {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = deliberations)]
    pub deliberation_id: i64,

    #[api_model(summary, action = create)]
    pub email: String,
    #[api_model(summary, action = create, type = INTEGER)]
    pub role: Role,
}

impl Into<DeliberationRoleCreateRequest> for DeliberationRole {
    fn into(self) -> DeliberationRoleCreateRequest {
        DeliberationRoleCreateRequest {
            email: self.email,
            role: self.role,
        }
    }
}
