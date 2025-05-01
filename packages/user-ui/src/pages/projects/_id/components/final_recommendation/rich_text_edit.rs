use bdk::prelude::*;
use by_components::rich_texts::RichText;

use crate::{
    components::input::InputBox,
    pages::projects::_id::components::final_recommendation::i18n::RichTextEditTranslate,
};

#[component]
pub fn RichTextEdit(
    lang: Language,
    title: String,
    description: String,
    visibility: bool,

    on_title_change: EventHandler<String>,
    on_description_change: EventHandler<String>,

    onupdate: EventHandler<MouseEvent>,
) -> Element {
    let tr: RichTextEditTranslate = translate(&lang);
    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start aria-active:hidden",
            "aria-active": !visibility,
            div { class: "flex flex-col min-w-350 w-full justify-center items-center gap-15",
                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "font-semibold text-[15px] text-text-black", {tr.title} }
                    InputBox {
                        class: "flex flex-row w-full rounded-[10px] px-15 py-10 placeholder-hint-gray bg-transparent text-text-black border border-gray-300 focus:outline-none focus:border focus:border-button-primary",
                        placeholder: tr.title_hint,
                        value: title,
                        onchange: move |value: String| {
                            on_title_change.call(value);
                        },
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "font-semibold text-[15px] text-text-black", {tr.description} }
                    RichText {
                        content: description,
                        onchange: move |value: String| {
                            on_description_change.call(value);
                        },
                    }
                }

                div {
                    class: "cursor-pointer flex flex-row w-200 justify-center items-center bg-button-primary rounded-lg px-16 py-14 font-bold text-white text-base",
                    onclick: move |e: Event<MouseData>| {
                        onupdate.call(e);
                    },
                    {tr.update}
                }
            }
        }
    }
}
