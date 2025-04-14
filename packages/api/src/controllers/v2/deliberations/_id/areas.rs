use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, State},
        routing::get,
        Extension, Json,
    },
};
use deliberation_areas::deliberation_area::*;
use models::*;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationAreaPath {
    deliberation_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationAreaController {
    _repo: DeliberationAreaRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationAreaController {
    pub async fn get_deliberation_area(
        State(ctrl): State<DeliberationAreaController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationAreaPath { deliberation_id }): Path<DeliberationAreaPath>,
    ) -> Result<Json<Vec<DeliberationArea>>> {
        Ok(Json(
            DeliberationArea::query_builder()
                .deliberation_id_equals(deliberation_id)
                .query()
                .map(DeliberationArea::from)
                .fetch_all(&ctrl.pool)
                .await?,
        ))
    }
}

impl DeliberationAreaController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = DeliberationAreaRepository::new(pool.clone());
        Self { _repo: repo, pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::get_deliberation_area))
            .with_state(self.clone())
    }
}
