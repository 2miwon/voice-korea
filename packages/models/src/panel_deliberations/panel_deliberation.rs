use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = panel_deliberations)]
pub struct PanelDeliberation {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update], version = v0.1)]
    pub updated_at: i64,

    #[api_model(many_to_one = panels)]
    pub panel_id: i64,
    #[api_model(many_to_one = deliberations)]
    pub deliberation_id: i64,
}
