use crate::{
    components::{block_header::BlockHeader, file_list::FileList, icons::Upload},
    config,
    pages::deliberations::new::step::material_upload::ImportDocument,
    service::login_service::LoginService,
};
use bdk::prelude::*;
use models::{File, ResourceFile, ResourceFileQuery, ResourceFileSummary};

#[derive(Clone, Copy, DioxusController)]
struct Controller {
    pub resources: Signal<Vec<ResourceFile>>,
    pub metadatas: Resource<Vec<ResourceFileSummary>>,
    #[allow(dead_code)]
    pub search_keyword: Signal<String>,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let metadatas = use_server_future(move || {
            let page = 1;
            let size = 100;
            async move {
                let client = ResourceFile::get_client(&config::get().api_url);
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }
                client
                    .query(
                        org_id.unwrap().id,
                        ResourceFileQuery::new(size).with_page(page),
                    )
                    .await
                    .unwrap_or_default()
                    .items
            }
        })?;

        Ok(Self {
            resources: use_signal(|| vec![]),
            metadatas,
            search_keyword: use_signal(|| "".to_string()),
        })
    }

    pub fn get_selected_resources(&self) -> Vec<ResourceFile> {
        let metadatas = self.metadatas().unwrap_or_default();
        let resources = self.resources();

        metadatas
            .clone()
            .into_iter()
            .filter(|resource| resources.iter().any(|r| r.id == resource.id))
            .map(|v| v.into())
            .collect()
    }

    pub fn add_resource(&mut self, resource: ResourceFile) {
        self.resources.push(resource);
    }

    pub fn delete_resource(&mut self, id: i64) {
        self.resources.retain(|resource| resource.id != id);
    }
}

#[component]
pub fn LoadDataModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    onupload: EventHandler<ResourceFile>,
) -> Element {
    let mut files = use_signal(|| vec![]);
    let tr: LoadDataModalTranslate = translate(&lang);
    let mut ctrl = Controller::new()?;

    use_effect(use_reactive(&ctrl.resources(), move |resources| {
        let all_files: Vec<File> = resources.iter().flat_map(|r| &r.files).cloned().collect();
        files.set(all_files);
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-40",
            BlockHeader {
                required: false,
                header: "Load Data".to_string(),
                description: "Select the data you want to load.".to_string(),
            }
            // TODO: Only one file can be uploaded at a time
            ImportDocument {
                lang,
                metadatas: ctrl.metadatas()?.clone(),
                resources: ctrl.get_selected_resources(),
                onadd: move |resource: ResourceFileSummary| {
                    ctrl.add_resource(resource.clone().into());
                },
                onremove: move |id: i64| {
                    ctrl.delete_resource(id);
                },
            }
            FileList {
                items: files(),
                onremove: move |index: usize| {
                    let id = ctrl.resources()[index].id;
                    ctrl.delete_resource(id);
                },
            }
            div { class: "flex flex-row gap-20",
                button {
                    class: "flex flex-row h-40 px-14 py-8 justify-center items-center bg-primary rounded-[4px] gap-4",
                    onclick: move |_| {
                        if let Some(resource) = ctrl.get_selected_resources().get(0) {
                            onupload.call(resource.clone());
                        }
                    },
                    Upload { width: "24", height: "24" }
                    div { class: "text-white font-semibold text-base", {tr.upload} }
                }
                button {
                    class: "flex flex-row h-40 px-14 py-8 justify-center items-center bg-white rounded-[4px]",
                    onclick: move |event| {
                        onclose.call(event);
                    },
                    div { class: "text-black font-semibold text-base", {tr.cancel} }
                }
            }
        }
    }
}

translate! {
    LoadDataModalTranslate;

    header: {
        ko: "자료 관리에서 불러오기",
        en: "Load from Data Management",
    }

    description: {
        ko: "각 챕터에는 단일 파일만 업로드할 수 있습니다.",
        en: "You can upload only one file per chapter.",
    }

    upload: {
        en: "Upload",
        ko: "업로드하기",
    }

    cancel: {
        en: "Cancel",
        ko: "취소",
    }
}
