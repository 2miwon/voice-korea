use bdk::prelude::*;
use std::collections::HashMap;

use crate::{
    components::{
        icons::{Switch, Trash},
        pagination::Pagination,
        radio::RadioButton,
    },
    pages::surveys::models::attribute_group_info::AttributeGroupInfo,
};

#[component]
pub fn ParticipantDistributeTable(
    lang: Language,
    attribute_options: HashMap<String, Vec<AttributeGroupInfo>>,
    selected_attributes: Vec<String>,
    selected_tab: bool,

    change_selected_tab: EventHandler<bool>,
    remove_attribute_option: EventHandler<(String, String)>,
    update_attribute_rate: EventHandler<(String, String, i64)>,
    clear_attribute: EventHandler<MouseEvent>,
) -> Element {
    let tr: ParticipantDistributeTableTranslate = translate(&lang);
    let mut attribute_groups = use_signal(|| vec![]);
    let mut current_page = use_signal(|| 1);

    let total_page = use_memo(move || {
        let total = attribute_groups().len().max(1);
        (total - 1) / 7 + 1
    });

    use_effect(use_reactive(&(attribute_options, selected_attributes), {
        move |(attribute_options, selected_attributes)| {
            let new_groups = attribute_options
                .iter()
                .filter(|(key, _)| selected_attributes.contains(key))
                .flat_map(|(_, groups)| groups.clone())
                .collect::<Vec<AttributeGroupInfo>>();

            attribute_groups.set(new_groups);
        }
    }));

    use_effect(use_reactive(&(total_page(), current_page()), {
        move |(total, current)| {
            if current > total {
                current_page.set(1);
            }
        }
    }));

    let groups = attribute_groups();
    let total_len = groups.len();
    let page = current_page();
    let start_index = (page - 1) * 7;

    let paginated_groups: Vec<(usize, AttributeGroupInfo)> = if start_index >= total_len {
        vec![]
    } else {
        let end_index = (start_index + 7).min(total_len);
        groups[start_index..end_index]
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, group)| (start_index + i, group))
            .collect()
    };

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-20",
            div { class: "flex flex-row w-full justify-start items-center",
                div { class: "min-w-150 font-medium text-[15px] text-black leading-18",
                    {tr.personnel_distribution}
                }
                div { class: "flex flex-row w-full justify-start items-center gap-50",
                    div { class: "flex flex-row w-fit justify-start items-center",
                        RadioButton {
                            name: "distribution",
                            value: tr.equal_distribution,
                            checked: selected_tab,
                            onchange: move |_| {
                                change_selected_tab.call(true);
                            },
                        }
                    }
                    div { class: "flex flex-row w-fit justify-start items-center",
                        RadioButton {
                            name: "distribution",
                            value: tr.manual_specification,
                            checked: !selected_tab,
                            onchange: move |_| {
                                change_selected_tab.call(false);
                            },
                        }
                    }
                }
                div {
                    class: "flex flex-row cursor-pointer px-20 py-10 bg-hover font-semibold text-sm rounded-sm text-white w-150 h-fit justify-center items-center",
                    onclick: move |e| {
                        clear_attribute.call(e);
                    },
                    {tr.init_attribute}
                }
            }

            div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-label-border-gray",
                div { class: "flex flex-row w-full min-h-55 justify-start items-center",
                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                        div { class: "text-table-text-gray font-semibold text-sm",
                            {tr.attribute_group}
                        }
                        Switch { width: "19", height: "19" }
                    }
                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                        div { class: "text-table-text-gray font-semibold text-sm", {tr.attribute} }
                        Switch { width: "19", height: "19" }
                    }
                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                        div { class: "text-table-text-gray font-semibold text-sm", {tr.rate} }
                        Switch { width: "19", height: "19" }
                    }
                    div { class: "flex flex-row w-100 h-full justify-center items-center" }
                }

                for (real_index , group) in paginated_groups.clone() {
                    div { class: "flex flex-col w-full justify-start items-start",
                        div { class: "flex flex-row w-full h-1 bg-label-border-gray" }
                        div { class: "flex flex-row w-full min-h-55 h-fit py-5",
                            div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                div { class: "font-semibold text-sm text-third", "{group.name}" }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                AttributeLabel { label: group.attribute.clone() }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                input {
                                    r#type: "number",
                                    class: "flex flex-row w-full h-full bg-background-gray text-text-black focus:outline-none focus:bg-white focus:border focus:border-hover text-center",
                                    value: group.rate,
                                    oninput: {
                                        let name = group.name.clone();
                                        let attribute = group.attribute.clone();
                                        move |e: Event<FormData>| {
                                            if let Ok(v) = e.value().parse::<i64>() {
                                                update_attribute_rate.call((name.clone(), attribute.clone(), v));
                                            } else {
                                                btracing::error!("{}", tr.number_format_error);
                                            }
                                        }
                                    },
                                }
                            }
                            div {
                                class: "cursor-pointer flex flex-row w-100 h-full justify-center items-center",
                                onclick: {
                                    let name = group.name.clone();
                                    let attribute = group.attribute.clone();
                                    move |_| {
                                        attribute_groups.remove(real_index);
                                        remove_attribute_option.call((name.clone(), attribute.clone()));
                                    }
                                },
                                div { class: "flex flex-row w-fit h-fit px-8 py-4 border border-delete-border-gray rounded-b-sm gap-5",
                                    div { class: "font-medium text-sm text-table-text-gray leading-22",
                                        {tr.remove}
                                    }
                                    Trash { width: "18", height: "18" }
                                }
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
    ParticipantDistributeTableTranslate;

    number_format_error: {
        ko: "오직 숫자만 입력 가능합니다.",
        en: "Only numbers can be entered."
    }

    personnel_distribution: {
        ko: "인원 분배",
        en: "Personnal Distribution"
    }

    equal_distribution: {
        ko: "균등 분배",
        en: "Equal Distribution"
    }

    manual_specification: {
        ko: "수동 지정",
        en: "Manual Specification"
    }

    attribute_group: {
        ko: "속성 그룹",
        en: "Attribute Group"
    }
    init_attribute: {
        ko: "속성 초기화",
        en: "Init Attribute"
    }
    attribute: {
        ko: "속성",
        en: "Attribute"
    }
    rate: {
        ko: "비율(%)",
        en: "Rate(%)"
    }

    remove: {
        ko: "삭제",
        en: "Remove"
    }
}
