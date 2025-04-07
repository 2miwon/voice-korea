use bdk::prelude::*;
use by_components::icons::edit::Edit1;

use crate::routes::Route;

#[component]
pub fn UpdatableCard(
    lang: Language,
    enable_line: bool,
    title: String,
    route: Route,
    children: Element,
) -> Element {
    let tr: UpdatableCardTranslate = translate(&lang);
    let nav = use_navigator();
    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start px-40 py-24 bg-white rounded-sm gap-10",
            style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
            div { class: "flex flex-row w-full justify-between items-center",
                div { class: "font-bold text-text-black text-lg leading-26", {title} }
                div {
                    class: "cursor-pointer flex flex-row px-10 py-8 justify-start items-center gap-4 rounded-sm border-1 border-label-border-gray",
                    onclick: move |_| {
                        nav.push(route.clone());
                    },
                    Edit1 {}
                    div { class: "font-semibold text-base text-table-text-gray leading-24",
                        {tr.update}
                    }
                }
            }
            div {
                class: "flex flex-row w-full h-1 border border-period-border-gray aria-active:hidden",
                "aria-active": !enable_line,
            }
            {children}
        }
    }
}

translate! {
    UpdatableCardTranslate;
    update: {
        ko: "수정하기",
        en: "Update"
    }
}
