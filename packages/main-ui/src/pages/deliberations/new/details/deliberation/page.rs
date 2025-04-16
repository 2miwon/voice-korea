#![allow(unused_variables)]
use super::*;
use crate::{
    components::{icons::ArrowLeft, section::AddSection},
    pages::deliberations::new::details::deliberation::components::{
        elearning::DeliberationElearning, evaluation::Evaluation, introduction::Introduction,
        member::DeliberationMember,
    },
    service::metadata_api::MetadataApi,
};
use bdk::prelude::*;
use controller::*;
use i18n::*;
use models::File;

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
            div { class: "flex flex-col w-full justify-start items-start gap-20",
                div { class: "font-medium text-base text-text-black", {tr.post_setting} }
                Introduction {
                    lang,
                    deliberation: ctrl.deliberation(),
                    set_title: move |title: String| {
                        ctrl.set_title(title);
                    },
                    set_description: move |description: String| {
                        ctrl.set_description(description);
                    },
                    set_start_date: move |timestamp: i64| {
                        ctrl.set_start_date(timestamp);
                    },
                    set_end_date: move |timestamp: i64| {
                        ctrl.set_end_date(timestamp);
                    },
                }

                DeliberationMember {
                    lang,
                    deliberation: ctrl.deliberation(),

                    total_committees: ctrl.get_committees(),
                    selected_committees: ctrl.get_selected_committee(),

                    add_committee: move |user_id: i64| {
                        ctrl.add_committee(user_id);
                    },
                    remove_committee: move |user_id: i64| {
                        ctrl.remove_committee(user_id);
                    },
                    clear_committee: move |_| {
                        ctrl.clear_committee();
                    },
                }

                div { class: "flex flex-col w-full justify-start items-start gap-10 mt-20",
                    div { class: "flex flex-row w-full justify-start items-center gap-10",
                        div {
                            class: "flex items-center justify-center w-197 h-46 bg-primary-deep aria-active:!bg-white rounded-[100px] cursor-pointer",
                            "aria-active": ctrl.e_learning_tab(),
                            onclick: move |_| ctrl.e_learning_tab.set(true),
                            p { class: "text-text-black font-bold text-lg", {tr.e_learning_setting} }
                        }
                        div {
                            class: "flex items-center justify-center w-139 h-46 bg-primary-deep aria-active:!bg-white rounded-[100px] cursor-pointer",
                            "aria-active": !ctrl.e_learning_tab(),
                            onclick: move |_| ctrl.e_learning_tab.set(false),
                            p { class: "text-text-black font-bold text-lg", {tr.evaluation_setting} }
                        }
                    }

                    if ctrl.e_learning_tab() {
                        DeliberationElearning {
                            lang,
                            elearnings: ctrl.deliberation().elearnings,
                            set_elearning_necessary: move |(index, necessary): (usize, bool)| {
                                ctrl.set_elearning_necessary(index, necessary);
                            },
                            set_elearning_title: move |(index, title): (usize, String)| {
                                ctrl.set_elearning_title(index, title);
                            },
                            set_elearning_metadata: move |(index, file): (usize, File)| async move {
                                ctrl.set_elearning_metadata(index, file).await;
                            },
                            add_elearning: move |_| {
                                ctrl.add_elearning();
                            },
                            remove_elearning: move |index: usize| {
                                ctrl.remove_elearning(index);
                            },
                        }
                    } else {
                        Evaluation {
                            lang,
                            evaluations: vec![], // FIXME: this is dummy data
                            // selected_field: ctrl.selected_field(),
                            set_form: move |(index, field): (usize, String)| {
                                ctrl.set_selected_field(index, field);
                            },
                            set_title: move |(index, title): (usize, String)| {
                                ctrl.set_evaluation_title(index, title);
                            },
                            set_content: move |(index, content): (usize, String)| {
                                ctrl.set_evaluation_content(index, content);
                            },
                            removing_evaluation: move |index: usize| {
                                ctrl.remove_evaluation(index);
                            },
                        }
                    }

                    AddSection {
                        lang,
                        onclick: move |e| {
                            if ctrl.e_learning_tab() {
                                ctrl.add_elearning();
                            } else {
                                ctrl.add_evaluation();
                            }
                        },
                    }
                }

                div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {
                            ctrl.back();
                        },
                        {tr.backward}
                    }
                    div {
                        class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| async move {
                            ctrl.temp_save().await;
                        },
                        {tr.temporary_save}
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                        onclick: move |_| {
                            ctrl.next();
                        },
                        {tr.next}
                    }
                }
            }
        }
    }
}
