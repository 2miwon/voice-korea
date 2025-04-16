use bdk::prelude::*;
use models::{File, ResourceFileSummary};

use crate::pages::deliberations::new::details::basic_info::components::{
    introduction::Introduction, material::Material, member::Member,
};

use super::*;
use controller::*;
use i18n::*;

#[component]
pub fn DeliberationBasicInfoSettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: BasicInfoTranslate = translate(&lang);
    let basic_info = ctrl.get_basic_info();

    let metadatas = ctrl.metadatas()?;

    let surveys = ctrl.surveys()?;

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", {tr.post_setting} }
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    Introduction {
                        lang,
                        basic_info,
                        start_date_id: "basic_start_date",
                        end_date_id: "basic_end_date",
                        set_title: move |title: String| {
                            ctrl.set_title(title);
                        },
                        set_description: move |description: String| {
                            ctrl.set_description(description);
                        },
                        set_start_date: move |start_date: i64| {
                            ctrl.set_start_date(start_date);
                        },
                        set_end_date: move |end_date: i64| {
                            ctrl.set_end_date(end_date);
                        },
                    }
                    Member {
                        lang,
                        total_committees: ctrl.get_committees(),
                        selected_committees: ctrl.get_selected_committee(),
                        add_committee: move |id: i64| {
                            ctrl.add_committee(id);
                        },
                        remove_committee: move |id: i64| {
                            ctrl.remove_committee(id);
                        },
                        clear_committee: move |_| {
                            ctrl.clear_committee();
                        },
                    }
                    Material {
                        lang,
                        metadatas,
                        resources: ctrl.get_selected_resources(),
                        total_surveys: surveys,
                        selected_surveys: ctrl.get_selected_surveys(),
                        create_resource: move |file: File| async move {
                            let _ = ctrl.create_resource(file).await;
                        },
                        remove_resource: move |id: i64| {
                            let _ = ctrl.delete_resource(id);
                        },
                        add_resource: move |resource: ResourceFileSummary| {
                            let _ = ctrl.add_resource(resource.into());
                        },
                        add_survey: move |id: i64| {
                            ctrl.add_survey(id);
                        },
                        remove_survey: move |id: i64| {
                            ctrl.remove_survey(id);
                        },
                        clear_survey: move |_| {
                            ctrl.clear_survey();
                        },
                    }
                }

                div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {
                            ctrl.back();
                        },
                        {tr.backward}
                    }
                    div {
                        class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| async move {
                            ctrl.temp_save().await;
                        },
                        {tr.temporary_save}
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                        onclick: move |_| {
                            ctrl.next();
                        },
                        {tr.next}
                    }
                }
            }
        }
    }
}
