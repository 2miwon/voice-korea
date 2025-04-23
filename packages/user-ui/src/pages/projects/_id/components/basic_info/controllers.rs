use bdk::prelude::*;
use indexmap::IndexMap;
use models::{
    deliberation_role::DeliberationRole, DeliberationBasicInfo, DeliberationBasicInfoQuery,
    DeliberationBasicInfoSummary,
};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    deliberation_id: ReadOnlySignal<i64>,

    basic_info: Resource<DeliberationBasicInfoSummary>,
    committees: Memo<IndexMap<String, Vec<DeliberationRole>>>,
}

impl Controller {
    pub fn new(
        lang: Language,
        deliberation_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let basic_info = use_server_future(move || {
            let deliberation_id = deliberation_id();
            async move {
                let res = DeliberationBasicInfo::get_client(&crate::config::get().api_url)
                    .query(
                        deliberation_id,
                        DeliberationBasicInfoQuery::new(1).with_page(1),
                    )
                    .await;
                match res {
                    Ok(v) => {
                        if v.total_count == 1 {
                            v.items[0].clone()
                        } else {
                            DeliberationBasicInfoSummary::default()
                        }
                    }
                    _ => DeliberationBasicInfoSummary::default(),
                }
            }
        })?;

        let committees: Memo<IndexMap<String, Vec<DeliberationRole>>> = use_memo(move || {
            if let Some(basic_info) = basic_info() {
                let mut committees: IndexMap<String, Vec<DeliberationRole>> = IndexMap::new();
                for deliberation_role in basic_info.roles.iter() {
                    let role_name = deliberation_role.role.translate(&lang);
                    committees
                        .entry(role_name.to_string())
                        .or_insert_with(Vec::new)
                        .push(deliberation_role.clone());
                }
                committees
            } else {
                IndexMap::new()
            }
        });
        let ctrl = Self {
            lang,
            deliberation_id,
            basic_info,
            committees,
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
