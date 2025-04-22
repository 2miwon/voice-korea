use bdk::prelude::*;

use crate::{
    components::{icons::Switch, pagination::Pagination, updatable_card::UpdatableCard},
    routes::Route,
};

#[component]
pub fn PanelSummaryTable(lang: Language, emails: Vec<String>) -> Element {
    let tr: PanelSummaryTableTranslate = translate(&lang);

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
        UpdatableCard {
            lang,
            enable_line: true,
            title: tr.composition_panel,
            route: Route::CompositionPanel { lang },
            div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-[#ededed] mt-20",
                div { class: "flex flex-row w-full min-h-55 justify-start items-center",
                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-10",
                        div { class: "font-medium text-[15px] text-text-black", {tr.email} }
                        Switch { width: "19", height: "19" }
                    }
                }

                for email in paginated_emails() {
                    div { class: "flex flex-row w-full min-h-55 justify-start items-center border-t border-t-[#ededed]",
                        div { class: "flex flex-row flex-1 h-full justify-center items-center",
                            input {
                                r#type: "text",
                                class: "flex flex-row w-full justify-center items-center bg-transparent font-normal text-[13px] text-text-black focus:outline-none text-center",
                                value: email.clone(),
                                readonly: true,
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
}

translate! {
    PanelSummaryTableTranslate;

    composition_panel: {
        ko: "참여자 패널 구성",
        en: "Participation Panel Composition"
    }
    email: {
        ko: "이메일",
        en: "Email"
    },
    remove: {
        ko: "삭제",
        en: "Remove"
    }
}
