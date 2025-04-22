use bdk::prelude::*;
use by_macros::DioxusController;
use models::{deliberation_role::DeliberationRoleCreateRequest, DeliberationStatus, *};

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
            Route::DeliberationNewPage { .. } | Route::DeliberationEditPage { .. } => {
                Self::SettingInfo
            }
            Route::CompositionCommitee { .. } => Self::CompositionCommittee,
            Route::CompositionPanel { .. } => Self::CompositionPanel,
            Route::Preview { .. } => Self::Preview,

            _ => DeliberationNewStep::DeliberationSchedule,
        }
    }
}

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    #[allow(dead_code)]
    popup_service: Signal<PopupService>,
    current_path: Signal<String>,
    current_step: DeliberationNewStep,
    pub user: LoginService,
    pub nav: Navigator,

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

        let route: Route = use_route();
        let current_path = route.to_string();
        let ctrl = Self {
            lang,
            user,
            current_step,
            current_path: use_signal(|| current_path),
            nav: use_navigator(),
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

    pub fn project_areas(&self) -> Vec<ProjectArea> {
        self.deliberation_requests
            .with(|req| req.project_areas.clone())
    }

    pub async fn set_deliberation_id(&mut self, deliberation_id: Option<i64>) {
        self.deliberation_id.set(deliberation_id);
        let lang = self.lang;

        let org_id = if let Some(org) = self.user.get_selected_org() {
            org.id
        } else {
            btracing::e!(lang, ApiError::OrganizationNotFound);
            return;
        };

        let client = Deliberation::get_client(config::get().api_url);

        match client
            .get_draft(org_id, deliberation_id.unwrap_or_default())
            .await
        {
            Ok(d) => {
                self.deliberation_requests.set(d.into());
            }
            Err(e) => {
                tracing::error!("Failed to get draft: {:?}", e);
            }
        }
    }

    pub fn save_panels(&mut self, emails: Vec<String>) {
        self.deliberation_requests.with_mut(|req| {
            req.panel_emails = emails;
        });
    }

    pub fn save_committees(&mut self, roles: Vec<DeliberationRoleCreateRequest>) {
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

    pub async fn start_deliberation(&mut self) {
        let cli = Deliberation::get_client(config::get().api_url);

        let org = self.user.get_selected_org();
        if org.is_none() {
            btracing::e!(self.lang, ApiError::OrganizationNotFound);
            return;
        }
        let org_id = org.unwrap().id;

        let _ = self.temporary_save(true).await;

        let deliberation_id = self.deliberation_id();

        match cli
            .start_deliberation(org_id, deliberation_id.unwrap_or_default())
            .await
        {
            Ok(_) => {
                self.nav
                    .replace(Route::DeliberationPage { lang: self.lang });
            }
            Err(e) => {
                btracing::error!("start deliberation failed with error: {:?}", e);
            }
        }
    }

    pub async fn temporary_save(&mut self, is_start: bool) {
        let current_path = self.current_path();
        tracing::debug!("current path: {}", current_path);
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

        let id = self.deliberation_id();

        let DeliberationCreateRequest {
            thumbnail_image,
            title,
            description,
            project_area,
            project_areas,
            resource_ids,
            survey_ids,
            roles,
            panel_ids,
            panel_emails,
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

        let default_basic_info = &DeliberationBasicInfoCreateRequest::default();
        let default_sample_survey = &DeliberationSampleSurveyCreateRequest::default();
        let default_content = &DeliberationContentCreateRequest::default();
        let default_discussion = &DeliberationDiscussionCreateRequest::default();
        let default_final_survey = &DeliberationFinalSurveyCreateRequest::default();

        let basic_info = basic_infos.get(0).unwrap_or(default_basic_info);
        let sample_survey = sample_surveys.get(0).unwrap_or(default_sample_survey);
        let content = contents.get(0).unwrap_or(default_content);
        let discussion = deliberation_discussions
            .get(0)
            .unwrap_or(default_discussion);
        let final_survey = final_surveys.get(0).unwrap_or(default_final_survey);

        let start_dates = vec![
            basic_info.started_at,
            sample_survey.started_at,
            content.started_at,
            discussion.started_at,
            final_survey.started_at,
        ];

        let end_dates = vec![
            basic_info.ended_at,
            sample_survey.ended_at,
            content.ended_at,
            discussion.ended_at,
            final_survey.ended_at,
        ];

        let mut started_at = start_dates[0];
        let mut ended_at = end_dates[0];

        for s in start_dates {
            if started_at > s {
                started_at = s;
            }
        }

        for e in end_dates {
            if ended_at < e {
                ended_at = e;
            }
        }

        if id.is_none() {
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
                    panel_emails,
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

                    if !is_start && current_path.ends_with("/deliberations/new") {
                        self.nav.replace(Route::DeliberationEditPage {
                            lang: self.lang,
                            deliberation_id: d.id,
                        });
                    }
                }
                Err(e) => btracing::e!(self.lang, e),
            }
        } else {
            match cli
                .update(
                    org_id,
                    id.unwrap_or_default(),
                    DeliberationCreateRequest {
                        started_at,
                        ended_at,
                        thumbnail_image,
                        title,
                        description,
                        project_area,
                        project_areas,
                        status: DeliberationStatus::Draft,
                        creator_id,
                        resource_ids,
                        survey_ids,
                        roles,
                        panel_emails,
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
                    btracing::i!(self.lang, Info::TempSave);
                }
                Err(e) => {
                    btracing::e!(self.lang, e);
                }
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

#[allow(unused_variables)]
impl OverviewController {
    pub fn new(
        lang: Language,
        deliberation_id: Option<i64>,
    ) -> std::result::Result<Self, RenderError> {
        // let mut parent: Controller = use_context();
        let mut parent: Controller = use_context();

        use_future(move || async move {
            parent.set_deliberation_id(deliberation_id).await;
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

    pub async fn temp_save(&mut self) {
        self.parent.save_overview(
            self.title(),
            self.description(),
            self.thumbnail_image(),
            self.project_areas(),
        );

        self.parent.temporary_save(false).await;
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
