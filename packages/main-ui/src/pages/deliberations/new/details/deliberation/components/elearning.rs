#![allow(unused_variables)]
#![allow(unused_mut)]
use bdk::prelude::*;
use models::{elearning::ElearningCreateRequest, File};

use crate::{
    components::{icons::Trash, section::SubSection, upload_button::UploadButton},
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
        div { class: "flex flex-col w-full justify-start items-start gap-10",
            for (index , elearning) in elearnings.iter().enumerate() {
                div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-20 mb-20 gap-10",
                    SubSection { required: false, title: tr.sub_title1.to_string(),
                        div { class: "flex flex-row w-full focus:outline-none justify-start items-center bg-background-gray rounded-[4px] h-54",
                            div { class: "flex px-15 w-full",
                                input {
                                    class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                    r#type: "text",
                                    placeholder: tr.sub_placeholder1,
                                    value: elearning.title.clone(),
                                    oninput: move |event| {
                                        set_elearning_title.call((index, event.value()));
                                    },
                                }
                            }
                        }
                    }
                    SubSection { required: false, title: tr.sub_title2.to_string(),
                        div { class: "flex flex-col w-full focus:outline-none justify-center items-center gap-10",
                            div { class: "flex flex-col w-full",
                                div { class: "flex flex-row gap-20 px-15 w-full h-54 bg-background-gray rounded-sm justify-center items-center",
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

                                    // div { class: "flex min-w-[165px] h-[40px] border border-[#2a60d3] bg-white rounded-sm text-[#2a60d3] text-center font-semibold text-sm justify-center items-center",
                                    //     {tr.load_from}
                                    // }

                                    div { class: "flex flex-wrap flex-1 w-full justify-start items-start gap-10",
                                        for resource in elearning.resources.clone() {
                                            input {
                                                class: "flex flex-row w-full justify-start items-center bg-transparent text-disabled focus:outline-none",
                                                r#type: "text",
                                                placeholder: tr.sub_placeholder2,
                                                readonly: true,
                                                value: resource.title,
                                            }
                                        }
                                    }
                                }
                                p { class: "text-text-gray text-start w-full text-sm font-normal",
                                    {tr.e_learning_desc}
                                }
                            }
                        }
                    }
                    div { class: "flex flex-row w-full justify-end items-center gap-5 mt-10",
                        button {
                            class: "cursor-pointer flex flex-row w-80 items-center justify-end",
                            onclick: move |_| {
                                remove_elearning.call(index);
                            },
                            div { class: "font-medium text-text-black text-[15px]",
                                {tr.remove}
                            }
                            Trash { width: "18", height: "18" }
                        }
                    }
                }
            }

            AddElearning {
                lang,
                onclick: move |e| {
                    add_elearning.call(e);
                },
            }
        }
    }
}

#[component]
pub fn AddElearning(lang: Language, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "relative w-full flex items-center justify-center",
            div { class: "absolute w-full h-1 border border-dashed border-hint-gray" }
            button {
                class: "cursor-pointer z-10 bg-white border border-hint-gray rounded-full w-45 h-45 flex items-center justify-center hover:shadow-md",
                onclick: move |e| {
                    onclick.call(e);
                },
                "+"
            }
        }
    }
}
