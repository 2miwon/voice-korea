use bdk::prelude::*;
use chrono::Local;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,

    // pub _parent: super::super::Controller,
    pub is_focusing_title: Signal<bool>,
    pub is_focusing_eval: Signal<bool>,
    pub start_date: Signal<i64>,
    pub end_date: Signal<i64>,
    pub title: Signal<String>,
    pub content: Signal<String>,
    pub e_learning_title: Signal<String>,
    pub e_learning_file_url: Signal<String>,
    pub e_learning_tab: Signal<bool>,
    pub select_field: Signal<Option<String>>, // FIXME: which type?
    pub eval_title: Signal<String>,
    pub eval_content: Signal<String>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,

            // _parent: use_context(),
            is_focusing_title: use_signal(|| false),
            is_focusing_eval: use_signal(|| false),
            start_date: use_signal(|| Local::now().timestamp()),
            end_date: use_signal(|| Local::now().timestamp()),
            title: use_signal(|| "".to_string()),
            content: use_signal(|| "".to_string()),
            e_learning_title: use_signal(|| "".to_string()),
            e_learning_file_url: use_signal(|| "".to_string()),
            e_learning_tab: use_signal(|| true),
            select_field: use_signal(|| None),
            eval_title: use_signal(|| "".to_string()),
            eval_content: use_signal(|| "".to_string()),
        };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn get_file_name(&self) -> String {
        let url = self.e_learning_file_url();
        if url.is_empty() {
            return String::new();
        }
        url.split('/').last().unwrap_or_default().to_string()
    }
}
