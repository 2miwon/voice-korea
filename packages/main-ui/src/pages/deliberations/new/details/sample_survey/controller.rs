use bdk::prelude::*;

use models::{
    deliberation_sample_surveys::deliberation_sample_survey::DeliberationSampleSurveyCreateRequest,
    deliberation_user::DeliberationUserCreateRequest, *,
};

use crate::{routes::Route, service::login_service::LoginService, utils::time::current_timestamp};

use super::DeliberationNewController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    sample_survey: Signal<DeliberationSampleSurveyCreateRequest>,
    pub parent: DeliberationNewController,
    pub nav: Navigator,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub committee_members: Signal<Vec<DeliberationUserCreateRequest>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let sample_survey = use_signal(|| DeliberationSampleSurveyCreateRequest::default());

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
            sample_survey,
            parent: use_context(),
            nav: use_navigator(),

            members,
            committee_members: use_signal(|| vec![]),
        };

        use_effect({
            let req = ctrl.parent.deliberation_requests();
            let mut sample_surveys = req
                .sample_surveys
                .get(0)
                .unwrap_or(&DeliberationSampleSurveyCreateRequest::default())
                .clone();
            let current_timestamp = current_timestamp();
            let committees = req.roles.clone();

            move || {
                let started_at = sample_surveys.clone().started_at;
                let ended_at = sample_surveys.clone().ended_at;

                if started_at == 0 {
                    sample_surveys.started_at = current_timestamp;
                }

                if ended_at == 0 {
                    sample_surveys.ended_at = current_timestamp;
                }

                ctrl.sample_survey.set(sample_surveys.clone());
                ctrl.committee_members.set(committees.clone());
            }
        });

        Ok(ctrl)
    }

    pub fn set_title(&mut self, title: String) {
        self.sample_survey.with_mut(|req| {
            req.title = title;
        });
    }

    pub fn set_description(&mut self, description: String) {
        self.sample_survey.with_mut(|req| {
            req.description = description;
        });
    }

    pub fn set_start_date(&mut self, started_at: i64) {
        self.sample_survey.with_mut(|req| {
            req.started_at = started_at;
        });
    }

    pub fn set_end_date(&mut self, ended_at: i64) {
        self.sample_survey.with_mut(|req| {
            req.ended_at = ended_at;
        });
    }

    pub fn set_estimate_time(&mut self, estimate_time: i64) {
        self.sample_survey.with_mut(|req| {
            req.estimate_time = estimate_time;
        });
    }

    pub fn set_point(&mut self, point: i64) {
        self.sample_survey.with_mut(|req| {
            req.point = point;
        });
    }

    pub fn add_committee(&mut self, user_id: i64) {
        self.sample_survey.with_mut(|req| {
            req.users.push(user_id);
        });
    }

    pub fn remove_committee(&mut self, user_id: i64) {
        self.sample_survey.with_mut(|req| {
            req.users
                .retain(|committee_id| !(committee_id.clone() == user_id));
        })
    }

    pub fn clear_committee(&mut self) {
        self.sample_survey.with_mut(|req| req.users = vec![]);
    }

    pub fn add_question(&mut self) {
        self.sample_survey.with_mut(|req| {
            req.surveys.push(Question::default());
        });
    }

    pub fn remove_question(&mut self, index: usize) {
        self.sample_survey.with_mut(|req| {
            req.surveys.remove(index);
        });
    }

    pub fn update_question(&mut self, index: usize, question: Question) {
        self.sample_survey.with_mut(|req| {
            req.surveys[index] = question;
        });
    }

    pub fn get_sample_survey(&self) -> DeliberationSampleSurveyCreateRequest {
        (self.sample_survey)()
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
        let sample_survey = self.get_sample_survey();
        let roles = sample_survey.clone().users;
        total_committees
            .clone()
            .into_iter()
            .filter(|member| roles.iter().any(|id| id.clone() == member.user_id))
            .collect()
    }

    pub fn back(&mut self) {
        self.parent.save_sample_survey(self.sample_survey());
        self.nav
            .replace(Route::DeliberationBasicInfoSettingPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.save_sample_survey(self.sample_survey());
        self.parent.temporary_save(false).await;
    }

    pub fn next(&mut self) {
        self.parent.save_sample_survey(self.sample_survey());
        self.nav
            .push(Route::DeliberationSettingPage { lang: self.lang });
    }
}
