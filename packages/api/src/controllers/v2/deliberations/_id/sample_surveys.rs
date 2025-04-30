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
    deliberation_response::{DeliberationResponse, DeliberationType},
    deliberation_sample_surveys::deliberation_sample_survey::{
        DeliberationSampleSurveyGetResponse, DeliberationSampleSurveyParam,
        DeliberationSampleSurveyQuery, DeliberationSampleSurveySummary,
    },
    *,
};
use sqlx::postgres::PgRow;

use crate::utils::app_claims::AppClaims;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationSampleSurveyParentPath {
    pub deliberation_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationSampleSurveyController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationSampleSurveyController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::get_surveys))
            .with_state(self.clone())
    }

    pub async fn get_surveys(
        State(ctrl): State<DeliberationSampleSurveyController>,
        Path(DeliberationSampleSurveyParentPath { deliberation_id }): Path<
            DeliberationSampleSurveyParentPath,
        >,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationSampleSurveyParam>,
    ) -> Result<Json<DeliberationSampleSurveyGetResponse>> {
        tracing::debug!("get_contents: {:?}", q);

        let res = match q {
            DeliberationSampleSurveyParam::Query(param) => {
                let res = match param {
                    param => ctrl.query(deliberation_id, auth, param).await?,
                };
                DeliberationSampleSurveyGetResponse::Query(res)
            }
            DeliberationSampleSurveyParam::Read(action) => match action.action {
                Some(DeliberationSampleSurveyReadActionType::GetById) => {
                    ctrl.read(deliberation_id, auth).await?
                }
                _ => return Err(ApiError::InvalidAction),
            },
        };

        Ok(Json(res))
    }
}

impl DeliberationSampleSurveyController {
    async fn query(
        &self,
        deliberation_id: i64,
        _auth: Option<Authorization>,
        param: DeliberationSampleSurveyQuery,
    ) -> Result<QueryResponse<DeliberationSampleSurveySummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationSampleSurveySummary> =
            DeliberationSampleSurveySummary::query_builder()
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
    ) -> Result<DeliberationSampleSurveyGetResponse> {
        let user_id = match auth {
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
            .await?;

        let user_response = if user_id != 0 {
            DeliberationResponse::query_builder()
                .deliberation_id_equals(deliberation_id)
                .user_id_equals(user_id)
                .deliberation_type_equals(DeliberationType::Sample)
                .query()
                .map(Into::into)
                .fetch_optional(&self.pool)
                .await?
                .map_or_else(Vec::new, |res| vec![res])
        } else {
            Vec::new()
        };

        let mut res: DeliberationSampleSurvey = DeliberationSampleSurvey::query_builder()
            .deliberation_id_equals(deliberation_id)
            .query()
            .map(Into::into)
            .fetch_one(&self.pool)
            .await?;
        res.user_response = user_response;
        res.responses = responses;
        res.is_member = is_member;
        Ok(DeliberationSampleSurveyGetResponse::Read(res))
    }
}
