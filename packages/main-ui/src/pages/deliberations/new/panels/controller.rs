use bdk::prelude::*;
use regex::Regex;

use super::{i18n::CompositionPanelTranslate, *};
use crate::routes::Route;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,

    #[allow(dead_code)]
    pub parent_ctrl: ParentController,

    pub emails: Signal<Vec<String>>,
    pub panel_emails: Signal<String>,
    nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let mut ctrl = Self {
            lang,
            nav: use_navigator(),

            parent_ctrl: use_context(),

            emails: use_signal(|| vec![]),
            panel_emails: use_signal(|| "".to_string()),
        };

        use_effect({
            let req = ctrl.parent_ctrl.deliberation_requests();
            move || {
                // let selected_panels: Vec<PanelV2Summary> = panels
                //     .iter()
                //     .filter(|panel| req.panel_ids.contains(&panel.id))
                //     .cloned()
                //     .collect();

                // ctrl.selected_panels.set(selected_panels);
                ctrl.emails.set(req.panel_emails.clone());
            }
        });

        Ok(ctrl)
    }

    pub fn back(&mut self) {
        self.save_deliberation();
        self.nav.go_back();
    }

    pub async fn temp_save(&mut self) {
        self.save_deliberation();
        self.parent_ctrl.temporary_save(false).await;
    }

    pub fn next(&mut self) {
        if self.validation_check() {
            self.save_deliberation();
            self.nav
                .push(Route::DeliberationBasicInfoSettingPage { lang: self.lang });
        }
    }

    pub fn save_deliberation(&mut self) {
        let mut parent_ctrl = self.parent_ctrl;
        let emails = self.emails();
        parent_ctrl.save_panels(emails);
    }

    // pub fn add_selected_panel(&mut self, panel: PanelV2Summary) {
    //     self.selected_panels.push(panel);
    // }

    // pub fn remove_selected_panel(&mut self, panel_id: i64) {
    //     self.selected_panels.retain(|panel| !(panel.id == panel_id));
    // }

    // pub fn clear_selected_panel(&mut self) {
    //     self.selected_panels.set(vec![]);
    // }

    // pub fn change_selected_panel_by_index(&mut self, index: usize, value: u64) {
    //     self.selected_panels.with_mut(|panels| {
    //         panels[index].user_count = value;
    //     });
    // }

    pub fn remove_email(&mut self, index: usize) {
        self.emails.with_mut(|emails| {
            emails.remove(index);
        });
    }

    pub fn update_panel_email(&mut self, value: String) {
        self.panel_emails.set(value);
    }

    pub fn add_panel_email(&mut self) {
        let lang = self.lang;
        let tr: CompositionPanelTranslate = translate(&lang);
        let input = self.panel_emails();

        let emails: Vec<String> = input
            .split(",")
            .map(|s| s.trim())
            .map(|v| v.to_string())
            .collect();

        for email in emails.clone() {
            let email_regex = Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();

            if !email_regex.is_match(&email) {
                btracing::error!("{}", tr.format_error);
                return;
            }

            if self.emails().iter().any(|e| e.clone() == email) {
                btracing::error!("{}", tr.already_exists_error);
                return;
            }
        }

        self.emails.with_mut(|panel_emails| {
            for email in emails.clone() {
                panel_emails.push(email);
            }
        });

        self.panel_emails.set("".to_string());
    }

    pub fn is_valid(&self) -> bool {
        !(self.emails().is_empty())
    }

    pub fn validation_check(&self) -> bool {
        if self.emails().is_empty() {
            btracing::e!(self.lang, ValidationError::PanelRequired);
            return false;
        }

        true
    }
}

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum ValidationError {
    #[translate(
        ko = "공론에 참여할 패널을 1개 이상 선택해주세요.",
        en = "Please select at least one panelist to participate in the discussion."
    )]
    PanelRequired,
}
