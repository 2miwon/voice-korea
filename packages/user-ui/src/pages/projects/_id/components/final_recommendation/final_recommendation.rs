use bdk::prelude::*;
use models::Tab;

use crate::{
    components::AvatarLabel,
    pages::projects::_id::components::{survey::Statistics, tab_title::TabTitle},
    utils::time::formatted_timestamp,
};

use super::super::{accordion::Accordion, rich_text_viewer::RichTextViewer, section::Section};

use super::controllers::Controller;
use super::i18n::FinalRecommendationTranslate;

#[component]
pub fn FinalRecommendation(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    children: Element,
) -> Element {
    let _tr: FinalRecommendationTranslate = translate(&lang);
    let ctrl = Controller::new(lang, project_id)?;

    let recommendation = ctrl.recommendation()?;
    let tab_title: &str = Tab::FinalDraft.translate(&lang);

    rsx! {
        Section { id: "final-recommendation",
            TabTitle { title: tab_title,
                div { class: "font-medium text-[15px] text-black",
                    {
                        format!(
                            "{} ~ {}",
                            formatted_timestamp(lang, recommendation.started_at),
                            formatted_timestamp(lang, recommendation.ended_at),
                        )
                    }
                }
            }
            div { class: "flex flex-col gap-10",
                Accordion { title: recommendation.title, default_open: true,
                    RichTextViewer {
                        class: "w-full flex justify-start text-[15px]",
                        contenteditable: false,
                        html: recommendation.description,
                    }
                    div { class: "w-full mt-20 flex max-[700px]:flex-col max-[700px]:gap-10 flex-row justify-start gap-40",
                        div { class: "w-full flex flex-row justify-start gap-20",
                            for _member in recommendation.members {
                                AvatarLabel {
                                    //FIXME: use role
                                    label: "UNKNOWN",
                                    sub: "DAO",
                                }
                            }
                        }
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

// #[component]
// pub fn EditDraft(
//     lang: Language,
//     content: String,
//     change_content: EventHandler<String>,
//     title: String,
//     change_title: EventHandler<String>,
//     update_draft: EventHandler<MouseEvent>,
// ) -> Element {
//     let tr: FinalRecommendationTranslate = translate(&lang);

//     rsx! {
//         div { class: "flex flex-col w-full justify-start items-start",
//             div { class: "flex flex-col min-w-350 w-full justify-center items-center gap-15",
//                 div { class: "flex flex-col w-full justify-start items-start gap-10",
//                     div { class: "font-semibold text-[15px] text-text-black", {tr.name} }
//                     InputBox {
//                         class: "flex flex-row w-full rounded-[10px] px-15 py-10 placeholder-hint-gray bg-transparent text-text-black border border-gray-300 focus:outline-none focus:border focus:border-button-primary",
//                         placeholder: tr.name_hint,
//                         value: title,
//                         onchange: move |value: String| {
//                             change_title.call(value);
//                         },
//                     }
//                 }
//                 div { class: "flex flex-col w-full justify-start items-start gap-10",
//                     div { class: "font-semibold text-[15px] text-text-black", {tr.description} }
//                     RichText {
//                         content,
//                         onchange: move |value: String| {
//                             change_content.call(value);
//                         },
//                     }
//                 }

//                 div {
//                     class: "cursor-pointer flex flex-row w-200 justify-center items-center bg-button-primary rounded-lg px-16 py-14 font-bold text-white text-base",
//                     onclick: move |e: Event<MouseData>| {
//                         update_draft.call(e);
//                     },
//                     {tr.update}
//                 }
//             }
//         }
//     }
// }
