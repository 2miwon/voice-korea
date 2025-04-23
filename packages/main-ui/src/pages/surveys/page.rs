#![allow(non_snake_case, dead_code, unused_variables)]
use bdk::prelude::*;
use models::{ProjectStatus, ProjectType};

use crate::{
    components::{
        icons::{RowOption, Search, Switch},
        pagination::Pagination,
    },
    pages::surveys::{
        controller::Controller,
        i18n::{ErrorModalTranslate, RemoveSurveyModalTranslate, SurveyTranslate},
    },
    routes::Route,
};

#[derive(Props, Clone, PartialEq)]
pub struct SurveyProps {
    lang: Language,
}

#[component]
pub fn SurveyPage(props: SurveyProps) -> Element {
    let mut ctrl = Controller::new(props.lang)?;
    let translate: SurveyTranslate = translate(&props.lang);

    let mut is_focused = use_signal(|| false);
    let mut project_name = use_signal(|| "".to_string());

    let navigator = use_navigator();

    // let surveys = ctrl.get_surveys();

    // FIXME: it seems to be anti-pattern due should be refactoring to use_memo when implementing panel
    // let mut clicked_panel_index = use_signal(|| 0);

    // use_effect(use_reactive(&survey_len, move |len| {
    //     clicked_panel_index.set(len);
    // }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-header-gray font-medium text-sm mb-10", "{translate.survey_title}" }
            div { class: "text-header-black font-semibold text-[28px] mb-25",
                "{translate.survey_title}"
            }
            div { class: "text-label-black font-normal text-sm mb-40", "{translate.survey_description}" }

            div { class: "flex flex-col w-full justify-start items-start mb-[50px]",
                div {
                    class: "flex flex-col w-full justify-start items-start px-20 pt-20 pb-30 bg-white rounded-lg",
                    style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                    div { class: "flex flex-row w-full justify-between items-center pb-20",
                        div {
                            class: "flex flex-row w-590 h-45 justify-between items-center rounded-lg px-11 py-13 border bg-background-gray border-third aria-active:!bg-white aria-active:!border-primary",
                            "aria-active": (is_focused)(),
                            input {
                                class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                                r#type: "text",
                                placeholder: "{translate.search_hint}",
                                value: (project_name)(),
                                onfocus: move |_| {
                                    is_focused.set(true);
                                },
                                onblur: move |_| {
                                    is_focused.set(false);
                                },
                                oninput: move |event| {
                                    project_name.set(event.value());
                                },
                            }
                            Search { width: "18", height: "18", color: "#7c8292" }
                        }
                        Link {
                            to: Route::SurveyCreatePage {
                                lang: props.lang,
                            },
                            div { class: "flex flex-row justify-center items-center px-14 py-8 bg-primary rounded-sm",
                                div { class: "text-white font-semibold text-base",
                                    "{translate.start_survey}"
                                }
                            }
                        }
                    }

                    //project table
                    div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-label-border-gray",
                        div { class: "flex flex-row w-full h-55 justify-start items-center",
                            div { class: "flex flex-row w-150 min-w-[150px] h-full justify-center items-center gap-10",
                                div { class: "text-third font-semibold text-sm",
                                    "{translate.survey_type}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row w-120 min-w-[150px] h-full justify-center items-center gap-10",
                                div { class: "text-third font-semibold text-sm",
                                    "{translate.survey_field}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                                div { class: "text-third font-semibold text-sm",
                                    "{translate.survey_project}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                                div { class: "text-third font-semibold text-sm",
                                    "{translate.survey_response_rate}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            // div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            //     div { class: "text-[#7c8292] font-semibold text-[14px]",
                            //         "{translate.survey_panel}"
                            //     }
                            //     Switch { width: "19", height: "19" }
                            // }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                                div { class: "text-third font-semibold text-sm",
                                    "{translate.survey_period}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row w-120 min-w-[120px] h-full justify-center items-center gap-10",
                                div { class: "text-third font-semibold text-sm",
                                    "{translate.survey_status}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row w-120 min-w-[120px] h-full justify-center items-center gap-10",
                                div { class: "text-third font-semibold text-sm",
                                    "{translate.survey_view}"
                                }
                            }
                            div { class: "flex flex-row w-90 min-w-[90px] h-full justify-center items-center gap-10" }
                        }

                        if let Some(surveys) = ctrl.get_surveys() {
                            for survey in surveys.items {
                                div { class: "flex flex-col w-full justify-start items-start",
                                    div { class: "flex flex-row w-full h-[1px] bg-label-border-gray" }
                                    div { class: "flex flex-row w-full min-h-[55px]",
                                        div { class: "flex flex-row w-150 min-w-[150px] h-full justify-center items-center",
                                            div { class: "text-label-black font-semibold text-sm",
                                                {survey.project_type.translate(&props.lang)}
                                            }
                                        }
                                        div { class: "flex flex-row w-150 min-w-[150px] h-full justify-center items-center",
                                            div { class: "text-label-black font-semibold text-sm",
                                                {survey.project_area.translate(&props.lang)}
                                            }
                                        }
                                        div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                            div { class: "text-label-black font-semibold text-sm",
                                                "{survey.name}"
                                            }
                                        }
                                        div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                            div { class: "text-label-black font-semibold text-sm",
                                                {survey.response_rate()}
                                            }
                                        }

                                        // TODO: implement panel in survey list view
                                        // div { class: "flex flex-wrap flex-1 min-h-[55px] justify-center items-center gap-[5px] py-[5px]",
                                        //     for panel in survey.panels.clone() {
                                        //         PanelLabel {
                                        //             label: panel.name.clone(),
                                        //             background_color: if survey.status == ProjectStatus::Ready { "#35343f".to_string() } else { "#b4b4b4".to_string() },
                                        //         }
                                        //     }
                                        // }

                                        div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                            div { class: "text-label-black font-semibold text-sm",
                                                "{survey.period()}"
                                            }
                                        }
                                        div { class: "flex flex-row w-120 min-w-[120px] h-full justify-center items-center",
                                            div { class: "text-label-black font-semibold text-sm",
                                                {survey.survey_status().translate(&props.lang)}
                                            }
                                        }

                                        match survey.status {
                                            ProjectStatus::Ready => {
                                                rsx! {
                                                    div {
                                                        class: "flex flex-row w-120 min-w-[120px] h-full justify-center items-center cursor-pointer",
                                                        visibility: if survey.finished() || survey.project_type == ProjectType::SampleSurvey
                                                        || survey.project_type == ProjectType::FinalSurvey { "hidden" } else { "" },
                                                        onclick: {
                                                            let id = survey.id.clone();
                                                            move |_| async move {
                                                                ctrl.open_start_survey_popup(props.lang, id).await;
                                                            }
                                                        },
                                                        div { class: "text-hover font-semibold text-sm", "{translate.start_survey_create}" }
                                                    }
                                                }
                                            }
                                            _ => {
                                                rsx! {
                                                    Link {
                                                        class: "flex flex-row w-120 min-w-[120px] h-full justify-center items-center cursor-pointer",
                                                        to: Route::SurveyResultPage {
                                                            lang: props.lang,
                                                            survey_id: survey.id,
                                                        },
                                                        div { class: "text-hover font-semibold text-sm", "{translate.view_results}" }
                                                    }
                                                }
                                            }
                                        }

                                        div {
                                            class: "group relative",
                                            visibility: if survey.finished() { "hidden" } else { "" },
                                            div { class: "flex flex-row w-90 min-w-[90px] h-full justify-center items-center",
                                                if survey.status == ProjectStatus::Ready {
                                                    button {
                                                        RowOption {
                                                            width: "24",
                                                            height: "24",
                                                        }
                                                    }
                                                    nav { class: "border-2 bg-white invisible border-none shadow-lg rounded w-fit absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                                        ul { class: "py-1",
                                                            li {
                                                                class: "px-20 py-15 min-w-[200px] w-full text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                                onclick: move |_| {
                                                                    navigator
                                                                        .push(Route::SurveyUpdatePage {
                                                                            lang: props.lang,
                                                                            survey_id: survey.id,
                                                                        });
                                                                },
                                                                "{translate.update_survey}"
                                                            }
                                                            li {
                                                                class: "px-20 py-15 min-w-[200px] w-full  text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                                onclick: move |_| {
                                                                    let id = survey.id.clone();
                                                                    async move {
                                                                        ctrl.open_remove_survey_modal(id.to_string()).await;
                                                                    }
                                                                },
                                                                "{translate.remove_survey}"
                                                            }
                                                            li {
                                                                class: "px-20 py-15 min-w-[200px] w-full  text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                                onclick: {
                                                                    let survey = survey.clone();
                                                                    move |_| {
                                                                        let questions = survey.questions.clone();
                                                                        let id = survey.id.clone();
                                                                        async move {
                                                                            ctrl.open_setting_reward_modal(
                                                                                    id,
                                                                                    props.lang,
                                                                                    survey.estimate_time,
                                                                                    survey.point,
                                                                                    questions.len() as i64,
                                                                                )
                                                                                .await;
                                                                        }
                                                                    }
                                                                },
                                                                "{translate.update_reward}"
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

                    Pagination {
                        total_page: if ctrl.size != 0 { ctrl.total_pages() } else { 0 },
                        current_page: ctrl.page(),
                        size: ctrl.size,
                        onclick: move |page| {
                            ctrl.set_page(page);
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn ErrorModal(lang: Language, onclose: EventHandler<MouseEvent>) -> Element {
    let i18n: ErrorModalTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[40px]",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] mt-[30px] whitespace-pre-line",
                div { "{i18n.error_info}" }
            }
            div { class: "flex flex-row w-full justify-end items-end",
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{i18n.confirm}"
                }
            }
        }
    }
}

#[component]
pub fn RemoveSurveyModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    onremove: EventHandler<MouseEvent>,
) -> Element {
    let i18n: RemoveSurveyModalTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-text-black font-normal text-sm gap-5",
                div { "{i18n.remove_info}" }
                div { "{i18n.remove_warning}" }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-20",
                div {
                    class: "flex flex-row w-85 h-40 justify-center items-center bg-primary rounded-md cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onremove.call(e);
                    },
                    div { class: "text-white font-bold text-base", "{i18n.remove}" }
                }
                div {
                    class: "flex flex-row w-85 h-40 font-semibold text-base text-text-black justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{i18n.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn PanelLabel(label: String, background_color: String) -> Element {
    rsx! {
        div {
            class: "flex flex-row justify-center items-center px-8 py-3 rounded-[100px] font-semibold text-sm text-white",
            style: format!("background-color: {}", background_color),
            {label}
        }
    }
}
