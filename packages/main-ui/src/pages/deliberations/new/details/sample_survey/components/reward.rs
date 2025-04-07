use bdk::prelude::*;
use models::DeliberationSampleSurveyCreateRequest;

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::details::sample_survey::i18n::SampleSurveyRewardTranslate,
};

#[component]
pub fn SampleSurveyReward(
    lang: Language,

    sample_survey: DeliberationSampleSurveyCreateRequest,
    set_sample_survey: EventHandler<DeliberationSampleSurveyCreateRequest>,
) -> Element {
    let tr: SampleSurveyRewardTranslate = translate(&lang);

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            div { class: "flex flex-row w-full justify-start items-center gap-100",
                ResponseForm {
                    label: tr.expected_time,
                    hint: tr.expected_time_hint,
                    value: sample_survey.estimate_time,
                    oninput: {
                        let mut sample = sample_survey.clone();
                        move |e: Event<FormData>| {
                            if let Ok(v) = e.value().trim().parse::<i64>() {
                                sample.estimate_time = v;
                                set_sample_survey.call(sample.clone());
                            }
                        }
                    },
                }

                ResponseForm {
                    label: tr.expected_point,
                    hint: tr.expected_point_hint,
                    value: sample_survey.point,
                    oninput: {
                        let mut sample = sample_survey.clone();
                        move |e: Event<FormData>| {
                            if let Ok(v) = e.value().trim().parse::<i64>() {
                                sample.point = v;
                                set_sample_survey.call(sample.clone());
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
