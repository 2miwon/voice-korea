use bdk::prelude::*;
use by_components::rich_texts::RichText;
use models::DeliberationBasicInfoCreateRequest;

use crate::{
    components::{
        form_field::{Divide, InputDateField},
        section::MainSection,
    },
    pages::deliberations::new::details::basic_info::i18n::BasicInfoIntroductionTranslate,
};

#[component]
pub fn Introduction(
    lang: Language,
    basic_info: DeliberationBasicInfoCreateRequest,
    start_date_id: String,
    end_date_id: String,
    set_title: EventHandler<String>,
    set_description: EventHandler<String>,
    set_start_date: EventHandler<i64>,
    set_end_date: EventHandler<i64>,
) -> Element {
    let tr: BasicInfoIntroductionTranslate = translate(&lang);
    rsx! {
        MainSection {
            lang,
            required: true,
            header: Some(tr.input_introduction_title.to_string()),
            description: Some(tr.input_introduction_description.to_string()),
            open: Some(true),
            InputDateField {
                start_date_id,
                end_date_id,
                placeholder: tr.input_title_hint.to_string(),
                text_value: basic_info.title,
                started_at: basic_info.started_at,
                ended_at: basic_info.ended_at,
                oninput: move |e: Event<FormData>| {
                    set_title.call(e.value());
                },
                onupdate_start_date: move |timestamp: i64| {
                    set_start_date.call(timestamp);
                },
                onupdate_end_date: move |timestamp: i64| {
                    set_end_date.call(timestamp);
                },
            }
            Divide {}
            RichText {
                id: "introduction-rich-text",
                content: basic_info.description,
                onchange: move |e| {
                    set_description.call(e);
                },
            }
        }
    }
}
