#![allow(unused_variables)]
use super::*;
#[cfg(feature = "web")]
use crate::components::drop_zone::handle_file_upload;
use crate::{
    components::{
        calendar::Calendar,
        icons::ArrowLeft,
        icons::CalendarIcon,
        section::{MainSection, SubSection},
        upload_button::UploadButton,
    },
    service::metadata_api::MetadataApi,
    utils::time::change_date_from_timestamp,
};
use bdk::prelude::*;
use controller::*;
use i18n::*;
#[cfg(feature = "web")]
use models::ApiError;

#[component]
pub fn DeliberationSettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: DeliberationTranslate = translate(&lang);
    let api: MetadataApi = use_context();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-header-gray font-medium text-sm mb-10",
                "{tr.organization_management} / {tr.deliberation_management} / {tr.start_deliberation}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-25 gap-10",
                div { onclick: move |_| {},
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-header-black font-semibold text-[28px] mr-20", "{tr.deliberation}" }
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", {tr.post_setting} }

                MainSection {
                    required: true,
                    header: tr.main_section1_title.to_string(),
                    description: tr.main_section1_description.to_string(),
                    open: Some(true),
                    div {
                        div { class: "flex flex-row w-full justify-start items-center",
                            //input_title
                            input {
                                class: "flex flex-row w-full h-[55px] justify-start items-center bg-[#f7f7f7] aria-active:!bg-white aria-active:!border aria-active:!border-active focus:outline-none px-[15px] py-[10px] font-medium text-[#b4b4b4] text-[15px] leading-[22px] rounded-[4px] mr-[10px]",
                                "aria-active": ctrl.is_focusing_title(),
                                r#type: "text",
                                placeholder: tr.title_placeholder.to_string(),
                                value: ctrl.title(),
                                onfocus: move |_| {
                                    ctrl.is_focusing_title.set(true);
                                },
                                onblur: move |_| {
                                    ctrl.is_focusing_title.set(false);
                                },
                                oninput: move |e: Event<FormData>| {
                                    ctrl.title.set(e.value());
                                },
                            }

                            // start date
                            div { class: "group relative",
                                button { class: "flex flex-row w-[190px] focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px]",
                                    div { class: "font-normal text-[16px] text-[#9b9b9b] leading-[24px]",
                                        {change_date_from_timestamp(ctrl.start_date())}
                                    }
                                    CalendarIcon { width: "28", height: "28" }
                                }
                                nav { class: "invisible border-none rounded w-full absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                    Calendar {
                                        timestamp: ctrl.start_date() as u64,
                                        update_date: move |timestamp: i64| {
                                            ctrl.start_date.set(timestamp);
                                        },
                                    }
                                }
                            }

                            div { class: "flex flex-row w-[16px] h-[2px] bg-[#bfc8d9] mx-[10px]" }

                            // end date
                            div { class: "group relative w-[450px]",
                                button { class: "flex flex-row w-[190px]  focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px]",
                                    div { class: "font-normal text-[16px] text-[#9b9b9b] leading-[24px]",
                                        {change_date_from_timestamp(ctrl.end_date())}
                                    }
                                    CalendarIcon { width: "28", height: "28" }
                                }
                                nav { class: "invisible border-none rounded w-full absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                    Calendar {
                                        timestamp: ctrl.end_date() as u64,
                                        update_date: move |timestamp: i64| {
                                            ctrl.end_date.set(timestamp);
                                        },
                                    }
                                }
                            }
                        }

                        input {
                            class: "flex flex-row w-full h-[55px] justify-start items-center bg-white focus:outline-none border-b-[1px] border-[#bfc8d9] px-[15px] py-[15px] font-medium text-[#b4b4b4] text-[15px] leading-[22px]",
                            r#type: "text",
                            placeholder: tr.content_placeholder.to_string(),
                            value: ctrl.content(),
                            oninput: move |e: Event<FormData>| {
                                ctrl.content.set(e.value());
                            },
                        }
                    }
                }

                MainSection {
                    required: false,
                    header: tr.main_section2_title.to_string(),
                    description: tr.main_section2_description.to_string(),
                    open: Some(false),
                }
                div { class: "flex flex-col w-full gap-10 mt-20",
                    div { class: "flex flex-row w-full justify-start items-center gap-10",
                        div {
                            class: "flex items-center justify-center w-197 h-46 bg-primary-deep aria-active:!bg-white rounded-[100px]",
                            "aria-active": ctrl.e_learning_tab(),
                            onclick: move |_| ctrl.e_learning_tab.set(true),
                            p { class: "text-text-black font-bold text-lg", {tr.e_learning_setting} }
                        }
                        div {
                            class: "flex items-center justify-center w-139 h-46 bg-primary-deep aria-active:!bg-white rounded-[100px]",
                            "aria-active": !ctrl.e_learning_tab(),
                            onclick: move |_| ctrl.e_learning_tab.set(false),
                            p { class: "text-text-black font-bold text-lg", {tr.evaluation_setting} }
                        }
                    }

                    div { class: "flex flex-col w-full justify-start items-start",
                        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-20 mb-20 gap-10",
                            if ctrl.e_learning_tab() {
                                SubSection {
                                    required: false,
                                    title: tr.sub_title1.to_string(),
                                    div { class: "flex flex-row w-full focus:outline-none justify-start items-center bg-background-gray rounded-[4px] h-54",
                                        div { class: "flex px-15 w-full",
                                            input {
                                                class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                                r#type: "text",
                                                placeholder: tr.sub_placeholder1,
                                                value: ctrl.e_learning_title(),
                                                oninput: move |event| ctrl.e_learning_title.set(event.value()),
                                            }
                                        }
                                    }
                                }
                                SubSection {
                                    required: false,
                                    title: tr.sub_title2.to_string(),
                                    div { class: "flex flex-col w-full focus:outline-none justify-center items-center gap-10",
                                        div { class: "flex flex-col w-full",
                                            div { class: "flex flex-row gap-20 px-15 w-full h-54 bg-background-gray rounded-sm justify-center items-center",
                                                UploadButton {
                                                    class: "flex min-w-[130px] h-[40px] bg-[#2a60d3] rounded-sm text-white text-center font-semibold text-sm justify-center items-center",
                                                    text: tr.upload_directly,
                                                    onuploaded: move |event: FormEvent| {
                                                        spawn(async move {
                                                            #[cfg(feature = "web")]
                                                            if let Some(file_engine) = event.files() {
                                                                let result = handle_file_upload(file_engine, api).await;
                                                                if !result.is_empty() {
                                                                    if let Some(url) = result[0].url.as_ref() {
                                                                        ctrl.e_learning_file_url.set(url.clone());
                                                                    }
                                                                } else {
                                                                    btracing::e!(lang, ApiError::DeliberationResourceException);
                                                                }
                                                            }
                                                        });
                                                    },
                                                }

                                                div { class: "flex min-w-[165px] h-[40px] border border-[#2a60d3] bg-white rounded-sm text-[#2a60d3] text-center font-semibold text-sm justify-center items-center",
                                                    {tr.load_from}
                                                }

                                                input {
                                                    class: "flex flex-row w-full justify-start items-center bg-transparent text-disabled focus:outline-none",
                                                    r#type: "text",
                                                    placeholder: tr.sub_placeholder2,
                                                    readonly: true,
                                                    value: ctrl.get_file_name(),
                                                }
                                            }
                                            p { class: "text-text-gray text-start w-full text-sm font-normal",
                                                {tr.e_learning_desc}
                                            }
                                        }
                                    }
                                }
                            } else {
                                // FIXME: arrangement row @henry

                                //select box
                                select {
                                    class: "focus:outline-none w-[215px] h-[55px] justify-start items-start p-[15px] bg-[#f7f7f7] rounded-[4px] mr-[20px] font-medium text-[15px] text-[#b4b4b4]",
                                    value: match ctrl.select_field() {
                                        Some(v) => format!("{}", v),
                                        None => "".to_string(),
                                    },
                                                                // FIXME: which type should be used here?
                                // onchange: move |e: Event<FormData>| {
                                //     let v = match ProjectArea::from_str(e.value().as_str()) {
                                //         Ok(v) => v,
                                //         Err(_) => return,
                                //     };
                                //     ctrl.select_field.set(Some(v));
                                // },
                                // option {
                                //     value: "",
                                //     disabled: true,
                                //     selected: ctrl.select_field() == None,
                                //     {translate.select_field}
                                // }
                                // for field in ProjectArea::VARIANTS.iter() {
                                //     option {
                                //         value: format!("{}", field).as_str(),
                                //         selected: Some(field) == ctrl.select_field().as_ref(),
                                //         {field.translate(&lang)}
                                //     }
                                // }
                                }

                                //input evaluation title
                                input {
                                    class: "flex flex-row w-full h-[55px] justify-start items-center bg-[#f7f7f7] aria-active:!bg-white aria-active:!border aria-active:!border-active focus:outline-none px-[15px] py-[10px] font-medium text-[#b4b4b4] text-[15px] leading-[22px] rounded-[4px] mr-[10px]",
                                    "aria-active": ctrl.is_focusing_eval(),
                                    r#type: "text",
                                    placeholder: tr.title_placeholder.to_string(),
                                    value: ctrl.title(),
                                    onfocus: move |_| ctrl.is_focusing_eval.set(true),
                                    onblur: move |_| ctrl.is_focusing_eval.set(false),
                                    oninput: move |e: Event<FormData>| ctrl.eval_title.set(e.value()),
                                }

                                input {
                                    class: "flex flex-row w-full h-[55px] justify-start items-center bg-white focus:outline-none border-b-[1px] border-[#bfc8d9] px-[15px] py-[15px] font-medium text-[#b4b4b4] text-[15px] leading-[22px]",
                                    r#type: "text",
                                    placeholder: tr.content_placeholder.to_string(),
                                    value: ctrl.content(),
                                    oninput: move |e: Event<FormData>| ctrl.eval_content.set(e.value()),
                                }
                            }
                        }
                    
                    // 필수입력, 삭제 우측정렬 @henry
                    }
                }
                // + 추가 버튼 @henry

                div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {},
                        {tr.backward}
                    }
                    div {
                        class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {},
                        {tr.temporary_save}
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                        onclick: move |_| {},
                        {tr.next}
                    }
                }
            }
        }
    }
}
