use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = deliberation_sample_survey_roles)]
pub struct DeliberationSampleSurveyRole {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update], version = v0.1)]
    pub updated_at: i64,

    #[api_model(many_to_one = deliberation_roles)]
    pub role_id: i64,
    #[api_model(many_to_one = deliberation_sample_surveys)]
    pub sample_survey_id: i64,
}
