use bdk::prelude::*;
use models::Tab;

use crate::{
    components::icons::right_arrow::RightArrow,
    pages::projects::_id::components::{
        sample_survey::i18n::SampleSurveyTranslate, tab_title::TabTitle,
    },
    utils::time::formatted_timestamp,
};

#[component]
pub fn Submitted(
    lang: Language,
    start_date: i64,
    end_date: i64,
    on_response_click: EventHandler<MouseEvent>,
    on_statistic_click: EventHandler<MouseEvent>,
) -> Element {
    let tr: SampleSurveyTranslate = translate(&lang);

    rsx! {
        div { class: "w-full flex flex-col justify-center items-center",
            TabTitle { title: Tab::SampleSurvey.translate(&lang),
                div { class: "font-medium text-[15px] text-black",
                    {
                        format!(
                            "{} ~ {}",
                            formatted_timestamp(lang, start_date),
                            formatted_timestamp(lang, end_date),
                        )
                    }
                }
            }
            div { class: "w-full flex flex-col justify-center items-center gap-10 mb-40",
                LinkComponent {
                    lang,
                    label: tr.my_answer,
                    link_label: tr.see_detail,
                    onclick: move |e: Event<MouseData>| {
                        on_response_click.call(e);
                    },
                }
                LinkComponent {
                    lang,
                    label: tr.response_per_question,
                    link_label: tr.see_detail,
                    onclick: move |e: Event<MouseData>| {
                        on_statistic_click.call(e);
                    },
                }
            }
        }
    }
}

#[component]
pub fn LinkComponent(
    lang: Language,
    label: String,
    link_label: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center px-20 py-9 bg-white rounded-lg",
            div { class: "font-bold text-base text-text-black", {label} }
            div { class: "flex flex-row justify-start items-center gap-5",
                div {
                    class: "cursor-pointer font-semibold text-optional-blue text-sm underline",
                    onclick: move |e: Event<MouseData>| {
                        onclick.call(e);
                    },
                    {link_label}
                }
                RightArrow { class: "[&>stroke]:stroke-text-gray" }
            }
        }
    }
}
