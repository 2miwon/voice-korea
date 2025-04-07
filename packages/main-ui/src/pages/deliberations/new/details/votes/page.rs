use bdk::prelude::*;

use crate::{
    components::icons::ArrowLeft,
    pages::deliberations::new::details::votes::components::introduction::Introduction,
};

use super::*;
use controller::*;
use i18n::*;

#[component]
pub fn DeliberationVoteSettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: VoteTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",

            div { class: "text-header-gray font-medium text-sm mb-10",
                "{tr.organization_management} / {tr.deliberation_management} / {tr.start_deliberation}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-25 gap-10",
                div { onclick: move |_| {},
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-header-black font-semibold text-[28px] mr-20", {tr.vote} }
            }

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", {tr.vote_setting} }
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    Introduction {
                        lang,
                        final_survey: ctrl.get_final_survey(),
                        set_final_survey: move |survey| {
                            ctrl.set_final_survey(survey);
                        },
                    }
                }
                div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {},
                        {tr.backward}
                    }
                    div {
                        class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {},
                        {tr.temporary_save}
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                        onclick: move |_| {},
                        {tr.next}
                    }
                }
            }
        }
    }
}
