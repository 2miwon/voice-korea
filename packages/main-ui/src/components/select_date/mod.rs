use bdk::prelude::*;

use crate::{
    components::{calendar::Calendar, icons::CalendarIcon},
    utils::time::change_date_from_timestamp,
};

#[component]
pub fn SelectDate(date: i64, onupdate: EventHandler<i64>) -> Element {
    rsx! {
        div { class: "flex",
            button { class: "flex flex-row w-190 focus:outline-none h-55 justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-20",
                div { class: "font-normal text-base/24 text-[#9b9b9b]",
                    {change_date_from_timestamp(date)}
                }
                CalendarIcon { width: "28", height: "28" }
            }
            nav { class: "invisible border-none rounded w-full absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                Calendar {
                    timestamp: Some(date as u64),
                    update_date: move |timestamp: i64| {
                        onupdate.call(timestamp);
                    },
                }
            }
        }
    }
}
