use bdk::prelude::*;
use models::{DeliberationDiscussionCreateRequest, DiscussionCreateRequest};

use crate::{
    components::{expandable_card::ExpandableCard, icons::Trash, textarea::TextArea},
    pages::deliberations::new::{
        components::{calendar_dropdown::CalendarDropdown, clock_dropdown::ClockDropdown},
        details::discussions::i18n::DiscussionGroupTranslate,
    },
    utils::time::{current_timestamp, update_hour_in_timestamp},
};

#[component]
pub fn DiscussionGroup(
    lang: Language,
    discussion: DeliberationDiscussionCreateRequest,
    set_discussion: EventHandler<DeliberationDiscussionCreateRequest>,
) -> Element {
    let tr: DiscussionGroupTranslate = translate(&lang);

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,

            div { class: "flex flex-col w-full justify-start items-start gap-20",
                for (i , disc) in discussion.discussions.clone().iter().enumerate() {
                    div { class: "flex flex-col w-full justify-start items-center gap-10",
                        div { class: "flex flex-row w-full justify-between items-center",
                            div { class: "font-medium text-[15px] text-text-black",
                                {tr.setting_period}
                            }
                            div { class: "flex flex-row w-fit justify-start items-center gap-10",
                                div { class: "flex flex-row w-fit justify-start items-center gap-10",
                                    CalendarDropdown {
                                        id: format!("calendar_discussion_{}_start_date", i),
                                        date: disc.started_at,
                                        onchange: {
                                            let mut discussion = discussion.clone();
                                            let mut disc = disc.clone();
                                            move |e| {
                                                disc.started_at = e;
                                                discussion.discussions[i] = disc.clone();
                                                set_discussion.call(discussion.clone());
                                            }
                                        },
                                    }
                                    ClockDropdown {
                                        id: format!("clock_discussion_{}_start_date", i),
                                        time: disc.started_at,
                                        onchange: {
                                            let mut discussion = discussion.clone();
                                            let mut disc = disc.clone();
                                            move |hour: i64| {
                                                let date = disc.started_at;
                                                disc.started_at = update_hour_in_timestamp(date, hour as u32);
                                                discussion.discussions[i] = disc.clone();
                                                set_discussion.call(discussion.clone());
                                            }
                                        },
                                    }
                                }
                                div { class: "flex flex-row w-16 h-2 bg-label-border-gray" }
                                div { class: "flex flex-row w-fit justify-start items-center gap-10",
                                    CalendarDropdown {
                                        id: format!("calendar_discussion_{}_end_date", i),
                                        date: disc.ended_at,
                                        onchange: {
                                            let mut discussion = discussion.clone();
                                            let mut disc = disc.clone();
                                            move |e| {
                                                disc.ended_at = e;
                                                discussion.discussions[i] = disc.clone();
                                                set_discussion.call(discussion.clone());
                                            }
                                        },
                                    }
                                    ClockDropdown {
                                        id: format!("clock_discussion_{}_end_date", i),
                                        time: disc.ended_at,
                                        onchange: {
                                            let mut discussion = discussion.clone();
                                            let mut disc = disc.clone();
                                            move |hour: i64| {
                                                let date = disc.ended_at;
                                                disc.ended_at = update_hour_in_timestamp(date, hour as u32);
                                                discussion.discussions[i] = disc.clone();
                                                set_discussion.call(discussion.clone());
                                            }
                                        },
                                    }
                                }
                            }
                        }

                        div { class: "flex flex-row w-full justify-start items-center",
                            div { class: "max-w-150 w-full font-medium text-[15px] text-text-black",
                                {tr.discussion_topic}
                            }
                            div { class: "flex flex-row w-full h-55 justify-start items-center p-15 bg-background-gray rounded-sm",
                                input {
                                    class: "flex flex-row w-full h-full bg-transparent focus:outline-none placeholder:text-hint-gray placeholder:font-medium placeholder:text-[15px] font-medium text-[15px] text-text-black",
                                    r#type: "text",
                                    placeholder: tr.input_title_hint,
                                    value: disc.clone().name,
                                    oninput: {
                                        let mut discussion = discussion.clone();
                                        let mut disc = disc.clone();
                                        move |e: Event<FormData>| {
                                            disc.name = e.value();
                                            discussion.discussions[i] = disc.clone();
                                            set_discussion.call(discussion.clone());
                                        }
                                    },
                                }
                            }
                        }

                        div { class: "flex flex-row w-full justify-start items-center",
                            div { class: "max-w-150 w-full font-medium text-[15px] text-text-black",
                                {tr.discussion_hint}
                            }
                            TextArea {
                                placeholder: tr.input_description_hint,
                                value: disc.clone().description,
                                onchange: {
                                    let mut discussion = discussion.clone();
                                    let mut disc = disc.clone();
                                    move |value: String| {
                                        disc.description = value.clone();
                                        discussion.discussions[i] = disc.clone();
                                        set_discussion.call(discussion.clone());
                                    }
                                },
                            }
                        }

                        div { class: "flex flex-row w-full justify-end items-center gap-5",
                            button {
                                class: "cursor-pointer flex flex-row w-80 items-center justify-end",
                                onclick: {
                                    let mut discussion = discussion.clone();
                                    move |_| {
                                        discussion.discussions.remove(i);
                                        set_discussion.call(discussion.clone());
                                    }
                                },
                                div { class: "font-medium text-text-black text-[15px]",
                                    {tr.delete}
                                }
                                Trash { width: "18", height: "18" }
                            }
                        }
                    }
                }

                div { class: "relative w-full flex items-center justify-center my-20",
                    div { class: "border-t border-dashed border-gray-300 w-full" }
                    button {
                        class: "cursor-pointer absolute bg-background-gray border border-label-border-gray rounded-[100px] w-43 h-43 flex items-center justify-center shadow",
                        onclick: {
                            let mut discussion = discussion.clone();
                            let mut disc = DiscussionCreateRequest::default();
                            move |_| {
                                disc.started_at = current_timestamp();
                                disc.ended_at = current_timestamp();
                                discussion.discussions.push(disc.clone());
                                set_discussion.call(discussion.clone());
                            }
                        },
                        "+"
                    }
                }
            }
        }
    }
}
