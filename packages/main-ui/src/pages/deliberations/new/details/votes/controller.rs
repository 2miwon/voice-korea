use bdk::prelude::*;
use models::{
    deliberation_user::DeliberationUserCreateRequest, DeliberationFinalSurveyCreateRequest,
    OrganizationMember, OrganizationMemberQuery, OrganizationMemberSummary,
};

use crate::{routes::Route, service::login_service::LoginService, utils::time::current_timestamp};

use super::DeliberationNewController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    final_survey: Signal<DeliberationFinalSurveyCreateRequest>,
    pub parent: DeliberationNewController,
    pub nav: Navigator,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub committee_members: Signal<Vec<DeliberationUserCreateRequest>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let final_survey = use_signal(|| DeliberationFinalSurveyCreateRequest::default());

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
            final_survey,
            parent: use_context(),
            nav: use_navigator(),

            members,
            committee_members: use_signal(|| vec![]),
        };

        use_effect({
            let req = ctrl.parent.deliberation_requests();
            let mut final_survey = req
                .final_surveys
                .get(0)
                .unwrap_or(&DeliberationFinalSurveyCreateRequest::default())
                .clone();
            let current_timestamp = current_timestamp();
            let committees = req.roles.clone();

            move || {
                let started_at = final_survey.clone().started_at;
                let ended_at = final_survey.clone().ended_at;

                if started_at == 0 {
                    final_survey.started_at = current_timestamp;
                }

                if ended_at == 0 {
                    final_survey.ended_at = current_timestamp;
                }

                ctrl.final_survey.set(final_survey.clone());
                ctrl.committee_members.set(committees.clone());
            }
        });
        Ok(ctrl)
    }

    pub fn set_final_survey(&mut self, info: DeliberationFinalSurveyCreateRequest) {
        self.final_survey.set(info);
    }

    pub fn get_final_survey(&self) -> DeliberationFinalSurveyCreateRequest {
        (self.final_survey)()
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

    pub fn add_committee(&mut self, user_id: i64) {
        self.final_survey.with_mut(|req| {
            req.users.push(user_id);
        });
    }

    pub fn remove_committee(&mut self, user_id: i64) {
        self.final_survey.with_mut(|req| {
            req.users.retain(|id| id.clone() != user_id);
        });
    }

    pub fn clear_committee(&mut self) {
        self.final_survey.with_mut(|req| req.users = vec![]);
    }

    pub fn get_selected_committee(&self) -> Vec<OrganizationMemberSummary> {
        let total_committees = self.members().unwrap_or_default();
        let final_survey = self.get_final_survey();
        let roles = final_survey.clone().users;
        total_committees
            .clone()
            .into_iter()
            .filter(|member| roles.iter().any(|id| id.clone() == member.user_id))
            .collect()
    }

    pub fn back(&mut self) {
        self.parent.save_final_survey(self.final_survey());
        self.nav
            .replace(Route::DeliberationDiscussionSettingPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.save_final_survey(self.final_survey());
        self.parent.temporary_save().await;
    }

    pub fn next(&mut self) {
        self.parent.save_final_survey(self.final_survey());
        self.nav.push(Route::Preview { lang: self.lang });
    }

    pub fn set_title(&mut self, title: String) {
        self.final_survey.with_mut(|req| {
            req.title = title;
        });
    }

    pub fn set_description(&mut self, description: String) {
        self.final_survey.with_mut(|req| {
            req.description = description;
        });
    }

    pub fn set_start_date(&mut self, started_at: i64) {
        self.final_survey.with_mut(|req| {
            req.started_at = started_at;
        });
    }

    pub fn set_end_date(&mut self, ended_at: i64) {
        self.final_survey.with_mut(|req| {
            req.ended_at = ended_at;
        });
    }
}
