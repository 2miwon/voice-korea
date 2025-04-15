use crate::{
    components::{
        form_field::{Divide, SelectInputField, UnderlineField},
        section::MainSection,
    },
    pages::deliberations::new::details::deliberation::i18n::DeliberationTranslate,
};
use bdk::prelude::*;
use models::DeliberationContentCreateRequest;

#[component]
pub fn Evaluation(
    lang: Language,
    deliberation: DeliberationContentCreateRequest,
    selected_field: Option<String>,
    set_form: EventHandler<String>,
    set_title: EventHandler<String>,
    set_content: EventHandler<String>,
) -> Element {
    let tr: DeliberationTranslate = translate(&lang);
    rsx! {
        div { class: "flex w-full",
            MainSection { lang, header: None, description: None,
                SelectInputField {
                    name: "evaluation_title".to_string(),
                    selected_field,
                    select_placeholder: tr.select_format.to_string(),
                    placeholder: tr.title_placeholder.to_string(),
                    text_value: "".to_string(), // FIXME: This is a temporary solution. We need to implement a proper translation system.
                    onchange: move |e: Event<FormData>| {
                        set_form.call(e.value());
                    },
                    oninput: move |e: FormEvent| {
                        set_title.call(e.value());
                    },
                    options: rsx! {
                        // FIXME: This is a temporary solution. We need to implement a proper translation system.
                        option { value: "evaluation".to_string(), {"evaluation".to_string()} }
                        option { value: "evaluation_2".to_string(), {"evaluation_2".to_string()} }
                        option { value: "evaluation_3".to_string(), {"evaluation_3".to_string()} }
                        option { value: "evaluation_4".to_string(), {"evaluation_4".to_string()} }
                    },
                }
                Divide {}
                UnderlineField {
                    placeholder: tr.content_placeholder.to_string(),
                    value: "".to_string(), // FIXME: This is a temporary solution. We need to implement a proper translation system.
                    oninput: move |e: Event<FormData>| {
                        set_content.call(e.value());
                    },
                }
            }
        }
    }
}
