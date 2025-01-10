use by_axum::{
    axum::{
        extract::{Query, State},
        middleware,
        routing::get,
        Extension, Json, Router,
    },
    log::root,
};
use slog::o;

use crate::{
    common::CommonQueryResponse,
    middleware::auth::authorization_middleware,
    utils::{error::ApiError, jwt::Claims},
};

use models::prelude::*;

#[derive(Clone, Debug)]
pub struct OrganizationControllerV1 {
    log: slog::Logger,
}

#[derive(Debug, serde::Deserialize)]
pub struct Pagination {
    pub size: Option<usize>,
    pub bookmark: Option<String>,
}

impl OrganizationControllerV1 {
    pub fn router(_db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
        let log = root().new(o!("api-controller" => "OrganizationControllerV1"));
        let ctrl = OrganizationControllerV1 { log };

        Router::new()
            .route("/", get(Self::list_organizations))
            .with_state(ctrl.clone())
            .layer(middleware::from_fn(authorization_middleware))
    }

    pub async fn list_organizations(
        Extension(claims): Extension<Claims>,
        State(ctrl): State<OrganizationControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<OrganizationMemberResponse>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "list_organizations"));
        let cli = easy_dynamodb::get_client(log.clone());
        slog::debug!(
            log,
            "list_organizations {:?} {:?}",
            pagination,
            claims.id.clone()
        );

        let size = if let Some(size) = pagination.size {
            if size > 100 {
                Some(100)
            } else {
                Some(size as i32)
            }
        } else {
            Some(100)
        };

        let bookmark = pagination.bookmark;

        let res: CommonQueryResponse<OrganizationMember> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            bookmark,
            size,
            vec![("gsi1", OrganizationMember::get_gsi1(&claims.id))],
        )
        .await?;

        let mut organizations: Vec<OrganizationMemberResponse> = vec![];

        for item in res.items {
            let res = cli
                .get::<Organization>(&item.organization_id)
                .await
                .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?;

            organizations.push(OrganizationMemberResponse {
                id: item.id,
                created_at: item.created_at,
                updated_at: item.updated_at,
                deleted_at: item.deleted_at,
                user_id: item.user_id.clone(),
                organization_id: item.organization_id.clone(),
                organization_name: res.clone().unwrap().name.clone(),
                creator: res.unwrap().user_id.clone(),
            });
        }

        Ok(Json(CommonQueryResponse {
            items: organizations,
            bookmark: res.bookmark,
        }))
    }
}
