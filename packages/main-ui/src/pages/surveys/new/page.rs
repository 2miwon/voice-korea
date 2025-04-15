use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::ArrowLeft,
    pages::surveys::{
        models::current_step::CurrentStep,
        new::{
            controller::Controller, create_survey::CreateSurvey, i18n::SurveyNewTranslate,
            setting_attribute::SettingAttribute,
        },
    },
    routes::Route,
};

#[component]
pub fn SurveyCreatePage(lang: Language, survey_id: Option<i64>) -> Element {
    let translates: SurveyNewTranslate = translate(&lang);
    // FIXME: impelement handling with survey_id
    let mut ctrl = Controller::new(lang, survey_id);

    let attribute_options = ctrl.attribute_options();
    let selected_attributes = ctrl.selected_attributes();
    let selected_tab = ctrl.selected_tab();

    let total_counts = ctrl.total_counts();
    let attribute_combinations = ctrl.attribute_combinations();

    rsx! {
        div { class: "flex flex-col gap-[40px] items-end justify-start mb-[40px]",
            div { class: "flex flex-col w-full h-full justify-start items-start",
                div { class: "text-[#b4b4b4] font-medium text-[14px] mb-[10px]",
                    "{translates.survey_title}"
                }
                div { class: "flex flex-row w-full justify-start items-center mb-[40px]",
                    Link { class: "mr-[6px]", to: Route::SurveyPage { lang },
                        ArrowLeft { width: "24", height: "24", color: "#555462" }
                    }
                    div { class: "text-[#222222] font-semibold text-[28px]",
                        "{translates.start_survey}"
                    }
                }

                CreateSurvey {
                    lang,
                    visibility: ctrl.get_current_step() == CurrentStep::CreateSurvey,
                    value: ctrl.get_survey_request(),
                    onnext: move |req| ctrl.handle_survey_request(req),
                    onchange: move |req| ctrl.change_survey_request(req),
                }

                SettingAttribute {
                    lang,
                    survey_id,

                    attribute_combinations,

                    attribute_options,
                    selected_attributes,
                    selected_tab,

                    combination_error: ctrl.combination_error(),

                    clicked_complete_button: move |_| async move {
                        ctrl.clicked_start_project().await;
                    },

                    change_selected_tab: move |selected: bool| {
                        ctrl.change_selected_tab(selected);
                    },

                    add_selected_attribute: move |attribute: String| {
                        ctrl.add_selected_attribute(attribute);
                    },
                    remove_selected_attribute: move |index: usize| {
                        ctrl.remove_selected_attribute(index);
                    },
                    clear_selected_attributes: move |_| {
                        ctrl.clear_selected_attributes();
                    },

                    remove_attribute_option: move |(key, name): (String, String)| {
                        ctrl.remove_attribute_option(key, name);
                    },
                    update_attribute_rate: move |(key, name, rate): (String, String, i64)| {
                        ctrl.update_attribute_rate(key, name, rate);
                    },
                    change_attribute_combination_value: move |(index, total_count): (usize, usize)| {
                        ctrl.change_attribute_combination_value(index, total_count);
                    },

                    total_counts,
                    set_total_counts: move |total_counts: i64| {
                        ctrl.set_total_counts(total_counts);
                    },

                    visibility: ctrl.get_current_step() == CurrentStep::SettingPanel,
                    onback: move |_| ctrl.change_step(CurrentStep::CreateSurvey),
                }
                        // SettingPanel {
            //     lang,
            //     survey_id,
            //     visibility: ctrl.get_current_step() == CurrentStep::SettingPanel,
            //     onnext: move |req: PanelRequest| async move {
            //         ctrl.open_setting_reward_modal(lang, req).await;
            //     },
            //     onback: move || ctrl.change_step(CurrentStep::CreateSurvey),
            // }
            }
        }
    }
}
