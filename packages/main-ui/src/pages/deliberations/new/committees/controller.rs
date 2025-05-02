use std::collections::HashSet;

use bdk::prelude::*;
use models::{deliberation_role::DeliberationRoleCreateRequest, Role};
use regex::Regex;

use super::*;
use crate::pages::deliberations::new::committees::i18n::CompositionCommitteeTranslate;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    pub parent_ctrl: ParentController,
    pub roles: Signal<Vec<Role>>,

    pub committees: Signal<Vec<DeliberationRoleCreateRequest>>,
    pub committee_emails: Signal<Vec<String>>,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let mut ctrl = Self {
            lang,
            parent_ctrl: use_context(),
            nav: use_navigator(),

            committee_emails: use_signal(|| vec![]),
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
        };

        use_effect({
            let req = ctrl.parent_ctrl.deliberation_requests();
            let roles = ctrl.roles();

            move || {
                for _ in roles.clone() {
                    ctrl.committee_emails.push("".to_string());
                }

                ctrl.committees.set(req.roles.clone());
            }
        });

        Ok(ctrl)
    }

    pub fn back(&mut self) {
        self.save_deliberation();
        self.nav
            .replace(crate::routes::Route::DeliberationNewPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.save_deliberation();
        self.parent_ctrl.temporary_save(false).await;
    }

    pub fn next(&mut self) {
        if self.validation_check() {
            self.save_deliberation();
            self.nav
                .push(crate::routes::Route::CompositionPanel { lang: self.lang });
        }
    }

    pub fn save_deliberation(&mut self) {
        let mut parent_ctrl = self.parent_ctrl;
        // let roles = self.committees().iter().map(|v| v.clone()).collect();
        parent_ctrl.save_committees(self.committees());
    }

    pub fn update_email_by_role(&mut self, index: usize, email: String) {
        self.committee_emails.with_mut(|emails| {
            emails[index] = email;
        });
    }

    pub fn remove_email_by_role(&mut self, index: usize) {
        self.committees.with_mut(|roles| {
            roles.remove(index);
        });
    }

    pub fn add_email_by_role(&mut self, index: usize, role: Role) {
        let lang = self.lang;
        let tr: CompositionCommitteeTranslate = translate(&lang);
        let input = self.committee_emails()[index].clone();
        let emails: Vec<String> = input
            .split(",")
            .map(|s| s.trim())
            .map(|v| v.to_string())
            .collect();

        let mut seen = HashSet::new();

        for email in emails.clone() {
            let email_regex = Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();

            if !email_regex.is_match(&email) {
                btracing::error!("{}", tr.email_format_error);
                return;
            }

            if self.committees().iter().any(|r| r.email == email) {
                btracing::error!("{}", tr.role_exist_error);
                return;
            }

            if !seen.insert(email.clone()) {
                btracing::error!("{}", tr.role_exist_error);
                return;
            }
        }

        self.committees.with_mut(|roles| {
            for email in emails.clone() {
                roles.push(DeliberationRoleCreateRequest {
                    email,
                    role: role.clone(),
                });
            }

            roles.sort_by_key(|r| r.role.clone() as i32);
        });

        self.committee_emails.with_mut(|e| {
            e[index] = "".to_string();
        });
    }

    pub fn is_valid(&self) -> bool {
        !(self.committees().is_empty())
    }

    pub fn validation_check(&self) -> bool {
        if self.committees().is_empty() {
            btracing::e!(self.lang, ValidationError::CommitteeRequired);
            return false;
        }

        true
    }
}

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum ValidationError {
    #[translate(
        ko = "공론에 참여할 참여자를 1명 이상 입력해주세요.",
        en = "Please enter at least one participant who will participate in the public discussion."
    )]
    CommitteeRequired,
}
