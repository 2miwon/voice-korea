use bdk::prelude::*;
use models::{
    DeliberationDiscussionCreateRequest, DiscussionCreateRequest, File, ResourceFile,
    ResourceFileQuery, ResourceFileSummary,
};

use crate::{
    config,
    routes::Route,
    service::login_service::LoginService,
    utils::time::{
        current_midnight_timestamp, current_timestamp_with_time, parsed_timestamp_with_time,
    },
};

use super::DeliberationNewController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    discussion: Signal<DeliberationDiscussionCreateRequest>,

    #[allow(dead_code)]
    lang: Language,
    pub parent: DeliberationNewController,
    pub nav: Navigator,

    pub metadatas: Resource<Vec<ResourceFileSummary>>,
    pub committee_members: Signal<Vec<String>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let discussion = use_signal(|| DeliberationDiscussionCreateRequest::default());

        let metadatas = use_server_future(move || {
            let page = 1;
            let size = 300;
            async move {
                let client = ResourceFile::get_client(&config::get().api_url);
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }

                let query = ResourceFileQuery::new(size).with_page(page);
                client
                    .query(org_id.unwrap().id, query)
                    .await
                    .unwrap_or_default()
                    .items
            }
        })?;

        let mut ctrl = Self {
            lang,
            parent: use_context(),
            nav: use_navigator(),
            discussion,

            metadatas,
            committee_members: use_signal(|| vec![]),
        };

        use_effect({
            let req = ctrl.parent.deliberation_requests();
            let mut discussion = req
                .deliberation_discussions
                .get(0)
                .unwrap_or(&DeliberationDiscussionCreateRequest::default())
                .clone();

            move || {
                let committees = req.roles.iter().map(|v| v.email.clone()).collect();
                let started_at = discussion.clone().started_at;
                let ended_at = discussion.clone().ended_at;

                if started_at == 0 {
                    discussion.started_at = current_timestamp_with_time(0, 0, 0);
                }

                if ended_at == 0 {
                    discussion.ended_at = current_timestamp_with_time(23, 59, 59);
                }

                ctrl.discussion.set(discussion.clone());
                ctrl.committee_members.set(committees);
            }
        });
        Ok(ctrl)
    }

    pub fn get_selected_resources(&self) -> Vec<ResourceFile> {
        let metadatas = self.metadatas().unwrap_or_default();
        let resources = self.discussion().resources;

        metadatas
            .clone()
            .into_iter()
            .filter(|resource| resources.iter().any(|id| id.clone() == resource.id))
            .map(|v| v.into())
            .collect()
    }

    pub fn get_selected_committee(&self) -> Vec<String> {
        let discussion = self.discussion();
        let roles = discussion.clone().users;
        roles
    }

    pub fn set_title(&mut self, title: String) {
        self.discussion.with_mut(|req| {
            req.title = title;
        });
    }

    pub fn set_description(&mut self, description: String) {
        self.discussion.with_mut(|req| {
            req.description = description;
        });
    }

    pub fn set_start_date(&mut self, started_at: i64) {
        self.discussion.with_mut(|req| {
            req.started_at = parsed_timestamp_with_time(started_at, 0, 0, 0);
        });
    }

    pub fn set_end_date(&mut self, ended_at: i64) {
        self.discussion.with_mut(|req| {
            req.ended_at = parsed_timestamp_with_time(ended_at, 23, 59, 59);
        });
    }

    pub fn add_committee(&mut self, email: String) {
        self.discussion.with_mut(|req| {
            req.users.push(email);
        });
    }

    pub fn remove_committee(&mut self, email: String) {
        self.discussion.with_mut(|req| {
            req.users.retain(|e| !(e.clone() == email));
        })
    }

    pub fn clear_committee(&mut self) {
        self.discussion.with_mut(|req| req.users = vec![]);
    }

    pub fn add_discussion(&mut self) {
        let mut disc = DiscussionCreateRequest::default();
        disc.started_at = current_midnight_timestamp();
        disc.ended_at = current_midnight_timestamp();

        self.discussion.with_mut(|req| {
            req.discussions.push(disc);
        })
    }

    pub fn remove_discussion(&mut self, index: usize) {
        self.discussion.with_mut(|req| {
            req.discussions.remove(index);
        })
    }

    pub fn update_discussion(&mut self, index: usize, discussion: DiscussionCreateRequest) {
        self.discussion.with_mut(|req| {
            req.discussions[index] = discussion;
        })
    }

    pub fn set_discussion(&mut self, info: DeliberationDiscussionCreateRequest) {
        self.discussion.set(info);
    }

    pub fn remove_resource(&mut self, id: i64) {
        let mut discussion = self.discussion();
        discussion
            .resources
            .retain(|resource_id| !(resource_id.clone() == id));
        self.set_discussion(discussion);
    }

    pub fn clear_resource(&mut self) {
        let mut discussion = self.discussion();
        discussion.resources = vec![];
        self.set_discussion(discussion);
    }

    pub async fn create_metadata(&mut self, file: File) {
        let user: LoginService = use_context();
        let org = user.get_selected_org();
        if org.is_none() {
            btracing::error!("This service requires login.");
            return;
        }
        let org_id = org.unwrap().id;
        let client = models::ResourceFile::get_client(&config::get().api_url);

        let file = client
            .create(
                org_id,
                file.name.clone(),
                None,
                None,
                None,
                None,
                None,
                vec![file],
            )
            .await
            .unwrap_or_default();

        let mut discussion = self.discussion();
        discussion.resources.push(file.id);
        self.set_discussion(discussion);

        self.metadatas.restart();
    }

    pub fn back(&mut self) {
        self.parent.save_discussion(self.discussion());
        self.nav
            .replace(Route::DeliberationSettingPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.save_discussion(self.discussion());
        self.parent.temporary_save(false).await;
    }

    pub fn next(&mut self) {
        if self.validation_check() {
            self.parent.save_discussion(self.discussion());
            self.nav
                .push(Route::DeliberationVoteSettingPage { lang: self.lang });
        }
    }

    pub fn is_valid(&self) -> bool {
        let discussion = self.discussion();

        let title = discussion.title;
        let description = discussion.description;
        let started_at = discussion.started_at;
        let ended_at = discussion.ended_at;

        let members = discussion.users;
        let resources = discussion.resources;
        let discussions = discussion.discussions;

        !(title.is_empty()
            || description.is_empty()
            || started_at >= ended_at
            || members.is_empty()
            || resources.is_empty()
            || discussions.is_empty())
    }

    pub fn validation_check(&self) -> bool {
        let discussion = self.discussion();

        let title = discussion.title;
        let description = discussion.description;
        let started_at = discussion.started_at;
        let ended_at = discussion.ended_at;

        let members = discussion.users;
        let resources = discussion.resources;
        let discussions = discussion.discussions;

        if title.is_empty() {
            btracing::e!(self.lang, ValidationError::TitleRequired);
            return false;
        }
        if description.is_empty() {
            btracing::e!(self.lang, ValidationError::DescriptionRequired);
            return false;
        }
        if started_at >= ended_at {
            btracing::e!(self.lang, ValidationError::TimeValidationFailed);
            return false;
        }
        if members.is_empty() {
            btracing::e!(self.lang, ValidationError::MemberRequired);
            return false;
        }
        if resources.is_empty() {
            btracing::e!(self.lang, ValidationError::ScheduleRequired);
            return false;
        }
        if discussions.is_empty() {
            btracing::e!(self.lang, ValidationError::DiscussionRequired);
            return false;
        }

        true
    }
}

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum ValidationError {
    #[translate(
        ko = "토론 제목을 입력해주세요.",
        en = "Please enter the discussion title."
    )]
    TitleRequired,
    #[translate(
        ko = "토론 설명을 입력해주세요.",
        en = "Please enter the discussion description."
    )]
    DescriptionRequired,
    #[translate(
        ko = "시작 날짜는 종료 날짜보다 작아야합니다.",
        en = "The start date must be less than the end date."
    )]
    TimeValidationFailed,
    #[translate(
        ko = "1명 이상의 담당자를 선택해주세요.",
        en = "Please select one or more contact persons."
    )]
    MemberRequired,
    #[translate(
        ko = "일정표를 업로드해주세요.",
        en = "Please upload discussion schedule."
    )]
    ScheduleRequired,
    #[translate(
        ko = "토론 방을 생성해주세요.",
        en = "Please create a discussion room."
    )]
    DiscussionRequired,
}
