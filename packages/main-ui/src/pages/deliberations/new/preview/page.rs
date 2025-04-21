use crate::pages::deliberations::new::preview::{
    components::{
        committee::Committee, panel_summary_table::PanelSummaryTable, procedure::Procedure,
    },
    controller::Controller,
    i18n::PreviewTranslate,
};
use bdk::prelude::*;

#[component]
pub fn Preview(lang: Language) -> Element {
    let tr: PreviewTranslate = translate(&lang);
    let mut ctrl = Controller::new(lang)?;

    let roles = ctrl.roles();
    let committees = ctrl.committees();

    let emails = ctrl.emails();

    let basic_info = ctrl.basic_info();
    let sample_survey = ctrl.sample_survey();
    let deliberation = ctrl.deliberation();
    let discussion = ctrl.discussion();
    let final_survey = ctrl.final_survey();

    let basic_info_members = basic_info.clone().users;
    let sample_survey_members = sample_survey.clone().users;
    let deliberation_members = deliberation.clone().users;
    let discussion_members = discussion.clone().users;
    let final_survey_members = final_survey.clone().users;

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-20",
            div { class: "font-medium text-base text-text-black mb-10", {tr.final_review} }
            Committee { lang, roles, committees }
            PanelSummaryTable { lang, emails }
            Procedure {
                lang,
                basic_info,
                sample_survey,
                deliberation,
                discussion,
                final_survey,

                basic_info_members,
                sample_survey_members,
                deliberation_members,
                discussion_members,
                final_survey_members,
            }
            div { class: "flex flex-row w-full justify-end items-end mt-20 mb-50",
                button {
                    class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20 hover:!bg-primary hover:!text-white",
                    onclick: move |_| {
                        ctrl.go_list();
                    },
                    {tr.go_to_list}
                }
                button {
                    class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20 hover:!bg-primary hover:!text-white",
                    onclick: move |_| async move {
                        ctrl.temp_save().await;
                    },
                    {tr.temporary_save}
                }
                button {
                    class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                    onclick: move |_| {
                        spawn(async move {
                            ctrl.start_deliberation().await;
                        });
                    },
                    {tr.start}
                }
            }
        }
    }
}
