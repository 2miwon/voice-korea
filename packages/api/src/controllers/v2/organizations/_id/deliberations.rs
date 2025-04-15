use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::QueryResponse;
use deliberation_resources::deliberation_resource::*;
use deliberation_surveys::DeliberationSurvey;
use deliberation_user::*;
use discussion_resources::DiscussionResource;
use discussions::Discussion;
use models::{
    deliberation::*, deliberation_areas::deliberation_area::*,
    deliberation_basic_info_members::deliberation_basic_info_member::*,
    deliberation_basic_info_resources::deliberation_basic_info_resource::*,
    deliberation_basic_info_surveys::deliberation_basic_info_survey::*,
    deliberation_basic_infos::deliberation_basic_info::*,
    deliberation_content_members::deliberation_content_member::*,
    deliberation_contents::deliberation_content::*,
    deliberation_discussion_members::deliberation_discussion_member::*,
    deliberation_discussion_resources::deliberation_discussion_resource::*,
    deliberation_discussions::deliberation_discussion::*,
    deliberation_draft_members::deliberation_draft_member::*,
    deliberation_draft_resources::deliberation_draft_resource::*,
    deliberation_draft_surveys::deliberation_draft_survey::*,
    deliberation_drafts::deliberation_draft::*,
    deliberation_final_survey_members::deliberation_final_survey_member::*,
    deliberation_final_survey_surveys::deliberation_final_survey_survey::*,
    deliberation_final_surveys::deliberation_final_survey::*,
    deliberation_sample_survey_members::deliberation_sample_survey_member::*,
    deliberation_sample_survey_surveys::deliberation_sample_survey_survey::*,
    deliberation_sample_surveys::deliberation_sample_survey::*, discussion_groups::DiscussionGroup,
    elearnings::elearning::*, step::*, *,
};
use panel_deliberations::*;
use sqlx::postgres::PgRow;
use step::StepCreateRequest;

