use bdk::prelude::*;
use models::{
    deliberation_user::DeliberationUserCreateRequest, DeliberationDiscussionCreateRequest,
    DiscussionCreateRequest, File, OrganizationMember, OrganizationMemberQuery,
    OrganizationMemberSummary, ResourceFile, ResourceFileQuery, ResourceFileSummary,
};

use crate::{
    config, routes::Route, service::login_service::LoginService, utils::time::current_timestamp,
};

use super::DeliberationNewController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    discussion: Signal<DeliberationDiscussionCreateRequest>,

    #[allow(dead_code)]
    lang: Language,
    pub parent: DeliberationNewController,
    pub nav: Navigator,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub metadatas: Resource<Vec<ResourceFileSummary>>,
    pub committee_members: Signal<Vec<DeliberationUserCreateRequest>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let discussion = use_signal(|| DeliberationDiscussionCreateRequest::default());
        let members = use_server_future(move || {
            let page = 1;
            let size = 100;
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

        let metadatas = use_server_future(move || {
            let page = 1;
            let size = 100;
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

            members,
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
            let current_timestamp = current_timestamp();
            let _committees = req.roles.clone();

            move || {
                let started_at = discussion.clone().started_at;
                let ended_at = discussion.clone().ended_at;

                if started_at == 0 {
                    discussion.started_at = current_timestamp;
                }

                if ended_at == 0 {
                    discussion.ended_at = current_timestamp;
                }

                ctrl.discussion.set(discussion.clone());
                ctrl.committee_members.set(vec![]);
            }
        });
        Ok(ctrl)
    }

    pub fn get_committees(&self) -> Vec<OrganizationMemberSummary> {
        let committees = self.committee_members();
        let members = self.members().unwrap_or_default();

        let d = members
            .clone()
            .into_iter()
            .filter(|member| {
                committees
                    .iter()
                    .any(|committee| committee.user_id == member.user_id)
            })
            .collect();

        d
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

    pub fn get_selected_committee(&self) -> Vec<OrganizationMemberSummary> {
        let total_committees = self.members().unwrap_or_default();
        let discussion = self.discussion();
        let roles = discussion.clone().users;
        total_committees
            .clone()
            .into_iter()
            .filter(|member| roles.iter().any(|id| id.clone() == member.user_id))
            .collect()
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
            req.started_at = started_at;
        });
    }

    pub fn set_end_date(&mut self, ended_at: i64) {
        self.discussion.with_mut(|req| {
            req.ended_at = ended_at;
        });
    }

    pub fn add_committee(&mut self, user_id: i64) {
        self.discussion.with_mut(|req| {
            req.users.push(user_id);
        });
    }

    pub fn remove_committee(&mut self, user_id: i64) {
        self.discussion.with_mut(|req| {
            req.users
                .retain(|committee_id| !(committee_id.clone() == user_id));
        })
    }

    pub fn clear_committee(&mut self) {
        self.discussion.with_mut(|req| req.users = vec![]);
    }

    pub fn add_discussion(&mut self) {
        let mut disc = DiscussionCreateRequest::default();
        disc.started_at = current_timestamp();
        disc.ended_at = current_timestamp();

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
        self.parent.save_discussion(self.discussion());
        self.nav
            .push(Route::DeliberationVoteSettingPage { lang: self.lang });
    }
}
