use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::post,
        Extension, Json,
    },
};
use by_types::QueryResponse;
use models::{
    deliberation_drafts::deliberation_draft::{
        DeliberationDraftGetResponse, DeliberationDraftParam, DeliberationDraftQuery,
        DeliberationDraftSummary,
    },
    deliberation_response::{DeliberationResponse, DeliberationType},
    *,
};
use sqlx::postgres::PgRow;

use crate::utils::app_claims::AppClaims;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationDraftParentPath {
    pub deliberation_id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationDraftPath {
    pub deliberation_id: i64,
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationDraftController {
    repo: DeliberationDraftRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationDraftController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = DeliberationDraft::get_repository(pool.clone());
        Self { repo, pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", post(Self::act_draft).get(Self::get_drafts))
            .route("/:id", post(Self::act_draft_by_id))
            .with_state(self.clone())
    }

    pub async fn act_draft_by_id(
        State(ctrl): State<DeliberationDraftController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(DeliberationDraftPath {
            deliberation_id,
            id,
        }): Path<DeliberationDraftPath>,
        Json(body): Json<DeliberationDraftByIdAction>,
    ) -> Result<Json<DeliberationDraft>> {
        let res = match body {
            DeliberationDraftByIdAction::Update(params) => {
                ctrl.update_draft(deliberation_id, id, auth, params).await?
            }
        };

        Ok(Json(res))
    }

    pub async fn act_draft(
        State(ctrl): State<DeliberationDraftController>,
        Path(DeliberationDraftParentPath { deliberation_id }): Path<DeliberationDraftParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<DeliberationDraftAction>,
    ) -> Result<Json<DeliberationDraft>> {
        tracing::debug!("act_draft_response {} {:?}", deliberation_id, body);

        let res = match body {
            DeliberationDraftAction::Create(req) => {
                ctrl.create_draft(deliberation_id, auth, req).await?
            }
        };

        Ok(Json(res))
    }

    pub async fn get_drafts(
        State(ctrl): State<DeliberationDraftController>,
        Path(DeliberationDraftParentPath { deliberation_id }): Path<DeliberationDraftParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationDraftParam>,
    ) -> Result<Json<DeliberationDraftGetResponse>> {
        tracing::debug!("get_contents: {:?}", q);

        let res = match q {
            DeliberationDraftParam::Query(param) => {
                let res = match param {
                    param => ctrl.query(deliberation_id, auth, param).await?,
                };
                DeliberationDraftGetResponse::Query(res)
            }
            DeliberationDraftParam::Read(action) => match action.action {
                Some(DeliberationDraftReadActionType::Read) => {
                    ctrl.read(auth, deliberation_id).await?
                }
                _ => return Err(ApiError::InvalidAction),
            },
        };

        Ok(Json(res))
    }
}

impl DeliberationDraftController {
    pub async fn update_draft(
        &self,
        deliberation_id: i64,
        id: i64,
        auth: Option<Authorization>,
        req: DeliberationDraftUpdateRequest,
    ) -> Result<DeliberationDraft> {
        let _: i64 = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let res = self
            .repo
            .update(
                id,
                DeliberationDraftRepositoryUpdateRequest {
                    title: Some(req.title),
                    description: Some(req.description),
                    deliberation_id: Some(deliberation_id),
                    ..Default::default()
                },
            )
            .await?;

        Ok(res)
    }

    pub async fn create_draft(
        &self,
        deliberation_id: i64,
        auth: Option<Authorization>,
        req: DeliberationDraftCreateRequest,
    ) -> Result<DeliberationDraft> {
        let _: i64 = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let res = self
            .repo
            .insert(req.title, req.description, deliberation_id)
            .await?;

        Ok(res)
    }

    async fn query(
        &self,
        deliberation_id: i64,
        _auth: Option<Authorization>,
        param: DeliberationDraftQuery,
    ) -> Result<QueryResponse<DeliberationDraftSummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationDraftSummary> = DeliberationDraftSummary::query_builder()
            .limit(param.size())
            .page(param.page())
            .deliberation_id_equals(deliberation_id)
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn read(
        &self,
        auth: Option<Authorization>,
        deliberation_id: i64,
    ) -> Result<DeliberationDraftGetResponse> {
        let user_id: i64 = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => 0,
        };

        let mut is_member = false;

        if user_id != 0 {
            let user = User::query_builder()
                .id_equals(user_id)
                .query()
                .map(User::from)
                .fetch_one(&self.pool)
                .await?;

            let email = user.email;

            let emails: Vec<String> = Deliberation::query_builder()
                .id_equals(deliberation_id)
                .query()
                .map(Deliberation::from)
                .fetch_one(&self.pool)
                .await?
                .roles
                .iter()
                .map(|v| v.email.clone())
                .collect();

            is_member = emails.contains(&email);
        }

        let responses = DeliberationResponse::query_builder()
            .deliberation_id_equals(deliberation_id)
            .deliberation_type_equals(DeliberationType::Survey)
            .query()
            .map(Into::into)
            .fetch_all(&self.pool)
            .await
            .unwrap_or(vec![]);

        let final_surveys = DeliberationFinalSurvey::query_builder()
            .deliberation_id_equals(deliberation_id)
            .query()
            .map(DeliberationFinalSurvey::from)
            .fetch_all(&self.pool)
            .await
            .unwrap_or(vec![]);

        let mut res: DeliberationDraft = DeliberationDraft::query_builder()
            .deliberation_id_equals(deliberation_id)
            .query()
            .map(Into::into)
            .fetch_one(&self.pool)
            .await
            .unwrap_or(DeliberationDraft::default());
        res.responses = responses;
        res.final_surveys = final_surveys;
        res.is_member = is_member;
        Ok(DeliberationDraftGetResponse::Read(res))
    }
}
