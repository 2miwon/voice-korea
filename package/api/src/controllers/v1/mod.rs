use attributes::AttributeControllerV1;
use by_axum::axum::Router;
use groups::GroupControllerV1;
use members::MemberControllerV1;
use metadatas::MetadataControllerV1;
use organizations::OrganizationControllerV1;
use panels::PanelControllerV1;
use public_opinions::PublicOpinionControllerV1;
use public_surveys::PublicSurveyControllerV1;

mod attributes;
mod auth;
mod groups;
mod members;
mod metadatas;
mod organizations;
mod panels;
mod public_opinions;
mod public_surveys;
mod search;
mod survey;
mod verification;

pub fn router(db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
    Router::new()
        .nest("/search", search::router())
        .nest("/auth", auth::router(db.clone()))
        .nest("/verification", verification::router(db.clone()))
        .nest("/survey", survey::router(db.clone()))
        .nest("/groups", GroupControllerV1::router(db.clone()))
        .nest("/members", MemberControllerV1::router(db.clone()))
        .nest("/opinions", PublicOpinionControllerV1::router())
        .nest("/organizations", OrganizationControllerV1::router())
        .nest("/attributes", AttributeControllerV1::router())
        .nest("/panels", PanelControllerV1::router())
        .nest("/metadatas", MetadataControllerV1::router())
        .nest("/surveys", PublicSurveyControllerV1::router())
}
