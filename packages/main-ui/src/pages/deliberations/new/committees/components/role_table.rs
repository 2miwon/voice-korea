use bdk::prelude::*;
use dioxus_translate::Language;
use models::deliberation_role::DeliberationRoleCreateRequest;

use crate::components::{
    icons::{Switch, Trash},
    pagination::Pagination,
};

#[component]
pub fn RoleTable(
    lang: Language,
    members: Vec<DeliberationRoleCreateRequest>,
    onremove: EventHandler<usize>,
) -> Element {
    let tr: RoleTableTranslate = translate(&lang);
    let mut role_members = use_signal(|| vec![]);
    let mut current_page = use_signal(|| 1);

    let total_page = use_memo({
        let members = role_members.clone();
        move || {
            let total = members().len().max(1);
            (total - 1) / 7 + 1
        }
    });

    let paginated_members = use_memo({
        let members = role_members.clone();
        let current_page = current_page.clone();
        move || {
            let current_members = members();
            let start = (current_page() - 1) * 7;
            let end = (start + 7).min(current_members.len());
            current_members[start..end].to_vec()
        }
    });

    use_effect(use_reactive(&members, move |members| {
        role_members.set(members);
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-label-border-gray mt-20",
            // Header
            div { class: "flex flex-row w-full min-h-55 justify-start items-center",
                div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                    div { class: "text-table-text-gray font-semibold text-sm", {tr.email} }
                    Switch { width: "19", height: "19" }
                }
                div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                    div { class: "text-table-text-gray font-semibold text-sm", {tr.role} }
                    Switch { width: "19", height: "19" }
                }
                div { class: "flex flex-row w-100 h-full justify-center items-center" }
            }

            // Body
            for (i , member) in paginated_members().iter().enumerate() {
                div { class: "flex flex-row w-full min-h-55 justify-start items-center border-t border-t-label-border-gray",
                    div { class: "flex flex-row flex-1 h-full justify-center items-center",
                        input {
                            r#type: "text",
                            class: "flex flex-row w-full justify-center items-center bg-transparent text-text-black focus:outline-none text-center",
                            value: member.email.clone(),
                            readonly: true,
                        }
                    }
                    div { class: "flex flex-row flex-1 h-full justify-center items-center",
                        input {
                            r#type: "text",
                            class: "flex flex-row w-full bg-transparent text-text-black focus:outline-none text-center",
                            value: member.role.translate(&lang),
                            readonly: true,
                        }
                    }
                    div {
                        class: "cursor-pointer flex flex-row w-100 h-full justify-center items-center",
                        onclick: move |_| {
                            let global_index = (current_page() - 1) * 7 + i;
                            onremove.call(global_index);
                        },
                        div { class: "flex flex-row w-fit h-fit px-8 py-4 border border-delete-border-gray rounded-sm gap-5",
                            div { class: "font-medium text-sm text-table-text-gray leading-22",
                                {tr.remove}
                            }
                            Trash { width: "18", height: "18" }
                        }
                    }
                }
            }
        }

        // Pagination
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

translate! {
    RoleTableTranslate;

    email: {
        ko: "이메일",
        en: "Email"
    },
    role: {
        ko: "역할",
        en: "Role"
    }
    remove: {
        ko: "삭제",
        en: "Remove"
    }
}
