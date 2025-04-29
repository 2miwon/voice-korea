use bdk::prelude::*;
use models::{DeliberationDiscussion, DeliberationDiscussionQuery, DeliberationDiscussionSummary};

use crate::{routes::Route, service::user_service::UserService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,

    discussion: Resource<DeliberationDiscussionSummary>,
    pub nav: Navigator,
    pub user: UserService,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let discussion = use_server_future(move || async move {
            let res = DeliberationDiscussion::get_client(&crate::config::get().api_url)
                .query(project_id(), DeliberationDiscussionQuery::new(1))
                .await
                .unwrap_or_default();
            if res.items.is_empty() {
                DeliberationDiscussionSummary::default()
            } else {
                res.items[0].clone()
            }
        })?;

        let ctrl = Self {
            lang,
            project_id,
            discussion,
            nav: use_navigator(),
            user: use_context(),
        };

        Ok(ctrl)
    }

    pub fn is_valid(&self) -> bool {
        let discussion = self.discussion().unwrap();
        let emails: Vec<String> = discussion.emails.iter().map(|v| v.email.clone()).collect();

        let email = (self.user.email)();
        !(!self.user.is_login() || !emails.contains(&email))
    }

    pub fn validation_check(&self) -> bool {
        let discussion = self.discussion().unwrap();
        let emails: Vec<String> = discussion.emails.iter().map(|v| v.email.clone()).collect();

        let email = (self.user.email)();

        if !self.user.is_login() {
            btracing::e!(self.lang, ValidationError::LoginRequired);
            return false;
        }
        if !emails.contains(&email) {
            btracing::e!(self.lang, ValidationError::NoAuthorization);
            return false;
        }

        true
    }

    pub async fn start_meeting(&self, discussion_id: i64) {
        if self.validation_check() {
            self.nav.push(Route::DiscussionVideoPage {
                lang: self.lang,
                project_id: self.project_id(),
                discussion_id,
            });
        }
    }
}

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum ValidationError {
    #[translate(
        ko = "입장 권한이 없습니다.",
        en = "You do not have permission to enter."
    )]
    NoAuthorization,
    #[translate(
        ko = "로그인이 필요한 서비스입니다.",
        en = "This service requires login."
    )]
    LoginRequired,
}
