pub mod deliberations;
pub mod surveys;

use models::*;

#[derive(Clone, Debug)]
pub struct M1Controller {}

impl M1Controller {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .nest(
                "/deliberations",
                deliberations::DeliberationController::new(pool.clone()).route()?,
            )
            .nest(
                "/surveys",
                surveys::SurveyController::new(pool.clone()).route()?,
            ))
    }
}
