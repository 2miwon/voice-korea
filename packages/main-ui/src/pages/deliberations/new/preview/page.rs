use crate::{
    components::updatable_card::UpdatableCard,
    pages::deliberations::new::preview::{controller::Controller, i18n::PreviewTranslate},
    routes::Route,
};

use bdk::prelude::*;

#[component]
pub fn Preview(lang: Language) -> Element {
    let tr: PreviewTranslate = translate(&lang);
    let mut ctrl = Controller::new(lang)?;
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-20",
            div { class: "font-medium text-base text-text-black mb-10", "최종 검토" }
            UpdatableCard {
                lang,
                enable_line: true,
                title: tr.composition_committee,
                route: Route::CompositionCommitee { lang },
                div { {tr.composition_committee} }
            }
            UpdatableCard {
                lang,
                enable_line: true,
                title: tr.composition_panel,
                route: Route::CompositionPanel { lang },
                div { {tr.composition_panel} }
            }
            UpdatableCard {
                lang,
                enable_line: false,
                title: tr.setting_deliberation_procedure,
                route: Route::DeliberationBasicInfoSettingPage {
                    lang,
                },
                div { {tr.setting_deliberation_procedure} }
            }
            div { class: "flex flex-row w-full justify-end items-end mt-20 mb-50",
                div {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {
                        ctrl.go_list();
                    },
                    {tr.go_to_list}
                }
                div {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| async move {
                        ctrl.temp_save().await;
                    },
                    {tr.temporary_save}
                }
                div {
                    class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                    onclick: move |_| {
                        ctrl.start_deliberation();
                    },
                    {tr.start}
                }
            }
        }
    }
}
