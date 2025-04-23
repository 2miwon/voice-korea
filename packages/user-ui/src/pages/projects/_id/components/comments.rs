use by_components::icons::chat::SquareChat;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::pages::projects::_id::{components::comment_item::CommentItem, controller::CommentTree};

#[component]
pub fn Comment(
    lang: Language,
    comments: Vec<CommentTree>,
    send_comment: EventHandler<String>,
    send_reply: EventHandler<(i64, String)>,
    like_comment: EventHandler<i64>,
    is_login: bool,
) -> Element {
    let tr: CommentTranslate = translate(&lang);
    let mut comment = use_signal(|| "".to_string());

    rsx! {
        div { class: "max-w-desktop flex flex-row w-full justify-center items-center mt-[40px] max-desktop:px-20",
            div { class: "flex flex-col w-full justify-start items-start gap-[20px]",
                div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                    div { class: "w-full h-[24px] flex flex-row justify-start items-center gap-[8px]",
                        div { SquareChat {} }
                        p { "{comments.len()}" }
                    }

                    // text write area
                    div { class: "w-full relative border-[1px] border-[#B4B4B4] rounded-lg flex justify-start items-center pl-[12px] gap-[8px]",
                        SquareChat { color: "#8095EA" }
                        // text input area
                        div { class: "flex flex-row font-semibold text-[15px] w-full",
                            input {
                                class: "flex-1 outline-none  py-12",
                                placeholder: tr.reply_box_text,
                                value: comment(),
                                oninput: move |e| comment.set(e.value().clone()),
                                onkeypress: move |e| {
                                    if e.key() == Key::Enter {
                                        e.prevent_default();
                                        send_comment.call(comment());
                                        comment.set("".to_string());
                                    }
                                },
                            }
                            button {
                                class: "bg-primary disabled:bg-gray-400 text-white rounded-r-lg px-12 min-w-80 desktop:hidden",
                                disabled: !is_login,
                                onclick: move |_| {
                                    send_comment.call(comment());
                                    comment.set("".to_string());
                                },
                                {tr.submit}
                            }
                        }
                    }

                    //comments
                    div { class: "w-full h-auto flex flex-col justify-center items-start mt-[20px]",
                        for comment in comments.clone() {
                            CommentItem {
                                lang,
                                comment: comment.clone(),
                                like_comment: move |_| {
                                    like_comment.call(comment.id);
                                },
                                send_reply: move |(id, reply)| {
                                    send_reply.call((id, reply));
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

translate! {
    CommentTranslate;

    reply: {
        ko: "답글",
        en: "Reply"
    }

    submit: {
        ko: "제출",
        en: "Submit"
    }

    unit: {
        ko: "개",
        en: "Unit"
    }

    reply_comment: {
        ko: "답글하기",
        en: "Reply"
    }

    reply_box_text: {
        ko: "답글 남기기...",
        en: "Leave a reply..."
    }

    anonymity: {
        ko: "익명",
        en: "Anonymity"
    }
}
