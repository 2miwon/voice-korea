use bdk::prelude::*;
use models::{
    deliberation_user::DeliberationUserCreateRequest, elearning::ElearningCreateRequest,
    DeliberationContentCreateRequest, File, OrganizationMember, OrganizationMemberQuery,
    OrganizationMemberSummary,
};

use crate::{
    config, routes::Route, service::login_service::LoginService, utils::time::current_timestamp,
};

use super::DeliberationNewController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,

    // pub _parent: super::super::Controller,
    pub e_learning_tab: Signal<bool>,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub committee_members: Signal<Vec<DeliberationUserCreateRequest>>,

    deliberation: Signal<DeliberationContentCreateRequest>,
    pub parent: DeliberationNewController,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let deliberation = use_signal(|| DeliberationContentCreateRequest::default());

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

        let mut ctrl = Self {
            lang,
            e_learning_tab: use_signal(|| true),

            members,
            committee_members: use_signal(|| vec![]),

            parent: use_context(),
            nav: use_navigator(),
            deliberation,
        };

        let req = ctrl.parent.deliberation_requests();

        let current_timestamp = current_timestamp();

        let committees = req.roles;

        use_effect(move || {
            let mut deliberation = req
                .contents
                .get(0)
                .unwrap_or(&DeliberationContentCreateRequest::default())
                .clone();
            let started_at = deliberation.started_at;
            let ended_at = deliberation.ended_at;
            if started_at == 0 {
                deliberation.started_at = current_timestamp;
            }
            if ended_at == 0 {
                deliberation.ended_at = current_timestamp;
            }
            ctrl.deliberation.set(deliberation);
            ctrl.committee_members.set(committees.clone());
        });

        Ok(ctrl)
    }

    pub fn set_title(&mut self, title: String) {
        self.deliberation.with_mut(|req| {
            req.title = title;
        });
    }

    pub fn set_description(&mut self, description: String) {
        self.deliberation.with_mut(|req| {
            req.description = description;
        });
    }

    pub fn set_start_date(&mut self, started_at: i64) {
        self.deliberation.with_mut(|req| {
            req.started_at = started_at;
        });
    }

    pub fn set_end_date(&mut self, ended_at: i64) {
        self.deliberation.with_mut(|req| {
            req.ended_at = ended_at;
        });
    }

    pub fn add_committee(&mut self, user_id: i64) {
        self.deliberation.with_mut(|req| {
            req.users.push(user_id);
        });
    }

    pub fn remove_committee(&mut self, user_id: i64) {
        self.deliberation.with_mut(|req| {
            req.users
                .retain(|committee_id| !(committee_id.clone() == user_id));
        })
    }

    pub fn clear_committee(&mut self) {
        self.deliberation.with_mut(|req| req.users = vec![]);
    }

    pub fn get_committees(&self) -> Vec<OrganizationMemberSummary> {
        let committees = self.committee_members();
        let members = self.members().unwrap_or_default();

        tracing::debug!("members: {:?} committees: {:?}", members, committees);

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

    pub fn get_selected_committee(&self) -> Vec<OrganizationMemberSummary> {
        let total_committees = self.members().unwrap_or_default();
        let deliberation = self.deliberation();
        let roles = deliberation.clone().users;
        total_committees
            .clone()
            .into_iter()
            .filter(|member| roles.iter().any(|id| id.clone() == member.id))
            .collect()
    }

    pub fn remove_elearning(&mut self, index: usize) {
        self.deliberation.with_mut(|req| {
            req.elearnings.remove(index);
        });
    }

    pub fn add_elearning(&mut self) {
        self.deliberation.with_mut(|req| {
            req.elearnings.push(ElearningCreateRequest::default());
        });
    }

    pub fn set_elearning_necessary(&mut self, index: usize, necessary: bool) {
        self.deliberation.with_mut(|req| {
            req.elearnings[index].necessary = necessary;
        });
    }

    pub fn set_elearning_title(&mut self, index: usize, title: String) {
        self.deliberation.with_mut(|req| {
            req.elearnings[index].title = title;
        });
    }

    pub async fn set_elearning_metadata(&mut self, index: usize, file: File) {
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

        self.deliberation.with_mut(|req| {
            tracing::debug!("elearnings: {:?} index: {:?}", req.elearnings, index);
            req.elearnings[index].resources.push(file);
        });
    }

    pub fn back(&mut self) {
        self.parent.save_content(self.deliberation());
        self.nav
            .replace(Route::DeliberationSampleSurveySettingPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.save_content(self.deliberation());
        self.parent.temporary_save().await;
    }

    pub fn next(&mut self) {
        self.parent.save_content(self.deliberation());
        self.nav
            .push(Route::DeliberationDiscussionSettingPage { lang: self.lang });
    }
}
