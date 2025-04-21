use bdk::prelude::*;
use models::{DeliberationFinalSurveyCreateRequest, Question};

use crate::{routes::Route, utils::time::current_timestamp};

use super::DeliberationNewController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    final_survey: Signal<DeliberationFinalSurveyCreateRequest>,
    pub parent: DeliberationNewController,
    pub nav: Navigator,

    pub committee_members: Signal<Vec<String>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let final_survey = use_signal(|| DeliberationFinalSurveyCreateRequest::default());

        let mut ctrl = Self {
            lang,
            final_survey,
            parent: use_context(),
            nav: use_navigator(),

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

            move || {
                let committees = req.roles.iter().map(|v| v.email.clone()).collect();
                let started_at = final_survey.clone().started_at;
                let ended_at = final_survey.clone().ended_at;

                if started_at == 0 {
                    final_survey.started_at = current_timestamp;
                }

                if ended_at == 0 {
                    final_survey.ended_at = current_timestamp;
                }

                ctrl.final_survey.set(final_survey.clone());
                ctrl.committee_members.set(committees);
            }
        });
        Ok(ctrl)
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

    pub fn set_estimate_time(&mut self, estimate_time: i64) {
        self.final_survey.with_mut(|req| {
            req.estimate_time = estimate_time;
        });
    }

    pub fn set_point(&mut self, point: i64) {
        self.final_survey.with_mut(|req| {
            req.point = point;
        });
    }

    pub fn add_committee(&mut self, email: String) {
        self.final_survey.with_mut(|req| {
            req.users.push(email);
        });
    }

    pub fn remove_committee(&mut self, email: String) {
        self.final_survey.with_mut(|req| {
            req.users.retain(|e| !(e.clone() == email));
        })
    }

    pub fn clear_committee(&mut self) {
        self.final_survey.with_mut(|req| req.users = vec![]);
    }

    pub fn add_question(&mut self) {
        self.final_survey.with_mut(|req| {
            req.surveys.push(Question::default());
        });
    }

    pub fn remove_question(&mut self, index: usize) {
        self.final_survey.with_mut(|req| {
            req.surveys.remove(index);
        });
    }

    pub fn update_question(&mut self, index: usize, question: Question) {
        self.final_survey.with_mut(|req| {
            req.surveys[index] = question;
        });
    }

    pub fn get_final_survey(&self) -> DeliberationFinalSurveyCreateRequest {
        (self.final_survey)()
    }

    pub fn get_selected_committee(&self) -> Vec<String> {
        let final_survey = self.get_final_survey();
        let roles = final_survey.clone().users;

        roles
    }

    pub fn back(&mut self) {
        self.parent.save_final_survey(self.final_survey());
        self.nav
            .replace(Route::DeliberationDiscussionSettingPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.save_final_survey(self.final_survey());
        self.parent.temporary_save(false).await;
    }

    pub fn next(&mut self) {
        self.parent.save_final_survey(self.final_survey());
        self.nav.push(Route::Preview { lang: self.lang });
    }
}
