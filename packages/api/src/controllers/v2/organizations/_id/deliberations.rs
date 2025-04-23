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
use discussion_resources::DiscussionResource;
use discussions::Discussion;
use models::{
    deliberation::*,
    deliberation_areas::deliberation_area::*,
    deliberation_basic_info_resources::deliberation_basic_info_resource::*,
    deliberation_basic_info_roles::deliberation_basic_info_role::*,
    deliberation_basic_info_surveys::deliberation_basic_info_survey::*,
    deliberation_basic_infos::deliberation_basic_info::*,
    deliberation_content_roles::deliberation_content_role::*,
    deliberation_contents::deliberation_content::*,
    deliberation_discussion_resources::deliberation_discussion_resource::*,
    deliberation_discussion_roles::deliberation_discussion_role::*,
    deliberation_discussions::deliberation_discussion::*,
    deliberation_draft_members::deliberation_draft_member::*,
    deliberation_draft_resources::deliberation_draft_resource::*,
    deliberation_draft_surveys::deliberation_draft_survey::*,
    deliberation_drafts::deliberation_draft::*,
    deliberation_final_survey_roles::deliberation_final_survey_role::*,
    deliberation_final_survey_surveys::deliberation_final_survey_survey::*,
    deliberation_final_surveys::deliberation_final_survey::*,
    deliberation_panel_email::{DeliberationPanelEmail, DeliberationPanelEmailRepository},
    deliberation_role::{
        DeliberationRole, DeliberationRoleCreateRequest, DeliberationRoleRepository,
    },
    deliberation_sample_survey_roles::deliberation_sample_survey_role::*,
    deliberation_sample_survey_surveys::deliberation_sample_survey_survey::*,
    deliberation_sample_surveys::deliberation_sample_survey::*,
    discussion_groups::DiscussionGroup,
    elearnings::elearning::*,
    step::*,
    *,
};
use sqlx::postgres::PgRow;
use step::StepCreateRequest;

