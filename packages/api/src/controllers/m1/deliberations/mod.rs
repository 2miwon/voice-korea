use by_axum::{
    auth::Authorization,
    axum::{extract::State, routing::post, Extension},
};
use chrono::Utc;
use models::*;

#[derive(Clone, Debug)]
pub struct DeliberationController {
    repo: SurveyV2Repository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = SurveyV2::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::update_deliberation_status))
            .with_state(self.clone()))
    }

    pub async fn update_deliberation_status(
        State(ctrl): State<DeliberationController>,
        Extension(_auth): Extension<Option<Authorization>>,
    ) -> Result<()> {
        let current = Utc::now().timestamp();

        let progress_surveys = SurveyV2::query_builder()
            .started_at_less_than_equals(current)
            .ended_at_greater_than_equals(current)
            .status_not_equals(ProjectStatus::InProgress)
            .project_type_not_equals(ProjectType::Survey)
            .query()
            .map(SurveyV2::from)
            .fetch_all(&ctrl.pool)
            .await?;

        tracing::info!("progress deliberation surveys: {:?}", progress_surveys);

        let finish_surveys = SurveyV2::query_builder()
            .ended_at_less_than_equals(current)
            .status_not_equals(ProjectStatus::Finish)
            .project_type_not_equals(ProjectType::Survey)
            .query()
            .map(SurveyV2::from)
            .fetch_all(&ctrl.pool)
            .await?;

        tracing::info!("finish deliberation surveys: {:?}", finish_surveys);

        for survey in progress_surveys {
            let _ = match ctrl
                .repo
                .update(
                    survey.id,
                    SurveyV2RepositoryUpdateRequest {
                        status: Some(ProjectStatus::InProgress),
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(_) => {
                    tracing::debug!(
                        "success to update deliberation survey status. (survey id: {:?})",
                        survey.id
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "failed to update deliberation survey status with error: {:?} (survey id: {:?})",
                        e,
                        survey.id
                    );
                }
            };
        }

        for survey in finish_surveys {
            let _ = match ctrl
                .repo
                .update(
                    survey.id,
                    SurveyV2RepositoryUpdateRequest {
                        status: Some(ProjectStatus::Finish),
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(_) => {
                    tracing::debug!(
                        "success to update deliberation survey status. (survey id: {:?})",
                        survey.id
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "failed to update deliberation survey status with error: {:?} (survey id: {:?})",
                        e,
                        survey.id
                    );
                }
            };
        }

        Ok(())
    }
}
