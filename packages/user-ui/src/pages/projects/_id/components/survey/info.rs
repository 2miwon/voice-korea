use bdk::prelude::*;
use dioxus_translate::Language;
use models::{deliberation_role::DeliberationRole, ProjectStatus, Tab};

use crate::{
    components::{button::Button, AvatarLabel},
    pages::projects::_id::components::{
        accordion::Accordion, rich_text_viewer::RichTextViewer, tab_title::TabTitle,
    },
    utils::time::{current_timestamp, formatted_timestamp},
};

use super::i18n::SurveyInfoTranslate;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SurveyData {
    pub title: String,
    pub description: String,
    pub start_date: i64,
    pub end_date: i64,
    pub status: ProjectStatus,
    pub roles: Vec<SurveyInfoUser>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SurveyInfoUser {
    pub email: String,
}

impl From<DeliberationRole> for SurveyInfoUser {
    fn from(role: DeliberationRole) -> Self {
        Self { email: role.email }
    }
}

#[component]
pub fn SurveyInfo(
    lang: Language,
    title: String,
    data: SurveyData,
    is_login: bool,
    is_member: bool,
    on_process_survey: EventHandler<MouseEvent>,
) -> Element {
    let tr: SurveyInfoTranslate = translate(&lang);
    rsx! {
        TabTitle { title: Tab::SampleSurvey.translate(&lang),
            div { class: "font-medium text-[15px] text-black",
                {
                    format!(
                        "{} ~ {}",
                        formatted_timestamp(lang, data.start_date),
                        formatted_timestamp(lang, data.end_date),
                    )
                }
            }
        }
        div { class: "w-full flex flex-col justify-center items-center gap-10",
            Accordion { title, default_open: true,
                div { class: "w-full flex flex-col gap-20",
                    div { class: "font-bold text-lg", {data.title} }
                    RichTextViewer { contenteditable: false, html: data.description }
                    div { class: "w-full flex flex-row justify-start gap-20",
                        for role in data.roles {
                            AvatarLabel {
                                label: role.email,
                                //FIXME: use organization name
                                sub: "DAO",
                            }
                        }
                    }
                }
            }

            // FIXME: implement with project status value when complete implementing status fetcher code
            // disabled: data.status != ProjectStatus::InProgress || !is_login || !is_member,
            Button {
                class: "flex flex-row px-15 py-13 disabled:bg-hint-gray disabled:cursor-not-allowed rounded-lg text-white text-base",
                disabled: !(data.start_date <= current_timestamp() && data.end_date >= current_timestamp())
                    || !is_login || !is_member,
                onclick: move |e| {
                    on_process_survey.call(e);
                },

                if data.start_date > current_timestamp() {
                    {tr.status_ready}
                } else if data.end_date < current_timestamp() {
                    {tr.status_finish}
                } else {
                    {tr.status_progress}
                }
                        // match data.status {
            //     ProjectStatus::Ready => tr.status_ready,
            //     ProjectStatus::InProgress => tr.status_progress,
            //     ProjectStatus::Finish => tr.status_finish,
            // }
            }
        }
    }
}
