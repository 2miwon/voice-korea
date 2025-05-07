#![allow(unused_variables)]
use super::{components::footer_buttons::FooterButtons, controller::OverviewController, i18n::*};
#[cfg(feature = "web")]
use crate::components::drop_zone::handle_file_upload;
#[allow(non_snake_case)]
use crate::{
    components::{
        dropdown::Dropdown,
        form_field::{InputField, TextField, UploadField},
        section::{MainSection, SubSection},
        upload_button::UploadButton,
    },
    service::metadata_api::MetadataApi,
};
use bdk::prelude::*;
#[cfg(feature = "web")]
use models::ApiError;
use models::ProjectArea;

// TODO: implement setting deliberation
#[component]
pub fn DeliberationNewPage(lang: Language, deliberation_id: Option<i64>) -> Element {
    let api: MetadataApi = use_context();
    let tr: SettingDeliberationTranslate = translate(&lang);
    let nav = use_navigator();
    let mut ctrl = OverviewController::new(lang, deliberation_id)?;

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-20",
            div { class: "flex flex-col w-full gap-10",
                div { class: "font-medium text-base text-text-black", {tr.overview} }
                MainSection {
                    lang,
                    required: true,
                    header: Some(tr.title.to_string()),
                    description: Some(tr.description.to_string()),
                    SubSection { required: true, title: tr.proj_title.to_string(),
                        InputField {
                            name: "deliberation-title".to_string(),
                            placeholder: tr.proj_title_placeholder,
                            value: ctrl.title(),
                            oninput: move |event: Event<FormData>| ctrl.parent.save_title(event.value()),
                        }
                    }
                    SubSection { required: true, title: tr.proj_desc.to_string(),
                        TextField {
                            name: "deliberation-description".to_string(),
                            placeholder: tr.proj_desc_placeholder,
                            value: ctrl.description(),
                            oninput: move |event: Event<FormData>| ctrl.parent.save_description(event.value()),
                        }
                    }
                    SubSection {
                        required: true,
                        title: tr.deliberation_field.to_string(),
                        div { class: "flex w-full",
                            Dropdown {
                                id: "deliberation fields",
                                items: ProjectArea::variants(&lang),
                                hint: tr.deliberation_field_hint,
                                onselect: move |selected_items| ctrl.save_project_area(selected_items),
                                value: ctrl.project_areas()
                                    .iter()
                                    .map(|area| area.translate(&lang).to_string())
                                    .collect::<Vec<String>>(),
                            }
                        }
                    }
                    SubSection { required: true, title: tr.thumbnail.to_string(),
                        div { class: "flex flex-col w-full focus:outline-none justify-center items-center gap-10",
                            UploadField {
                                lang,
                                description: tr.upload_desc,
                                value: ctrl.get_file_name(),
                                UploadButton {
                                    class: "flex min-w-[130px] h-[40px] border bg-white border-[#2a60d3] rounded-sm text-[#2a60d3] text-center font-semibold text-sm justify-center items-center",
                                    text: tr.upload_directly,
                                    onuploaded: move |event: FormEvent| async move {
                                        #[cfg(feature = "web")]
                                        if let Some(file_engine) = event.files() {
                                            let result = handle_file_upload(file_engine, api).await;
                                            if !result.is_empty() {
                                                if let Some(url) = result[0].url.as_ref() {
                                                    ctrl.parent.save_thumbnail_image(url.clone());
                                                }
                                            } else {
                                                btracing::e!(lang, ApiError::DeliberationResourceException);
                                            }
                                        }
                                    },
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
            }
            FooterButtons {
                lang,
                on_backward: None,
                on_temp_save: move |_| async move { ctrl.temp_save().await },
                on_next: move |_| {
                    ctrl.next();
                },
                on_save: None,
                next_valid: ctrl.is_valid(),
            }
        }
    }
}
