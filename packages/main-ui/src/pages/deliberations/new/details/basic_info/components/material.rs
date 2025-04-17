use bdk::prelude::*;
use models::{File, ResourceFile, ResourceFileSummary, SurveyV2Summary};

use crate::{
    components::expandable_card::ExpandableCard,
    pages::{
        deliberations::new::{
            components::survey_dropdown::SurveyDropdown,
            details::basic_info::i18n::{BasicMaterialTranslate, ConnectProjectTranslate},
        },
        MaterialUpload,
    },
};

#[component]
pub fn Material(
    lang: Language,
    total_surveys: Vec<SurveyV2Summary>,
    selected_surveys: Vec<SurveyV2Summary>,
    metadatas: Vec<ResourceFileSummary>,
    resources: Vec<ResourceFile>,

    create_resource: EventHandler<File>,
    add_resource: EventHandler<ResourceFileSummary>,
    remove_resource: EventHandler<i64>,
    add_survey: EventHandler<i64>,
    remove_survey: EventHandler<i64>,
    clear_survey: EventHandler<MouseEvent>,
) -> Element {
    let tr: BasicMaterialTranslate = translate(&lang);

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            div { class: "flex flex-col w-full justify-start items-start gap-20",
                MaterialUpload {
                    lang,
                    resources,
                    metadatas,
                    oncreate: move |file: File| {
                        create_resource.call(file);
                    },
                    onremove: move |id: i64| {
                        remove_resource.call(id);
                    },
                    onadd: move |file: ResourceFileSummary| {
                        add_resource.call(file);
                    },
                }
                div { class: "flex flex-row w-full h-1 bg-period-border-gray" }
                Project {
                    lang,

                    total_surveys,
                    selected_surveys,

                    add_survey,
                    remove_survey,
                    clear_survey,
                }
            }
        }
    }
}

#[component]
pub fn Project(
    lang: Language,

    total_surveys: Vec<SurveyV2Summary>,
    selected_surveys: Vec<SurveyV2Summary>,

    add_survey: EventHandler<i64>,
    remove_survey: EventHandler<i64>,
    clear_survey: EventHandler<MouseEvent>,
) -> Element {
    let tr: ConnectProjectTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-10",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "text-lg font-bold text-text-black", {tr.title} }
                div { class: "text-sm font-normal text-text-gray", {tr.description} }
            }

            SurveyDropdown {
                id: "basic-survey",
                hint: tr.survey_hint,

                selected_surveys,
                surveys: total_surveys,

                add_survey: move |survey: SurveyV2Summary| {
                    add_survey.call(survey.id);
                },
                remove_survey: move |id: i64| {
                    remove_survey.call(id);
                },
                clear_survey: move |e| {
                    clear_survey.call(e);
                },
            }
        }
    }
}
