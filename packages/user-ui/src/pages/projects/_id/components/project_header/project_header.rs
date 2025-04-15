use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{deliberation_project::DeliberationProject, DeliberationStatus};

use super::i18n::HeaderTranslate;
use crate::{
    components::{
        icons::{
            adopted::Adopted, in_progress::InProgress, right_arrow::RightArrow, waiting::Waiting,
        },
        label::Label,
    },
    utils::time::formatted_timestamp,
};

use super::super::super::page::Tab;

#[component]
pub fn ProjectHeader(
    lang: Language,
    deliberation: DeliberationProject,
    active_tab: Signal<Tab>,
) -> Element {
    let tr: HeaderTranslate = translate(&lang);

    tracing::debug!("active_tab_value: {:?}", deliberation);
    let started_at = formatted_timestamp(lang, deliberation.started_at);
    let ended_at = formatted_timestamp(lang, deliberation.ended_at);

    rsx! {
        div { class: " max-w-1300 h-fit mb-40 flex max-[1000px]:flex-col-reverse flex-row w-full justify-center items-center gap-40 px-10",
            // TODO: connect to data and UI
            //data section
            div { class: "w-full max-w-720 h-260 flex flex-col justify-center",
                div { class: "flex flex-col justify-start",
                    div { class: "w-full flex justify-start items-center font-medium text-lg/24 gap-8 h-fit",
                        div { class: "w-24 h-24",
                            match deliberation.status {
                                DeliberationStatus::Ready => rsx! {
                                    Waiting {}
                                },
                                DeliberationStatus::InProgress => rsx! {
                                    InProgress {}
                                },
                                DeliberationStatus::Finish => rsx! {
                                    Adopted {}
                                },
                            }
                        }

                        span { "{started_at} ~ {ended_at}" }
                    }
                    div { class: "w-full flex justify-start items-center font-semibold text-[32px]/60",
                        "{deliberation.title}"
                    }
                    div { class: "w-full flex justify-start items-center font-md text-sm gap-4",
                        for area in deliberation.project_areas.iter() {
                            Label { name: area.project_area.translate(&lang) }
                        }
                    }

                    // FIXME: add organization
                    // div { class: "w-full my-20 flex flex-row justify-start items-center gap-8",
                    //     img {
                    //         class: "w-50 h-50",
                    //         src: asset!("/public/images/organization.png"),
                    //     }
                    //     div {
                    //         div { class: "flex justify-start items-center font-normal text-[15px]",
                    //             "{tr.organization}"
                    //         }
                    //     }
                    // }
                    div { class: "flex flex-row justify-start items-center gap-60",
                        div { class: "w-hug h-59 flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "{deliberation.participants}"
                            }
                            div { class: "justify-center items-center font-md text-sm",
                                "{tr.participant}"
                            }
                        }
                        div { class: "w-hug h-59 flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "{deliberation.votes}"
                            }
                            div { class: "justify-center items-center font-md text-sm",
                                "{tr.vote}"
                            }
                        }
                    }
                }
            }
            //img section
            div { class: "block",
                img {
                    class: "w-540 h-300 rounded-xl bg-gray-100 object-cover",
                    src: deliberation.thumbnail_image,
                    alt: "Project Thumbnail Image",
                
                }
            }
        }
        //menu
        div { class: "flex flex-col w-full justify-center items-center bg-box-gray whitespace-nowrap",
            div { class: "flex flex-col w-full",
                // Tab menu
                div { class: "group w-full flex flex-row justify-between gap-10 items-center overflow-x-auto max-[1300px]:no-scrollbar [&>:last-child]:hidden",
                    for tab in Tab::VARIANTS.iter() {
                        div {
                            class: format!(
                                "flex flex-col items-center flex-1 border-b-2 cursor-pointer {}",
                                if active_tab() == *tab { "border-button-primary" } else { "border-transparent" },
                            ),
                            onclick: move |_| {
                                active_tab.set(*tab);
                            },
                            p { class: "font-md text-[15px]/22 py-10", {tab.translate(&lang)} }
                        }
                        RightArrow { color: "#B4B4B4" }
                    }
                }

                // line
                div { class: "w-full h-1 bg-line-gray" }
            }
        }
    }
}
