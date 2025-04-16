use bdk::prelude::*;
use models::{
    deliberation_user::DeliberationUserCreateRequest, DeliberationBasicInfoCreateRequest,
    DeliberationContentCreateRequest, DeliberationDiscussionCreateRequest,
    DeliberationFinalSurveyCreateRequest, DeliberationSampleSurveyCreateRequest,
    DeliberationStatus, OrganizationMember, OrganizationMemberQuery, OrganizationMemberSummary,
    PanelV2, PanelV2Query, PanelV2Summary, Role, *,
};

use crate::{config, routes::Route, service::login_service::LoginService};

use super::ParentController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,

    #[allow(dead_code)]
    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub committees: Signal<Vec<DeliberationUserCreateRequest>>,
    pub committee_roles: Signal<Vec<Vec<OrganizationMemberSummary>>>,
    pub roles: Signal<Vec<Role>>,

    #[allow(dead_code)]
    pub panels: Resource<Vec<PanelV2Summary>>,
    pub selected_panels: Signal<Vec<PanelV2Summary>>,

    pub basic_info: Signal<DeliberationBasicInfoCreateRequest>,
    pub sample_survey: Signal<DeliberationSampleSurveyCreateRequest>,
    pub deliberation: Signal<DeliberationContentCreateRequest>,
    pub discussion: Signal<DeliberationDiscussionCreateRequest>,
    pub final_survey: Signal<DeliberationFinalSurveyCreateRequest>,

    #[allow(dead_code)]
    pub parent: ParentController,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();

        let members = use_server_future(move || {
            let page = 1;
            let size = 20;
            async move {
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }
                let endpoint = crate::config::get().api_url;
                let res = OrganizationMember::get_client(endpoint)
                    .query(
                        org_id.unwrap().id,
                        OrganizationMemberQuery::new(size).with_page(page),
                    )
                    .await;

                res.unwrap_or_default().items
            }
        })?;

        let panels = use_server_future(move || {
            let page = 1;
            let size = 100;
            let org_id = user.get_selected_org();

            async move {
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }
                let endpoint = crate::config::get().api_url;
                let res = PanelV2::get_client(endpoint)
                    .query(org_id.unwrap().id, PanelV2Query::new(size).with_page(page))
                    .await;

                res.unwrap_or_default().items
            }
        })?;

        let mut ctrl = Self {
            lang,
            parent: use_context(),
            nav: use_navigator(),

            members,
            panels,
            committee_roles: use_signal(|| vec![]),
            committees: use_signal(|| vec![]),
            roles: use_signal(|| {
                vec![
                    Role::Admin,
                    Role::DeliberationAdmin,
                    Role::Analyst,
                    Role::Moderator,
                    Role::Speaker,
                ]
            }),

            selected_panels: use_signal(|| vec![]),

            basic_info: use_signal(|| DeliberationBasicInfoCreateRequest::default()),
            sample_survey: use_signal(|| DeliberationSampleSurveyCreateRequest::default()),
            deliberation: use_signal(|| DeliberationContentCreateRequest::default()),
            discussion: use_signal(|| DeliberationDiscussionCreateRequest::default()),
            final_survey: use_signal(|| DeliberationFinalSurveyCreateRequest::default()),
        };

        let req = ctrl.parent.deliberation_requests();

        // committee
        let roles = ctrl.roles();
        let members = members().unwrap_or_default();
        let committees = req.roles.clone();
        ctrl.committees.set(committees.clone());
        let mut committee_roles = vec![];
        for role in roles.clone() {
            let members = ctrl.get_role_list(members.clone(), committees.clone(), role);

            committee_roles.push(members);
        }
        ctrl.committee_roles.set(committee_roles);

        // panel
        let panels = panels().unwrap_or_default();
        let selected_panels: Vec<PanelV2Summary> = panels
            .iter()
            .filter(|panel| req.panel_ids.contains(&panel.id))
            .cloned()
            .collect();
        ctrl.selected_panels.set(selected_panels);

        // deliberation step
        ctrl.basic_info.set(
            req.basic_infos
                .get(0)
                .unwrap_or(&DeliberationBasicInfoCreateRequest::default())
                .clone(),
        );
        ctrl.sample_survey.set(
            req.sample_surveys
                .get(0)
                .unwrap_or(&DeliberationSampleSurveyCreateRequest::default())
                .clone(),
        );
        ctrl.deliberation.set(
            req.contents
                .get(0)
                .unwrap_or(&DeliberationContentCreateRequest::default())
                .clone(),
        );
        ctrl.discussion.set(
            req.deliberation_discussions
                .get(0)
                .unwrap_or(&DeliberationDiscussionCreateRequest::default())
                .clone(),
        );
        ctrl.final_survey.set(
            req.final_surveys
                .get(0)
                .unwrap_or(&DeliberationFinalSurveyCreateRequest::default())
                .clone(),
        );

        Ok(ctrl)
    }

    pub fn go_list(&mut self) {
        self.nav
            .replace(Route::DeliberationPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.temporary_save().await;
    }

    pub async fn start_deliberation(&mut self) {
        tracing::debug!("start button click");

        let cli = Deliberation::get_client(config::get().api_url);

        let org = self.parent.user.get_selected_org();
        if org.is_none() {
            btracing::e!(self.lang, ApiError::OrganizationNotFound);
            return;
        }

        let org_id = org.unwrap().id;
        let creator_id = if let Some(user_id) = self.parent.user.get_user_id() {
            user_id
        } else {
            btracing::e!(self.lang, ApiError::Unauthorized);
            return;
        };

        let DeliberationCreateRequest {
            started_at,
            ended_at,
            thumbnail_image,
            title,
            description,
            project_area,
            project_areas,
            resource_ids,
            survey_ids,
            roles,
            panel_ids,
            steps,
            elearning,
            basic_infos,
            sample_surveys,
            contents,
            deliberation_discussions,
            final_surveys,
            drafts,
            ..
        } = self.parent.deliberation_requests();

        match cli.get_draft(org_id, creator_id).await {
            Ok(d) => {
                match cli
                    .update(
                        org_id,
                        d.id,
                        DeliberationCreateRequest {
                            started_at,
                            ended_at,
                            thumbnail_image,
                            title,
                            description,
                            project_area,
                            project_areas,
                            status: DeliberationStatus::Ready,
                            creator_id,
                            resource_ids,
                            survey_ids,
                            roles,
                            panel_ids,
                            steps,
                            elearning,
                            basic_infos,
                            sample_surveys,
                            contents,
                            deliberation_discussions,
                            final_surveys,
                            drafts,
                        },
                    )
                    .await
                {
                    Ok(_) => {
                        btracing::i!(self.lang, Info::Save);
                        self.nav.push(Route::DeliberationPage { lang: self.lang });
                    }
                    Err(e) => {
                        btracing::e!(self.lang, e);
                    }
                }
            }
            Err(_) => {
                match cli
                    .create(
                        org_id,
                        started_at,
                        ended_at,
                        thumbnail_image,
                        title,
                        description,
                        project_area,
                        DeliberationStatus::Ready,
                        creator_id,
                        project_areas,
                        resource_ids,
                        survey_ids,
                        roles,
                        panel_ids,
                        steps,
                        elearning,
                        basic_infos,
                        sample_surveys,
                        contents,
                        deliberation_discussions,
                        final_surveys,
                        drafts,
                    )
                    .await
                {
                    Ok(_) => {
                        btracing::i!(self.lang, Info::Save);
                        self.nav.push(Route::DeliberationPage { lang: self.lang });
                    }
                    Err(e) => btracing::e!(self.lang, e),
                }
            }
        }
    }

    pub fn convert_user_ids_to_members(
        &mut self,
        user_ids: Vec<i64>,
    ) -> Vec<OrganizationMemberSummary> {
        let members = self.members().unwrap_or(vec![]);
        tracing::debug!("user ids: {:?} {:?}", user_ids, members);
        let members = members
            .into_iter()
            .filter(|member| user_ids.contains(&member.user_id))
            .collect();
        members
    }

    pub fn get_role_list(
        &mut self,
        members: Vec<OrganizationMemberSummary>,
        committees: Vec<DeliberationUserCreateRequest>,
        role: Role,
    ) -> Vec<OrganizationMemberSummary> {
        let user_ids: Vec<i64> = committees
            .iter()
            .filter(|committee| committee.role == role)
            .map(|committee| committee.user_id)
            .collect();

        let members = members
            .into_iter()
            .filter(|member| user_ids.contains(&member.user_id))
            .collect();

        members
    }
}
