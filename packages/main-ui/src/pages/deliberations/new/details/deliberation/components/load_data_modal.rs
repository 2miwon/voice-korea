use crate::{
    components::{block_header::BlockHeader, file_list::FileList, icons::Upload},
    pages::deliberations::new::step::material_upload::ImportDocument,
};
use bdk::prelude::*;
use models::{File, ResourceFile, ResourceFileSummary};

#[derive(Clone, Copy, DioxusController)]
struct Controller {
    pub resources: Signal<Vec<ResourceFile>>,
    pub metadatas: Signal<Vec<ResourceFileSummary>>,
}

impl Controller {
    pub fn new(metadatas: Vec<ResourceFileSummary>) -> std::result::Result<Self, RenderError> {
        Ok(Self {
            metadatas: use_signal(|| metadatas),
            resources: use_signal(|| vec![]),
        })
    }

    pub fn get_selected_resources(&self) -> Vec<ResourceFile> {
        let metadatas = self.metadatas();
        let resources = self.resources();

        metadatas
            .clone()
            .into_iter()
            .filter(|resource| resources.iter().any(|r| r.id == resource.id))
            .map(|v| v.into())
            .collect()
    }

    pub fn add_resource(&mut self, resource: ResourceFile) {
        self.resources.with_mut(|resources| {
            if let Some(first) = resources.get_mut(0) {
                *first = resource;
            } else {
                resources.push(resource);
            }
        });
    }

    pub fn delete_resource(&mut self, id: i64) {
        self.resources.retain(|resource| resource.id != id);
    }
}

#[component]
pub fn LoadDataModal(
    lang: Language,
    metadatas: Vec<ResourceFileSummary>,

    onclose: EventHandler<MouseEvent>,
    onupload: EventHandler<ResourceFile>,
) -> Element {
    let mut files = use_signal(|| vec![]);
    let tr: LoadDataModalTranslate = translate(&lang);
    let mut ctrl = Controller::new(metadatas)?;

    use_effect(use_reactive(&ctrl.resources(), move |resources| {
        let all_files: Vec<File> = resources.iter().flat_map(|r| &r.files).cloned().collect();
        files.set(all_files);
        tracing::debug!("Files: {:?}", files());
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-40",
            BlockHeader {
                required: false,
                header: tr.header.to_string(),
                description: tr.description.to_string(),
            }
            // TODO: Only one file can be uploaded at a time
            ImportDocument {
                lang,
                metadatas: ctrl.metadatas(),
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
                onremove: {
                    let resources = ctrl.resources();
                    move |index: usize| {
                        let id = resources[index].id;
                        ctrl.delete_resource(id);
                    }
                },
            }
            div { class: "flex flex-row gap-20",
                button {
                    class: "flex flex-row h-40 px-14 py-8 justify-center items-center bg-primary rounded-[4px] gap-4",
                    onclick: {
                        move |_| {
                            if let Some(resource) = ctrl.get_selected_resources().get(0) {
                                onupload.call(resource.clone());
                            }
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
