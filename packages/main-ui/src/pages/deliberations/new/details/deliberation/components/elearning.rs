#![allow(unused_variables)]
#![allow(unused_mut)]
use bdk::prelude::*;
use models::{elearning::ElearningCreateRequest, File};

use crate::{
    components::{
        form_field::{InputField, UploadField},
        section::{AddSection, MainSection, SubSection},
        upload_button::UploadButton,
    },
    pages::deliberations::new::details::deliberation::i18n::DeliberationTranslate,
    service::metadata_api::MetadataApi,
};

#[cfg(feature = "web")]
use crate::components::drop_zone::handle_file_upload;
#[cfg(feature = "web")]
use models::ApiError;

#[component]
pub fn DeliberationElearning(
    lang: Language,
    elearnings: Vec<ElearningCreateRequest>,
    set_elearning_title: EventHandler<(usize, String)>,
    set_elearning_necessary: EventHandler<(usize, bool)>,
    set_elearning_metadata: EventHandler<(usize, File)>,
    add_elearning: EventHandler<MouseEvent>,
    remove_elearning: EventHandler<usize>,
) -> Element {
    let api: MetadataApi = use_context();
    let tr: DeliberationTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col gap-20 w-full",
            for (index , elearning) in elearnings.iter().enumerate() {
                MainSection {
                    lang,
                    header: None,
                    description: None,
                    ondelete: move |_| {
                        remove_elearning.call(index);
                    },
                    SubSection { required: false, title: tr.sub_title1.to_string(),
                        InputField {
                            name: format!("elearning-title-{index}"),
                            placeholder: tr.sub_placeholder1,
                            value: elearning.title.clone(),
                            oninput: move |event: Event<FormData>| {
                                set_elearning_title.call((index, event.value()));
                            },
                        }
                    }
                    SubSection { required: false, title: tr.sub_title2.to_string(),
                        div { class: "flex flex-col w-full focus:outline-none justify-center items-center gap-10",
                            UploadField {
                                lang,
                                description: tr.e_learning_desc.to_string(),
                                value: elearning.resources[0].title.clone(), // only one file per elearning chapter
                                UploadButton {
                                    id: format!("elearning-file-upload-{index}"),
                                    class: "flex min-w-[130px] h-[40px] bg-[#2a60d3] rounded-sm text-white text-center font-semibold text-sm justify-center items-center",
                                    text: tr.upload_directly,
                                    onuploaded: move |event: FormEvent| {
                                        spawn(async move {
                                            #[cfg(feature = "web")]
                                            if let Some(file_engine) = event.files() {
                                                let result = handle_file_upload(file_engine, api).await;
                                                if !result.is_empty() {
                                                    set_elearning_metadata.call((index, result[0].clone()));
                                                } else {
                                                    btracing::e!(lang, ApiError::DeliberationResourceException);
                                                }
                                            }
                                        });
                                    },
                                }

                            // TODO: add a file loading from "자료관리"
                            // div { class: "flex min-w-[165px] h-[40px] border border-[#2a60d3] bg-white rounded-sm text-[#2a60d3] text-center font-semibold text-sm justify-center items-center",
                            //     {tr.load_from}
                            // }
                            }
                        }
                    }
                }
            }
        }

        AddSection {
            lang,
            onclick: move |e| {
                add_elearning.call(e);
            },
        }
    }
}
