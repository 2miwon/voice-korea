use bdk::prelude::*;
use models::DeliberationContentCreateRequest;

use crate::{
    components::{calendar::Calendar, icons::CalendarIcon, section::MainSection},
    pages::deliberations::new::details::deliberation::i18n::DeliberationTranslate,
    utils::time::change_date_from_timestamp,
};

#[component]
pub fn Introduction(
    lang: Language,
    deliberation: DeliberationContentCreateRequest,
    set_title: EventHandler<String>,
    set_description: EventHandler<String>,
    set_start_date: EventHandler<i64>,
    set_end_date: EventHandler<i64>,
) -> Element {
    let mut is_focusing_title: Signal<bool> = use_signal(|| false);
    let tr: DeliberationTranslate = translate(&lang);
    rsx! {
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
                        "aria-active": is_focusing_title(),
                        r#type: "text",
                        placeholder: tr.title_placeholder.to_string(),
                        value: deliberation.title,
                        onfocus: move |_| {
                            is_focusing_title.set(true);
                        },
                        onblur: move |_| {
                            is_focusing_title.set(false);
                        },
                        oninput: move |e: Event<FormData>| {
                            set_title.call(e.value());
                        },
                    }

                    // start date
                    div { class: "group relative",
                        button { class: "flex flex-row w-[190px] focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px]",
                            div { class: "font-normal text-[16px] text-[#9b9b9b] leading-[24px]",
                                {change_date_from_timestamp(deliberation.started_at)}
                            }
                            CalendarIcon { width: "28", height: "28" }
                        }
                        nav { class: "invisible border-none rounded w-full absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                            Calendar {
                                timestamp: Some(deliberation.started_at as u64),
                                update_date: move |timestamp: i64| {
                                    set_start_date.call(timestamp);
                                },
                            }
                        }
                    }

                    div { class: "flex flex-row w-[16px] h-[2px] bg-[#bfc8d9] mx-[10px]" }

                    // end date
                    div { class: "group relative w-[450px]",
                        button { class: "flex flex-row w-[190px]  focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px]",
                            div { class: "font-normal text-[16px] text-[#9b9b9b] leading-[24px]",
                                {change_date_from_timestamp(deliberation.ended_at)}
                            }
                            CalendarIcon { width: "28", height: "28" }
                        }
                        nav { class: "invisible border-none rounded w-full absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                            Calendar {
                                timestamp: Some(deliberation.ended_at as u64),
                                update_date: move |timestamp: i64| {
                                    set_end_date.call(timestamp);
                                },
                            }
                        }
                    }
                }

                input {
                    class: "flex flex-row w-full h-[55px] justify-start items-center bg-white focus:outline-none border-b-[1px] border-[#bfc8d9] px-[15px] py-[15px] font-medium text-[#b4b4b4] text-[15px] leading-[22px]",
                    r#type: "text",
                    placeholder: tr.content_placeholder.to_string(),
                    value: deliberation.description,
                    oninput: move |e: Event<FormData>| {
                        set_description.call(e.value());
                    },
                }
            }
        }
    }
}
