use crate::{
    components::{
        form_field::{Divide, SelectInputField},
        section::{AddSection, MainSection},
    },
    pages::deliberations::new::details::deliberation::{
        components::objective_options::ObjectiveOptions, i18n::DeliberationTranslate,
    },
};
use bdk::prelude::*;
use models::Question;

#[component]
pub fn Evaluation(
    lang: Language,
    questions: Vec<Question>,
    set_form: EventHandler<(usize, String)>,
    set_title: EventHandler<(usize, String)>,
    set_description: EventHandler<(usize, String)>,
    add_question: EventHandler<MouseEvent>,
    removing_question: EventHandler<usize>,

    change_option: EventHandler<(usize, usize, String)>,
    remove_option: EventHandler<(usize, usize)>,
    add_option: EventHandler<usize>,
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
                        selected_field: Some(question.to_type(&lang).to_string()),
                        select_placeholder: tr.select_format.to_string(),
                        placeholder: tr.title_placeholder.to_string(),
                        text_value: question.title(),
                        onchange: move |e: Event<FormData>| {
                            set_form.call((index, e.value()));
                        },
                        oninput: move |e: FormEvent| {
                            set_title.call((index, e.value()));
                        },
                        options: Question::types(&lang),
                    }
                    Divide {}
                    // UnderlineField {
                    //     placeholder: tr.content_placeholder.to_string(),
                    //     value: question.description(),
                    //     oninput: move |e: Event<FormData>| {
                    //         set_description.call((index, e.value()));
                    //     },
                    // }

                    match question {
                        Question::SingleChoice(_) | Question::MultipleChoice(_) => {
                            rsx! {
                                ObjectiveOptions {
                                    lang,
                                    options: question.clone().options(),
                                    change_option: move |(ind, option): (usize, String)| {
                                        change_option.call((index, ind, option));
                                    },
                                    remove_option: move |ind: usize| {
                                        remove_option.call((index, ind));
                                    },
                                    add_option: move |_| {
                                        add_option.call(index);
                                    },
                                }
                            }
                        }
                        _ => rsx! {},
                    }
                }
            }
            AddSection {
                lang,
                onclick: move |e| {
                    add_question.call(e);
                },
            }
        }
    }
}
