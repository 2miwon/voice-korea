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
use models::DeliberationStatus;

#[component]
pub fn DeliberationPage(lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;
    let translates: OpinionTranslate = translate(&lang);
    let deliberations = ctrl.get_deliberations();
    let mut is_focused = use_signal(|| false);

    let mut search_keyword = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.public_opinion_management}"
            }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
                "{translates.public_opinion_management}"
            }
            div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
                "{translates.public_opinion_info}"
            }

            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg p-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
                    div {
                        class: format!(
                            "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                            if (is_focused)() {
                                "bg-[#ffffff] border border-[#2a60d3]"
                            } else {
                                "bg-[#f7f7f7] border border-[#7c8292]"
                            },
                        ),
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
                    div { class: "flex flex-row gap-[10px]",
                        Link { to: Route::DeliberationNewPage { lang },
                            div { class: "flex flex-row w-[130px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md gap-[5px]",
                                div {
                                    class: "text-white font-semibold text-[16px]",
                                    onclick: move |_| {},
                                    "{translates.start_public_opinion}"
                                }
                            }
                        }
                    }
                }
                //table section
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9] mb-[30px]",
                    //header
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-davy-gray font-semibold text-[14px]",
                                "{translates.field}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-davy-gray font-semibold text-[14px]",
                                "{translates.project}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-davy-gray font-semibold text-[14px]",
                                "{translates.response_rate}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-davy-gray font-semibold text-[14px]",
                                "{translates.panel}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-davy-gray font-semibold text-[14px]",
                                "{translates.period}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-davy-gray font-semibold text-[14px]",
                                "{translates.status}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-davy-gray font-semibold text-[14px]",
                                "{translates.view}"
                            }
                        }
                        div { class: "w-[90px] h-full justify-center items-center gap-[10px]" }
                    }

                    //data
                    for deliberation in deliberations {
                        div { class: "flex flex-row w-full min-h-[55px] justify-start items-center",
                            div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                div { class: "text-davy-gray font-semibold text-[14px]",
                                    {deliberation.project_area.translate(&lang)}
                                }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                div { class: "text-davy-gray font-semibold text-[14px]",
                                    "{deliberation.title}"
                                }
                            }
                            //FIXME: fix to real response data
                            div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                div { class: "text-davy-gray font-semibold text-[14px]",
                                    "0% (0/0)"
                                }
                            }
                            div { class: "flex flex-wrap flex-1 h-full justify-center items-center gap-[5px]",
                                for panel in deliberation.panels {
                                    PanelLabel { label: panel.name.clone() }
                                }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                div { class: "font-semibold text-[14px] text-[#222222] text-center",
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
                            div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                div { class: "font-semibold text-[14px] text-[#222222] text-center",
                                    {deliberation.status.translate(&lang)}
                                }
                            }
                            div { class: "cursor-pointer flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                if deliberation.status == DeliberationStatus::Finish {
                                    div { class: "font-semibold text-[14px] text-[#2A60D3] text-center",
                                        "{translates.view_result}"
                                    }
                                } else {
                                    div { class: "font-semibold text-[14px] text-[#2A60D3] text-center",
                                        "{translates.view_more}"
                                    }
                                }
                            }
                            div { class: "cursor-pointer flex flex-row w-[90px] h-full justify-center items-center",
                                RowOption { width: "24", height: "24" }
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
        div { class: "flex flex-row h-[25px] justify-center items-center px-[8px] py-[3px] bg-[#35343f] rounded-[40px] font-semibold text-[14px] text-white",
            {label}
        }
    }
}
