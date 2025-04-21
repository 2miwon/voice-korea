use std::collections::BTreeMap;

use by_components::icons::arrows::ChevronLeft;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{response::Answer, SurveyV2};

use crate::pages::projects::_id::components::question::QuestionComponent;

use super::i18n::SampleSurveyTranslate;
#[component]
pub fn SurveyComponent(
    lang: Language,
    survey: SurveyV2,
    answers: BTreeMap<usize, Answer>,
    onprev: EventHandler<MouseEvent>,
    onsend: EventHandler<MouseEvent>,
    onchange: EventHandler<(usize, Answer)>,
) -> Element {
    let tr: SampleSurveyTranslate = translate(&lang);
    let survey_title = survey.name;
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[10px] mt-[28px]",
            div { class: "flex flex-row w-full justify-start items-center gap-[8px] mb-[10px]",
                div {
                    class: "cursor-pointer w-[24px] h-[24px]",
                    onclick: move |e: Event<MouseData>| {
                        onprev.call(e);
                    },
                    ChevronLeft { class: "[&>stroke]:storke-black" }
                }
                div { class: "font-semibold text-light-gray text-[20px]", {survey_title} }
            }
            for (i , question) in survey.questions.iter().enumerate() {
                QuestionComponent {
                    placeholder: tr.survey_input_placehoder,
                    question: question.clone(),
                    answer: answers.get(&i).cloned(),
                    onchange: move |e: Answer| {
                        onchange.call((i, e));
                    },
                }
            }


            div { class: "flex flex-row w-full justify-center items-center mb-[40px]",
                div {
                    class: "cursor-pointer flex flex-row justify-center items-center w-[200px] py-[13px] font-bold text-white text-[16px] bg-[#8095EA] rounded-[8px]",
                    onclick: move |e: Event<MouseData>| {
                        onsend.call(e);
                    },
                    {tr.submit}
                }
            }
        }
    }
}
