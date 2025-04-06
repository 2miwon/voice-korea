use bdk::prelude::*;
use models::{
    deliberation_user::DeliberationUserCreateRequest, OrganizationMember, OrganizationMemberQuery,
    OrganizationMemberSummary, Role,
};

use super::*;
use crate::service::login_service::LoginService;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    #[allow(dead_code)]
    pub parent_ctrl: ParentController,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub committees: Signal<Vec<DeliberationUserCreateRequest>>,
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

        let ctrl = Self {
            lang,
            members,
            parent_ctrl: use_context(),
            nav: use_navigator(),
            committees: use_signal(move || vec![]),
        };

        Ok(ctrl)
    }

    pub fn back(&self) {
        self.nav.go_back();
    }

    pub fn next(&self) {
        self.nav
            .push(crate::routes::Route::CompositionPanel { lang: self.lang });
    }

    pub fn add_committee(&mut self, committee: DeliberationUserCreateRequest) {
        self.committees.push(committee);
    }

    pub fn remove_committee(&mut self, user_id: i64, role: Role) {
        self.committees
            .retain(|committee| !(committee.user_id == user_id && committee.role == role));
    }

    pub fn clear_committee(&mut self, role: Role) {
        self.committees
            .retain(|committee| !(committee.role == role));
    }
}
