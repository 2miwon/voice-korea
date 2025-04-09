#![allow(unused_variables)]
use super::controller::OverviewController;
use super::i18n::*;
#[cfg(feature = "web")]
use crate::components::drop_zone::handle_file_upload;
#[allow(non_snake_case)]
use crate::{
    components::{
        dropdown::Dropdown,
        section::{MainSection, SubSection},
        upload_button::UploadButton,
    },
    routes::Route,
    service::metadata_api::MetadataApi,
};
use bdk::prelude::*;
#[cfg(feature = "web")]
use models::ApiError;
use models::ProjectArea;

// TODO: implement setting deliberation
#[component]
pub fn DeliberationNewPage(lang: Language) -> Element {
    let api: MetadataApi = use_context();
    let tr: SettingDeliberationTranslate = translate(&lang);
    let nav = use_navigator();
    let mut ctrl = OverviewController::new(lang)?;

    rsx! {
        div { class: format!("flex flex-col w-full justify-start items-start gap-10"),
            div { class: "font-medium text-base text-text-black", {tr.overview} }
            MainSection {
                required: true,
                header: tr.title.to_string(),
                description: tr.description.to_string(),
                SubSection { required: true, title: tr.proj_title.to_string(),
                    div { class: "flex flex-row w-full focus:outline-none justify-start items-center bg-background-gray rounded-[4px] h-54",
                        div { class: "flex px-15 w-full",
                            input {
                                class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                r#type: "text",
                                name: "deliberation-name",
                                placeholder: tr.proj_title_placeholder,
                                value: ctrl.title(),
                                oninput: move |event| {
                                    ctrl.title.set(event.value());
                                },
                            }
                        }
                    }
                }
                SubSection { required: true, title: tr.proj_desc.to_string(),
                    div { class: "flex flex-row w-full focus:outline-none justify-start items-start bg-background-gray rounded-[4px] h-248",
                        div { class: "flex px-15 py-10 w-full h-full justify-start items-start",
                            textarea {
                                class: "flex w-full h-full justify-start items-start bg-transparent focus:outline-none my-10 break-words whitespace-normal",
                                placeholder: tr.proj_desc_placeholder,
                                name: "deliberation-description",
                                value: ctrl.description(),
                                oninput: move |event| ctrl.description.set(event.value()),
                            }
                        }
                    }
                }
                SubSection { required: true, title: tr.deliberation_field.to_string(),
                    div { class: "flex w-full",
                        Dropdown {
                            id: "deliberation fields",
                            items: ProjectArea::variants(&lang),
                            hint: tr.deliberation_field_hint,
                            onselect: move |selected_items| ctrl.save_project_area(selected_items),
                            value: Some(ctrl.fields()),
                        }
                    }
                }
                SubSection { required: true, title: tr.thumbnail.to_string(),
                    div { class: "flex flex-col w-full focus:outline-none justify-center items-center gap-10",
                        div { class: "flex flex-col w-full",
                            div { class: "flex flex-row gap-20 px-15 w-full h-54 bg-background-gray rounded-sm justify-center items-center",
                                UploadButton {
                                    class: "flex min-w-[130px] h-[40px] border bg-white border-[#2a60d3] rounded-sm text-[#2a60d3] text-center font-semibold text-sm justify-center items-center",
                                    text: tr.upload_directly,
                                    onuploaded: move |event: FormEvent| {
                                        spawn(async move {
                                            #[cfg(feature = "web")]
                                            if let Some(file_engine) = event.files() {
                                                let result = handle_file_upload(file_engine, api).await;
                                                if !result.is_empty() {
                                                    if let Some(url) = result[0].url.as_ref() {
                                                        ctrl.thumbnail_image.set(url.clone());
                                                    }
                                                } else {
                                                    btracing::e!(lang, ApiError::DeliberationResourceException);
                                                }
                                            }
                                        });
                                    },
                                }
                                input {
                                    class: "flex flex-row w-full justify-start items-center bg-transparent text-disabled focus:outline-none",
                                    r#type: "text",
                                    placeholder: tr.no_file,
                                    readonly: true,
                                    value: ctrl.get_file_name(),
                                }
                            }
                            p { class: "text-text-gray text-start w-full text-sm font-normal",
                                {tr.upload_desc}
                            }
                        }
                        div { class: "flex flex-col w-full",

                            if !ctrl.thumbnail_image().is_empty() {
                                img {
                                    class: "w-250 h-250 bg-background-gray",
                                    src: ctrl.thumbnail_image(),
                                    alt: "thumbnail preview",
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-end items-end mt-30 mb-50",
                Link {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    to: Route::DeliberationPage { lang },
                    {tr.go_to_deliberation_management_list}
                }
                div {
                    class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20 cursor-pointer hover:!bg-primary hover:!text-white",
                    onclick: move |_| async move {
                        ctrl.temp_save().await;
                    },
                    {tr.temporary_save}
                }
                div {
                    class: "aria-active:cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-disabled aria-active:!bg-hover font-semibold text-base text-white",
                    "aria-active": ctrl.is_valid(),
                    onclick: move |_| ctrl.next(),
                    {tr.next}
                }
            }
        }
    }
}