use crate::controllers::v2::organizations::OrganizationPath;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationPath {
    pub org_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationParentPath {
    pub org_id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationDraftPath {
    pub org_id: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationController {
    repo: DeliberationRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
    step: StepRepository,
    deliberation_user: DeliberationUserRepository,
    deliberation_resource: DeliberationResourceRepository,
    deliberation_survey: DeliberationSurveyRepository,
    deliberation_area: DeliberationAreaRepository,
    panel_deliberation: PanelDeliberationRepository,
    survey: SurveyV2Repository,
    basic_info: DeliberationBasicInfoRepository,
    basic_info_member: DeliberationBasicInfoMemberRepository,
    basic_info_resource: DeliberationBasicInfoResourceRepository,
    basic_info_survey: DeliberationBasicInfoSurveyRepository,
    sample_survey: DeliberationSampleSurveyRepository,
    sample_survey_member: DeliberationSampleSurveyMemberRepository,
    sample_survey_survey: DeliberationSampleSurveySurveyRepository,
    deliberation_contents: DeliberationContentRepository,
    deliberation_contents_member: DeliberationContentMemberRepository,
    elearning_repo: ElearningRepository,
    discussion_repo: DeliberationDiscussionRepository,
    discussion_member: DeliberationDiscussionMemberRepository,
    discussion_resource: DeliberationDiscussionResourceRepository,
    disc_repo: DiscussionRepository,
    disc_group: DiscussionGroupRepository,
    disc_res: DiscussionResourceRepository,
    final_repo: DeliberationFinalSurveyRepository,
    final_member: DeliberationFinalSurveyMemberRepository,
    final_survey: DeliberationFinalSurveySurveyRepository,
    draft_repo: DeliberationDraftRepository,
    draft_member: DeliberationDraftMemberRepository,
    draft_survey: DeliberationDraftSurveyRepository,
    draft_resource: DeliberationDraftResourceRepository,
}

impl DeliberationController {
    pub async fn create(
        &self,
        org_id: i64,
        DeliberationCreateRequest {
            started_at,
            ended_at,
            project_area,
            project_areas,
            title,
            description,
            panel_ids,
            resource_ids,
            survey_ids,
            roles,
            steps,
            elearning,
            thumbnail_image,
            basic_infos,
            sample_surveys,
            contents,
            deliberation_discussions,
            final_surveys,
            drafts,
            status,
            creator_id,
        }: DeliberationCreateRequest,
    ) -> Result<Deliberation> {
        self.validate_inputs(
            // org_id,
            started_at, ended_at, status,
        )?;

        let mut tx = self.pool.begin().await?;

        let deliberation = self
            .repo
            .insert_with_tx(
                &mut *tx,
                org_id,
                started_at,
                ended_at,
                thumbnail_image,
                title,
                description,
                project_area,
                status,
                creator_id,
            )
            .await?
            .ok_or(ApiError::DeliberationException)?;

        self.insert_deliberation_areas(&mut *tx, deliberation.id, project_areas.clone())
            .await?;

        self.insert_deliberation_users(&mut *tx, org_id, deliberation.id, roles.clone())
            .await?;

        for resource_id in resource_ids {
            self.deliberation_resource
                .insert_with_tx(
                    &mut *tx,
                    deliberation.id,
                    resource_id,
                    DeliberationResourceType::Reference,
                )
                .await?
                .ok_or(ApiError::DeliberationResourceException)?;
        }

        for resource_id in elearning {
            self.deliberation_resource
                .insert_with_tx(
                    &mut *tx,
                    deliberation.id,
                    resource_id,
                    DeliberationResourceType::Elearning,
                )
                .await?
                .ok_or(ApiError::DeliberationResourceException)?;
        }

        for survey_id in survey_ids {
            self.deliberation_survey
                .insert_with_tx(&mut *tx, deliberation.id, survey_id)
                .await?
                .ok_or(ApiError::DeliberationSurveyException)?;
        }

        for StepCreateRequest {
            ended_at,
            step_type,
            started_at,
            name,
        } in steps
        {
            self.step
                .insert_with_tx(
                    &mut *tx,
                    deliberation.id,
                    step_type,
                    name,
                    started_at,
                    ended_at,
                )
                .await?
                .ok_or(ApiError::DeliberationStepException)?;
        }

        self.create_basic_info(&mut *tx, deliberation.id, basic_infos)
            .await?;
        self.create_sample_survey(
            &mut *tx,
            org_id,
            deliberation.id,
            project_areas.clone(),
            sample_surveys,
        )
        .await?;
        self.create_content(&mut *tx, deliberation.id, contents)
            .await?;
        self.create_disscussion(&mut *tx, deliberation.id, deliberation_discussions)
            .await?;
        self.create_final_survey(
            &mut *tx,
            org_id,
            deliberation.id,
            project_areas.clone(),
            final_surveys,
        )
        .await?;
        self.create_draft(&mut *tx, deliberation.id, drafts).await?;

        for id in panel_ids {
            self.panel_deliberation
                .insert_with_tx(&mut *tx, id, deliberation.id)
                .await?
                .ok_or(ApiError::DeliberationPanelException)?;
        }

        tx.commit().await?;

        Ok(deliberation)
    }

    #[allow(unused_variables)]
    pub async fn update(
        &self,
        id: i64,
        org_id: i64,
        param: DeliberationUpdateRequest,
    ) -> Result<Deliberation> {
        let DeliberationCreateRequest {
            started_at,
            ended_at,
            project_area,
            project_areas,
            title,
            description,
            panel_ids,
            resource_ids,
            survey_ids,
            roles,
            steps,
            elearning,
            thumbnail_image,
            basic_infos,
            sample_surveys,
            contents,
            deliberation_discussions,
            final_surveys,
            drafts,
            status,
            creator_id,
        } = param.req;

        self.validate_inputs(
            // org_id,
            started_at, ended_at, status,
        )?;

        let mut tx = self.pool.begin().await?;

        let deliberation = self
            .repo
            .update_with_tx(
                &mut *tx,
                id,
                DeliberationRepositoryUpdateRequest {
                    org_id: Some(org_id),
                    started_at: Some(started_at),
                    ended_at: Some(ended_at),
                    thumbnail_image: Some(thumbnail_image),
                    title: Some(title),
                    description: Some(description),
                    project_area: Some(project_area),
                    status: Some(status),
                    creator_id: Some(creator_id),
                },
            )
            .await?
            .ok_or(ApiError::DeliberationException)?;

        self.upsert_deliberation_areas(&mut *tx, id, project_areas.clone())
            .await?;

        self.upsert_deliberation_users(&mut *tx, org_id, id, roles.clone())
            .await?;

        self.upsert_deliberation_panels(&mut *tx, id, panel_ids)
            .await?;

        self.upsert_basic_info(&mut *tx, id, basic_infos).await?;

        self.upsert_sample_survey(
            &mut *tx,
            org_id,
            deliberation.id,
            project_areas.clone(),
            sample_surveys,
        )
        .await?;

        tx.commit().await?;

        Ok(deliberation)
    }

    pub async fn query(
        &self,
        org_id: i64,
        DeliberationQuery { size, bookmark, .. }: DeliberationQuery,
    ) -> Result<QueryResponse<DeliberationSummary>> {
        let mut total_count: i64 = 0;
        let items: Vec<DeliberationSummary> = Deliberation::query_builder()
            .org_id_equals(org_id)
            .limit(size as i32)
            .page(bookmark.unwrap_or("1".to_string()).parse::<i32>().unwrap())
            .with_count()
            .query()
            .map(|r: sqlx::postgres::PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    fn validate_inputs(
        &self,
        // org_id: i64,
        started_at: i64,
        ended_at: i64,
        status: DeliberationStatus,
    ) -> Result<()> {
        if started_at > ended_at {
            return Err(ApiError::ValidationError(
                "started_at should be less than ended_at".to_string(),
            )
            .into());
        }
        if status != DeliberationStatus::Draft && status != DeliberationStatus::Ready {
            return Err(
                ApiError::ValidationError("status should be Draft or Ready".to_string()).into(),
            );
        }
        Ok(())
    }

    async fn create_basic_info(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_id: i64,
        basic_infos: Vec<DeliberationBasicInfoCreateRequest>,
    ) -> Result<Vec<DeliberationBasicInfo>> {
        let mut v = vec![];
        for DeliberationBasicInfoCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            users,
            resources,
            surveys,
        } in basic_infos
        {
            let info = self
                .basic_info
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title,
                    description,
                    deliberation_id,
                )
                .await?
                .ok_or(ApiError::DeliberationBasicInfoException)?;

            for user_id in users {
                let _ = self
                    .basic_info_member
                    .insert_with_tx(&mut *tx, user_id, info.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }

            for resource_id in resources {
                let _ = self
                    .basic_info_resource
                    .insert_with_tx(&mut *tx, resource_id, info.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }

            for survey_id in surveys {
                let _ = self
                    .basic_info_survey
                    .insert_with_tx(&mut *tx, survey_id, info.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }

            v.push(info);
        }
        Ok(v)
    }

    async fn upsert_basic_info(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_id: i64,
        basic_infos: Vec<DeliberationBasicInfoCreateRequest>,
    ) -> Result<()> {
        for DeliberationBasicInfoCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            users,
            resources,
            surveys,
        } in basic_infos.clone()
        {
            let results = DeliberationBasicInfo::query_builder()
                .deliberation_id_equals(deliberation_id)
                .query()
                .map(DeliberationBasicInfo::from)
                .fetch_all(&self.pool)
                .await?;

            let basic = if results.is_empty() {
                let v = self
                    .create_basic_info(&mut *tx, deliberation_id, basic_infos.clone())
                    .await?;

                v.into_iter()
                    .next()
                    .unwrap_or_else(DeliberationBasicInfo::default)
            } else {
                results
                    .into_iter()
                    .next()
                    .unwrap_or_else(DeliberationBasicInfo::default)
            };

            let _ = self
                .basic_info
                .update_with_tx(
                    &mut *tx,
                    basic.id,
                    DeliberationBasicInfoRepositoryUpdateRequest {
                        started_at: Some(started_at),
                        ended_at: Some(ended_at),
                        title: Some(title),
                        description: Some(description),
                        deliberation_id: None,
                    },
                )
                .await?
                .ok_or(ApiError::DeliberationBasicInfoException)?;

            // update user
            let remain_users = DeliberationBasicInfoMember::query_builder()
                .basic_id_equals(basic.id)
                .query()
                .map(DeliberationBasicInfoMember::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_users: {:?}", remain_users);

            for remain in remain_users {
                self.basic_info_member
                    .delete_with_tx(&mut *tx, remain.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }

            for user_id in users {
                let _ = self
                    .basic_info_member
                    .insert_with_tx(&mut *tx, user_id, basic.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }

            //update resource
            let remain_resources = DeliberationBasicInfoResource::query_builder()
                .basic_id_equals(basic.id)
                .query()
                .map(DeliberationBasicInfoResource::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_resources: {:?}", remain_resources);

            for remain in remain_resources {
                self.basic_info_resource
                    .delete_with_tx(&mut *tx, remain.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }

            for resource_id in resources {
                let _ = self
                    .basic_info_resource
                    .insert_with_tx(&mut *tx, resource_id, basic.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }

            //update survey
            let remain_surveys = DeliberationBasicInfoSurvey::query_builder()
                .basic_id_equals(basic.id)
                .query()
                .map(DeliberationBasicInfoSurvey::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_surveys: {:?}", remain_surveys);

            for remain in remain_surveys {
                self.basic_info_survey
                    .delete_with_tx(&mut *tx, remain.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }

            for survey_id in surveys {
                let _ = self
                    .basic_info_survey
                    .insert_with_tx(&mut *tx, survey_id, basic.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }
        }
        Ok(())
    }

    async fn create_sample_survey(
        &self,
        tx: &mut sqlx::PgConnection,
        org_id: i64,
        deliberation_id: i64,
        project_areas: Vec<ProjectArea>,
        sample_surveys: Vec<DeliberationSampleSurveyCreateRequest>,
    ) -> Result<Vec<DeliberationSampleSurvey>> {
        let mut v = vec![];

        for DeliberationSampleSurveyCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            estimate_time,
            point,
            users,
            surveys,
        } in sample_surveys
        {
            let sample = self
                .sample_survey
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title.clone(),
                    description.clone(),
                    deliberation_id,
                    estimate_time,
                    point,
                )
                .await?
                .ok_or(ApiError::DeliberationSampleSurveyException)?;

            for user_id in users {
                let _ = self
                    .sample_survey_member
                    .insert_with_tx(&mut *tx, user_id, sample.id)
                    .await?
                    .ok_or(ApiError::DeliberationSampleSurveyException)?;
            }

            let survey = self
                .survey
                .insert_with_tx(
                    &mut *tx,
                    title,
                    ProjectType::Deliberation,
                    project_areas
                        .clone()
                        .get(0)
                        .unwrap_or(&ProjectArea::Economy)
                        .clone(),
                    ProjectStatus::InProgress,
                    started_at,
                    ended_at,
                    description,
                    0, //FIXME: fix quota
                    org_id,
                    surveys,
                    vec![],
                    vec![],
                    vec![], //FIXME: fix panel count
                    estimate_time,
                    point,
                    None,
                )
                .await?
                .ok_or(ApiError::DeliberationSampleSurveyException)?;

            let _ = self
                .sample_survey_survey
                .insert_with_tx(&mut *tx, survey.id, sample.id)
                .await?
                .ok_or(ApiError::DeliberationSampleSurveyException)?;

            v.push(sample);
        }
        Ok(v)
    }

    async fn upsert_sample_survey(
        &self,
        tx: &mut sqlx::PgConnection,
        org_id: i64,
        deliberation_id: i64,
        project_areas: Vec<ProjectArea>,
        sample_surveys: Vec<DeliberationSampleSurveyCreateRequest>,
    ) -> Result<()> {
        for DeliberationSampleSurveyCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            estimate_time,
            point,
            users,
            surveys,
        } in sample_surveys.clone()
        {
            let results = DeliberationSampleSurvey::query_builder()
                .deliberation_id_equals(deliberation_id)
                .query()
                .map(DeliberationSampleSurvey::from)
                .fetch_all(&self.pool)
                .await?;

            let sample = if results.is_empty() {
                let v = self
                    .create_sample_survey(
                        &mut *tx,
                        org_id,
                        deliberation_id,
                        project_areas.clone(),
                        sample_surveys.clone(),
                    )
                    .await?;

                v.into_iter()
                    .next()
                    .unwrap_or_else(DeliberationSampleSurvey::default)
            } else {
                results
                    .into_iter()
                    .next()
                    .unwrap_or_else(DeliberationSampleSurvey::default)
            };

            // update user
            let remain_users = DeliberationSampleSurveyMember::query_builder()
                .sample_survey_id_equals(sample.id)
                .query()
                .map(DeliberationSampleSurveyMember::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_users: {:?}", remain_users);

            for remain in remain_users {
                self.sample_survey_member
                    .delete_with_tx(&mut *tx, remain.id)
                    .await?
                    .ok_or(ApiError::DeliberationSampleSurveyException)?;
            }

            for user_id in users {
                let _ = self
                    .sample_survey_member
                    .insert_with_tx(&mut *tx, user_id, sample.id)
                    .await?
                    .ok_or(ApiError::DeliberationSampleSurveyException)?;
            }

            //update survey
            let remain_surveys = DeliberationSampleSurveySurvey::query_builder()
                .sample_survey_id_equals(sample.id)
                .query()
                .map(DeliberationSampleSurveySurvey::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_surveys: {:?}", remain_surveys);

            for survey in remain_surveys {
                self.sample_survey_survey
                    .delete_with_tx(&mut *tx, survey.id)
                    .await?
                    .ok_or(ApiError::DeliberationSampleSurveyException)?;
            }

            tracing::debug!("tttt");

            let survey = self
                .survey
                .insert_with_tx(
                    &mut *tx,
                    title,
                    ProjectType::Deliberation,
                    project_areas
                        .clone()
                        .get(0)
                        .unwrap_or(&ProjectArea::Economy)
                        .clone(),
                    ProjectStatus::InProgress,
                    started_at,
                    ended_at,
                    description,
                    0, //FIXME: fix quota
                    org_id,
                    surveys,
                    vec![],
                    vec![],
                    vec![], //FIXME: fix panel count
                    estimate_time,
                    point,
                    None,
                )
                .await?
                .ok_or(ApiError::DeliberationSampleSurveyException)?;
            tracing::debug!("rrrr");

            let _ = self
                .sample_survey_survey
                .insert_with_tx(&mut *tx, survey.id, sample.id)
                .await?
                .ok_or(ApiError::DeliberationSampleSurveyException)?;
            tracing::debug!("uuuu");
        }

        Ok(())
    }

    async fn create_content(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_id: i64,
        contents: Vec<DeliberationContentCreateRequest>,
    ) -> Result<()> {
        for DeliberationContentCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            questions,
            users,
            elearnings,
        } in contents
        {
            let content = self
                .deliberation_contents
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title,
                    description,
                    deliberation_id,
                    questions,
                )
                .await?
                .ok_or(ApiError::DeliberationLearningException)?;

            for user_id in users {
                let _ = self
                    .deliberation_contents_member
                    .insert_with_tx(&mut *tx, user_id, content.id)
                    .await?
                    .ok_or(ApiError::DeliberationLearningException)?;
            }

            for elearning in elearnings {
                let _ = self
                    .elearning_repo
                    .insert_with_tx(
                        &mut *tx,
                        content.id,
                        elearning.title,
                        elearning.resources,
                        elearning.necessary,
                    )
                    .await?
                    .ok_or(ApiError::DeliberationLearningException)?;
            }
        }
        Ok(())
    }

    async fn create_disscussion(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_id: i64,
        discussions: Vec<DeliberationDiscussionCreateRequest>,
    ) -> Result<()> {
        for DeliberationDiscussionCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            users,
            resources,
            discussions,
        } in discussions
        {
            let discussion = self
                .discussion_repo
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title,
                    description,
                    deliberation_id,
                )
                .await?
                .ok_or(ApiError::DeliberationDiscussionException)?;

            for user_id in users {
                let _ = self
                    .discussion_member
                    .insert_with_tx(&mut *tx, user_id, discussion.id)
                    .await?
                    .ok_or(ApiError::DeliberationDiscussionException)?;
            }

            for resource_id in resources {
                let _ = self
                    .discussion_resource
                    .insert_with_tx(&mut *tx, resource_id, discussion.id)
                    .await?
                    .ok_or(ApiError::DeliberationDiscussionException)?;
            }

            for disc in discussions {
                let d = self
                    .disc_repo
                    .insert_with_tx(
                        &mut *tx,
                        deliberation_id,
                        disc.started_at,
                        disc.ended_at,
                        disc.name,
                        disc.description,
                        disc.maximum_count,
                        None,
                    )
                    .await?
                    .ok_or(ApiError::DeliberationDiscussionException)?;

                for user_id in disc.users {
                    let _ = self
                        .disc_group
                        .insert_with_tx(&mut *tx, d.id, user_id)
                        .await?
                        .ok_or(ApiError::DeliberationDiscussionException)?;
                }

                for res_id in disc.resources {
                    let _ = self
                        .disc_res
                        .insert_with_tx(&mut *tx, d.id, res_id)
                        .await?
                        .ok_or(ApiError::DeliberationDiscussionException)?;
                }
            }
        }
        Ok(())
    }

    async fn create_final_survey(
        &self,
        tx: &mut sqlx::PgConnection,
        org_id: i64,
        deliberation_id: i64,
        project_areas: Vec<ProjectArea>,
        final_surveys: Vec<DeliberationFinalSurveyCreateRequest>,
    ) -> Result<()> {
        for DeliberationFinalSurveyCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            estimate_time,
            point,
            users,
            surveys,
        } in final_surveys
        {
            let d = self
                .final_repo
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title.clone(),
                    description.clone(),
                    deliberation_id,
                    estimate_time,
                    point,
                )
                .await?
                .ok_or(ApiError::DeliberationFinalSurveyException)?;

            for user_id in users {
                let _ = self
                    .final_member
                    .insert_with_tx(&mut *tx, user_id, d.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalSurveyException)?;
            }

            let survey = self
                .survey
                .insert_with_tx(
                    &mut *tx,
                    title,
                    ProjectType::Deliberation,
                    project_areas
                        .clone()
                        .get(0)
                        .unwrap_or(&ProjectArea::Economy)
                        .clone(),
                    ProjectStatus::InProgress,
                    started_at,
                    ended_at,
                    description,
                    0, //FIXME: fix quota
                    org_id,
                    surveys,
                    vec![],
                    vec![],
                    vec![], //FIXME: fix panel count
                    estimate_time,
                    point,
                    None,
                )
                .await?
                .ok_or(ApiError::DeliberationFinalSurveyException)?;

            let _ = self
                .final_survey
                .insert_with_tx(&mut *tx, survey.id, d.id)
                .await?
                .ok_or(ApiError::DeliberationFinalSurveyException)?;
        }
        Ok(())
    }

    async fn create_draft(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_id: i64,
        drafts: Vec<DeliberationDraftCreateRequest>,
    ) -> Result<()> {
        for DeliberationDraftCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            users,
            resources,
            surveys,
        } in drafts
        {
            let d = self
                .draft_repo
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title,
                    description,
                    deliberation_id,
                )
                .await?
                .ok_or(ApiError::DeliberationFinalRecommendationException)?;

            for user_id in users {
                let _ = self
                    .draft_member
                    .insert_with_tx(&mut *tx, user_id, d.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalRecommendationException)?;
            }

            for survey_id in surveys {
                let _ = self
                    .draft_survey
                    .insert_with_tx(&mut *tx, survey_id, d.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalRecommendationException)?;
            }

            for resource_id in resources {
                let _ = self
                    .draft_resource
                    .insert_with_tx(&mut *tx, resource_id, d.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalRecommendationException)?;
            }
        }
        Ok(())
    }

    async fn insert_deliberation_areas(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_id: i64,
        project_areas: Vec<ProjectArea>,
    ) -> Result<()> {
        for project_area in project_areas {
            self.deliberation_area
                .insert_with_tx(
                    &mut *tx,
                    project_area as i64, // FIXME: it should not be id value
                    deliberation_id,
                )
                .await?
                .ok_or(ApiError::DeliberationResourceException)?;
        }
        Ok(())
    }

    async fn upsert_deliberation_areas(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_id: i64,
        project_areas: Vec<ProjectArea>,
    ) -> Result<()> {
        let remain_project_areas = DeliberationArea::query_builder()
            .deliberation_id_equals(deliberation_id)
            .query()
            .map(DeliberationArea::from)
            .fetch_all(&self.pool)
            .await?;

        tracing::debug!("remain_project_areas: {:?}", remain_project_areas);

        for remain in remain_project_areas {
            self.deliberation_area
                .delete_with_tx(&mut *tx, remain.id)
                .await?
                .ok_or(ApiError::DeliberationResourceException)?;
        }

        tracing::debug!("project_areas: {:?}", project_areas.clone());

        self.insert_deliberation_areas(&mut *tx, deliberation_id, project_areas.clone())
            .await?;

        Ok(())
    }

    async fn insert_deliberation_users(
        &self,
        tx: &mut sqlx::PgConnection,
        org_id: i64,
        deliberation_id: i64,
        roles: Vec<DeliberationUserCreateRequest>,
    ) -> Result<()> {
        for DeliberationUserCreateRequest { user_id, role } in roles {
            match self
                .deliberation_user
                .insert_with_tx(&mut *tx, user_id, org_id, deliberation_id, role)
                .await?
                .ok_or(ApiError::DeliberationUserException)
            {
                Ok(_) => {
                    tracing::debug!("success to create user");
                }
                Err(e) => {
                    tracing::error!("failed to create user with error: {e}");
                }
            }
        }

        Ok(())
    }

    async fn upsert_deliberation_users(
        &self,
        tx: &mut sqlx::PgConnection,
        org_id: i64,
        deliberation_id: i64,
        roles: Vec<DeliberationUserCreateRequest>,
    ) -> Result<()> {
        let remain_users = DeliberationUser::query_builder()
            .deliberation_id_equals(deliberation_id)
            .query()
            .map(DeliberationUser::from)
            .fetch_all(&self.pool)
            .await?;

        tracing::debug!("remain_users: {:?}", remain_users);

        for remain in remain_users {
            self.deliberation_user
                .delete_with_tx(&mut *tx, remain.id)
                .await?
                .ok_or(ApiError::DeliberationResourceException)?;
        }

        tracing::debug!("deliberation users: {:?}", roles.clone());

        self.insert_deliberation_users(&mut *tx, org_id, deliberation_id, roles.clone())
            .await?;

        Ok(())
    }

    async fn upsert_deliberation_panels(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_id: i64,
        panel_ids: Vec<i64>,
    ) -> Result<()> {
        let remain_panels = PanelDeliberation::query_builder()
            .deliberation_id_equals(deliberation_id)
            .query()
            .map(PanelDeliberation::from)
            .fetch_all(&self.pool)
            .await?;

        tracing::debug!("remain_panels: {:?}", remain_panels);

        for remain in remain_panels {
            self.panel_deliberation
                .delete_with_tx(&mut *tx, remain.id)
                .await?
                .ok_or(ApiError::DeliberationPanelException)?;
        }

        tracing::debug!("deliberation panels: {:?}", panel_ids.clone());

        for id in panel_ids.clone() {
            self.panel_deliberation
                .insert_with_tx(&mut *tx, id, deliberation_id)
                .await?
                .ok_or(ApiError::DeliberationPanelException)?;
        }

        Ok(())
    }
}

impl DeliberationController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Deliberation::get_repository(pool.clone());
        let step = Step::get_repository(pool.clone());
        let deliberation_user = DeliberationUser::get_repository(pool.clone());
        let deliberation_resource = DeliberationResource::get_repository(pool.clone());
        let deliberation_survey = DeliberationSurvey::get_repository(pool.clone());
        let deliberation_area = DeliberationArea::get_repository(pool.clone());
        let panel_deliberation = PanelDeliberation::get_repository(pool.clone());
        let survey = SurveyV2::get_repository(pool.clone());
        let basic_info = DeliberationBasicInfo::get_repository(pool.clone());
        let basic_info_member = DeliberationBasicInfoMember::get_repository(pool.clone());
        let basic_info_resource = DeliberationBasicInfoResource::get_repository(pool.clone());
        let basic_info_survey = DeliberationBasicInfoSurvey::get_repository(pool.clone());
        let sample_survey = DeliberationSampleSurvey::get_repository(pool.clone());
        let sample_survey_member = DeliberationSampleSurveyMember::get_repository(pool.clone());
        let sample_survey_survey = DeliberationSampleSurveySurvey::get_repository(pool.clone());
        let deliberation_contents = DeliberationContent::get_repository(pool.clone());
        let deliberation_contents_member = DeliberationContentMember::get_repository(pool.clone());
        let elearning_repo = Elearning::get_repository(pool.clone());
        let discussion_repo = DeliberationDiscussion::get_repository(pool.clone());
        let discussion_member = DeliberationDiscussionMember::get_repository(pool.clone());
        let discussion_resource = DeliberationDiscussionResource::get_repository(pool.clone());
        let disc_repo = Discussion::get_repository(pool.clone());
        let disc_group = DiscussionGroup::get_repository(pool.clone());
        let disc_res = DiscussionResource::get_repository(pool.clone());
        let final_repo = DeliberationFinalSurvey::get_repository(pool.clone());
        let final_member = DeliberationFinalSurveyMember::get_repository(pool.clone());
        let final_survey = DeliberationFinalSurveySurvey::get_repository(pool.clone());
        let draft_repo = DeliberationDraft::get_repository(pool.clone());
        let draft_member = DeliberationDraftMember::get_repository(pool.clone());
        let draft_survey = DeliberationDraftSurvey::get_repository(pool.clone());
        let draft_resource = DeliberationDraftResource::get_repository(pool.clone());
        Self {
            pool,
            repo,
            step,
            deliberation_user,
            deliberation_resource,
            deliberation_survey,
            deliberation_area,
            panel_deliberation,
            survey,
            basic_info,
            basic_info_member,
            basic_info_resource,
            basic_info_survey,
            sample_survey,
            sample_survey_member,
            sample_survey_survey,
            deliberation_contents,
            deliberation_contents_member,
            elearning_repo,
            discussion_repo,
            discussion_member,
            discussion_resource,
            disc_repo,
            disc_group,
            disc_res,
            final_repo,
            final_member,
            final_survey,
            draft_repo,
            draft_member,
            draft_survey,
            draft_resource,
        }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_deliberation_by_id).post(Self::act_deliberation_by_id), // .post(Self::act_deliberation_by_id)
            )
            .with_state(self.clone())
            .route(
                "/",
                post(Self::act_deliberation).get(Self::get_deliberation),
            )
            .with_state(self.clone()))
    }

    pub async fn search_by(
        &self,
        org_id: i64,
        q: DeliberationQuery,
    ) -> Result<Json<DeliberationGetResponse>> {
        let mut total_count: i64 = 0;

        let items = DeliberationSummary::query_builder()
            .org_id_equals(org_id)
            .title_contains(q.clone().title.unwrap_or_default())
            .limit(q.size())
            .page(q.page())
            .query()
            .map(|r: PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(Json(DeliberationGetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }

    pub async fn act_deliberation(
        State(ctrl): State<DeliberationController>,
        Path(OrganizationPath { org_id }): Path<OrganizationPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<DeliberationAction>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("act_deliberation {} {:?}", org_id, body);

        match body {
            DeliberationAction::Create(param) => Ok(Json(ctrl.create(org_id, param).await?)),
        }
    }

    pub async fn get_deliberation_by_id(
        State(ctrl): State<DeliberationController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationPath { org_id, id }): Path<DeliberationPath>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("get_deliberation {} {:?}", org_id, id);
        // FIXME: {"DatabaseQueryError": "error returned from database: relation \"f\" does not exist"
        Ok(Json(
            Deliberation::query_builder()
                .id_equals(id)
                .org_id_equals(org_id)
                .basic_infos_builder(DeliberationBasicInfo::query_builder())
                .sample_surveys_builder(DeliberationSampleSurvey::query_builder())
                .query()
                .map(Deliberation::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn act_deliberation_by_id(
        State(ctrl): State<DeliberationController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationPath { org_id, id }): Path<DeliberationPath>,
        Json(body): Json<DeliberationByIdAction>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("get_deliberation {} {:?}", org_id, id);
        match body {
            DeliberationByIdAction::Update(param) => {
                Ok(Json(ctrl.update(id, org_id, param).await?))
            }
        }
    }

    pub async fn get_deliberation(
        State(ctrl): State<DeliberationController>,
        Path(OrganizationPath { org_id }): Path<OrganizationPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<DeliberationParam>,
    ) -> Result<Json<DeliberationGetResponse>> {
        tracing::debug!("list_deliberation {} {:?}", org_id, param);

        match param {
            // "DatabaseQueryError": "error returned from database: relation \"f\" does not exist"
            DeliberationParam::Query(q) => match q.action {
                Some(DeliberationQueryActionType::SearchBy) => ctrl.search_by(org_id, q).await,
                None => {
                    return Ok(Json(DeliberationGetResponse::Query(
                        ctrl.query(org_id, q).await?,
                    )));
                }
            },
            DeliberationParam::Read(action) => {
                return Ok(Json(DeliberationGetResponse::Read(
                    ctrl.get_draft(org_id, action.id).await?,
                )));
            }
        }
    }

    pub async fn get_draft(&self, org_id: i64, id: Option<i64>) -> Result<Deliberation> {
        if id.is_none() {
            return Err(
                ApiError::ValidationError("deliberation id is required".to_string()).into(),
            );
        }

        let id = id.unwrap();

        let deliberation = Deliberation::query_builder()
            .basic_infos_builder(DeliberationBasicInfo::query_builder())
            .sample_surveys_builder(DeliberationSampleSurvey::query_builder())
            .org_id_equals(org_id)
            .id_equals(id)
            .status_equals(DeliberationStatus::Draft)
            .query()
            .map(Deliberation::from)
            .fetch_one(&self.pool)
            .await?;

        Ok(deliberation)
    }
}

#[cfg(test)]
mod tests {
    use models::{
        deliberation::{Deliberation, DeliberationQuery, DeliberationStatus},
        ProjectArea,
    };

    use crate::tests::{setup, TestContext};

    #[tokio::test]
    async fn test_deliberation_empty() {
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let org_id = user.orgs[0].id;

        let cli = Deliberation::get_client(&endpoint);
        let res = cli
            .create(
                org_id,
                now,
                now + 1000,
                "".to_string(),
                format!("test deliberation {now}"),
                "test description".to_string(),
                ProjectArea::City,
                DeliberationStatus::Draft,
                user.id,
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            )
            .await;
        assert!(res.is_ok());

        let res = cli.query(org_id, DeliberationQuery::new(10)).await.unwrap();

        assert_eq!(res.items.len(), 1)
    }
}
