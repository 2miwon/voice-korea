use bdk::prelude::*;
use by_components::rich_texts::RichText;
use models::DeliberationFinalSurveyCreateRequest;

use crate::{
    components::{
        form_field::{Divide, InputDateField},
        section::MainSection,
    },
    pages::deliberations::new::details::votes::i18n::IntroductionTranslate,
};

#[component]
pub fn Introduction(
    lang: Language,

    final_survey: DeliberationFinalSurveyCreateRequest,
    start_date_id: String,
    end_date_id: String,
    set_title: EventHandler<String>,
    set_description: EventHandler<String>,
    set_start_date: EventHandler<i64>,
    set_end_date: EventHandler<i64>,
) -> Element {
    let tr: IntroductionTranslate = translate(&lang);

    rsx! {
        MainSection {
            lang,
            required: true,
            header: Some(tr.input_introduction_title.to_string()),
            description: Some(tr.input_introduction_description.to_string()),
            open: Some(true),
            div { class: "flex flex-col w-full h-fit gap-10",
                InputDateField {
                    start_date_id,
                    end_date_id,
                    placeholder: tr.input_title_hint.to_string(),
                    text_value: final_survey.title,
                    started_at: final_survey.started_at,
                    ended_at: final_survey.ended_at,
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
                    id: "final-survey-rich-text",
                    content: final_survey.description,
                    onchange: move |e| {
                        set_description.call(e);
                    },
                }
            }
        }
    }
}
