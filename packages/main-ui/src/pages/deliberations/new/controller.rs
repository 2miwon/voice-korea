use bdk::prelude::*;
use by_macros::DioxusController;
use models::{deliberation_user::DeliberationUserCreateRequest, DeliberationStatus, *};

use crate::{
    config,
    routes::Route,
    service::{login_service::LoginService, popup_service::PopupService},
};

#[derive(
    Debug, Clone, PartialEq, Copy, serde::Serialize, serde::Deserialize, EnumProp, Default,
)]
#[serde(rename_all = "kebab-case")]
pub enum DeliberationNewStep {
    #[default]
    SettingInfo, // Setting Deliberation Overview
    CompositionCommittee, // Composition Deliberation Committee
    CompositionPanel,     // Composition Participant Panel
    DeliberationSchedule, // Deliberation Procedure and Schedule
    Preview,              // Final Recommendation

    EditContent,
}

impl From<Route> for DeliberationNewStep {
    fn from(route: Route) -> Self {
        match route {
            Route::DeliberationNewPage { .. } => Self::SettingInfo,
            Route::CompositionCommitee { .. } => Self::CompositionCommittee,
            Route::CompositionPanel { .. } => Self::CompositionPanel,
            Route::Preview { .. } => Self::Preview,

            _ => DeliberationNewStep::DeliberationSchedule,
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    popup_service: Signal<PopupService>,
    current_step: DeliberationNewStep,
    user: LoginService,

    deliberation_requests: Signal<DeliberationCreateRequest>,
    deliberation_id: Signal<Option<i64>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let popup_service: PopupService = use_context();
        let route = use_route::<Route>();
        let current_step = route.clone().into();
        let deliberation_requests = use_signal(|| DeliberationCreateRequest::default());

        let ctrl = Self {
            lang,
            user,
            current_step,
            popup_service: use_signal(|| popup_service),
            deliberation_requests,
            deliberation_id: use_signal(|| None),
        };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn get_current_step(&self) -> DeliberationNewStep {
        self.current_step
    }

    // pub fn use_service() -> Self {
    //     use_context()
    // }

    // pub async fn create_metadata(&self, file: File) -> Result<ResourceFile> {
    //     let org = self.user.get_selected_org();
    //     if org.is_none() {
    //         return Err(models::ApiError::OrganizationNotFound);
    //     }
    //     let org_id = org.unwrap().id;
    //     let client = models::ResourceFile::get_client(&config::get().api_url);

    //     client
    //         .create(
    //             org_id,
    //             file.name.clone(),
    //             None,
    //             None,
    //             None,
    //             None,
    //             None,
    //             vec![file],
    //         )
    //         .await
    // }

    // pub async fn create_deliberation(&self, lang: Language) -> Result<()> {
    //     let navigator = use_navigator();

    //     let endpoint = crate::config::get().api_url;
    //     let client = Deliberation::get_client(endpoint);

    //     let org_id = self.user.get_selected_org();
    //     if org_id.is_none() {
    //         tracing::error!("Organization ID is missing");
    //         return Err(ApiError::OrganizationNotFound);
    //     }

    //     let req = self.deliberation_requests();

    //     match client
    //         .create(
    //             org_id.unwrap().id,
    //             req.started_at,
    //             req.ended_at,
    //             req.thumbnail_image,
    //             req.title.clone(),
    //             req.description.clone(),
    //             req.project_area,
    //             req.project_areas,
    //             req.resource_ids,
    //             req.survey_ids,
    //             req.roles,
    //             req.panel_ids,
    //             req.steps,
    //             req.elearning,
    //             req.basic_infos,
    //             req.sample_surveys,
    //             req.contents,
    //             req.deliberation_discussions,
    //             req.final_surveys,
    //             req.drafts,
    //         )
    //         .await
    //     {
    //         Ok(_) => {
    //             btracing::debug!("success to create deliberation");
    //             navigator.push(Route::DeliberationPage { lang });
    //             Ok(())
    //         }
    //         Err(e) => {
    //             btracing::error!("failed to create deliberation: {}", e.translate(&lang));
    //             return Err(e);
    //         }
    //     }
    // }

    pub fn project_areas(&self) -> Vec<ProjectArea> {
        self.deliberation_requests
            .with(|req| req.project_areas.clone())
    }

    // pub fn get_deliberation_time(&self, steps: Vec<StepCreateRequest>) -> (i64, i64) {
    //     let started_at = steps.iter().map(|s| s.started_at).min().unwrap_or(0);
    //     let ended_at = steps.iter().map(|s| s.ended_at).max().unwrap_or(0);

    //     (started_at, ended_at)
    // }

    pub fn save_panels(&mut self, panel_ids: Vec<i64>) {
        self.deliberation_requests.with_mut(|req| {
            req.panel_ids = panel_ids;
        });
    }

    pub fn save_committees(&mut self, roles: Vec<DeliberationUserCreateRequest>) {
        self.deliberation_requests.with_mut(|req| {
            req.roles = roles;
        });
    }

    pub fn save_basic_info(&mut self, basic_info: DeliberationBasicInfoCreateRequest) {
        self.deliberation_requests.with_mut(|req| {
            req.basic_infos = vec![basic_info];
        });
    }

    pub fn save_sample_survey(&mut self, sample_surveys: DeliberationSampleSurveyCreateRequest) {
        self.deliberation_requests.with_mut(|req| {
            req.sample_surveys = vec![sample_surveys];
        });
    }

    pub fn save_content(&mut self, content: DeliberationContentCreateRequest) {
        self.deliberation_requests.with_mut(|req| {
            req.contents = vec![content];
        });
    }

    pub fn save_discussion(&mut self, discussion: DeliberationDiscussionCreateRequest) {
        self.deliberation_requests.with_mut(|req| {
            req.deliberation_discussions = vec![discussion];
        });
    }

    pub fn save_final_survey(&mut self, final_survey: DeliberationFinalSurveyCreateRequest) {
        self.deliberation_requests.with_mut(|req| {
            req.final_surveys = vec![final_survey];
        });
    }

    pub fn save_overview(
        &mut self,
        title: String,
        description: String,
        thumbnail_image: String,
        project_areas: Vec<ProjectArea>,
    ) {
        self.deliberation_requests.with_mut(|req| {
            req.title = title;
            req.description = description;
            req.thumbnail_image = thumbnail_image;
            req.project_areas = project_areas;
        });
    }

    pub fn save_title(&mut self, title: String) {
        self.deliberation_requests.with_mut(|req| {
            req.title = title;
        });
    }

    pub fn save_description(&mut self, description: String) {
        self.deliberation_requests.with_mut(|req| {
            req.description = description;
        });
    }

    #[cfg(feature = "web")]
    pub fn save_thumbnail_image(&mut self, thumbnail_image: String) {
        self.deliberation_requests.with_mut(|req| {
            req.thumbnail_image = thumbnail_image;
        });
    }

    pub fn save_project_areas(&mut self, project_areas: Vec<ProjectArea>) {
        self.deliberation_requests.with_mut(|req| {
            req.project_areas = project_areas;
        });
    }

    pub async fn temporary_save(&mut self) {
        let cli = Deliberation::get_client(config::get().api_url);

        let org = self.user.get_selected_org();
        if org.is_none() {
            btracing::e!(self.lang, ApiError::OrganizationNotFound);
            return;
        }

        let org_id = org.unwrap().id;
        let creator_id = if let Some(user_id) = self.user.get_user_id() {
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
        } = self.deliberation_requests();

        match cli.get_draft(org_id, creator_id).await {
            Ok(_d) => {
                // let _update_request = DeliberationRepositoryUpdateRequest {
                //     org_id: Some(org_id),
                //     started_at: Some(started_at),
                //     ended_at: Some(ended_at),
                //     thumbnail_image: Some(thumbnail_image),
                //     title: Some(title),
                //     description: Some(description),
                //     project_area: Some(project_area),
                //     status: Some(DeliberationStatus::Draft),
                //     creator_id: Some(creator_id),
                // };

                // TODO: update the following fields
                // match cli.update(d.id, update_request).await {
                //     Ok(_) => {
                //         btracing::i!(self.lang, Info::TempSave);
                //         self.deliberation_id.set(Some(d.id));
                //     }
                //     Err(e) => {
                //         btracing::e!(self.lang, e);
                //     }
                // }

                // TODO: update sub structures
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
                        DeliberationStatus::Draft,
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
                    Ok(d) => {
                        btracing::i!(self.lang, Info::TempSave);
                        self.deliberation_id.set(Some(d.id));
                    }
                    Err(e) => btracing::e!(self.lang, e),
                }

                // TODO: update deliberation_areas in DB
            }
        }
    }
}

#[derive(Clone, Copy, DioxusController)]
pub struct OverviewController {
    lang: Language,

