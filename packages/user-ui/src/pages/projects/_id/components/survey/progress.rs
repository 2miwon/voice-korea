use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{response::Answer, SurveyV2};

use crate::pages::projects::_id::components::{
    question::QuestionComponent, tab_title::TabTitleWithPrev,
};

use super::i18n::SurveyProgressTranslate;

#[component]
pub fn SurveyProgress(
    lang: Language,
    survey: SurveyV2,
    answers: ReadOnlySignal<BTreeMap<usize, Answer>>,
    onprev: EventHandler<MouseEvent>,
    onsend: EventHandler<MouseEvent>,
    onchange: EventHandler<(usize, Answer)>,
) -> Element {
    let tr: SurveyProgressTranslate = translate(&lang);
    let survey_title = survey.name;
    rsx! {
        TabTitleWithPrev {
            title: survey_title,
            onprev: move |e: Event<MouseData>| {
                onprev.call(e);
            },
        }

        div { class: "flex flex-col w-full gap-10",
            for (i , question) in survey.questions.iter().enumerate() {
                QuestionComponent {
                    placeholder: tr.survey_input_placehoder,
                    question: question.clone(),
                    answer: answers().get(&i).cloned(),
                    onchange: move |e: Answer| {
                        onchange.call((i, e));
                    },
                }
            }
        }
        div { class: "flex flex-row w-full justify-center items-center",
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
