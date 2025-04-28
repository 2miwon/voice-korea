use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, State},
        routing::get,
        Extension, Json,
    },
};

use models::{discussion_participants::DiscussionParticipant, dto::ParticipantData, *};

use crate::utils::app_claims::AppClaims;

#[derive(Clone, Debug)]
pub struct ContentController {
    #[allow(dead_code)]
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct ContentPath {
    pub content_id: i64,
}

impl ContentController {
    async fn query(&self, content_id: i64) -> Result<ParticipantData> {
        let mut tx = self.pool.begin().await?;

        let participants = DiscussionParticipant::query_builder()
            .discussion_id_equals(content_id)
            .query()
            .map(DiscussionParticipant::from)
            .fetch_all(&mut *tx)
            .await?;

        let mut users = vec![];

        for participant in participants.clone() {
            let id = participant.user_id;

            let u = User::query_builder()
                .id_equals(id)
                .query()
                .map(UserSummary::from)
                .fetch_one(&mut *tx)
                .await?;

            users.push(u);
        }

        tx.commit().await?;

        Ok(ParticipantData {
            content_id,
            participants,
            users,
        })
    }
}

impl ContentController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:content-id", get(Self::get_contents))
            .with_state(self.clone()))
    }

    pub async fn get_contents(
        State(ctrl): State<ContentController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(ContentPath { content_id }): Path<ContentPath>,
    ) -> Result<Json<ParticipantData>> {
        let _ = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        Ok(Json(ctrl.query(content_id).await?))
    }
}