    pub parent: Controller,
    pub nav: Navigator,

    pub title: Memo<String>,
    pub description: Memo<String>,
    pub thumbnail_image: Memo<String>,
    pub project_areas: Memo<Vec<ProjectArea>>,
}

impl OverviewController {
    pub fn new(
        lang: Language,
        deliberation_id: Option<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let mut parent: Controller = use_context();
        use_future(move || async move {
            tracing::debug!("Deliberation ID: {:?}", deliberation_id);
            if let Some(deliberation_id) = deliberation_id {
                let client = Deliberation::get_client(config::get().api_url);
                let org_id = parent.user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return;
                }
                let org_id = org_id.unwrap().id;
                let deliberation = client
                    .get(org_id, deliberation_id)
                    .await
                    .unwrap_or_default();
                parent.deliberation_requests.set(deliberation.into());
                parent.deliberation_id.set(Some(deliberation_id));
            }
        });

        let title = use_memo(move || parent.deliberation_requests().title);
        let description = use_memo(move || parent.deliberation_requests().description);
        let thumbnail_image = use_memo(move || parent.deliberation_requests().thumbnail_image);

        let ctrl = Self {
            lang,
            parent: use_context(),
            nav: use_navigator(),
            title,
            description,
            thumbnail_image,
            project_areas: use_memo(move || parent.deliberation_requests().project_areas),
        };

