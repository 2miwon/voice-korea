use bdk::prelude::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    id: ReadOnlySignal<i64>,
    #[allow(dead_code)]
    discussion_id: ReadOnlySignal<i64>,
    pub nav: Navigator,
}

impl Controller {
    pub fn init(
        lang: Language,
        id: ReadOnlySignal<i64>,
        discussion_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            id,
            discussion_id,
            nav: use_navigator(),
        };

        Ok(ctrl)
    }

    pub fn back(&self) {
        self.nav.go_back();
    }
}
