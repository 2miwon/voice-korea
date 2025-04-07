use bdk::prelude::*;
use models::DeliberationFinalSurveyCreateRequest;

use crate::utils::time::current_timestamp;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    final_survey: Signal<DeliberationFinalSurveyCreateRequest>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let final_survey = use_signal(|| DeliberationFinalSurveyCreateRequest::default());
        let mut ctrl = Self { lang, final_survey };

        use_effect({
            let mut final_survey = ctrl.final_survey();
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
}