        Ok(ctrl)
    }

    pub fn save_project_area(&mut self, selected_items: Vec<String>) {
        let project_areas: Vec<ProjectArea> = selected_items
            .iter()
            .map(|s| s.parse().unwrap_or_default())
            .collect();
        self.parent.save_project_areas(project_areas);
    }

    // pub fn back(&mut self) {
    //     self.nav
    //         .replace(Route::DeliberationPage { lang: self.lang });
    // }

    pub async fn temp_save(&mut self) {
        self.parent.save_overview(
            self.title(),
            self.description(),
            self.thumbnail_image(),
            self.project_areas(),
        );

        self.parent.temporary_save().await;
        tracing::debug!("{:?}", self.parent.deliberation_requests());
    }

    pub fn next(&mut self) {
        if self.validation_check() {
            tracing::debug!("Submit");
            self.parent.save_overview(
                self.title(),
                self.description(),
                self.thumbnail_image(),
                self.project_areas(),
            );

            self.nav
                .push(Route::CompositionCommitee { lang: self.lang });
        }
    }

    pub fn get_file_name(&self) -> String {
        let url = self.thumbnail_image();
        if url.is_empty() {
            return String::new();
        }
        url.split('/').last().unwrap_or_default().to_string()
    }

    pub fn is_valid(&self) -> bool {
        !(self.title().is_empty()
            || self.description().is_empty()
            || self.thumbnail_image().is_empty()
            || self.parent.project_areas().is_empty())
    }

    pub fn validation_check(&self) -> bool {
        if self.title().is_empty() {
            btracing::e!(self.lang, ValidationError::TitleRequired);
            return false;
        }
        if self.description().is_empty() {
            btracing::e!(self.lang, ValidationError::DescriptionRequired);
            return false;
        }
        if self.thumbnail_image().is_empty() {
            btracing::e!(self.lang, ValidationError::ThumbnailImageRequired);
            return false;
        }
        if self.parent.project_areas().is_empty() {
            btracing::e!(self.lang, ValidationError::ProjectAreaRequired);
            return false;
        }
        true
    }
}

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum ValidationError {
    #[translate(
        ko = "프로젝트 제목을 입력해주세요.",
        en = "Please enter the project title."
    )]
    TitleRequired,
    #[translate(
        ko = "프로젝트 설명을 입력해주세요.",
        en = "Please enter the project description."
    )]
    DescriptionRequired,
    #[translate(
        ko = "프로젝트 썸네일 이미지를 선택해주세요.",
        en = "Please select a project thumbnail image."
    )]
    ThumbnailImageRequired,
    #[translate(
        ko = "프로젝트 분야를 선택해주세요.",
        en = "Please select a project area."
    )]
    ProjectAreaRequired,
}
