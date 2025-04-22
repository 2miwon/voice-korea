use bdk::prelude::*;

use crate::components::{
    icons::{Switch, Trash},
    pagination::Pagination,
};

#[component]
pub fn PanelTable(lang: Language, emails: Vec<String>, onremove: EventHandler<usize>) -> Element {
    let tr: PanelTableTranslate = translate(&lang);

    let mut current_page = use_signal(|| 1);
    let mut internal_emails = use_signal(|| vec![]);

    use_effect(use_reactive(&emails, move |emails| {
        internal_emails.set(emails.clone());
        current_page.set(1);
    }));

    let total_page = use_memo({
        move || {
            let total = internal_emails().len().max(1);
            (total - 1) / 7 + 1
        }
    });

    let paginated_emails = use_memo({
        let current_page = current_page.clone();
        move || {
            let emails = internal_emails();
            let start = (current_page() - 1) * 7;
            let end = (start + 7).min(emails.len());
            emails[start..end].to_vec()
        }
    });

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-label-border-gray mt-20",
            div { class: "flex flex-row w-full min-h-55 justify-start items-center",
                div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                    div { class: "text-table-text-gray font-semibold text-sm", {tr.email} }
                    Switch { width: "19", height: "19" }
                }
                div { class: "flex flex-row w-100 h-full justify-center items-center" }
            }

            for (i , email) in paginated_emails().iter().enumerate() {
                div { class: "flex flex-row w-full min-h-55 justify-start items-center border-t border-t-label-border-gray",
                    div { class: "flex flex-row flex-1 h-full justify-center items-center",
                        input {
                            r#type: "text",
                            class: "flex flex-row w-full justify-center items-center bg-transparent text-text-black focus:outline-none text-center",
                            value: email.clone(),
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
    PanelTableTranslate;

    email: {
        ko: "이메일",
        en: "Email"
    },
    remove: {
        ko: "삭제",
        en: "Remove"
    }
}