use crate::{controllers::v2::organizations::OrganizationPath, utils::app_claims::AppClaims};

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
    deliberation_role: DeliberationRoleRepository,
    deliberation_resource: DeliberationResourceRepository,
    deliberation_survey: DeliberationSurveyRepository,
    deliberation_area: DeliberationAreaRepository,
    survey: SurveyV2Repository,
    basic_info: DeliberationBasicInfoRepository,
    basic_info_role: DeliberationBasicInfoRoleRepository,
    basic_info_resource: DeliberationBasicInfoResourceRepository,
    basic_info_survey: DeliberationBasicInfoSurveyRepository,
    sample_survey: DeliberationSampleSurveyRepository,
    sample_survey_role: DeliberationSampleSurveyRoleRepository,
    sample_survey_survey: DeliberationSampleSurveySurveyRepository,
    deliberation_contents: DeliberationContentRepository,
    deliberation_contents_role: DeliberationContentRoleRepository,
    elearning_repo: ElearningRepository,
    discussion_repo: DeliberationDiscussionRepository,
    discussion_role: DeliberationDiscussionRoleRepository,
    discussion_resource: DeliberationDiscussionResourceRepository,
    disc_repo: DiscussionRepository,
    disc_group: DiscussionGroupRepository,
    disc_res: DiscussionResourceRepository,
    final_repo: DeliberationFinalSurveyRepository,
    final_role: DeliberationFinalSurveyRoleRepository,
    final_survey: DeliberationFinalSurveySurveyRepository,
    draft_repo: DeliberationDraftRepository,
    draft_member: DeliberationDraftMemberRepository,
    draft_survey: DeliberationDraftSurveyRepository,
    draft_resource: DeliberationDraftResourceRepository,
    panel_emails: DeliberationPanelEmailRepository,
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
            panel_emails,
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
            ..
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

        self.insert_deliberation_panel_with_emails(&mut *tx, deliberation.id, panel_emails.clone())
            .await?;

        self.insert_deliberation_areas(&mut *tx, deliberation.id, project_areas.clone())
            .await?;

        let deliberation_roles = self
            .insert_deliberation_users(&mut *tx, deliberation.id, roles.clone())
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

        self.create_basic_info(
            &mut *tx,
            deliberation_roles.clone(),
            deliberation.id,
            basic_infos,
        )
        .await?;
        self.create_sample_survey(
            &mut *tx,
            deliberation_roles.clone(),
            org_id,
            deliberation.id,
            project_areas.clone(),
            sample_surveys,
        )
        .await?;
        self.create_content(
            &mut *tx,
            deliberation_roles.clone(),
            deliberation.id,
            contents,
        )
        .await?;
        self.create_disscussion(
            &mut *tx,
            deliberation_roles.clone(),
            deliberation.id,
            deliberation_discussions,
        )
        .await?;
        self.create_final_survey(
            &mut *tx,
            deliberation_roles.clone(),
            org_id,
            deliberation.id,
            project_areas.clone(),
            final_surveys,
        )
        .await?;
        self.create_draft(&mut *tx, deliberation.id, drafts).await?;

        // for id in panel_ids {
        //     self.panel_deliberation
        //         .insert_with_tx(&mut *tx, id, deliberation.id)
        //         .await?
        //         .ok_or(ApiError::DeliberationPanelException)?;
        // }

        tx.commit().await?;

        Ok(deliberation)
    }

    pub async fn remove_deliberation(&self, id: i64) -> Result<Deliberation> {
        let deliberation = self.repo.delete(id).await?;

        Ok(deliberation)
    }

    pub async fn start_deliberation(&self, id: i64) -> Result<Deliberation> {
        let deliberation = self
            .repo
            .update(
                id,
                DeliberationRepositoryUpdateRequest {
                    status: Some(DeliberationStatus::Ready),
                    ..Default::default()
                },
            )
            .await?;

        Ok(deliberation)
    }

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
            panel_emails,
            roles,
            thumbnail_image,
            basic_infos,
            sample_surveys,
            contents,
            deliberation_discussions,
            final_surveys,
            status,
            creator_id,
            ..
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

        let deliberation_roles = self
            .upsert_deliberation_users(&mut *tx, id, roles.clone())
            .await?;

        self.upsert_deliberation_panel_with_emails(&mut *tx, id, panel_emails)
            .await?;

        // self.upsert_deliberation_panels(&mut *tx, id, panel_ids)
        //     .await?;

        self.upsert_basic_info(&mut *tx, deliberation_roles.clone(), id, basic_infos)
            .await?;

        self.upsert_sample_survey(
            &mut *tx,
            deliberation_roles.clone(),
            org_id,
            deliberation.id,
            project_areas.clone(),
            sample_surveys,
        )
        .await?;

        self.upsert_content(
            &mut *tx,
            deliberation_roles.clone(),
            deliberation.id,
            contents,
        )
        .await?;

        self.upsert_discussion(
            &mut *tx,
            deliberation_roles.clone(),
            deliberation.id,
            deliberation_discussions,
        )
        .await?;

        self.upsert_final_survey(
            &mut *tx,
            deliberation_roles,
            org_id,
            deliberation.id,
            project_areas.clone(),
            final_surveys,
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
            .order_by_created_at_desc()
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
        deliberation_roles: Vec<DeliberationRole>,
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

            let mut roles = vec![];

            for email in users {
                let d = deliberation_roles
                    .iter()
                    .find(|v| v.email == email)
                    .unwrap_or(&DeliberationRole::default())
                    .clone();

                roles.push(d);
            }

            for role in roles {
                let _ = self
                    .basic_info_role
                    .insert_with_tx(&mut *tx, role.id, info.id)
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
        deliberation_roles: Vec<DeliberationRole>,
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
                    .create_basic_info(
                        &mut *tx,
                        deliberation_roles.clone(),
                        deliberation_id,
                        basic_infos.clone(),
                    )
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
            let remain_users = DeliberationBasicInfoRole::query_builder()
                .basic_id_equals(basic.id)
                .query()
                .map(DeliberationBasicInfoRole::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_users: {:?}", remain_users);

            for remain in remain_users {
                match self
                    .basic_info_role
                    .delete_with_tx(&mut *tx, remain.id)
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("basic info delete failed with error: {:?}", e);
                        return Err(ApiError::DeliberationBasicInfoException);
                    }
                };
            }

            let mut roles = vec![];

            for email in users {
                let d = deliberation_roles
                    .iter()
                    .find(|v| v.email == email)
                    .unwrap_or(&DeliberationRole::default())
                    .clone();

                roles.push(d);
            }

            for role in roles {
                let _ = self
                    .basic_info_role
                    .insert_with_tx(&mut *tx, role.id, basic.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }
            tracing::debug!("88888");

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
        deliberation_roles: Vec<DeliberationRole>,
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

            let mut roles = vec![];

            for email in users {
                let d = deliberation_roles
                    .iter()
                    .find(|v| v.email == email)
                    .unwrap_or(&DeliberationRole::default())
                    .clone();

                roles.push(d);
            }

            for role in roles {
                let _ = self
                    .sample_survey_role
                    .insert_with_tx(&mut *tx, role.id, sample.id)
                    .await?
                    .ok_or(ApiError::DeliberationSampleSurveyException)?;
            }

            let survey = self
                .survey
                .insert_with_tx(
                    &mut *tx,
                    title,
                    ProjectType::SampleSurvey,
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
        deliberation_roles: Vec<DeliberationRole>,
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
                        deliberation_roles.clone(),
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

            let _ = self
                .sample_survey
                .update_with_tx(
                    &mut *tx,
                    sample.id,
                    DeliberationSampleSurveyRepositoryUpdateRequest {
                        started_at: Some(started_at),
                        ended_at: Some(ended_at),
                        title: Some(title.clone()),
                        description: Some(description.clone()),
                        deliberation_id: None,
                        estimate_time: Some(estimate_time),
                        point: Some(point),
                    },
                )
                .await?
                .ok_or(ApiError::DeliberationSampleSurveyException)?;

            // update user
            let remain_users = DeliberationSampleSurveyRole::query_builder()
                .sample_survey_id_equals(sample.id)
                .query()
                .map(DeliberationSampleSurveyRole::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_users: {:?}", remain_users);

            for remain in remain_users {
                match self
                    .sample_survey_role
                    .delete_with_tx(&mut *tx, remain.id)
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("sample survey delete failed with error: {:?}", e);
                        return Err(ApiError::DeliberationSampleSurveyException);
                    }
                }
            }

            let mut roles = vec![];

            for email in users {
                let d = deliberation_roles
                    .iter()
                    .find(|v| v.email == email)
                    .unwrap_or(&DeliberationRole::default())
                    .clone();

                roles.push(d);
            }

            for role in roles {
                let _ = self
                    .sample_survey_role
                    .insert_with_tx(&mut *tx, role.id, sample.id)
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

            let survey = self
                .survey
                .insert_with_tx(
                    &mut *tx,
                    title,
                    ProjectType::SampleSurvey,
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
        }

        Ok(())
    }

    async fn create_content(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_roles: Vec<DeliberationRole>,
        deliberation_id: i64,
        contents: Vec<DeliberationContentCreateRequest>,
    ) -> Result<Vec<DeliberationContent>> {
        let mut v = vec![];
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

            let mut roles = vec![];

            for email in users {
                let d = deliberation_roles
                    .iter()
                    .find(|v| v.email == email)
                    .unwrap_or(&DeliberationRole::default())
                    .clone();

                roles.push(d);
            }

            for role in roles {
                let _ = self
                    .deliberation_contents_role
                    .insert_with_tx(&mut *tx, role.id, content.id)
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

            v.push(content);
        }
        Ok(v)
    }

    async fn upsert_content(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_roles: Vec<DeliberationRole>,
        deliberation_id: i64,
        contents: Vec<DeliberationContentCreateRequest>,
    ) -> Result<()> {
        for DeliberationContentCreateRequest {
            users,
            elearnings,
            started_at,
            ended_at,
            title,
            description,
            questions,
        } in contents.clone()
        {
            let results = DeliberationContent::query_builder()
                .deliberation_id_equals(deliberation_id)
                .query()
                .map(DeliberationContent::from)
                .fetch_all(&self.pool)
                .await?;

            let content = if results.is_empty() {
                let v = self
                    .create_content(
                        &mut *tx,
                        deliberation_roles.clone(),
                        deliberation_id,
                        contents.clone(),
                    )
                    .await?;

                v.into_iter()
                    .next()
                    .unwrap_or_else(DeliberationContent::default)
            } else {
                let results = results[0].clone();
                let v = self
                    .deliberation_contents
                    .update_with_tx(
                        &mut *tx,
                        results.id,
                        DeliberationContentRepositoryUpdateRequest {
                            started_at: Some(started_at),
                            ended_at: Some(ended_at),
                            title: Some(title.clone()),
                            description: Some(description.clone()),
                            deliberation_id: Some(results.deliberation_id),
                            questions: Some(questions.clone()),
                        },
                    )
                    .await?;

                v.unwrap_or_default()
            };

            let _ = self
                .deliberation_contents
                .update_with_tx(
                    &mut *tx,
                    content.id,
                    DeliberationContentRepositoryUpdateRequest {
                        started_at: Some(started_at),
                        ended_at: Some(ended_at),
                        title: Some(title.clone()),
                        description: Some(description.clone()),
                        deliberation_id: None,
                        questions: Some(questions),
                    },
                )
                .await?
                .ok_or(ApiError::DeliberationLearningException)?;

            // update user
            let remain_users = DeliberationContentRole::query_builder()
                .content_id_equals(content.id)
                .query()
                .map(DeliberationContentRole::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_users: {:?}", remain_users);

            for remain in remain_users {
                match self
                    .deliberation_contents_role
                    .delete_with_tx(&mut *tx, remain.id)
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("contents delete failed with error: {:?}", e);
                        return Err(ApiError::DeliberationLearningException);
                    }
                }
            }

            let mut roles = vec![];

            for email in users {
                let d = deliberation_roles
                    .iter()
                    .find(|v| v.email == email)
                    .unwrap_or(&DeliberationRole::default())
                    .clone();

                roles.push(d);
            }

            for role in roles {
                let _ = self
                    .deliberation_contents_role
                    .insert_with_tx(&mut *tx, role.id, content.id)
                    .await?
                    .ok_or(ApiError::DeliberationLearningException)?;
            }

            // update elearnings
            let remain_elearnings = Elearning::query_builder()
                .content_id_equals(content.id)
                .query()
                .map(Elearning::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_elearning: {:?}", remain_elearnings);

            for elearning in remain_elearnings {
                self.elearning_repo
                    .delete_with_tx(&mut *tx, elearning.id)
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
        deliberation_roles: Vec<DeliberationRole>,
        deliberation_id: i64,
        discussions: Vec<DeliberationDiscussionCreateRequest>,
    ) -> Result<Vec<DeliberationDiscussion>> {
        let mut v = vec![];
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

            let mut roles = vec![];

            for email in users {
                let d = deliberation_roles
                    .iter()
                    .find(|v| v.email == email)
                    .unwrap_or(&DeliberationRole::default())
                    .clone();

                roles.push(d);
            }

            for role in roles {
                let _ = self
                    .discussion_role
                    .insert_with_tx(&mut *tx, role.id, discussion.id)
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

            v.push(discussion);
        }
        Ok(v)
    }

    async fn upsert_discussion(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_roles: Vec<DeliberationRole>,
        deliberation_id: i64,
        deliberation_discussions: Vec<DeliberationDiscussionCreateRequest>,
    ) -> Result<()> {
        for DeliberationDiscussionCreateRequest {
            users,
            resources,
            discussions,
            started_at,
            ended_at,
            title,
            description,
        } in deliberation_discussions.clone()
        {
            let results = DeliberationDiscussion::query_builder()
                .deliberation_id_equals(deliberation_id)
                .query()
                .map(DeliberationDiscussion::from)
                .fetch_all(&self.pool)
                .await?;

            let discussion = if results.is_empty() {
                let v = self
                    .create_disscussion(
                        &mut *tx,
                        deliberation_roles.clone(),
                        deliberation_id,
                        deliberation_discussions.clone(),
                    )
                    .await?;

                v.into_iter()
                    .next()
                    .unwrap_or_else(DeliberationDiscussion::default)
            } else {
                results
                    .into_iter()
                    .next()
                    .unwrap_or_else(DeliberationDiscussion::default)
            };

            let _ = self
                .discussion_repo
                .update_with_tx(
                    &mut *tx,
                    discussion.id,
                    DeliberationDiscussionRepositoryUpdateRequest {
                        started_at: Some(started_at),
                        ended_at: Some(ended_at),
                        title: Some(title.clone()),
                        description: Some(description.clone()),
                        deliberation_id: None,
                    },
                )
                .await?
                .ok_or(ApiError::DeliberationDiscussionException)?;

            // update user
            let remain_users = DeliberationDiscussionRole::query_builder()
                .discussion_id_equals(discussion.id)
                .query()
                .map(DeliberationDiscussionRole::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_users: {:?}", remain_users);

            for remain in remain_users {
                match self
                    .discussion_role
                    .delete_with_tx(&mut *tx, remain.id)
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("discussion delete failed with error: {:?}", e);
                        return Err(ApiError::DeliberationDiscussionException);
                    }
                }
            }

            let mut roles = vec![];

            for email in users {
                let d = deliberation_roles
                    .iter()
                    .find(|v| v.email == email)
                    .unwrap_or(&DeliberationRole::default())
                    .clone();

                roles.push(d);
            }

            for role in roles {
                let _ = self
                    .discussion_role
                    .insert_with_tx(&mut *tx, role.id, discussion.id)
                    .await?
                    .ok_or(ApiError::DeliberationDiscussionException)?;
            }

            // update resource
            let remain_resources = DeliberationDiscussionResource::query_builder()
                .discussion_id_equals(discussion.id)
                .query()
                .map(DeliberationDiscussionResource::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_resources: {:?}", remain_resources);

            for remain in remain_resources {
                self.discussion_resource
                    .delete_with_tx(&mut *tx, remain.id)
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

            let remain_discussions = Discussion::query_builder()
                .deliberation_id_equals(deliberation_id)
                .query()
                .map(Discussion::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_discussions: {:?}", remain_discussions);

            for discussion in remain_discussions {
                self.disc_repo
                    .delete_with_tx(&mut *tx, discussion.id)
                    .await?
                    .ok_or(ApiError::DeliberationDiscussionException)?;
            }

            let remain_discussion_resources = DiscussionResource::query_builder()
                .discussion_id_equals(discussion.id)
                .query()
                .map(Discussion::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!(
                "remain_discussion_resources: {:?}",
                remain_discussion_resources
            );

            for resource in remain_discussion_resources {
                self.disc_res
                    .delete_with_tx(&mut *tx, resource.id)
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
        deliberation_roles: Vec<DeliberationRole>,
        org_id: i64,
        deliberation_id: i64,
        project_areas: Vec<ProjectArea>,
        final_surveys: Vec<DeliberationFinalSurveyCreateRequest>,
    ) -> Result<Vec<DeliberationFinalSurvey>> {
        let mut v = vec![];
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

            let mut roles = vec![];

            for email in users {
                let d = deliberation_roles
                    .iter()
                    .find(|v| v.email == email)
                    .unwrap_or(&DeliberationRole::default())
                    .clone();

                roles.push(d);
            }

            for role in roles {
                let _ = self
                    .final_role
                    .insert_with_tx(&mut *tx, role.id, d.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalSurveyException)?;
            }

            let survey = self
                .survey
                .insert_with_tx(
                    &mut *tx,
                    title,
                    ProjectType::FinalSurvey,
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
            v.push(d);
        }
        Ok(v)
    }

    async fn upsert_final_survey(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_roles: Vec<DeliberationRole>,
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
        } in final_surveys.clone()
        {
            let results = DeliberationFinalSurvey::query_builder()
                .deliberation_id_equals(deliberation_id)
                .query()
                .map(DeliberationFinalSurvey::from)
                .fetch_all(&self.pool)
                .await?;

            let d = if results.is_empty() {
                let v = self
                    .create_final_survey(
                        &mut *tx,
                        deliberation_roles.clone(),
                        org_id,
                        deliberation_id,
                        project_areas.clone(),
                        final_surveys.clone(),
                    )
                    .await?;

                v.into_iter()
                    .next()
                    .unwrap_or_else(DeliberationFinalSurvey::default)
            } else {
                results
                    .into_iter()
                    .next()
                    .unwrap_or_else(DeliberationFinalSurvey::default)
            };

            let _ = self
                .final_repo
                .update_with_tx(
                    &mut *tx,
                    d.id,
                    DeliberationFinalSurveyRepositoryUpdateRequest {
                        title: Some(title.clone()),
                        description: Some(description.clone()),
                        estimate_time: Some(estimate_time),
                        point: Some(point),
                        started_at: Some(started_at),
                        ended_at: Some(ended_at),
                        deliberation_id: None,
                    },
                )
                .await?
                .ok_or(ApiError::DeliberationSampleSurveyException)?;

            // update user
            let remain_users = DeliberationFinalSurveyRole::query_builder()
                .final_survey_id_equals(d.id)
                .query()
                .map(DeliberationFinalSurveyRole::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_users: {:?}", remain_users);

            for remain in remain_users {
                match self.final_role.delete_with_tx(&mut *tx, remain.id).await {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("final survey delete failed with error: {:?}", e);
                        return Err(ApiError::DeliberationFinalSurveyException);
                    }
                }
            }

            let mut roles = vec![];

            for email in users {
                let d = deliberation_roles
                    .iter()
                    .find(|v| v.email == email)
                    .unwrap_or(&DeliberationRole::default())
                    .clone();

                roles.push(d);
            }

            for role in roles {
                let _ = self
                    .final_role
                    .insert_with_tx(&mut *tx, role.id, d.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalSurveyException)?;
            }

            //update survey
            let remain_surveys = DeliberationFinalSurveySurvey::query_builder()
                .final_survey_id_equals(d.id)
                .query()
                .map(DeliberationFinalSurveySurvey::from)
                .fetch_all(&self.pool)
                .await?;

            tracing::debug!("remain_surveys: {:?}", remain_surveys);

            for survey in remain_surveys {
                self.final_survey
                    .delete_with_tx(&mut *tx, survey.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalSurveyException)?;
            }

            let survey = self
                .survey
                .insert_with_tx(
                    &mut *tx,
                    title,
                    ProjectType::FinalSurvey,
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

    async fn insert_deliberation_panel_with_emails(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_id: i64,
        emails: Vec<String>,
    ) -> Result<()> {
        for email in emails {
            self.panel_emails
                .insert_with_tx(&mut *tx, email, deliberation_id)
                .await?
                .ok_or(ApiError::DeliberationPanelException)?;
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
        deliberation_id: i64,
        roles: Vec<DeliberationRoleCreateRequest>,
    ) -> Result<Vec<DeliberationRole>> {
        let mut deliberation_roles = vec![];
        for DeliberationRoleCreateRequest { email, role } in roles {
            match self
                .deliberation_role
                .insert_with_tx(&mut *tx, deliberation_id, email, role)
                .await?
                .ok_or(ApiError::DeliberationUserException)
            {
                Ok(v) => {
                    tracing::debug!("success to create user");
                    deliberation_roles.push(v);
                }
                Err(e) => {
                    tracing::error!("failed to create user with error: {e}");
                }
            }
        }

        Ok(deliberation_roles)
    }

    async fn upsert_deliberation_users(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_id: i64,
        roles: Vec<DeliberationRoleCreateRequest>,
    ) -> Result<Vec<DeliberationRole>> {
        let remain_roles = DeliberationRole::query_builder()
            .deliberation_id_equals(deliberation_id)
            .query()
            .map(DeliberationRole::from)
            .fetch_all(&self.pool)
            .await?;

        tracing::debug!("remain_users: {:?}", remain_roles);

        for remain in remain_roles {
            self.deliberation_role
                .delete_with_tx(&mut *tx, remain.id)
                .await?
                .ok_or(ApiError::DeliberationResourceException)?;
        }

        tracing::debug!("deliberation users: {:?}", roles.clone());

        let deliberation_roles = self
            .insert_deliberation_users(&mut *tx, deliberation_id, roles.clone())
            .await?;

        Ok(deliberation_roles)
    }

    async fn upsert_deliberation_panel_with_emails(
        &self,
        tx: &mut sqlx::PgConnection,
        deliberation_id: i64,
        emails: Vec<String>,
    ) -> Result<()> {
        let remain_panels = DeliberationPanelEmail::query_builder()
            .deliberation_id_equals(deliberation_id)
            .query()
            .map(DeliberationPanelEmail::from)
            .fetch_all(&self.pool)
            .await?;

        tracing::debug!("remain_panels: {:?}", remain_panels);

        for remain in remain_panels {
            match self.panel_emails.delete_with_tx(&mut *tx, remain.id).await {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("panel remove failed with error: {:?}", e);
                    return Err(ApiError::DeliberationPanelException);
                }
            }
        }

        for email in emails.clone() {
            self.panel_emails
                .insert_with_tx(&mut *tx, email, deliberation_id)
                .await?
                .ok_or(ApiError::DeliberationPanelException)?;
        }

        Ok(())
    }

    // async fn upsert_deliberation_panels(
    //     &self,
    //     tx: &mut sqlx::PgConnection,
    //     deliberation_id: i64,
    //     panel_ids: Vec<i64>,
    // ) -> Result<()> {
    //     let remain_panels = PanelDeliberation::query_builder()
    //         .deliberation_id_equals(deliberation_id)
    //         .query()
    //         .map(PanelDeliberation::from)
    //         .fetch_all(&self.pool)
    //         .await?;

    //     tracing::debug!("remain_panels: {:?}", remain_panels);

    //     for remain in remain_panels {
    //         self.panel_deliberation
    //             .delete_with_tx(&mut *tx, remain.id)
    //             .await?
    //             .ok_or(ApiError::DeliberationPanelException)?;
    //     }

    //     tracing::debug!("deliberation panels: {:?}", panel_ids.clone());

    //     for id in panel_ids.clone() {
    //         self.panel_deliberation
    //             .insert_with_tx(&mut *tx, id, deliberation_id)
    //             .await?
    //             .ok_or(ApiError::DeliberationPanelException)?;
    //     }

    //     Ok(())
    // }
}

impl DeliberationController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Deliberation::get_repository(pool.clone());
        let step = Step::get_repository(pool.clone());
        let deliberation_role = DeliberationRole::get_repository(pool.clone());
        let deliberation_resource = DeliberationResource::get_repository(pool.clone());
        let deliberation_survey = DeliberationSurvey::get_repository(pool.clone());
        let deliberation_area = DeliberationArea::get_repository(pool.clone());

        let survey = SurveyV2::get_repository(pool.clone());
        let basic_info = DeliberationBasicInfo::get_repository(pool.clone());
        let basic_info_role = DeliberationBasicInfoRole::get_repository(pool.clone());
        let basic_info_resource = DeliberationBasicInfoResource::get_repository(pool.clone());
        let basic_info_survey = DeliberationBasicInfoSurvey::get_repository(pool.clone());
        let sample_survey = DeliberationSampleSurvey::get_repository(pool.clone());
        let sample_survey_role = DeliberationSampleSurveyRole::get_repository(pool.clone());
        let sample_survey_survey = DeliberationSampleSurveySurvey::get_repository(pool.clone());
        let deliberation_contents = DeliberationContent::get_repository(pool.clone());
        let deliberation_contents_role = DeliberationContentRole::get_repository(pool.clone());
        let elearning_repo = Elearning::get_repository(pool.clone());
        let discussion_repo = DeliberationDiscussion::get_repository(pool.clone());
        let discussion_role = DeliberationDiscussionRole::get_repository(pool.clone());
        let discussion_resource = DeliberationDiscussionResource::get_repository(pool.clone());
        let disc_repo = Discussion::get_repository(pool.clone());
        let disc_group = DiscussionGroup::get_repository(pool.clone());
        let disc_res = DiscussionResource::get_repository(pool.clone());
        let final_repo = DeliberationFinalSurvey::get_repository(pool.clone());
        let final_role = DeliberationFinalSurveyRole::get_repository(pool.clone());
        let final_survey = DeliberationFinalSurveySurvey::get_repository(pool.clone());
        let draft_repo = DeliberationDraft::get_repository(pool.clone());
        let draft_member = DeliberationDraftMember::get_repository(pool.clone());
        let draft_survey = DeliberationDraftSurvey::get_repository(pool.clone());
        let draft_resource = DeliberationDraftResource::get_repository(pool.clone());
        let panel_emails = DeliberationPanelEmail::get_repository(pool.clone());

        Self {
            pool,
            repo,
            step,
            deliberation_role,
            deliberation_resource,
            deliberation_survey,
            deliberation_area,
            survey,
            basic_info,
            basic_info_role,
            basic_info_resource,
            basic_info_survey,
            sample_survey,
            sample_survey_role,
            sample_survey_survey,
            deliberation_contents,
            deliberation_contents_role,
            elearning_repo,
            discussion_repo,
            discussion_role,
            discussion_resource,
            disc_repo,
            disc_group,
            disc_res,
            final_repo,
            final_role,
            final_survey,
            draft_repo,
            draft_member,
            draft_survey,
            draft_resource,
            panel_emails,
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
            .order_by_created_at_desc()
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
        Ok(Json(
            Deliberation::query_builder()
                .id_equals(id)
                .org_id_equals(org_id)
                .basic_infos_builder(DeliberationBasicInfo::query_builder())
                .sample_surveys_builder(DeliberationSampleSurvey::query_builder())
                .contents_builder(DeliberationContent::query_builder())
                .deliberation_discussions_builder(DeliberationDiscussion::query_builder())
                .final_surveys_builder(DeliberationFinalSurvey::query_builder())
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
            DeliberationByIdAction::StartDeliberation(_) => {
                Ok(Json(ctrl.start_deliberation(id).await?))
            }
            DeliberationByIdAction::RemoveDeliberation(_) => {
                Ok(Json(ctrl.remove_deliberation(id).await?))
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
            .contents_builder(DeliberationContent::query_builder())
            .deliberation_discussions_builder(DeliberationDiscussion::query_builder())
            .final_surveys_builder(DeliberationFinalSurvey::query_builder())
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
                vec![],
            )
            .await;
        assert!(res.is_ok());

        let res = cli.query(org_id, DeliberationQuery::new(10)).await.unwrap();

        assert_eq!(res.items.len(), 1)
    }
}
