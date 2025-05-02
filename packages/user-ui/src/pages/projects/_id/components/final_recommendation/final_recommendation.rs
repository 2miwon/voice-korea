use bdk::prelude::*;
use models::Tab;

use crate::components::button::Button;
use crate::pages::projects::_id::components::final_recommendation::rich_text_edit::RichTextEdit;
use crate::pages::projects::_id::components::{survey::Statistics, tab_title::TabTitle};

use super::super::{accordion::Accordion, rich_text_viewer::RichTextViewer, section::Section};

use super::controllers::Controller;
use super::i18n::FinalRecommendationTranslate;

#[component]
pub fn FinalRecommendation(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    children: Element,
) -> Element {
    let tr: FinalRecommendationTranslate = translate(&lang);
    let mut ctrl = Controller::new(lang, project_id)?;

    let recommendation = ctrl.recommendation()?;
    let tab_title: &str = Tab::FinalDraft.translate(&lang);

    let mut title = use_signal(|| recommendation.title.clone());
    let mut description = use_signal(|| recommendation.description.clone());
    let mut update_clicked = use_signal(|| false);

    rsx! {
        Section { id: "final-recommendation",
            TabTitle { title: tab_title,
                div { class: "text-black",
                    Button {
                        class: "flex flex-row px-15 py-13 rounded-lg text-white text-base disabled:hidden",
                        disabled: update_clicked() || !recommendation.is_member,
                        onclick: move |_e| {
                            update_clicked.set(true);
                        },
                        {tr.update}
                    }
                }
            }

            RichTextEdit {
                lang,
                title,
                description,
                visibility: update_clicked(),

                on_title_change: move |t: String| {
                    title.set(t);
                },
                on_description_change: move |d: String| {
                    description.set(d);
                },

                onupdate: move |_| async move {
                    let _ = ctrl.upsert_recommendation(title(), description()).await;
                    title.set("".to_string());
                    description.set("".to_string());
                    update_clicked.set(false);
                },
            }

            div { class: "flex flex-col gap-10",
                Accordion { title: recommendation.title, default_open: true,
                    RichTextViewer {
                        class: "text-left",
                        contenteditable: false,
                        html: recommendation.description,
                    }
                    div { class: "w-full mt-20 flex max-[700px]:flex-col max-[700px]:gap-10 flex-row justify-start gap-40",
                        div { class: "w-full flex flex-row justify-start gap-20" }
                    }
                }
            }

            div { class: "flex flex-col w-full gap-20",
                //chart section
                Statistics { lang, grouped_answers: ctrl.get_grouped_responses() }
            }
        }
    }
}
