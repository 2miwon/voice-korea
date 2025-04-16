use bdk::prelude::*;
use models::{DiscussionCreateRequest, File};

use crate::pages::deliberations::new::details::discussions::components::{
    discussion_group::DiscussionGroup, document::Document, introduction::Introduction,
    member::Member,
};

use super::*;
use controller::*;
use i18n::*;

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
                    Introduction {
                        lang,
                        discussion: discussion.clone(),
                        start_date_id: "discussion_start_date",
                        end_date_id: "discussion_end_date",
                        set_title: move |title: String| {
                            ctrl.set_title(title);
                        },
                        set_description: move |description: String| {
                            ctrl.set_description(description);
                        },
                        set_start_date: move |start_date: i64| {
                            ctrl.set_start_date(start_date);
                        },
                        set_end_date: move |end_date: i64| {
                            ctrl.set_end_date(end_date);
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

                    Member {
                        lang,
                        total_committees: ctrl.get_committees(),
                        selected_committees: ctrl.get_selected_committee(),
                        add_committee: move |id: i64| {
                            ctrl.add_committee(id);
                        },
                        remove_committee: move |id: i64| {
                            ctrl.remove_committee(id);
                        },
                        clear_committee: move |_| {
                            ctrl.clear_committee();
                        },
                    }

                    DiscussionGroup {
                        lang,
                        discussion: discussion.clone(),
                        add_discussion: move |_| {
                            ctrl.add_discussion();
                        },
                        remove_discussion: move |index: usize| {
                            ctrl.remove_discussion(index);
                        },
                        update_discussion: move |(index, discussion): (usize, DiscussionCreateRequest)| {
                            ctrl.update_discussion(index, discussion);
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
