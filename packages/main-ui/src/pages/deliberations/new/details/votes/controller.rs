use bdk::prelude::*;
use models::DeliberationFinalSurveyCreateRequest;

use crate::{routes::Route, utils::time::current_timestamp};

use super::DeliberationNewController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    final_survey: Signal<DeliberationFinalSurveyCreateRequest>,
    pub parent: DeliberationNewController,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let final_survey = use_signal(|| DeliberationFinalSurveyCreateRequest::default());
        let mut ctrl = Self {
            lang,
            final_survey,
            parent: use_context(),
            nav: use_navigator(),
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
                let started_at = final_survey.clone().started_at;
                let ended_at = final_survey.clone().ended_at;

                if started_at == 0 {
                    final_survey.started_at = current_timestamp;
                }

                if ended_at == 0 {
                    final_survey.ended_at = current_timestamp;
                }

                ctrl.final_survey.set(final_survey.clone());
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
}
