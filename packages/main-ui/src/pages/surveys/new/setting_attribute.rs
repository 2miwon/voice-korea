use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::{close_label::CloseLabel, icons::Remove},
    pages::surveys::{
        components::participant_distribute_table::ParticipantDistributeTable,
        new::i18n::SettingAttributeTranslate,
    },
};

#[cfg(feature = "web")]
use crate::components::outside_hook::eventhook::use_outside_click;

use super::controller::AttributeGroupInfo;

#[component]
pub fn SettingAttribute(
    lang: Language,
    visibility: bool,

    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    survey_id: Option<i64>,
    attribute_options: HashMap<String, Vec<AttributeGroupInfo>>,
    selected_attributes: Vec<String>,
    selected_tab: bool,

    change_selected_tab: EventHandler<bool>,
    add_selected_attribute: EventHandler<String>,
    remove_selected_attribute: EventHandler<usize>,
    clear_selected_attributes: EventHandler<MouseEvent>,
    remove_attribute_option: EventHandler<(String, String)>,
    update_attribute_rate: EventHandler<(String, String, i64)>,
) -> Element {
    let tr: SettingAttributeTranslate = translate(&lang);

    let mut max_value = use_signal(|| 0);

    rsx! {
        div {
            class: format!(
                "flex flex-col w-full h-full justify-start items-start {}",
                if !visibility { "hidden" } else { "" },
            ),
            width: if !visibility { "0px" },
            height: if !visibility { "0px" },
            ..attributes,
            div { class: "flex flex-col w-full justify-start items-start gap-10",
                div { class: "font-medium text-black text-base leading-22",
                    "{tr.composition_participant}"
                }
                div {
                    class: "flex flex-col w-full justify-start items-start px-40 py-24 bg-white rounded-lg",
                    style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                    div { class: "flex flex-col w-full justify-start items-start mb-20",
                        div { class: "font-bold text-text-black text-lg mb-5",
                            "{tr.participant_attribute_setting}"
                        }
                        div { class: "font-normal text-text-gray text-sm",
                            "{tr.participant_attribute_setting_info}"
                        }
                    }

                    div { class: "flex flex-col w-full justify-start items-start gap-20",
                        div { class: "flex flex-row w-full justify-start items-start gap-100",
                            div { class: "flex flex-row w-fit justify-start items-center gap-20",
                                div { class: "min-w-130 font-medium text-[15px] text-black leading-18",
                                    "{tr.total_people}"
                                }

                                input {
                                    r#type: "number",
                                    class: "text-right flex flex-row w-215 rounded-10 p-15 placeholder-hint-gray bg-background-gray text-text-black focus:outline-none focus:border focus:border-focus",
                                    placeholder: tr.total_people_hint,
                                    value: max_value(),
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<i64>() {
                                            max_value.set(v);
                                        }
                                    },
                                }
                            }

                            div { class: "flex flex-row w-full justify-start items-center gap-20",
                                div { class: "min-w-130 font-medium text-[15px] text-black leading-18",
                                    "{tr.attribute_group}"
                                }

                                Dropdown {
                                    id: "attribute_dropdown",
                                    hint: tr.enter_contents,
                                    selected_attributes: selected_attributes.clone(),
                                    options: vec![
                                        tr.gender.to_string(),
                                        tr.region.to_string(),
                                        tr.salary.to_string(),
                                        tr.age.to_string(),
                                    ],

                                    add_selected_attribute,
                                    remove_selected_attribute,
                                    clear_selected_attributes,
                                }
                            }
                        }

                        div { class: "flex flex-row w-full h-1 bg-period-border-gray" }

                        ParticipantDistributeTable {
                            lang,
                            attribute_options,
                            selected_attributes,
                            selected_tab,

                            change_selected_tab,
                            remove_attribute_option,
                            update_attribute_rate,
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Dropdown(
    id: String,
    hint: String,
    selected_attributes: Vec<String>,
    options: Vec<String>,

    add_selected_attribute: EventHandler<String>,
    remove_selected_attribute: EventHandler<usize>,
    clear_selected_attributes: EventHandler<MouseEvent>,
) -> Element {
    let mut is_focused = use_signal(|| false);

    #[cfg(feature = "web")]
    use_outside_click(&id, move |_| is_focused.set(false));

    rsx! {
        div {
            id,
            class: "cursor-pointer relative flex flex-row w-full h-55 justify-center items-center bg-background-gray rounded-md",
            onclick: move |_| {
                let prev = is_focused();
                is_focused.set(!prev);
            },

            div { class: "flex flex-row w-full items-center px-15 gap-[10px] justify-between",

                if selected_attributes.len() != 0 {
                    div {
                        class: "flex flex-wrap flex-1 gap-4",
                        visibility: if selected_attributes.len() != 0 { "flex" } else { "hidden" },
                        for (i , option) in selected_attributes.clone().iter().enumerate() {
                            CloseLabel {
                                label: option.clone(),
                                onremove: move |event: Event<MouseData>| {
                                    event.stop_propagation();
                                    event.prevent_default();
                                    remove_selected_attribute.call(i);
                                },
                            }
                        }
                    }

                    button {
                        onclick: move |event: Event<MouseData>| {
                            event.stop_propagation();
                            event.prevent_default();
                            clear_selected_attributes.call(event);
                        },
                        Remove { width: "20", height: "20", fill: "#555462" }
                    }
                } else {
                    div { class: "font-medium text-[15px] text-hint-gray bg-background-gray",
                        "{hint}"
                    }
                }
            }
            if is_focused() {
                div {
                    class: "absolute top-full bg-white border border-label-border-gray shadow-lg rounded-lg w-full h-150 overflow-y-scroll z-50",
                    onclick: move |event| {
                        event.stop_propagation();
                        event.prevent_default();
                    },
                    div { class: "flex flex-col w-full justify-start items-start",
                        div { class: format!("flex flex-col w-full justify-start items-center bg-white"),
                            for option in options {
                                if !selected_attributes.iter().any(|selected| selected.clone() == option) {
                                    button {
                                        class: "flex flex-col w-full justify-start items-start px-12 py-20 hover:bg-background-gray hover:border-l-2 hover:border-hover",
                                        onclick: move |event: Event<MouseData>| {
                                            event.stop_propagation();
                                            event.prevent_default();
                                            add_selected_attribute.call(option.clone());
                                            is_focused.set(false);
                                        },
                                        div { class: "font-bold text-text-black text-[15px] mb-5",
                                            "{option}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn generate_combinations(
    selected: Vec<String>,
    options: &HashMap<String, Vec<AttributeGroupInfo>>,
) -> Vec<Vec<AttributeGroupInfo>> {
    let selected_values: Vec<Vec<AttributeGroupInfo>> = selected
        .iter()
        .filter_map(|key| options.get(key))
        .cloned()
        .collect();

    if selected_values.is_empty() {
        return vec![];
    }

    let initial: Vec<Vec<AttributeGroupInfo>> =
        selected_values[0].iter().map(|x| vec![x.clone()]).collect();

    let combinations = helper(initial, &selected_values[1..]);

    tracing::debug!("combinations: {:?}", combinations);

    combinations
}

pub fn helper(
    acc: Vec<Vec<AttributeGroupInfo>>,
    rest: &[Vec<AttributeGroupInfo>],
) -> Vec<Vec<AttributeGroupInfo>> {
    if rest.is_empty() {
        return acc;
    }

    let mut result = vec![];
    for a in &acc {
        for r in &rest[0] {
            let mut new_comb = a.clone();
            new_comb.push(r.clone());
            result.push(new_comb);
        }
    }

    helper(result, &rest[1..])
}
