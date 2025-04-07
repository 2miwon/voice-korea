use bdk::prelude::*;
use models::{
    deliberation_user::DeliberationUserCreateRequest, DeliberationDiscussionCreateRequest,
    OrganizationMember, OrganizationMemberQuery, OrganizationMemberSummary,
};

use crate::{routes::Route, service::login_service::LoginService, utils::time::current_timestamp};

use super::DeliberationNewController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    discussion: Signal<DeliberationDiscussionCreateRequest>,

    #[allow(dead_code)]
    lang: Language,
    pub parent: DeliberationNewController,
    pub nav: Navigator,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
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

        let mut ctrl = Self {
            lang,
            parent: use_context(),
            nav: use_navigator(),
            discussion,

            members,
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
            let committees = req.roles.clone();

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
                ctrl.committee_members.set(committees.clone());
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

    pub fn get_selected_committee(&self) -> Vec<OrganizationMemberSummary> {
        let total_committees = self.members().unwrap_or_default();
        let discussion = self.discussion();
        let roles = discussion.clone().users;
        total_committees
            .clone()
            .into_iter()
            .filter(|member| roles.iter().any(|id| id.clone() == member.id))
            .collect()
    }

    pub fn set_discussion(&mut self, info: DeliberationDiscussionCreateRequest) {
        self.discussion.set(info);
    }

    pub fn back(&mut self) {
        self.parent.save_discussion(self.discussion());
        self.nav
            .replace(Route::DeliberationSettingPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.save_discussion(self.discussion());
        self.parent.temporary_save().await;
    }

    pub fn next(&mut self) {
        self.parent.save_discussion(self.discussion());
        self.nav
            .push(Route::DeliberationVoteSettingPage { lang: self.lang });
    }
}
