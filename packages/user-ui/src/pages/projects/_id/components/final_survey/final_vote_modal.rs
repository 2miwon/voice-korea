use bdk::prelude::*;

use super::i18n::FinalVoteModalTranslate;

#[component]
pub fn FinalVoteModal(
    lang: Language,
    onsend: EventHandler<MouseEvent>,
    oncancel: EventHandler<MouseEvent>,
) -> Element {
    let tr: FinalVoteModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col min-w-[600px] max-[600px]:min-w-[350px] justify-start items-start gap-[40px]",
            div { class: "font-medium text-[14px] text-[#222222] leading-[22px] whitespace-pre-line",
                {tr.description}
            }
            div { class: "flex flex-row w-full justify-start items-center gap-[20px]",
                div {
                    class: "cursor-pointer flex flex-row bg-primary rounded-[8px] px-[14px] py-[8px] font-semibold text-white text-[16px]",
                    onclick: move |e: Event<MouseData>| {
                        onsend.call(e);
                    },
                    {tr.complete_voting}
                }
                div {
                    class: "cursor-pointer flex flex-row bg-white px-[14px] py-[8px] font-semibold text-light-gray text-[16px]",
                    onclick: move |e: Event<MouseData>| {
                        oncancel.call(e);
                    },
                    {tr.cancel}
                }
            }
        }
    }
}
