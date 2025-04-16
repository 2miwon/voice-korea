use crate::{
    components::{
        form_field::{Divide, SelectInputField, UnderlineField},
        section::MainSection,
    },
    pages::deliberations::new::details::deliberation::i18n::DeliberationTranslate,
};
use bdk::prelude::*;
use models::Question;

#[component]
pub fn Evaluation(
    lang: Language,
    questions: Vec<Option<Question>>,
    set_form: EventHandler<(usize, String)>,
    set_title: EventHandler<(usize, String)>,
    set_description: EventHandler<(usize, String)>,
    removing_question: EventHandler<usize>,
) -> Element {
    let tr: DeliberationTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col gap-20 w-full",
            for (index , question) in questions.iter().enumerate() {
                MainSection {
                    lang,
                    header: None,
                    description: None,
                    ondelete: move |_| {
                        removing_question.call(index);
                    },
                    SelectInputField {
                        name: format!("evaluation-{index}"),
                        selected_field: if let Some(ref question) = question { Some(question.to_type(&lang).to_string()) } else { None },
                        select_placeholder: tr.select_format.to_string(),
                        placeholder: tr.title_placeholder.to_string(),
                        text_value: if let Some(ref question) = question { question.title() } else { "".to_string() },
                        onchange: move |e: Event<FormData>| {
                            set_form.call((index, e.value()));
                        },
                        oninput: move |e: FormEvent| {
                            set_title.call((index, e.value()));
                        },
                        options: rsx! {
                            for question_type in Question::types(&lang).iter() {
                                option { value: question_type.clone(), {question_type.clone()} }
                            }
                        },
                    }
                    Divide {}
                    UnderlineField {
                        placeholder: tr.content_placeholder.to_string(),
                        value: if let Some(ref question) = question { question.description() } else { "".to_string() },
                        oninput: move |e: Event<FormData>| {
                            set_description.call((index, e.value()));
                        },
                    }
                }
            }
        }
    }
}
