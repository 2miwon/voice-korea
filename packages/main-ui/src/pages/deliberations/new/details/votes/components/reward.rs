use bdk::prelude::*;
use models::DeliberationFinalSurveyCreateRequest;

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::details::votes::i18n::RewardTranslate,
};

#[component]
pub fn FinalSurveyReward(
    lang: Language,
    final_survey: DeliberationFinalSurveyCreateRequest,
    set_final_survey: EventHandler<DeliberationFinalSurveyCreateRequest>,
) -> Element {
    let tr: RewardTranslate = translate(&lang);
    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            div { class: "flex flex-row w-full justify-start items-center gap-100",
                ResponseForm {
                    label: tr.expected_time,
                    hint: tr.expected_time_hint,
                    value: final_survey.estimate_time,
                    oninput: {
                        let mut final_survey = final_survey.clone();
                        move |e: Event<FormData>| {
                            if let Ok(v) = e.value().trim().parse::<i64>() {
                                final_survey.estimate_time = v;
                                set_final_survey.call(final_survey.clone());
                            }
                        }
                    },
                }

                ResponseForm {
                    label: tr.expected_point,
                    hint: tr.expected_point_hint,
                    value: final_survey.point,
                    oninput: {
                        let mut final_survey = final_survey.clone();
                        move |e: Event<FormData>| {
                            if let Ok(v) = e.value().trim().parse::<i64>() {
                                final_survey.point = v;
                                set_final_survey.call(final_survey.clone());
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn ResponseForm(
    label: String,
    hint: String,
    value: i64,
    oninput: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center gap-20",
            div { class: "flex flex-row max-w-180 w-full justify-start items-center font-normal text-[15px] text-black",
                "{label}"
            }
            input {
                class: "flex flex-row w-full justify-start items-center rounded-sm px-15 py-10 placeholder-hint-gray bg-background-gray font-medium text-text-black text-[15px]",
                r#type: "text",
                placeholder: hint,
                value,
                oninput,
            }
        }
    }
}
