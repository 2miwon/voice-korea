use bdk::prelude::*;
use models::{
    deliberation_role::DeliberationRoleCreateRequest, DeliberationBasicInfoCreateRequest,
    DeliberationContentCreateRequest, DeliberationDiscussionCreateRequest,
    DeliberationFinalSurveyCreateRequest, DeliberationSampleSurveyCreateRequest, Role,
};

use crate::routes::Route;

use super::ParentController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,

    pub committees: Signal<Vec<DeliberationRoleCreateRequest>>,
    pub roles: Signal<Vec<Role>>,

    pub emails: Signal<Vec<String>>,

    pub basic_info: Signal<DeliberationBasicInfoCreateRequest>,
    pub sample_survey: Signal<DeliberationSampleSurveyCreateRequest>,
    pub deliberation: Signal<DeliberationContentCreateRequest>,
    pub discussion: Signal<DeliberationDiscussionCreateRequest>,
    pub final_survey: Signal<DeliberationFinalSurveyCreateRequest>,

    #[allow(dead_code)]
    pub parent: ParentController,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let mut ctrl = Self {
            lang,
            parent: use_context(),
            nav: use_navigator(),

            emails: use_signal(|| vec![]),
            committees: use_signal(|| vec![]),
            roles: use_signal(|| {
                vec![
                    Role::Admin,
                    Role::DeliberationAdmin,
                    Role::Analyst,
                    Role::Moderator,
                    Role::Speaker,
                ]
            }),

            basic_info: use_signal(|| DeliberationBasicInfoCreateRequest::default()),
            sample_survey: use_signal(|| DeliberationSampleSurveyCreateRequest::default()),
            deliberation: use_signal(|| DeliberationContentCreateRequest::default()),
            discussion: use_signal(|| DeliberationDiscussionCreateRequest::default()),
            final_survey: use_signal(|| DeliberationFinalSurveyCreateRequest::default()),
        };

        let req = ctrl.parent.deliberation_requests();

        // committee
        ctrl.committees.set(req.roles.clone());

        // panel
        ctrl.emails.set(req.panel_emails.clone());

        // deliberation step
        ctrl.basic_info.set(
            req.basic_infos
                .get(0)
                .unwrap_or(&DeliberationBasicInfoCreateRequest::default())
                .clone(),
        );
        ctrl.sample_survey.set(
            req.sample_surveys
                .get(0)
                .unwrap_or(&DeliberationSampleSurveyCreateRequest::default())
                .clone(),
        );
        ctrl.deliberation.set(
            req.contents
                .get(0)
                .unwrap_or(&DeliberationContentCreateRequest::default())
                .clone(),
        );
        ctrl.discussion.set(
            req.deliberation_discussions
                .get(0)
                .unwrap_or(&DeliberationDiscussionCreateRequest::default())
                .clone(),
        );
        ctrl.final_survey.set(
            req.final_surveys
                .get(0)
                .unwrap_or(&DeliberationFinalSurveyCreateRequest::default())
                .clone(),
        );

        Ok(ctrl)
    }

    pub fn go_list(&mut self) {
        self.nav
            .replace(Route::DeliberationPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.temporary_save(false).await;
    }

    pub async fn start_deliberation(&mut self) {
        tracing::debug!("start button click");

        self.parent.start_deliberation().await;
    }
}
