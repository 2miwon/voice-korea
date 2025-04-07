use bdk::prelude::*;
use models::DeliberationDiscussionCreateRequest;

use crate::{routes::Route, utils::time::current_timestamp};

use super::DeliberationNewController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    discussion: Signal<DeliberationDiscussionCreateRequest>,

    #[allow(dead_code)]
    lang: Language,
    pub parent: DeliberationNewController,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let discussion = use_signal(|| DeliberationDiscussionCreateRequest::default());

        let mut ctrl = Self {
            lang,
            parent: use_context(),
            nav: use_navigator(),

            discussion,
        };

        use_effect({
            let req = ctrl.parent.deliberation_requests();
            let mut discussion = req
                .deliberation_discussions
                .get(0)
                .unwrap_or(&DeliberationDiscussionCreateRequest::default())
                .clone();
            let current_timestamp = current_timestamp();

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
            }
        });
        Ok(ctrl)
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
