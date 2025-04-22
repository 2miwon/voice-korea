use bdk::prelude::btracing;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::components::{close_label::CloseLabel, form_field::InputEnterField, icons::Remove};

#[component]
pub fn EmailDropdown(
    lang: Language,
    id: String,
    hint: String,

    selected_committees: Vec<String>,
    committees: Vec<String>,

    add_committee: EventHandler<String>,
    remove_committee: EventHandler<String>,
    clear_committee: EventHandler<MouseEvent>,
) -> Element {
    let tr: EmailDropdownTranslate = translate(&lang);

    #[cfg(feature = "web")]
    use crate::components::outside_hook::eventhook::use_outside_click;

    let mut is_focused = use_signal(|| false);
    let mut email = use_signal(|| "".to_string());

    #[cfg(feature = "web")]
    use_outside_click(&id, move |_| is_focused.set(false));

    rsx! {
        div {
            id,
            class: "cursor-pointer relative flex flex-row w-full h-55 justify-center items-center bg-background-gray rounded-md",
            onclick: move |_| {
                let prev = is_focused();
                is_focused.set(!prev);
            },

            div { class: "flex flex-row w-full items-center px-15 py-10 gap-10 justify-between",

                if selected_committees.clone().len() != 0 {
                    div {
                        class: "flex flex-wrap flex-1 gap-10",
                        visibility: if selected_committees.clone().len() != 0 { "flex" } else { "hidden" },
                        for committee in selected_committees.clone() {
                            CloseLabel {
                                label: committee.clone(),
                                onremove: move |event: Event<MouseData>| {
                                    event.stop_propagation();
                                    event.prevent_default();
                                    remove_committee.call(committee.clone());
                                },
                            }
                        }
                    }
                    button {
                        onclick: move |event: Event<MouseData>| {
                            event.stop_propagation();
                            event.prevent_default();
                            clear_committee.call(event);
                        },
                        Remove { width: "20", height: "20", fill: "#555462" }
                    }
                } else {
                    div { class: "font-medium text-[15px] text-hint-gray bg-background-gray",
                        "{hint}"
                    }
                }
            }
            if is_focused() {
                div {
                    class: "absolute top-full bg-white border border-label-border-gray shadow-lg rounded-lg w-full h-fit z-50",
                    onclick: move |event| {
                        event.stop_propagation();
                        event.prevent_default();
                    },

                    div { class: "flex flex-col w-full justify-start items-start",
                        InputEnterField {
                            placeholder: tr.email_hint,
                            value: email(),
                            oninput: move |e: FormEvent| {
                                email.set(e.value());
                            },
                            onenter: {
                                let committees = committees.clone();
                                move |_| {
                                    if !committees.contains(&email()) {
                                        btracing::error!("{}", tr.input_error);
                                        return;
                                    }
                                    add_committee.call(email());
                                    email.set("".to_string());
                                }
                            },
                        }
                        div { class: "flex flex-col w-full justify-start items-center bg-white h-150 overflow-y-scroll",
                            for committee in committees
                                .clone()
                                .into_iter()
                                .filter(|c| {
                                    !selected_committees.contains(c)
                                        && (email().is_empty()
                                            || c.to_lowercase().contains(&email().to_lowercase()))
                                })
                            {
                                button {
                                    class: "flex flex-col w-full justify-start items-start px-12 py-10 hover:bg-background-gray hover:border-l-2 hover:border-hover",
                                    onclick: move |event: Event<MouseData>| {
                                        event.stop_propagation();
                                        event.prevent_default();
                                        add_committee.call(committee.clone());
                                        is_focused.set(false);
                                        email.set("".to_string());
                                    },
                                    div { class: "font-bold text-text-black text-[15px] mb-5",
                                        "{committee}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

translate! {
    EmailDropdownTranslate;

    email_hint: {
        ko: "이메일을 입력하세요. ex) test@test.test",
        en: "Enter your email address. ex) test@test.test"
    }

    input_error: {
        ko: "이메일을 다시 확인해주세요.",
        en: "Please check your email again."
    }
}
