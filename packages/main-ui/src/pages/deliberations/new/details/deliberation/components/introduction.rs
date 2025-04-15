use bdk::prelude::*;
use models::DeliberationContentCreateRequest;

use crate::{
    components::{
        form_field::{Divide, InputDateField, UnderlineField},
        section::MainSection,
    },
    pages::deliberations::new::details::deliberation::i18n::DeliberationTranslate,
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
    let tr: DeliberationTranslate = translate(&lang);
    rsx! {
        MainSection {
            lang,
            required: true,
            header: Some(tr.main_section1_title.to_string()),
            description: Some(tr.main_section1_description.to_string()),
            open: Some(true),
            InputDateField {
                placeholder: tr.title_placeholder.to_string(),
                text_value: deliberation.title,
                started_at: deliberation.started_at,
                ended_at: deliberation.ended_at,
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
            UnderlineField {
                placeholder: tr.content_placeholder.to_string(),
                value: deliberation.description,
                oninput: move |e: Event<FormData>| {
                    set_description.call(e.value());
                },
            }
        }
    }
}
