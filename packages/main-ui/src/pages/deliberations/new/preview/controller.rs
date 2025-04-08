use bdk::prelude::*;

use crate::routes::Route;

use super::ParentController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    pub parent: ParentController,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            parent: use_context(),
            nav: use_navigator(),
        };

        Ok(ctrl)
    }

    pub fn go_list(&mut self) {
        self.nav
            .replace(Route::DeliberationPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.temporary_save().await;
    }

    pub fn start_deliberation(&mut self) {
        tracing::debug!("start button click");
    }
}
