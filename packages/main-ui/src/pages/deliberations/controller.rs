use bdk::prelude::*;
use dioxus_logger::tracing;
use models::{
    deliberation::{Deliberation, DeliberationQuery, DeliberationSummary},
    prelude::PanelInfo,
    ApiError, DeliberationBasicInfo, DeliberationContent, DeliberationDiscussion,
    DeliberationFinalSurvey, DeliberationSampleSurvey, QueryResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    config,
    pages::deliberations::components::remove_deliberation::RemoveDeliberationModal,
    routes::Route,
    service::{login_service::LoginService, popup_service::PopupService},
};

use super::components::remove_deliberation::RemoveDeliberationModalTranslate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Opinion {
    pub project_id: String,
    pub opinion_type: String,
    pub project_name: String,
    pub total_response_count: u64,
    pub response_count: u64,
    pub panels: Vec<PanelInfo>,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
}

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub deliberations: Resource<QueryResponse<DeliberationSummary>>,
    pub popup_service: PopupService,
    page: Signal<usize>,
    pub size: usize,
    pub search_keyword: Signal<String>,
    pub selected_id: Signal<i64>,
    pub org_id: Signal<i64>,
    pub context_menu: Signal<bool>,
    pub mouse_pos: Signal<(f64, f64)>,
    pub nav: Navigator,
    pub user: LoginService,
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let page = use_signal(|| 1);
        let size = 10;
        let search_keyword = use_signal(|| "".to_string());

        let deliberations = use_server_future(move || {
            let page = page();
            let keyword = search_keyword().clone();

            async move {
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return QueryResponse {
                        items: vec![],
                        total_count: 0,
                    };
                }
                let client = Deliberation::get_client(&crate::config::get().api_url);

                let query = DeliberationQuery::new(size).with_page(page);

                if keyword.is_empty() {
                    match client.query(org_id.unwrap().id, query).await {
                        Ok(res) => res,
                        Err(e) => {
                            tracing::error!("Failed to list deliberations: {:?}", e);
                            return QueryResponse {
                                items: vec![],
                                total_count: 0,
                            };
                        }
                    }
                } else {
                    match client
                        .search_by(size, Some(page.to_string()), org_id.unwrap().id, keyword)
                        .await
                    {
                        Ok(res) => res,
                        Err(e) => {
                            tracing::error!("Failed to list deliberations: {:?}", e);
                            return QueryResponse {
                                items: vec![],
                                total_count: 0,
                            };
                        }
                    }
                }
            }
        })?;

        let ctrl = Self {
            lang,
            popup_service: use_context(),
            deliberations,
            page,
            size,
            search_keyword,
            org_id: use_signal(|| user.get_selected_org().unwrap_or_default().id),
            selected_id: use_signal(|| 0),
            context_menu: use_signal(|| false),
            mouse_pos: use_signal(|| (0.0, 0.0)),
            nav: use_navigator(),
            user: use_context(),
        };

        Ok(ctrl)
    }

    pub fn set_page(&mut self, page: usize) {
        self.page.set(page);
    }

    pub fn get_x(&self) -> f64 {
        self.mouse_pos.with(|v| v.0 - 239.0)
    }

    pub fn get_y(&self) -> f64 {
        self.mouse_pos.with(|v| v.1 + 20.0)
    }

    pub fn move_user_page(&self, project_id: i64) {
        use web_sys::window;

        let url = crate::config::get().user_url;

        let user_domain = format!("{}/{}/projects/{}", url, self.lang, project_id);

        if let Some(w) = window() {
            let _ = w.open_with_url(&user_domain);
        }
    }

    pub fn total_pages(&self) -> usize {
        let size = self.size;
        self.deliberations.with(|v| {
            if let Some(v) = v {
                if v.total_count != 0 {
                    (v.total_count as usize - 1) / size + 1
                } else {
                    0
                }
            } else {
                0
            }
        }) as usize
    }

    pub fn handle_click_menu(&mut self, id: i64, e: MouseEvent) {
        e.prevent_default();
        e.stop_propagation();

        let should_open = !self.context_menu() || self.selected_id() != id;

        self.context_menu.set(should_open);
        self.selected_id.set(id);
        let rect = e.page_coordinates();
        self.mouse_pos.set((rect.x, rect.y));
        tracing::debug!("opened: {} Mouse position: {:?}", should_open, rect);
    }

    pub async fn handle_remove(&mut self) {
        let mut popup_service = self.popup_service;
        let lang = self.lang;
        let org_id = self.org_id();
        let id = self.selected_id();

        let tr: RemoveDeliberationModalTranslate = translate(&lang);

        let mut ctrl = self.clone();

        popup_service
            .open(rsx! {
                RemoveDeliberationModal {
                    lang: self.lang,
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                    onremove: move |_e: MouseEvent| async move {
                        let client = Deliberation::get_client(&crate::config::get().api_url);
                        match client.remove_deliberation(org_id, id).await {
                            Ok(_) => {
                                ctrl.deliberations.restart();
                                ctrl.context_menu.set(false);
                                popup_service.close();
                            }
                            Err(e) => {
                                btracing::error!("failed to remove deliberation with error: {:?}", e);
                                ctrl.context_menu.set(false);
                                popup_service.close();
                            }
                        }
                    },
                }
            })
            .with_id("remove_deliberation")
            .with_title(tr.title);
    }

    pub fn handle_edit(&mut self) {
        self.context_menu.set(false);

        self.nav.push(Route::DeliberationEditPage {
            lang: self.lang,
            deliberation_id: self.selected_id(),
        });
    }

    pub async fn start_deliberation(&mut self, deliberation_id: i64) {
        let cli = Deliberation::get_client(config::get().api_url);

        let org = self.user.get_selected_org();
        if org.is_none() {
            btracing::e!(self.lang, ApiError::OrganizationNotFound);
            return;
        }
        let org_id = org.unwrap().id;

        let deliberation = cli
            .get_draft(org_id, deliberation_id)
            .await
            .unwrap_or_default();

        if self.validation_check(deliberation) {
            match cli.start_deliberation(org_id, deliberation_id).await {
                Ok(_) => {
                    self.deliberations.restart();
                }
                Err(e) => {
                    btracing::error!("start deliberation failed with error: {:?}", e);
                }
            }
        }
    }

    pub fn validation_check(&self, deliberation: Deliberation) -> bool {
        let title = deliberation.title;
        let description = deliberation.description;
        let thumbnail = deliberation.thumbnail_image;
        let areas = deliberation.project_areas;

        let committees = deliberation.roles;

        let panels = deliberation.emails;

        let basic_info = deliberation
            .basic_infos
            .get(0)
            .unwrap_or(&DeliberationBasicInfo::default())
            .clone();

        let sample_survey = deliberation
            .sample_surveys
            .get(0)
            .unwrap_or(&DeliberationSampleSurvey::default())
            .clone();

        let content = deliberation
            .contents
            .get(0)
            .unwrap_or(&DeliberationContent::default())
            .clone();

        let discussion = deliberation
            .deliberation_discussions
            .get(0)
            .unwrap_or(&DeliberationDiscussion::default())
            .clone();

        let final_survey = deliberation
            .final_surveys
            .get(0)
            .unwrap_or(&DeliberationFinalSurvey::default())
            .clone();

        if title.is_empty() {
            btracing::e!(self.lang, ValidationError::TitleRequired);
            return false;
        }
        if description.is_empty() {
            btracing::e!(self.lang, ValidationError::DescriptionRequired);
            return false;
        }
        if thumbnail.is_empty() {
            btracing::e!(self.lang, ValidationError::ThumbnailImageRequired);
            return false;
        }
        if areas.is_empty() {
            btracing::e!(self.lang, ValidationError::ProjectAreaRequired);
            return false;
        }

        if committees.is_empty() {
            btracing::e!(self.lang, ValidationError::CommitteeRequired);
            return false;
        }

        if panels.is_empty() {
            btracing::e!(self.lang, ValidationError::PanelRequired);
            return false;
        }

        if basic_info.title.is_empty() {
            btracing::e!(self.lang, ValidationError::BasicInfoTitleRequired);
            return false;
        }
        if basic_info.description.is_empty() {
            btracing::e!(self.lang, ValidationError::BasicInfoDescriptionRequired);
            return false;
        }
        if basic_info.started_at >= basic_info.ended_at {
            btracing::e!(self.lang, ValidationError::BasicInfoTimeValidationFailed);
            return false;
        }
        if basic_info.roles.is_empty() {
            btracing::e!(self.lang, ValidationError::BasicInfoMemberRequired);
            return false;
        }

        if sample_survey.title.is_empty() {
            btracing::e!(self.lang, ValidationError::SampleSurveyTitleRequired);
            return false;
        }
        if sample_survey.description.is_empty() {
            btracing::e!(self.lang, ValidationError::SampleSurveyDescriptionRequired);
            return false;
        }
        if sample_survey.started_at >= sample_survey.ended_at {
            btracing::e!(self.lang, ValidationError::SampleSurveyTimeValidationFailed);
            return false;
        }
        if sample_survey.roles.is_empty() {
            btracing::e!(self.lang, ValidationError::SampleSurveyMemberRequired);
            return false;
        }
        if sample_survey.surveys.is_empty() {
            btracing::e!(self.lang, ValidationError::SampleSurveySurveyRequired);
            return false;
        }

        if content.title.is_empty() {
            btracing::e!(self.lang, ValidationError::DeliberationTitleRequired);
            return false;
        }
        if content.description.is_empty() {
            btracing::e!(self.lang, ValidationError::DeliberationDescriptionRequired);
            return false;
        }
        if content.started_at >= content.ended_at {
            btracing::e!(self.lang, ValidationError::DeliberationTimeValidationFailed);
            return false;
        }
        if content.roles.is_empty() {
            btracing::e!(self.lang, ValidationError::DeliberationMemberRequired);
            return false;
        }
        if content.elearnings.is_empty() {
            btracing::e!(self.lang, ValidationError::DeliberationElearningRequired);
            return false;
        }
        if content.questions.is_empty() {
            btracing::e!(self.lang, ValidationError::DeliberationQuestionRequired);
            return false;
        }

        if discussion.title.is_empty() {
            btracing::e!(self.lang, ValidationError::TitleRequired);
            return false;
        }
        if discussion.description.is_empty() {
            btracing::e!(self.lang, ValidationError::DescriptionRequired);
            return false;
        }
        if discussion.started_at >= discussion.ended_at {
            btracing::e!(self.lang, ValidationError::DiscussionTimeValidationFailed);
            return false;
        }
        if discussion.roles.is_empty() {
            btracing::e!(self.lang, ValidationError::DiscussionMemberRequired);
            return false;
        }
        if discussion.resources.is_empty() {
            btracing::e!(self.lang, ValidationError::DiscussionScheduleRequired);
            return false;
        }
        if discussion.discussions.is_empty() {
            btracing::e!(self.lang, ValidationError::DiscussionRequired);
            return false;
        }

        if final_survey.title.is_empty() {
            btracing::e!(self.lang, ValidationError::FinalSurveyTitleRequired);
            return false;
        }
        if final_survey.description.is_empty() {
            btracing::e!(self.lang, ValidationError::FinalSurveyDescriptionRequired);
            return false;
        }
        if final_survey.started_at >= final_survey.ended_at {
            btracing::e!(self.lang, ValidationError::FinalSurveyTimeValidationFailed);
            return false;
        }
        if final_survey.roles.is_empty() {
            btracing::e!(self.lang, ValidationError::FinalSurveyMemberRequired);
            return false;
        }
        if final_survey.surveys.is_empty() {
            btracing::e!(self.lang, ValidationError::FinalSurveySurveyRequired);
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

    #[translate(
        ko = "공론에 참여할 참여자를 1명 이상 입력해주세요.",
        en = "Please enter at least one participant who will participate in the public discussion."
    )]
    CommitteeRequired,

    #[translate(
        ko = "공론에 참여할 패널을 1개 이상 선택해주세요.",
        en = "Please select at least one panelist to participate in the discussion."
    )]
    PanelRequired,

    #[translate(
        ko = "기본 정보의 제목을 입력해주세요.",
        en = "Please enter the title in basic info section."
    )]
    BasicInfoTitleRequired,
    #[translate(
        ko = "기본 정보의 설명을 입력해주세요.",
        en = "Please enter the description in basic info section."
    )]
    BasicInfoDescriptionRequired,
    #[translate(
        ko = "기본 정보 내 시작 날짜는 종료 날짜보다 작아야합니다.",
        en = "The start date must be less than the end date in basic info section."
    )]
    BasicInfoTimeValidationFailed,
    #[translate(
        ko = "기본 정보 내 1명 이상의 담당자를 선택해주세요.",
        en = "Please select one or more contact persons in basic info section."
    )]
    BasicInfoMemberRequired,

    #[translate(
        ko = "표본 조사의 제목을 입력해주세요.",
        en = "Please enter the title in sample survey section."
    )]
    SampleSurveyTitleRequired,
    #[translate(
        ko = "표본 조사 설명을 입력해주세요.",
        en = "Please enter the description in sample survey section."
    )]
    SampleSurveyDescriptionRequired,
    #[translate(
        ko = "표본 조사 내 시작 날짜는 종료 날짜보다 작아야합니다.",
        en = "The start date must be less than the end date in sample survey section."
    )]
    SampleSurveyTimeValidationFailed,
    #[translate(
        ko = "표본 조사 내 1명 이상의 담당자를 선택해주세요.",
        en = "Please select one or more contact persons in sample survey section."
    )]
    SampleSurveyMemberRequired,
    #[translate(
        ko = "표본 조사 내 한 문항 이상의 설문을 입력해주세요.",
        en = "Please enter one or more questions in sample survey section."
    )]
    SampleSurveySurveyRequired,

    #[translate(
        ko = "숙의 제목을 입력해주세요.",
        en = "Please enter the title in deliberation section."
    )]
    DeliberationTitleRequired,
    #[translate(
        ko = "숙의 설명을 입력해주세요.",
        en = "Please enter the description in deliberation section."
    )]
    DeliberationDescriptionRequired,
    #[translate(
        ko = "숙의 내 시작 날짜는 종료 날짜보다 작아야합니다.",
        en = "The start date must be less than the end date in deliberation section."
    )]
    DeliberationTimeValidationFailed,
    #[translate(
        ko = "숙의 내 1명 이상의 담당자를 선택해주세요.",
        en = "Please select one or more contact persons in deliberation section."
    )]
    DeliberationMemberRequired,
    #[translate(
        ko = "숙의 내 하나 이상의 이러닝 자료를 생성해주세요.",
        en = "Please create one or more eLearning materials in deliberation section."
    )]
    DeliberationElearningRequired,
    #[translate(
        ko = "숙의 내 한 문항 이상의 설문을 입력해주세요.",
        en = "Please enter one or more questions in deliberation section."
    )]
    DeliberationQuestionRequired,

    #[translate(
        ko = "토론 제목을 입력해주세요.",
        en = "Please enter the title in discussion section."
    )]
    DiscussionTitleRequired,
    #[translate(
        ko = "토론 설명을 입력해주세요.",
        en = "Please enter the description in discussion section."
    )]
    DiscussionDescriptionRequired,
    #[translate(
        ko = "토론 내 시작 날짜는 종료 날짜보다 작아야합니다.",
        en = "The start date must be less than the end date in discussion section."
    )]
    DiscussionTimeValidationFailed,
    #[translate(
        ko = "토론 내 1명 이상의 담당자를 선택해주세요.",
        en = "Please select one or more contact persons in discussion section."
    )]
    DiscussionMemberRequired,
    #[translate(
        ko = "토론 내 일정표를 업로드해주세요.",
        en = "Please upload discussion schedule in discussion section."
    )]
    DiscussionScheduleRequired,
    #[translate(
        ko = "토론 방을 생성해주세요.",
        en = "Please create a discussion room."
    )]
    DiscussionRequired,

    #[translate(
        ko = "최종 설문의 제목을 입력해주세요.",
        en = "Please enter the title in final survey section."
    )]
    FinalSurveyTitleRequired,
    #[translate(
        ko = "최종 설문의 설명을 입력해주세요.",
        en = "Please enter the description in final survey section."
    )]
    FinalSurveyDescriptionRequired,
    #[translate(
        ko = "최종 설문 내 시작 날짜는 종료 날짜보다 작아야합니다.",
        en = "The start date must be less than the end date in final survey section."
    )]
    FinalSurveyTimeValidationFailed,
    #[translate(
        ko = "최종 설문 내 1명 이상의 담당자를 선택해주세요.",
        en = "Please select one or more contact persons in final survey section."
    )]
    FinalSurveyMemberRequired,
    #[translate(
        ko = "최종 내 한 문항 이상의 설문을 입력해주세요.",
        en = "Please enter one or more questions in final survey section."
    )]
    FinalSurveySurveyRequired,
}
