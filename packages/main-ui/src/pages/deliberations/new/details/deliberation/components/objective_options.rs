use bdk::prelude::*;

use crate::components::icons::Minus;

#[component]
pub fn ObjectiveOptions(
    lang: Language,
    options: Vec<String>,
    change_option: EventHandler<(usize, String)>,
    remove_option: EventHandler<usize>,
    add_option: EventHandler<MouseEvent>,
) -> Element {
    let tr: ObjectiveOptionsTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-10",

            for (index , option) in options.iter().enumerate() {
                div { class: "flex flex-row w-full justify-start items-center mt-10",
                    div { class: "flex flex-row w-20 h-20 border-3 bg-white border-header-gray mr-10 rounded-[100px]" }
                    input {
                        class: "flex flex-row max-w-888 w-full h-55 justify-start items-center bg-white focus:outline-none border-b-1 border-header-gray px-15 py-15 font-medium text-[#9f9f9f] text-[15px] leading-22 mr-10",
                        r#type: "text",
                        placeholder: format!("{} {}", tr.option, index + 1),
                        value: option.clone(),
                        oninput: move |e: Event<FormData>| {
                            change_option.call((index, e.value()));
                        },
                    }
                    button {
                        onclick: move |_| {
                            remove_option.call(index);
                        },
                        Minus { width: "20", height: "20" }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-start items-center mt-30",
                div { class: "flex flex-row w-20 h-20 border-3 bg-white border-[#9f9f9f] mr-10 rounded-[100px]" }
                button {
                    class: "cursor-pointer font-medium text-base text-[#3a94ff]",
                    onclick: move |e| {
                        add_option.call(e);
                    },
                    {tr.add_option}
                }
            }
        }
    }
}

translate! {
    ObjectiveOptionsTranslate;

    option: {
        ko: "옵션",
        en: "Option"
    }
    add_option: {
        ko: "옵션 추가하기",
        en: "Add Option"
    }
}
