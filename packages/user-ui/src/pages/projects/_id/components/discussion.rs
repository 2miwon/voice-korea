#![allow(non_snake_case, dead_code, unused_variables)]
use by_components::icons::upload_download::Download2;
use by_macros::*;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::*;
use models::{
    discussions::Discussion, DeliberationDiscussion, DeliberationDiscussionQuery,
    DeliberationDiscussionSummary, Tab,
};

use crate::{
    pages::projects::_id::components::accordion::Accordion,
    utils::time::{current_timestamp, format_time_range, formatted_timestamp},
};

#[derive(Translate, PartialEq, Default, Debug)]
pub enum DiscussionStatus {
    #[default]
    #[translate(ko = "예정된 토론", en = "Upcoming Discussion")]
    Ready,
    #[translate(ko = "진행중인 토론", en = "Ongoing Discussion")]
    InProgress,
    #[translate(ko = "종료된 토론", en = "Closed Discussion")]
    Finish,
}

#[component]
pub fn DiscussionTab(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let tr: DiscussionTranslate = translate(&lang);

    let deliberation_discussion: DeliberationDiscussionSummary = ctrl.discussion()?;

    let tab_title: &str = Tab::Discussion.translate(&lang);

    rsx! {
        div {
            id: "discussion",
            class: "max-[1000px]:px-30 flex flex-col w-full h-fit bg-box-gray gap-20",
            ..attributes,
            // header
            div { class: "w-full flex flex-row max-[500px]:flex-col max-[500px]:items-start max-[500px]:justify-start max-[500px]:gap-5 justify-between items-center mt-28",
                div { class: " font-semibold text-xl", "{tab_title}" }
                div { class: "font-medium text-[15px] text-black",
                    {
                        format!(
                            "{} ~ {}",
                            formatted_timestamp(lang, deliberation_discussion.started_at),
                            formatted_timestamp(lang, deliberation_discussion.ended_at),
                        )
                    }
                }
            }

            // information section
            div { class: "flex flex-col gap-10",

                // introduction section
                DiscussionIntroduction {
                    lang,
                    description: deliberation_discussion.description,
                    discussions: deliberation_discussion.discussions.clone(),
                }

                // video section
                VideoDiscussion {
                    lang,
                    discussions: deliberation_discussion.discussions.clone(),
                    start_meeting: move |id: i64| async move {
                        let _ = ctrl.start_meeting(id).await;
                    },
                }

                //Related Data
                div { class: "w-full flex flex-col rounded-lg mb-40 bg-white justify-start items-center py-14 px-20",
                    // title and button
                    div { class: "w-full flex justify-start items-center gap-13",
                        div { class: "w-180 flex flex-row items-center text-base font-bold",
                            span { "{tr.related_materials_title}" }
                        }
                        //file
                        div { class: "flex flex-wrap flex-1 justify-start items-center gap-8",
                            for resource in deliberation_discussion.resources {
                                div {
                                    class: "cursor-pointer flex flex-row justify-start items-center rounded-[100px] bg-light-gray gap-4 px-12 py-4",
                                    onclick: {
                                        let files = resource.files.clone();
                                        move |_| {
                                            let files = files.clone();
                                            async move {
                                                for file in files.clone() {
                                                    let name = file.name;
                                                    let link = file.url;
                                                    ctrl.download_file(name, link).await;
                                                }
                                            }
                                        }
                                    },
                                    Download2 {
                                        width: "18",
                                        height: "18",
                                        class: " [&>path]:fill-white",
                                    }
                                    div { class: "font-medium text-sm text-white", {resource.title} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn VideoDiscussion(
    lang: Language,
    discussions: Vec<Discussion>,
    start_meeting: EventHandler<i64>,
) -> Element {
    let tr: DiscussionTranslate = translate(&lang);

    rsx! {
        Accordion { title: tr.video_discussion,
            for discussion in discussions.iter() {
                div { class: "flex flex-col w-full gap-20",
                    Video {
                        lang,
                        discussion: discussion.clone(),
                        start_meeting,
                    }
                }
            }
        }
    }
}

#[component]
pub fn Video(lang: Language, discussion: Discussion, start_meeting: EventHandler<i64>) -> Element {
    let tr: DiscussionTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-row w-full justify-start items-start gap-20 rounded-lg",
            div { class: "w-240",
                img { src: asset!("/public/images/video.png"), width: 240 }
            }

            div { class: "flex flex-col w-full justify-between items-start",
                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "flex flex-col w-full gap-5",
                        div { class: "font-medium text-sm text-black leading-17",
                            {
                                get_discussion_status(discussion.started_at, discussion.ended_at)
                                    .translate(&lang)
                            }
                        }
                        div { class: "font-bold text-lg text-black leading-22", "{discussion.name}" }
                    }

                    div { class: "font-medium text-sm text-review-gray",
                        {
                            format!(
                                "{} · {} {}",
                                formatted_timestamp(lang, discussion.started_at),
                                tr.participant,
                                discussion.user_id.len(),
                            )
                        }
                    }
                }

                div { class: "flex flex-row w-full justify-end items-end",
                    div {
                        class: "cursor-pointer flex flex-row min-w-240 px-10 py-8 justify-center items-center bg-button-primary rounded-lg",
                        visibility: if get_discussion_status(discussion.started_at, discussion.ended_at)
    != DiscussionStatus::InProgress { "hidden" },
                        div {
                            class: "font-medium text-base text-white",
                            onclick: move |_| {
                                start_meeting.call(discussion.id);
                            },
                            "{tr.involved}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DiscussionIntroduction(
    lang: Language,
    description: String,
    discussions: Vec<Discussion>,
) -> Element {
    let tr: DiscussionTranslate = translate(&lang);
    rsx! {
        Accordion { title: tr.sub_title, default_open: true,
            div {
                class: "font-bold text-lg text-black",
                dangerous_inner_html: description,
            }
            for discussion in discussions {
                DiscussionTable { lang, discussion }
            }
        }
    }
}

#[component]
pub fn DiscussionTable(lang: Language, discussion: Discussion) -> Element {
    let tr: DiscussionTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-3/5 justify-start items-start border rounded-lg border-discussion-border-gray",
            div { class: "flex flex-row min-h-55 w-full justify-center items-center border-b border-b-discussion-border-gray font-semibold text-sm text-light-gray",
                {formatted_timestamp(lang, discussion.started_at)}
            }
            div { class: "flex flex-row min-h-55 w-full justify-center items-center border-b border-b-discussion-border-gray font-semibold text-sm text-light-gray",
                div { class: "flex flex-row min-w-200 justify-center items-center",
                    "{tr.time}"
                }
                div { class: "flex flex-row min-w-200 justify-center items-center",
                    "{tr.activity}"
                }
                div { class: "flex flex-row flex-1 justify-center items-center", "{tr.content}" }
            }
            div { class: "flex flex-row min-h-55 w-full justify-center items-center font-semibold text-sm text-text-black",
                div { class: "flex flex-row min-w-200 justify-center items-center",
                    {format_time_range(discussion.started_at, discussion.ended_at)}
                }
                div { class: "flex flex-row min-w-200 justify-center items-center",
                    "{discussion.name}"
                }
                div { class: "flex flex-row flex-1 justify-center items-center",
                    "{discussion.description}"
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,

    discussion: Resource<DeliberationDiscussionSummary>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let discussion = use_server_future(move || async move {
            let res = DeliberationDiscussion::get_client(&crate::config::get().api_url)
                .query(project_id(), DeliberationDiscussionQuery::new(1))
                .await
                .unwrap_or_default();
            if res.items.is_empty() {
                DeliberationDiscussionSummary::default()
            } else {
                res.items[0].clone()
            }
        })?;

        let ctrl = Self {
            lang,
            project_id,
            discussion,
        };

        Ok(ctrl)
    }

    pub async fn start_meeting(&self, discussion_id: i64) {
        let project_id = self.project_id();
        let _ = Discussion::get_client(&crate::config::get().api_url)
            .start_meeting(project_id, discussion_id)
            .await;
    }

    pub async fn download_file(&self, name: String, url: Option<String>) {
        if url.is_none() {
            return;
        }

        let url = url.unwrap_or_default();

        #[cfg(feature = "web")]
        {
            use wasm_bindgen::JsCast;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", &name).unwrap();

            document.body().unwrap().append_child(&a).unwrap();
            let a: web_sys::HtmlElement = a.unchecked_into();
            a.click();
            a.remove();
        }
    }
}

translate! {
    DiscussionTranslate;

    title: {
        ko: "토론",
        en: "Discussion",
    },

    sub_title: {
        ko: "주요 내용",
        en: "Highlights"
    }

    video_discussion: {
        ko: "화상 토론",
        en: "Video Discussion"
    }

    related_materials_title: {
        ko: "관련 자료",
        en: "Related materials"
    }

    time: {
        ko: "시간",
        en: "Time"
    }

    activity: {
        ko: "활동",
        en: "Activity"
    }

    content: {
        ko: "내용",
        en: "Content"
    }

    participant: {
        ko: "참여자",
        en: "Participant"
    }

    involved: {
        ko: "참여하기",
        en: "Involved"
    }
}

pub fn get_discussion_status(started_at: i64, ended_at: i64) -> DiscussionStatus {
    let current = current_timestamp();

    if started_at > 10000000000 {
        tracing::error!("time parsing failed");
        return DiscussionStatus::default();
    }

    if started_at > current {
        DiscussionStatus::Ready
    } else if ended_at < current {
        DiscussionStatus::Finish
    } else {
        DiscussionStatus::InProgress
    }
}
