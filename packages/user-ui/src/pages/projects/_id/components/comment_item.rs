use by_components::icons::{
    arrows::{ChevronDown, ChevronUp},
    chat::SquareChat,
    emoji::ThumbsUp,
};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::AvatarLabel,
    pages::projects::_id::{components::comments::CommentTranslate, controller::CommentTree},
    utils::time::format_prev_time,
};

#[component]
pub fn CommentItem(
    lang: Language,
    comment: CommentTree,
    like_comment: EventHandler<MouseEvent>,

    send_reply: EventHandler<(i64, String)>,
) -> Element {
    let prev_time = format_prev_time(comment.created_at);
    let tr: CommentTranslate = translate(&lang);

    let mut show_comments = use_signal(|| false);
    let mut show_reply = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",

            div { class: "flex flex-col w-full justify-start items-start gap-[14px]",

                AvatarLabel {
                    label: if let Some(nickname) = comment.nickname { nickname } else { tr.anonymity.to_string() },
                    sub: prev_time,
                }
                // Comment List Section
                div { class: "flex flex-col w-full justify-start items-start gap-[20px] px-[40px]",
                    div { class: "font-medium text-[16px] text-[#222222]", {comment.comment} }
                    div { class: "flex flex-row w-full justify-between items-end",
                        div { class: " flex flex-row gap-40 max-[500px]:gap-10",
                            div {
                                class: "cursor-pointer flex flex-row gap-[8px] w-fit justify-start items-center max-[500px]:gap-4",
                                onclick: move |_| {
                                    show_comments.set(!show_comments());
                                },
                                div { class: "font-medium text-[16px] text-[#2A60D3] leading-[24px] max-[500px]:text-sm",
                                    {format!("{} {} {}", tr.reply, comment.replies, tr.unit)}
                                }
                                if show_comments() {
                                    ChevronUp {
                                        class: "[&>path]:stroke-[#2A60D3]",
                                        width: "24px",
                                        height: "24px",
                                    }
                                } else {
                                    ChevronDown {
                                        class: "[&>path]:stroke-[#2A60D3]",
                                        width: "24px",
                                        height: "24px",
                                    }
                                }
                            }
                            div {
                                class: "cursor-pointer flex flex-row justify-start items-center",
                                onclick: move |_| {
                                    show_reply.set(!show_reply());
                                },
                                div { class: "font-medium text-[16px] text-[#222222] leading-[24px] max-[500px]:text-sm",
                                    {tr.reply_comment}
                                }
                            }
                        }
                        div { class: "flex flex-row justify-center gap-40 max-[500px]:gap-20",
                            div { class: "flex flex-row w-fit justify-start items-center gap-[8px]",
                                SquareChat {}
                                p { "{comment.replies}" }
                            }
                            div {
                                class: format!(
                                    "{} flex flex-row w-fit justify-start items-center gap-[8px]",
                                    if comment.liked { "cursor-not-allowed" } else { "cursor-pointer" },
                                ),
                                onclick: move |e: Event<MouseData>| {
                                    if !comment.liked {
                                        like_comment.call(e);
                                    }
                                },
                                div {
                                    ThumbsUp { fill: if comment.liked { "#18B583" } else { "none" } }
                                }
                                p { "{comment.likes}" }
                            }
                        }
                    }
                }

                // Reply Section
                if show_reply() {
                    div { class: "flex flex-row w-full px-[40px]",
                        ReplyComment {
                            lang,
                            send_reply: move |reply: String| {
                                send_reply.call((comment.id, reply));
                            },
                        }
                    }
                }


                // Reply List Section
                if show_comments() {
                    div { class: "flex flex-row w-full px-[40px]",
                        ReplyCommentList { lang, replies: comment.children }
                    }
                }
            }

            div { class: "flex flex-row w-full h-[1px] justify-start items-start bg-[#EEEEEE] my-[20px]" }
        }
    }
}

#[component]
pub fn ReplyCommentList(lang: Language, replies: Vec<CommentTree>) -> Element {
    let tr: CommentTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start pl-[14px] gap-[20px]",
            for reply in replies {
                div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                    AvatarLabel {
                        label: if let Some(nickname) = reply.nickname { nickname } else { tr.anonymity.to_string() },
                        sub: format_prev_time(reply.created_at),
                    }
                    div { class: "flex flex-row w-full justify-start items-start pl-[32px]",
                        div { class: "font-medium text-[16px] text-[#222222] leading-[24px]",
                            {reply.comment}
                        }
                    }

                    div { class: "flex flex-row w-full h-[1px] justify-start items-start bg-[#EEEEEE]" }
                }
            }
        }
    }
}

#[component]
pub fn ReplyComment(lang: Language, send_reply: EventHandler<String>) -> Element {
    let tr: CommentTranslate = translate(&lang);
    let mut reply = use_signal(|| "".to_string());

    rsx! {
        div { class: "max-w-[1300px] min-h-[48px] w-full relative border-[1px] border-[#2A60D3] rounded-[8px] flex justify-start items-center px-[14px] py-[12px] gap-[8px]",

            // text input area
            div { class: "flex flex-col w-full justify-end items-end ",
                input {
                    class: "w-full font-semibold text-[15px] leading-normal outline-none",
                    placeholder: tr.reply_box_text,
                    value: "{reply()}",
                    oninput: move |e| reply.set(e.value().clone()),
                    onkeypress: move |e| {
                        if e.key() == Key::Enter {
                            e.prevent_default();
                            send_reply.call(reply());
                            reply.set("".to_string());
                        }
                    },
                }

                div { class: "w-[40px] h-[40px] flex justify-center items-center bg-[#EBEFF5] rounded-full",
                    SquareChat { color: "#8095EA" }
                }
            }
        }
    }
}
