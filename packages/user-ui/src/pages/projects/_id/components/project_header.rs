use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{deliberation_project::DeliberationProject, DeliberationStatus, Tab};

use crate::{
    components::{
        icons::{
            adopted::Adopted, in_progress::InProgress, right_arrow::RightArrow, waiting::Waiting,
        },
        label::Label,
    },
    pages::projects::_id::components::i18n::HeaderTranslate,
    utils::time::formatted_timestamp,
};

#[component]
pub fn ProjectHeader(
    lang: Language,
    deliberation: DeliberationProject,
    active_tab: Signal<Tab>,
) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    let mut set_active_tab = move |value: Tab| active_tab.set(value);
    let active_tab_value = active_tab.read();
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
            div { class: "flex flex-col max-w-1300 w-full",

                // Tab menu
                div { class: "w-full h-42 flex flex-row justify-between items-center overflow-x-auto max-[1300px]:no-scrollbar",
                    for tab in Tab::all() {
                        div { class: "flex flex-col items-center min-w-160",
                            div {
                                class: "w-160 h-40 flex justify-center items-center font-md text-[15px] cursor-pointer",
                                class: if *active_tab_value == tab { " font-semibold" } else { "text-text-black" },
                                onclick: move |_| set_active_tab(tab),
                                p { {tab.translate(&lang)} }
                            }
                            div { class: if *active_tab_value == tab { "w-full h-2 bg-button-primary" } else { "w-full h-2 bg-transparent" } }
                        }
                        if tab != Tab::FinalDraft {
                            RightArrow { color: "#B4B4B4" }
                        }
                    }
                }

                // line
                div { class: "w-full h-1 bg-line-gray" }
            }
        }
    }
}
