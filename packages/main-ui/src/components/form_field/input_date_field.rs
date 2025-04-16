use super::InputField;
use crate::components::select_date::SelectDate;
use bdk::prelude::*;

#[component]
pub fn InputDateField(
    #[props(default = 54)] height: i64,
    #[props(default = "inputfield".to_string())] name: String,
    placeholder: String,
    text_value: String,
    started_at: i64,
    ended_at: i64,
    oninput: EventHandler<FormEvent>,
    onupdate_start_date: EventHandler<i64>,
    onupdate_end_date: EventHandler<i64>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center gap-20",
            InputField {
                height,
                name,
                placeholder,
                value: text_value,
                oninput: move |e: FormEvent| {
                    oninput.call(e);
                },
            }
            div { class: "flex flex-row items-center gap-10",
                SelectDate {
                    date: started_at,
                    onupdate: move |timestamp: i64| {
                        onupdate_start_date.call(timestamp);
                    },
                }
                div { class: "flex flex-row w-16 h-2 bg-label-border-gray" }
                SelectDate {
                    date: ended_at,
                    onupdate: move |timestamp: i64| {
                        onupdate_end_date.call(timestamp);
                    },
                }
            }
        }
    }
}
