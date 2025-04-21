use crate::routes::Route;

use super::models::*;
use bdk::prelude::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    nav: Navigator,
    pub current_step: DeliberationDetailSettingStep,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let route = use_route::<Route>();

        let ctrl = Self {
            lang,
            nav: use_navigator(),
            current_step: route.into(),
        };

        Ok(ctrl)
    }

    #[allow(dead_code)]
    pub fn goto_step(&self, step: &DeliberationDetailSettingStep) {
        self.nav.push(step.to_route(self.lang));
    }
}
