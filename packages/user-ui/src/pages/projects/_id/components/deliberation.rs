use by_components::icons::upload_download::Download2;
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use models::{
    deliberation_content::DeliberationContent, DeliberationContentQuery,
    DeliberationContentSummary, Tab,
};

use crate::{
    components::icons::triangle::{TriangleDown, TriangleUp},
    utils::time::formatted_timestamp,
};

#[component]
pub fn Deliberation(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let deliberation = ctrl.deliberation()?;

    let tr: DeliberationTranslate = translate(&lang);
    let tab_title: &str = Tab::Deliberation.translate(&lang);
    let mut clicked1 = use_signal(|| true);

    rsx! {
        div {
            id: "deliberation",
            class: "max-[1000px]:px-30 flex flex-col w-full h-fit bg-box-gray gap-20",
            ..attributes,
            // header
            div { class: "w-full flex flex-row max-[500px]:flex-col max-[500px]:items-start max-[500px]:justify-start max-[500px]:gap-5 justify-between items-center mt-28",
                div { class: " font-semibold text-xl", "{tab_title}" }
                div { class: "font-medium text-[15px] text-black",
                    {
                        format!(
                            "{} ~ {}",
                            formatted_timestamp(lang, deliberation.started_at),
                            formatted_timestamp(lang, deliberation.ended_at),
                        )
                    }
                }
            }
            // information section
            div { class: "flex flex-col gap-10",

                // introduction section
                div { class: "w-full flex flex-col rounded-lg bg-white justify-start items-center py-14 px-20",
                    div {
                        class: "w-full flex justify-start items-center text-base font-bold cursor-pointer",
                        onclick: move |_| {
                            clicked1.set(!clicked1());
                        },
                        div { class: "w-full flex flex-row justify-between items-center",
                            span { "{tr.main_title}" }
                            if clicked1() {
                                TriangleUp {}
                            } else {
                                TriangleDown {}
                            }
                        }
                    }
                    if clicked1() {
                        //line
                        hr { class: "w-full h-1 mt-12 mb-12 border-line-gray" }
                        div { class: "w-full justify-start mt-15 mb-20 font-bold text-lg",
                            "{deliberation.title}"
                        }
                        div { class: "w-full flex justify-start text-[15px]",
                            "{deliberation.description}"
                        }
                    }
                }

                //Related Data
                div { class: "w-full flex flex-col rounded-[8px] mb-40 bg-white justify-start items-center py-14 px-20",
                    // title and button
                    div { class: "w-full flex justify-start items-center gap-13",
                        div { class: "w-180 flex flex-row items-center text-base font-bold",
                            span { "{tr.deliberation_materials_title}" }
                        }
                        //file
                        div { class: "flex flex-wrap flex-1 justify-start items-center gap-8",
                            for resource in deliberation.resources {
                                div {
                                    class: "cursor-pointer flex flex-row justify-start items-center rounded-[100px] bg-light-gray gap-4 px-12 py-4",
                                    onclick: {
                                        let files = resource.files.clone();
                                        move |_| {
                                            let files = files.clone();
                                            async move {
                                                for file in files.clone() {
                                                    let name = file.name;
                                                    let link = file.url;
                                                    ctrl.download_file(name, link).await;
                                                }
                                            }
                                        }
                                    },
                                    Download2 {
                                        width: "18",
                                        height: "18",
                                        class: " [&>path]:fill-white",
                                    }
                                    div { class: "font-medium text-sm text-white", {resource.title} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(unused)]
    lang: Language,
    #[allow(unused)]
    deliberation_id: ReadOnlySignal<i64>,

    deliberation: Resource<DeliberationContentSummary>,
}

impl Controller {
    pub fn new(
        lang: Language,
        deliberation_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let deliberation = use_server_future(move || async move {
            let res = DeliberationContent::get_client(&crate::config::get().api_url)
                .query(deliberation_id(), DeliberationContentQuery::new(1))
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
            deliberation_id,
            deliberation,
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

translate! {
    DeliberationTranslate;

    main_title: {
        ko: "주요 내용",
        en: "Highlights"
    }

    e_learning_title: {
        ko: "e-Learning",
        en: "e-Learning"
    }

    deliberation_materials_title: {
        ko: "숙의 자료",
        en: "Deliberation materials"
    }
}
