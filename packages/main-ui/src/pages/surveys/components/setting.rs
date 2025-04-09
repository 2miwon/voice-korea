use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{PanelV2, PanelV2Summary};

use crate::{
    components::icons::{Clear, Remove},
    pages::surveys::{i18n::SettingPanelTranslate, models::current_step::CurrentStep},
};

#[component]
pub fn Setting(
    lang: Language,
    #[props(default = 0)] total_members: u64,
    #[props(default = vec![])] selected_panels: Vec<PanelV2>,
    #[props(default = vec![])] panels: Vec<PanelV2Summary>,
    #[props(default = vec![])] maximum_counts: Vec<u64>,

    open_create_panel_modal: EventHandler<MouseEvent>,
    remove_selected_panel: EventHandler<usize>,
    remove_all_selected_panel: EventHandler<MouseEvent>,
    add_selected_panel: EventHandler<PanelV2>,
    change_selected_panel_count: EventHandler<(usize, u64)>,
    change_total_panel_members: EventHandler<u64>,
    change_step: EventHandler<CurrentStep>,
    save_survey: EventHandler<MouseEvent>,

    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let translate: SettingPanelTranslate = translate(&lang);
    let mut is_open = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start",
            ..attributes,
            div { class: "flex flex-row w-full justify-between items-center mb-[10px]",
                div { class: "font-medium text-black text-[16px] leading-[22.5px]",
                    "{translate.composition_panel}"
                }
                button {
                    class: "bg-[#2a60d3] rounded-4px px-[14px] py-[8px] font-semibold text-white text-[16px] rounded-[4px]",
                    onclick: move |e: Event<MouseData>| {
                        open_create_panel_modal.call(e);
                    },
                    "{translate.create_panel}"
                }
            }

            div {
                class: "flex flex-col w-full justify-start items-start px-[40px] py-[24px] bg-white rounded-[8px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                    div { class: "font-bold text-[#222222] text-lg mb-[5px]",
                        "{translate.total_panel_setting}"
                    }
                    div { class: "font-normal text-[#6d6d6d] text-[14px]",
                        "{translate.total_panel_setting_description}"
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row w-full justify-between items-center mb-[10px]",
                        div { class: "flex flex-row w-[200px] font-medium text-[15px] text-black",
                            "{translate.total_panel}"
                        }
                        div { class: "flex flex-row gap-[10px] items-center",
                            div { class: "flex flex-row w-[213px] h-[55px] p-[15px] justify-end items-center font-medium text-[#222222] text-[15px] bg-[#f7f7f7] rounded-[4px]",
                                "{total_members}"
                            }
                            div { class: "font-normal text-black text-[15px]", "{translate.person}" }
                        }
                    }

                    div { class: "flex flex-row w-full justify-between items-center",
                        div { class: "flex flex-row w-[200px] font-medium text-[15px] text-black",
                            "{translate.select_panel}"
                        }
                        div { class: "relative w-full",
                            button {
                                class: "flex flex-row w-full min-h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] p-[15px]",
                                onclick: move |_| {
                                    is_open.set(true);
                                },
                                if selected_panels.len() != 0 {
                                    div { class: "flex flex-wrap flex-1 justify-start items-center gap-[5px]",
                                        for (i , panel) in selected_panels.iter().enumerate() {
                                            PanelLabel {
                                                label: panel.name.clone(),
                                                onclose: move |e: Event<MouseData>| {
                                                    e.stop_propagation();
                                                    e.prevent_default();
                                                    remove_selected_panel.call(i);
                                                },
                                            }
                                        }
                                    }
                                    button {
                                        onclick: move |e: Event<MouseData>| {
                                            e.stop_propagation();
                                            e.prevent_default();
                                            remove_all_selected_panel.call(e);
                                        },
                                        Remove {
                                            width: "15",
                                            height: "15",
                                            fill: "#555462",
                                        }
                                    }
                                }
                            }
                            if is_open() {
                                div { class: "absolute flex flex-col w-full justify-start items-center shadow-[0px_8px_20px_rgba(20,26,62,0.25)] bg-white py-4 rounded-md",
                                    div { class: "flex flex-row w-full justify-end px-[10px]",
                                        button {
                                            onclick: move |_| {
                                                is_open.set(false);
                                            },
                                            Remove {
                                                width: "15",
                                                height: "15",
                                                fill: "#555462",
                                            }
                                        }
                                    }
                                    for panel in panels.clone() {
                                        if !selected_panels.iter().any(|selected| selected.name == panel.name) {
                                            div {
                                                class: "flex flex-col w-full h-[60px] justify-start items-start py-[9px] bg-white hover:bg-[#f7f7f7] hover:border-l hover:border-l-[#2a60d3] cursor-pointer",
                                                onclick: {
                                                    let panel = panel.clone();
                                                    move |_| {
                                                        add_selected_panel
                                                            .call(PanelV2 {
                                                                id: panel.id.clone(),
                                                                created_at: panel.created_at.clone(),
                                                                updated_at: panel.updated_at.clone(),
                                                                name: panel.name.clone(),
                                                                user_count: panel.user_count.clone(),
                                                                attributes: panel.attributes.clone(),
                                                                org_id: panel.org_id.clone(),
                                                            });
                                                        is_open.set(false);
                                                    }
                                                },
                                                div { class: "flex flex-col w-full px-4",
                                                    div { class: "font-bold text-[15px] text-[#222222] mb-[5px]",
                                                        "{panel.name}"
                                                    }
                                                    div { class: "font-medium text-[10px] text-[#222222]",
                                                        "{translate.total_people}: {panel.user_count}"
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

                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[20px]" }
                    div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                        for (index , panel) in selected_panels.iter().enumerate() {
                            div { class: "flex flex-row w-full justify-between items-center",
                                div { class: "font-medium text-[#222222] text-[15px]",
                                    "{panel.name}"
                                }
                                div { class: "flex flex-row h-[55px] items-center gap-[10px]",
                                    input {
                                        class: "flex flex-row w-[215px] h-[55px] justify-end items-center rounded-[4px] px-[15px] py-[10px] bg-[#f7f7f7] font-medium text-[#222222] text-[15px]",
                                        r#type: "text",
                                        placeholder: "0",
                                        value: panel.user_count,
                                        onkeydown: move |e: KeyboardEvent| {
                                            let key = e.key();
                                            if key != Key::Backspace && key != Key::Delete {
                                                let s = match key {
                                                    Key::Character(c) => c,
                                                    _ => "".to_string(),
                                                };
                                                if !s.chars().all(|c| c.is_ascii_digit()) {
                                                    e.prevent_default();
                                                }
                                            }
                                        },
                                        oninput: {
                                            let prev_count = selected_panels[index].clone().user_count;
                                            let maximum_counts = maximum_counts.clone();
                                            move |e: Event<FormData>| {
                                                let maximum_value = maximum_counts[index];
                                                let value = e.value().parse::<u64>().unwrap_or(0);
                                                if maximum_value < value {
                                                    change_selected_panel_count.call((index, maximum_value));
                                                    change_total_panel_members
                                                        .call(total_members - prev_count + maximum_value);
                                                } else {
                                                    change_selected_panel_count.call((index, value));
                                                    change_total_panel_members
                                                        .call(total_members - prev_count + maximum_value);
                                                }
                                            }
                                        },
                                    }

                                    div { class: "font-normal text-black text-[15px]",
                                        "{translate.person}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-center gap-[20px] text-white mt-[40px]",
                button {
                    class: "px-[20px] py-[10px] border-[#BFC8D9] bg-white border-[1px] text-davy-gray font-semibold text-[14px] rounded-[4px]",
                    onclick: move |_| {
                        change_step.call(CurrentStep::CreateSurvey);
                    },
                    "{translate.btn_cancel}"
                }

                button {
                    class: "px-[20px] py-[10px] bg-[#2A60D3] font-semibold text-[14px] rounded-[4px]",
                    onclick: move |e: Event<MouseData>| async move {
                        save_survey.call(e);
                    },
                    "{translate.btn_complete}"
                }
            }
        }
    }
}

#[component]
pub fn PanelLabel(label: String, onclose: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-center items-center px-[8px] py-[3px] bg-[#35343f] rounded-[5px] gap-[10px]",
            div { class: "font-semibold text-[14px] text-white", {label} }
            button {
                onclick: move |e: Event<MouseData>| {
                    onclose.call(e);
                },
                Clear { width: "20", height: "20" }
            }
        }
    }
}
