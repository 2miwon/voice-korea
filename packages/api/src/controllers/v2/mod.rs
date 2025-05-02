pub mod comments;
pub mod contents;
pub mod inquiries;
pub mod landing;
pub mod metadata;
pub mod organizations;
pub mod profile;
pub mod projects;
pub mod reviews;

use models::*;

pub mod surveys {
    pub mod _id {
        pub mod responses;
    }
}

pub mod deliberations {
    pub mod _id {
        pub mod comments;
        pub mod discussions;
        pub mod meeting;
        pub mod responses;

        pub mod areas;
        pub mod contents;
        pub mod drafts;
        pub mod final_surveys;
        pub mod ideas;
        pub mod infos;
        pub mod sample_surveys;
    }
}

#[derive(Clone, Debug)]
pub struct Version2Controller {}

impl Version2Controller {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .nest(
                "/metadata",
                metadata::MetadataControllerV1::route(pool.clone())?,
            )
            .nest(
                "/landing",
                landing::LandingController::new(pool.clone()).route()?,
            )
            .nest(
                "/profile",
                profile::ProfileController::new(pool.clone()).route()?,
            )
            .nest(
                "/contents",
                contents::ContentController::new(pool.clone()).route()?,
            )
            .nest(
                "/projects",
                projects::DeliberationProjectController::new(pool.clone()).route()?,
            )
            .nest(
                "/organizations",
                organizations::OrganizationController::route(pool.clone())?,
            )
            .nest("/reviews", reviews::ReviewController::route(pool.clone())?)
            .nest(
                "/surveys/:survey-id/responses",
                surveys::_id::responses::SurveyResponseController::route(pool.clone())?,
            )
            .nest(
                "/deliberations/:deliberation-id/contents",
                deliberations::_id::contents::DeliberationContentController::new(pool.clone())
                    .route(),
            )
            .nest(
                "/deliberations/:deliberation-id/drafts",
                deliberations::_id::drafts::DeliberationDraftController::new(pool.clone()).route(),
            )
            .nest(
                "/deliberations/:deliberation-id/final-surveys",
                deliberations::_id::final_surveys::DeliberationFinalSurveyController::new(
                    pool.clone(),
                )
                .route(),
            )
            .nest(
                "/deliberations/:deliberation-id/ideas",
                deliberations::_id::ideas::DeliberationIdeaController::new(pool.clone()).route(),
            )
            .nest(
                "/deliberations/:deliberation-id/infos",
                deliberations::_id::infos::DeliberationBasicInfoController::new(pool.clone())
                    .route(),
            )
            .nest(
                "/deliberations/:deliberation-id/sample-surveys",
                deliberations::_id::sample_surveys::DeliberationSampleSurveyController::new(
                    pool.clone(),
                )
                .route(),
            )
            .nest(
                "/deliberations/:deliberation-id/discussions",
                deliberations::_id::discussions::DiscussionController::new(pool.clone()).route(),
            )
            .nest(
                "/deliberations/:deliberation-id/comments",
                deliberations::_id::comments::DeliberationCommentController::new(pool.clone())
                    .route(),
            )
            .nest(
                "/deliberations/:deliberation-id/areas",
                deliberations::_id::areas::DeliberationAreaController::new(pool.clone()).route(),
            )
            .nest(
                "/inquiries",
                inquiries::InquiryController::new(pool.clone()).route(),
            )
            .nest(
                "/deliberations/:deliberation-id/meeting",
                deliberations::_id::meeting::MeetingController::new(pool.clone()).route()?,
            )
            .nest(
                "/comments",
                comments::CommentController::new(pool.clone()).route(),
            )
            .nest(
                "/deliberations/:deliberation-id/responses",
                deliberations::_id::responses::DeliberationResponseController::route(pool)?,
            ))
    }
}
