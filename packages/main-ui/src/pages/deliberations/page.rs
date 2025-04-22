use crate::{
    components::{
        icons::{RowOption, Search, Switch},
        pagination::Pagination,
    },
    routes::Route,
    utils::time::convert_timestamp_to_date,
};

use super::controller::Controller;
use super::i18n::OpinionTranslate;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{deliberation_response::DeliberationType, DeliberationStatus};

#[component]
pub fn DeliberationPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let translates: OpinionTranslate = translate(&lang);
    let deliberations = ctrl.deliberations()?.items;
    let mut is_focused = use_signal(|| false);

    let mut search_keyword = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "hidden aria-opened:!flex flex-col absolute fixed items-center justify-center bg-white w-239 z-50",
            style: "box-shadow: 0px 8px 20px 0px rgba(20, 26, 62, 0.25); left: {ctrl.get_x()}px; top: {ctrl.get_y()}px;",
            "aria-opened": ctrl.context_menu(),

            button {
                class: "w-full px-20 py-15 text-black cursor-pointer hover:!bg-neutral1",
                onclick: move |_| ctrl.handle_edit(),
                {translates.edit}
            }

            button {
                class: "w-full px-20 py-15 text-black cursor-pointer hover:!bg-neutral1",
                onclick: move |_| async move { ctrl.handle_remove().await },
                {translates.remove}
            }
        }

        div {
            class: "flex flex-col w-full justify-start items-start",
            onclick: move |_| {
                ctrl.context_menu.set(false);
            },
            div { class: "text-header-gray font-medium text-sm mb-10",
                "{translates.organization_management} / {translates.public_opinion_management}"
            }
            div { class: "text-header-black font-semibold text-[28px] mb-25",
                "{translates.public_opinion_management}"
            }
            div { class: "text-label-black font-normal text-sm mb-40",
                "{translates.public_opinion_info}"
            }

            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg p-20",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
                    div {
                        class: "flex flex-row w-590 h-45 justify-between items-center rounded-lg bg-background-gray border border-third aria-active:!bg-white aria-active:border-hover px-11 py-13",
                        "aria-active": (is_focused)(),
                        input {
                            class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: translates.search_hint,
                            value: (search_keyword)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            onkeypress: {
                                let mut ctrl = ctrl.clone();
                                move |e: KeyboardEvent| {
                                    let key = e.key();
                                    if key == Key::Enter {
                                        ctrl.search_keyword.set(search_keyword());
                                    }
                                }
                            },
                            oninput: {
                                let mut ctrl = ctrl.clone();
                                move |event: Event<FormData>| {
                                    search_keyword.set(event.value());
                                    ctrl.search_keyword.set(search_keyword());
                                }
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row gap-10",
                        Link { to: Route::DeliberationNewPage { lang },
                            div { class: "flex flex-row w-130 h-40 justify-center items-center bg-hover rounded-md gap-5",
                                div {
                                    class: "text-white font-semibold text-base",
                                    onclick: move |_| {},
                                    {translates.start_public_opinion}
                                }
                            }
                        }
                    }
                }
                //table section
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-label-border-gray mb-30",
                    //header
                    div { class: "flex flex-row w-full h-55 justify-start items-center",
                        div { class: "flex flex-row w-120 max-w-[120px] h-full justify-center items-center gap-10",
                            div { class: "!text-davy-gray font-semibold text-sm", {translates.field} }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                            div { class: "!text-davy-gray font-semibold text-sm",
                                {translates.project}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                            div { class: "!text-davy-gray font-semibold text-sm",
                                {translates.response_rate}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                            div { class: "!text-davy-gray font-semibold text-sm", {translates.panel} }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                            div { class: "!text-davy-gray font-semibold text-sm", {translates.period} }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-120 max-w-[120px] h-full justify-center items-center gap-10",
                            div { class: "!text-davy-gray font-semibold text-sm", {translates.status} }
                            Switch { width: "19", height: "19" }
                        }
                        // div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                        //     div { class: "!text-davy-gray font-semibold text-[14px]",
                        //         {translates.view}
                        //     }
                        // }
                        div { class: "w-90 h-full justify-center items-center gap-10" }
                    }

                    //data
                    div { class: "flex flex-col w-full gap-5",
                        for deliberation in deliberations {
                            div { class: "flex flex-row w-full min-h-[55px] justify-start items-center",
                                div { class: "flex flex-row w-120 min-w-120 h-full justify-center items-center",
                                    div { class: "!text-davy-gray font-semibold text-sm",
                                        {deliberation.project_area.translate(&lang)}
                                    }
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                    div { class: "!text-davy-gray font-semibold text-sm",
                                        {deliberation.title}
                                    }
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                    div { class: "!text-davy-gray font-semibold text-sm",
                                        {
                                            format!(
                                                "{:.0}% ({}/{})",
                                                if deliberation.emails.len() > 0 {
                                                    (deliberation
                                                        .responses
                                                        .iter()
                                                        .filter(|v| v.deliberation_type == DeliberationType::Survey)
                                                        .count() as f64 / deliberation.emails.len() as f64) * 100.0
                                                } else {
                                                    0.0
                                                },
                                                deliberation
                                                    .responses
                                                    .iter()
                                                    .filter(|v| v.deliberation_type == DeliberationType::Survey)
                                                    .count(),
                                                deliberation.emails.len(),
                                            )
                                        }
                                    }
                                }
                                div { class: "flex flex-wrap flex-1 h-full justify-center items-center gap-5",
                                    for email in deliberation.emails.clone().into_iter().take(5) {
                                        PanelLabel { label: email.email.clone() }
                                    }
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                    div { class: "font-semibold text-sm text-text-black text-center",
                                        {
                                            if deliberation.started_at > 0 && deliberation.ended_at > 0 {
                                                format!(
                                                    "{} ~ {}",
                                                    convert_timestamp_to_date(deliberation.started_at),
                                                    convert_timestamp_to_date(deliberation.ended_at),
                                                )
                                            } else {
                                                "".to_string()
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-120 max-w-[120px] h-full justify-center items-center",
                                    div { class: "font-semibold text-sm text-text-black text-center",
                                        {deliberation.status.translate(&lang)}
                                    }
                                }
                                // div { class: "cursor-pointer flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                //     if deliberation.status == DeliberationStatus::Finish {
                                //         div { class: "font-semibold text-[14px] text-[#2A60D3] text-center",
                                //             {translates.view_result}
                                //         }
                                //     } else {
                                //         div { class: "font-semibold text-[14px] text-[#2A60D3] text-center",
                                //             {translates.view_more}
                                //         }
                                //     }
                                // }
                                if deliberation.status == DeliberationStatus::Draft {
                                    div {
                                        class: "cursor-pointer flex flex-row w-90 h-full justify-center items-center",
                                        onclick: move |evt| ctrl.handle_click_menu(deliberation.id, evt),
                                        RowOption { width: "24", height: "24" }
                                    }
                                } else {
                                    div { class: "flex flex-row w-90 h-full justify-center items-center" }
                                }
                            }
                        }
                    }
                }


                Pagination {
                    total_page: if ctrl.size != 0 { ctrl.total_pages() } else { 0 },
                    current_page: ctrl.page(),
                    size: ctrl.size,
                    onclick: {
                        let mut ctrl = ctrl.clone();
                        move |page| {
                            ctrl.set_page(page);
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn PanelLabel(label: String) -> Element {
    rsx! {
        div { class: "flex flex-row h-25 justify-center items-center px-8 py-3 bg-label-black rounded-[40px] font-semibold text-sm text-white",
            {label}
        }
    }
}
