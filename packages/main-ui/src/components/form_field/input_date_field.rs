use bdk::prelude::*;

use crate::{
    components::{calendar::Calendar, icons::CalendarIcon},
    utils::time::change_date_from_timestamp,
};

#[component]
pub fn InputDateField(
    #[props(default = 54)] height: i64,
    placeholder: String,
    text_value: String,
    started_at: i64,
    ended_at: i64,
    oninput: EventHandler<FormEvent>,
    onupdate_start_date: EventHandler<i64>,
    onupdate_end_date: EventHandler<i64>,
) -> Element {
    let mut is_focusing_title: Signal<bool> = use_signal(|| false);
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center gap-20",
            //input_title
            input {
                class: "flex flex-row w-full justify-start items-center bg-background-gray aria-active:!bg-white aria-active:!border aria-active:!border-active focus:outline-none px-15 py-10 font-medium text-disabled text-[15px]/22 rounded-[4px]",
                style: "height: {height}px",
                "aria-active": is_focusing_title(),
                r#type: "text",
                placeholder,
                value: text_value,
                onfocus: move |_| {
                    is_focusing_title.set(true);
                },
                onblur: move |_| {
                    is_focusing_title.set(false);
                },
                oninput,
            }
            div { class: "flex flex-row items-center",
                // start date
                div { class: "flex",
                    button { class: "flex flex-row w-[190px] focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px]",
                        div { class: "font-normal text-[16px] text-[#9b9b9b] leading-[24px]",
                            {change_date_from_timestamp(started_at)}
                        }
                        CalendarIcon { width: "28", height: "28" }
                    }
                    nav { class: "invisible border-none rounded w-full absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                        Calendar {
                            timestamp: Some(started_at as u64),
                            update_date: move |timestamp: i64| {
                                onupdate_start_date.call(timestamp);
                            },
                        }
                    }
                }

                div { class: "flex flex-row w-[16px] h-[2px] bg-[#bfc8d9] mx-[10px]" }

                // end date
                div { class: "flex",
                    button { class: "flex flex-row w-[190px] focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px]",
                        div { class: "font-normal text-[16px] text-[#9b9b9b] leading-[24px]",
                            {change_date_from_timestamp(ended_at)}
                        }
                        CalendarIcon { width: "28", height: "28" }
                    }
                    nav { class: "invisible border-none rounded w-full absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                        Calendar {
                            timestamp: Some(ended_at as u64),
                            update_date: move |timestamp: i64| {
                                onupdate_end_date.call(timestamp);
                            },
                        }
                    }
                }
            }
        }
    }
}
