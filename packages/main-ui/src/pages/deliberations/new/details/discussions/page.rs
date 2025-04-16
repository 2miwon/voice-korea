use super::super::components::introduction_card::IntroductionCard;
use super::*;
use crate::pages::deliberations::new::details::discussions::components::{
    discussion_group::DiscussionGroup, document::Document, member::DiscussionMember,
};
use bdk::prelude::*;
use controller::*;
use i18n::*;
use models::{DeliberationDiscussionCreateRequest, File};

// TODO: implement discussion
#[component]
pub fn DeliberationDiscussionSettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: DiscussionTranslate = translate(&lang);
    let discussion = ctrl.discussion();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", {tr.discussion_setting} }
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    IntroductionCard {
                        lang,
                        description: tr.introduction_description.to_string(),
                        text_value: discussion.clone().title,
                        started_at: discussion.clone().started_at,
                        ended_at: discussion.clone().ended_at,
                        content: discussion.clone().description,
                        set_title: move |title: String| {
                            ctrl.set_title(title);
                        },
                        set_description: move |description: String| {
                            ctrl.set_description(description);
                        },
                        set_start_date: move |timestamp: i64| {
                            ctrl.set_start_date(timestamp);
                        },
                        set_end_date: move |timestamp: i64| {
                            ctrl.set_end_date(timestamp);
                        },
                    }
                    Document {
                        lang,
                        discussion: discussion.clone(),
                        set_discussion: move |disc| {
                            ctrl.set_discussion(disc);
                        },

                        create_metadata: move |file: File| async move {
                            ctrl.create_metadata(file).await;
                        },
                        remove_resource: move |id: i64| {
                            ctrl.remove_resource(id);
                        },
                        clear_resource: move |_| {
                            ctrl.clear_resource();
                        },
                        selected_resources: ctrl.get_selected_resources(),
                    }

                    DiscussionMember {
                        lang,
                        total_committees: ctrl.get_committees(),
                        selected_committees: ctrl.get_selected_committee(),
                        discussion: discussion.clone(),
                        set_discussion: move |info: DeliberationDiscussionCreateRequest| {
                            ctrl.set_discussion(info.clone());
                        },
                    }

                    DiscussionGroup {
                        lang,
                        discussion: discussion.clone(),
                        set_discussion: move |disc| {
                            ctrl.set_discussion(disc);
                        },
                    }
                }

                div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {
                            ctrl.back();
                        },
                        {tr.backward}
                    }
                    div {
                        class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| async move {
                            ctrl.temp_save().await;
                        },
                        {tr.temporary_save}
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                        onclick: move |_| {
                            ctrl.next();
                        },
                        {tr.next}
                    }
                }
            }
        }
    }
}
