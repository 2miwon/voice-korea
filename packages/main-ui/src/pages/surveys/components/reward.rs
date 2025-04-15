use bdk::prelude::*;

use crate::components::expandable_card::ExpandableCard;

#[component]
pub fn SurveyReward(
    lang: Language,
    onchange_point: EventHandler<i64>,
    onchange_estimate_time: EventHandler<i64>,
    point: ReadOnlySignal<i64>,
    estimate_time: ReadOnlySignal<i64>,
) -> Element {
    let tr: SurveyRewardTranslate = translate(&lang);

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            div { class: "flex flex-row w-full justify-start items-center gap-100",
                ResponseForm {
                    label: tr.expected_time_label,
                    hint: tr.expected_time_hint,
                    value: estimate_time(),
                    oninput: {
                        move |e: Event<FormData>| {
                            if let Ok(v) = e.value().trim().parse::<i64>() {
                                onchange_estimate_time.call(v);
                            } else {
                                btracing::error!("{}", tr.error);
                            }
                        }
                    },
                }

                ResponseForm {
                    label: tr.point_label,
                    hint: tr.point_hint,
                    value: point(),
                    oninput: {
                        move |e: Event<FormData>| {
                            if let Ok(v) = e.value().trim().parse::<i64>() {
                                onchange_point.call(v);
                            } else {
                                btracing::error!("{}", tr.error);
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
    let mut is_focused = use_signal(|| false);

    rsx! {
        div { class: "flex flex-row w-full justify-start items-center gap-20",
            div { class: "flex flex-row max-w-180 w-full justify-start items-center font-normal text-[15px] text-black",
                "{label}"
            }
            input {
                class: "flex flex-row w-full justify-start items-center rounded-sm px-15 py-10 placeholder-hint-gray bg-background-gray font-medium text-text-black text-[15px] focus:outline-none aria-active:bg-white aria-active:!border aria-active:!border-hover",
                "aria-active": is_focused(),
                r#type: "text",
                placeholder: hint,
                value,
                oninput,
                onfocus: move |_| {
                    is_focused.set(true);
                },
                onblur: move |_| {
                    is_focused.set(false);
                },
            }
        }
    }
}

translate! {
    SurveyRewardTranslate;

    title: {
        ko: "예상 소요 및 리워드",
        en: "Expected Cost and Rewards"
    }
    description: {
        ko: "설문 응답에 걸리는 예상 소요 시간과 리워드를 입력해주세요. 입력된 시간은 리워드 지급과는 무관합니다.",
        en: "Please enter the estimated time it will take to complete the survey and the reward you wish to receive. The time you enter has no bearing on reward payment."
    }

    expected_time_label: {
        ko: "예상 소요 시간",
        en: "Estimated Time"
    }
    expected_time_hint: {
        ko: "예상 소요 시간 입력",
        en: "Enter estimated time"
    }

    point_label: {
        ko: "응답 시 지급 포인트 입력",
        en: "Enter payment points when responding"
    }
    point_hint: {
        ko: "포인트 입력",
        en: "Enter point"
    }

    error: {
        ko: "오직 숫자만 입력 가능합니다",
        en: "Only numbers can be entered."
    }
}
