#![allow(non_snake_case)]
use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

use models::{
    deliberation::DeliberationCreateRequest, deliberation_user::DeliberationUserCreateRequest,
    OrganizationMemberSummary, Role,
};

use crate::pages::deliberations::new::components::role_dropdown::RoleDropdown;

#[component]
pub fn CompositionCommitee(lang: Language) -> Element {
    let roles = vec![
        Role::Admin,
        Role::DeliberationAdmin,
        Role::Analyst,
        Role::Moderator,
        Role::Speaker,
    ];
    // FIXME: temporary request
    let req = DeliberationCreateRequest::default();
    let mut ctrl = Controller::new(lang)?;
    let tr: CompositionCommitteeTranslate = translate(&lang);

    let members = ctrl.members()?;

    let mut committee_roles: Signal<Vec<Vec<OrganizationMemberSummary>>> = use_signal(|| vec![]);

    use_effect({
        let roles = roles.clone();
        let members = members.clone();

        let committees = req.roles.clone();

        move || {
            for role in roles.clone() {
                let members = get_role_list(members.clone(), committees.clone(), role);

                committee_roles.push(members);
            }

            ctrl.committees.set(committees.clone());
        }
    });

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-base text-text-black mb-10",
                "{tr.composition_committee_title}"
            }

            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-24",
                div { class: "font-bold text-text-black text-lg mb-3", "{tr.division_roles}" }
                div { class: "font-normal text-text-gray text-sm mb-20",
                    "{tr.composition_committee_description}"
                }
                // selection box
                div { class: "flex flex-col w-full justify-start items-center mb-40",
                    for (i , committee_role) in committee_roles().iter().enumerate() {
                        RoleDropdown {
                            id: format!("{}_dropdown", roles[i].to_string()),
                            label: roles[i].translate(&lang),
                            hint: tr.enter_charge_person,
                            total_committees: ctrl.committees(),
                            members: members.clone(),
                            committees: committee_role.clone(),
                            add_committee: {
                                let role = roles[i].clone();
                                let members = members.clone();
                                move |user_id: i64| {
                                    ctrl.add_committee(DeliberationUserCreateRequest {
                                        user_id,
                                        role: role.clone(),
                                    });
                                    let mut list = committee_roles();
                                    if let Some(role_list) = list.get_mut(i) {
                                        let user = members.iter().find(|m| m.user_id == user_id);
                                        if let Some(user) = user {
                                            if !role_list.iter().any(|m| m.user_id == user_id) {
                                                role_list.push(user.clone());
                                            }
                                        }
                                    }
                                    committee_roles.set(list);
                                }
                            },
                            remove_committee: {
                                let role = roles[i].clone();
                                move |user_id: i64| {
                                    ctrl.remove_committee(user_id, role.clone());
                                    let mut list = committee_roles();
                                    if let Some(role_list) = list.get_mut(i) {
                                        role_list.retain(|m| m.user_id != user_id);
                                    }
                                    committee_roles.set(list);
                                }
                            },
                            clear_committee: {
                                let role = roles[i].clone();
                                move |_| {
                                    ctrl.clear_committee(role.clone());
                                    let mut list = committee_roles();
                                    if let Some(role_list) = list.get_mut(i) {
                                        role_list.clear();
                                    }
                                    committee_roles.set(list);
                                }
                            },
                        }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                div {
                    class: "flex flex-row w-70 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {
                        ctrl.back();
                    },
                    "{tr.backward}"
                }
                div {
                    class: "flex flex-row w-105 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {},
                    "{tr.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-110 h-55 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                    onclick: move |_| {
                        ctrl.next();
                    },
                    "{tr.next}"
                }
            }
        }
    }
}

pub fn get_role_list(
    members: Vec<OrganizationMemberSummary>,
    committees: Vec<DeliberationUserCreateRequest>,
    role: Role,
) -> Vec<OrganizationMemberSummary> {
    let user_ids: Vec<i64> = committees
        .iter()
        .filter(|committee| committee.role == role)
        .map(|committee| committee.user_id)
        .collect();

    let members = members
        .into_iter()
        .filter(|member| user_ids.contains(&member.user_id))
        .collect();

    members
}
