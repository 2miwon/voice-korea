use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::get,
        Extension, Json,
    },
};
use by_types::QueryResponse;
use models::{
    deliberation_final_surveys::deliberation_final_survey::{
        DeliberationFinalSurveyGetResponse, DeliberationFinalSurveyParam,
        DeliberationFinalSurveyQuery, DeliberationFinalSurveySummary,
    },
    deliberation_panel_email::DeliberationPanelEmail,
    deliberation_response::{DeliberationResponse, DeliberationType},
    *,
};
use sqlx::postgres::PgRow;

use crate::utils::app_claims::AppClaims;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationFinalSurveyParentPath {
    pub deliberation_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationFinalSurveyController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationFinalSurveyController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::get_surveys))
            .with_state(self.clone())
    }

    pub async fn get_surveys(
        State(ctrl): State<DeliberationFinalSurveyController>,
        Path(DeliberationFinalSurveyParentPath { deliberation_id }): Path<
            DeliberationFinalSurveyParentPath,
        >,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationFinalSurveyParam>,
    ) -> Result<Json<DeliberationFinalSurveyGetResponse>> {
        tracing::debug!("get_contents: {:?}", q);

        let res = match q {
            DeliberationFinalSurveyParam::Query(param) => {
                let res = match param {
                    param => ctrl.query(deliberation_id, auth, param).await?,
                };
                DeliberationFinalSurveyGetResponse::Query(res)
            }
            DeliberationFinalSurveyParam::Read(action) => match action.action {
                Some(DeliberationFinalSurveyReadActionType::GetById) => {
                    ctrl.read(deliberation_id, auth).await?
                }
                _ => return Err(ApiError::InvalidAction),
            },
        };

        Ok(Json(res))
    }
}

impl DeliberationFinalSurveyController {
    async fn query(
        &self,
        deliberation_id: i64,
        _auth: Option<Authorization>,
        param: DeliberationFinalSurveyQuery,
    ) -> Result<QueryResponse<DeliberationFinalSurveySummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationFinalSurveySummary> =
            DeliberationFinalSurveySummary::query_builder()
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
        deliberation_id: i64,
        auth: Option<Authorization>,
    ) -> Result<DeliberationFinalSurveyGetResponse> {
        let (user_id, email): (i64, String) = match auth {
            Some(Authorization::Bearer { ref claims }) => (
                AppClaims(claims).get_user_id(),
                AppClaims(claims).get_email(),
            ),
            _ => (0, "".to_string()),
        };

        let mut is_member = false;

        if user_id != 0 {
            let res: Option<DeliberationPanelEmail> = DeliberationPanelEmail::query_builder()
                .deliberation_id_equals(deliberation_id)
                .email_equals(email)
                .query()
                .map(Into::into)
                .fetch_optional(&self.pool)
                .await?;

            is_member = res.is_some();
        }

        let responses = DeliberationResponse::query_builder()
            .deliberation_id_equals(deliberation_id)
            .deliberation_type_equals(DeliberationType::Survey)
            .query()
            .map(Into::into)
            .fetch_all(&self.pool)
            .await?;

        let user_response = if user_id != 0 {
            DeliberationResponse::query_builder()
                .deliberation_id_equals(deliberation_id)
                .user_id_equals(user_id)
                .deliberation_type_equals(DeliberationType::Survey)
                .query()
                .map(Into::into)
                .fetch_optional(&self.pool)
                .await?
                .map_or_else(Vec::new, |res| vec![res])
        } else {
            Vec::new()
        };

        let mut res: DeliberationFinalSurvey = DeliberationFinalSurvey::query_builder()
            .deliberation_id_equals(deliberation_id)
            .query()
            .map(Into::into)
            .fetch_one(&self.pool)
            .await?;
        res.user_response = user_response;
        res.responses = responses;
        res.is_member = is_member;
        Ok(DeliberationFinalSurveyGetResponse::Read(res))
    }
}
