use bdk::prelude::*;

use crate::{
    components::{icons::Switch, pagination::Pagination},
    pages::surveys::{
        components::error_box::ErrorBox, models::attribute_combination::AttributeCombination,
    },
};

#[component]
pub fn CombinationTable(
    lang: Language,
    attribute_combinations: ReadOnlySignal<Vec<AttributeCombination>>,
    change_attribute_combination_value: EventHandler<(usize, usize)>,

    combination_error: bool,
) -> Element {
    let mut combinations = use_signal(|| attribute_combinations.clone());
    let tr: CombinationTableTranslate = translate(&lang);

    let mut current_page = use_signal(|| 1);

    let total_page = use_memo({
        move || {
            let total = combinations().len().max(1);
            (total - 1) / 7 + 1
        }
    });

    use_effect(move || {
        combinations.set(attribute_combinations);
    });

    let paginated_combinations = use_memo(move || {
        let all = combinations()();
        let total_len = all.len();
        let page = current_page();
        let mut start_index = (page - 1) * 7;
        let mut end_index = (start_index + 7).min(total_len);

        if start_index >= total_len {
            start_index = 0;
            end_index = 7.min(total_len);
        }

        all[start_index..end_index]
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, combination)| (start_index + i, combination))
            .collect::<Vec<(usize, AttributeCombination)>>()
    });

    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start px-40 py-24 bg-white rounded-lg gap-20",
            style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-bold text-text-black text-lg mb-5", {tr.title} }
                div { class: "font-normal text-text-gray text-sm", {tr.description} }
            }

            ErrorBox {
                hidden: !combination_error,
                title: tr.error_title,
                description: tr.error_description,
            }

            div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-label-border-gray",
                div { class: "flex flex-row w-full min-h-55 justify-start items-center",
                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                        div { class: "text-table-text-gray font-semibold text-sm", {tr.combination} }
                        Switch { width: "19", height: "19" }
                    }
                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                        div { class: "text-table-text-gray font-semibold text-sm", {tr.rate} }
                        Switch { width: "19", height: "19" }
                    }
                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                        div { class: "text-table-text-gray font-semibold text-sm", {tr.total_members} }
                        Switch { width: "19", height: "19" }
                    }
                }

                for (real_index , combination) in paginated_combinations() {
                    div { class: "flex flex-row w-full min-h-55 justify-start items-center",
                        div { class: "flex flex-wrap flex-1 w-full min-h-55 justify-center items-center gap-10",
                            for attribute in combination.group {
                                AttributeLabel { label: attribute.attribute.clone() }
                            }
                        }

                        div { class: "flex flex-row flex-1 h-full justify-center items-center",
                            input {
                                r#type: "number",
                                class: "flex flex-row w-50 bg-transparent text-text-black focus:outline-none",
                                value: combination.total_rate,
                                readonly: true,
                            }
                        }

                        div { class: "flex flex-row flex-1 h-full justify-center items-center",
                            input {
                                r#type: "number",
                                class: "flex flex-row w-50 bg-transparent text-text-black focus:outline-none",
                                value: combination.total_count,
                                oninput: move |e: Event<FormData>| {
                                    if let Ok(v) = e.value().parse::<i64>() {
                                        change_attribute_combination_value.call((real_index, v as usize));
                                    } else {
                                        btracing::error!("{}", tr.number_format_error);
                                    }
                                },
                            }
                        }
                    }
                }
            }

            Pagination {
                total_page: total_page(),
                current_page: current_page(),
                size: 7,
                onclick: move |page| {
                    current_page.set(page);
                },
            }
        }
    }
}

#[component]
pub fn AttributeLabel(label: String) -> Element {
    rsx! {
        div { class: "flex flex-row w-fit h-fit px-8 py-3 rounded-sm bg-label-black",
            div { class: "font-semibold text-white text-sm leading-18", {label} }
        }
    }
}

translate! {
    CombinationTableTranslate;

    number_format_error: {
        ko: "오직 숫자만 입력 가능합니다.",
        en: "Only numbers can be entered."
    }
    title: {
        ko: "조합별 요약",
        en: "Summary by combination"
    }
    description: {
        ko: "공론에 참여할 패널과 샘플링할 인원을 설정합니다.",
        en: "Set the panel to participate in the public discussion and the number of people to be sampled."
    }

    combination: {
        ko: "조합",
        en: "Combination"
    }
    rate: {
        ko: "비율(%)",
        en: "Rate(%)"
    }
    total_members: {
        ko: "총 인원",
        en: "Total Members"
    }

    error_title: {
        ko: "조합별 비율 총합 오류",
        en: "Total error by combination ratio"
    }

    error_description: {
        ko: "조합별 비율 합계는 반드시 100%여야 합니다. 입력값을 다시 확인해주세요.",
        en: "The sum of the percentages for each combination must be 100%. Please check your input values ​​again."
    }
}
