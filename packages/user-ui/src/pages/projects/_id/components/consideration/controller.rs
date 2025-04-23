use bdk::prelude::*;
use models::{DeliberationContent, DeliberationContentQuery, DeliberationContentSummary};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(unused)]
    lang: Language,
    #[allow(unused)]
    project_id: ReadOnlySignal<i64>,

    content: Resource<DeliberationContentSummary>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let content = use_server_future(move || async move {
            let res = DeliberationContent::get_client(&crate::config::get().api_url)
                .query(project_id(), DeliberationContentQuery::new(1))
                .await
                .unwrap_or_default();
            if res.items.is_empty() {
                DeliberationContentSummary::default()
            } else {
                res.items[0].clone()
            }
        })?;

        let ctrl = Self {
            lang,
            project_id,
            content,
        };

        Ok(ctrl)
    }

    #[allow(unused)]
    pub async fn download_file(&self, name: String, url: Option<String>) {
        if url.is_none() {
            return;
        }

        let url = url.unwrap_or_default();

        #[cfg(feature = "web")]
        {
            use wasm_bindgen::JsCast;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", &name).unwrap();

            document.body().unwrap().append_child(&a).unwrap();
            let a: web_sys::HtmlElement = a.unchecked_into();
            a.click();
            a.remove();
        }
    }
}
