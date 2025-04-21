use bdk::prelude::*;
use models::{response::Answer, SurveyV2};

use crate::components::icons::left_arrow::LeftArrow;

use super::i18n::FinalSurveyTranslate;

#[component]
pub fn MyFinalSurvey(
    lang: Language,
    survey: SurveyV2,
    answers: Vec<Answer>,
    onprev: EventHandler<MouseEvent>,
    onchange: EventHandler<(usize, Answer)>,
) -> Element {
    let tr: FinalSurveyTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full gap-[10px] mb-[40px] mt-[28px]",
            div { class: "flex flex-row w-full justify-between items-center mb-[10px]",
                div { class: "flex flex-row justify-start items-center gap-[8px]",
                    div {
                        class: "cursor-pointer w-[24px] h-[24px]",
                        onclick: move |e: Event<MouseData>| {
                            onprev.call(e);
                        },
                        LeftArrow { stroke: "black" }
                    }
                    div { class: "font-semibold text-[#222222] text-[20px]", "{tr.title}" }
                }
            }
        

        }
    }
}
