use bdk::prelude::*;

use super::i18n::RemoveSurveyModalTranslate;

#[component]
pub fn RemoveSurveyModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    onremove: EventHandler<MouseEvent>,
) -> Element {
    let tr: RemoveSurveyModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col min-w-600 max-[600px]:min-w-320 justify-start items-start gap-40",
            div { class: "font-medium text-base text-text-black leading-22 whitespace-pre-line",
                {tr.description}
            }
            div { class: "flex flex-row w-full justify-start items-center gap-20",
                div {
                    class: "cursor-pointer flex flex-row bg-button-primary rounded-lg px-14 py-8 font-semibold text-white text-base",
                    onclick: move |e: Event<MouseData>| {
                        onclose.call(e);
                    },
                    {tr.maintain}
                }
                div {
                    class: "cursor-pointer flex flex-row bg-white px-14 py-8 font-semibold text-text-black text-base",
                    onclick: move |e: Event<MouseData>| {
                        onremove.call(e);
                    },
                    {tr.remove}
                }
            }
        }
    }
}
