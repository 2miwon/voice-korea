use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::QueryResponse;
use models::response::*;
use models::*;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct SurveyResponseController {
    repo: SurveyResponseRepository,
    survey: SurveyV2Repository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SurveyResponseController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = SurveyResponse::get_repository(pool.clone());
        let survey = SurveyV2::get_repository(pool.clone());

        Self { repo, pool, survey }
    }

    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let ctrl = Self::new(pool);

        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_survey_response), //.post(Self::act_survey_response_by_id),
            )
            .with_state(ctrl.clone())
            .route(
                "/",
                post(Self::act_survey_response).get(Self::list_survey_response),
            )
            .with_state(ctrl.clone()))
    }

    pub async fn act_survey_response(
        State(ctrl): State<SurveyResponseController>,
        Path(parent_id): Path<i64>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<SurveyResponseAction>,
    ) -> Result<Json<SurveyResponse>> {
        tracing::debug!("act_survey_response {} {:?}", parent_id, body);

        match body {
            SurveyResponseAction::RespondAnswer(req) => {
                auth.ok_or(ApiError::Unauthorized)?;
                ctrl.respond_answer(parent_id, req).await
            }
        }
    }

    // pub async fn act_survey_response_by_id(
    //     State(_ctrl): State<SurveyResponseControllerV1>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path((parent_id, id)): Path<(i64, i64)>,
    //     Json(body): Json<SurveyResponseByIdAction>,
    // ) -> Result<Json<SurveyResponse>> {
    //     tracing::debug!(
    //         "act_survey_response_by_id {} {:?} {:?}",
    //         parent_id,
    //         id,
    //         body
    //     );
    //     Ok(Json(SurveyResponse::default()))
    // }

    pub async fn get_survey_response(
        State(_ctrl): State<SurveyResponseController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((parent_id, id)): Path<(i64, i64)>,
    ) -> Result<Json<SurveyResponse>> {
        tracing::debug!("get_survey_response {} {:?}", parent_id, id);
        Ok(Json(SurveyResponse::default()))
    }

    pub async fn list_survey_response(
        State(ctrl): State<SurveyResponseController>,
        Path(parent_id): Path<i64>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<SurveyResponseParam>,
    ) -> Result<Json<SurveyResponseGetResponse>> {
        tracing::debug!("list_survey_response {} {:?}", parent_id, q);

        match q {
            SurveyResponseParam::Query(q) => {
                let mut total_count = 0;
                let items = SurveyResponseSummary::query_builder()
                    .survey_id_equals(parent_id)
                    .limit(q.size as i32)
                    .page(q.page())
                    .query()
                    .map(|r: sqlx::postgres::PgRow| {
                        use sqlx::Row;
                        total_count = r.get("total_count");
                        r.into()
                    })
                    .fetch_all(&ctrl.pool)
                    .await?;

                Ok(Json(SurveyResponseGetResponse::Query(QueryResponse {
                    total_count,
                    items,
                })))
            }
        }
    }
}

impl SurveyResponseController {
    pub async fn respond_answer(
        &self,
        survey_id: i64,
        SurveyResponseRespondAnswerRequest {
            proof_id,
            attributes,
            answers,
        }: SurveyResponseRespondAnswerRequest,
    ) -> Result<Json<SurveyResponse>> {
        let survey = self
            .survey
            .find_one(&SurveyV2ReadAction::new().find_by_id(survey_id))
            .await?;
        tracing::debug!("survey {:?}", survey);

        let no_of_q = survey.questions.len();
        if no_of_q != answers.len() {
            return Err(ApiError::SurveyResponseMissingAnswer);
        }

        for i in 0..no_of_q {
            if answers[i] != survey.questions[i] {
                return Err(ApiError::SurveyResponseInconsistentAnswerType);
            }
        }

        let mut matched = false;
        for quota in survey.attribute_quotas.into_iter() {
            if quota == attributes {
                matched = true;
                break;
            }
        }

        if !matched {
            tracing::error!("no matched attribute group {:?}", attributes);
            return Err(ApiError::SurveyResponseNoMatchedAttributeGroup);
        }

        let mut total_count = 0;

        tracing::debug!("fetch_all for survey_id {}", survey_id);

        let responses: Vec<SurveyResponse> = SurveyResponse::query_builder()
            .with_count()
            .survey_id_equals(survey_id)
            .query()
            .map(|r: sqlx::postgres::PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        tracing::debug!("responses {} total_count {}", responses.len(), total_count);

        if survey.quotas <= total_count {
            return Err(ApiError::SurveyResponsePanelQuotaExceeded);
        }

        tracing::debug!(
            "respond_answer {} {} {:?} {:?}",
            survey_id,
            proof_id,
            attributes,
            answers
        );

        let res = self
            .repo
            .insert(0, proof_id, attributes, answers, survey_id)
            .await?;

        Ok(Json(res))
    }
}
