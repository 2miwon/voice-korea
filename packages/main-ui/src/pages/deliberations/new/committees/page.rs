#![allow(non_snake_case)]
use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

use crate::{
    components::section::MainSection,
    pages::deliberations::new::{
        committees::components::role_table::RoleTable,
        components::role_email_input_form::RoleEmailInputForm,
    },
};

#[component]
pub fn CompositionCommitee(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: CompositionCommitteeTranslate = translate(&lang);

    let roles = ctrl.roles();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-40",
            div { class: "flex flex-col w-full gap-10",
                div { class: "font-medium text-base text-text-black",
                    {tr.composition_committee_title}
                }

                MainSection {
                    lang,
                    required: false,
                    header: Some(tr.division_roles.to_string()),
                    description: Some(tr.composition_committee_description.to_string()),
                    // input box
                    div { class: "flex flex-col w-full justify-start items-start mt-5 gap-20",
                        div { class: "text-sm font-medium text-text-gray", {tr.role_input_info} }
                        div { class: "flex flex-col w-full justify-start items-start mt-5",
                            for (i , committee_email) in ctrl.committee_emails().iter().enumerate() {
                                RoleEmailInputForm {
                                    height: 55,
                                    label: roles[i].translate(&lang),
                                    placeholder: tr.enter_charge_person,
                                    value: committee_email,
                                    oninput: move |e: FormEvent| {
                                        ctrl.update_email_by_role(i, e.value());
                                    },
                                    onenter: {
                                        let role = roles[i].clone();
                                        move |_| {
                                            ctrl.add_email_by_role(i, role.clone());
                                        }
                                    },
                                }
                            }
                        }
                    }
                }

                MainSection {
                    lang,
                    required: false,
                    header: Some(tr.summary_title.to_string()),
                    description: Some(tr.summary_description.to_string()),
                    RoleTable {
                        lang,
                        members: ctrl.committees(),
                        onremove: move |index: usize| {
                            ctrl.remove_email_by_role(index);
                        },
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                button {
                    class: "flex flex-row w-70 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20 hover:!bg-primary hover:!text-white",
                    onclick: move |_| {
                        ctrl.save_deliberation();
                        ctrl.back();
                    },
                    {tr.backward}
                }
                button {
                    class: "flex flex-row w-105 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20 hover:!bg-primary hover:!text-white",
                    onclick: move |_| async move {
                        ctrl.temp_save().await;
                    },
                    {tr.temporary_save}
                }
                button {
                    class: "aria-active:cursor-pointer cursor-not-allowed flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-disabled aria-active:!bg-hover font-semibold text-base text-white",
                    "aria-active": ctrl.is_valid(),
                    onclick: move |_| ctrl.next(),
                    {tr.next}
                }
            }
        }
    }
}
