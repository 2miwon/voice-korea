use crate::{
    components::{
        form_field::{Divide, SelectInputField, UnderlineField},
        section::MainSection,
    },
    pages::deliberations::new::details::deliberation::i18n::DeliberationTranslate,
};
use bdk::prelude::*;
// use models::DeliberationContentCreateRequest;

#[component]
pub fn Evaluation(
    lang: Language,
    evaluations: Vec<String>, // FIXME: This should be a Vec<DeliberationContentCreateRequest> or a more specific type.
    set_form: EventHandler<(usize, String)>,
    set_title: EventHandler<(usize, String)>,
    set_content: EventHandler<(usize, String)>,
    removing_evaluation: EventHandler<usize>,
) -> Element {
    let tr: DeliberationTranslate = translate(&lang);
    rsx! {
        div { class: "flex w-full",
            for (index , _evaluation) in evaluations.iter().enumerate() {
                MainSection {
                    lang,
                    header: None,
                    description: None,
                    ondelete: move |_| {
                        removing_evaluation.call(index);
                    },
                    SelectInputField {
                        name: format!("evaluation-{index}"),
                        selected_field: "evaluation".to_string(), // FIXME: this is dummy_data
                        select_placeholder: tr.select_format.to_string(),
                        placeholder: tr.title_placeholder.to_string(),
                        text_value: "".to_string(), // FIXME: This is a temporary solution. We need to implement a proper translation system.
                        onchange: move |e: Event<FormData>| {
                            set_form.call((index, e.value()));
                        },
                        oninput: move |e: FormEvent| {
                            set_title.call((index, e.value()));
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
                            set_content.call((index, e.value()));
                        },
                    }
                }
            }
        }
    }
}
