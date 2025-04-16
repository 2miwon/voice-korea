#![allow(unused)]
use bdk::prelude::*;
use dioxus_popup::PopupService;

use super::i18n::Translate;
use super::{GoogleLoginPopup, SignupPopup};

use crate::service::user_service::UserService;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    pub user: UserService,
    lang: Language,

    popup_service: PopupService,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: UserService = use_context();

        let ctrl = Self {
            lang,
            user,

            popup_service: use_context(),
        };

        Ok(ctrl)
    }
}

impl Controller {
    pub fn move_to_console(&self) {
        let nav = use_navigator();
        let console_url = &crate::config::get().console_url;
        nav.push(format!("{}", console_url));
    }

    pub fn google_login(&mut self) {
        let tr: Translate = translate(&self.lang);
        let mut popup = self.popup_service.clone();
        popup
            .open(rsx! {
                GoogleLoginPopup {
                    lang: self.lang,
                    onclose: move |_| {
                        popup.close();
                    },
                }
            })
            .with_id("google_login")
            .with_title(tr.login);
    }
}
