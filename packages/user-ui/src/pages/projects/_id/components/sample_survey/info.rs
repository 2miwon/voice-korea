use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{DeliberationSampleSurveySummary, Tab};

use crate::{
    components::{button::Button, AvatarLabel},
    pages::projects::_id::components::{
        accordion::Accordion, rich_text_viewer::RichTextViewer, tab_title::TabTitle,
    },
    utils::time::formatted_timestamp,
};

use super::{controllers::SurveyStatus, i18n::SampleSurveyTranslate};

#[component]
pub fn Info(
    lang: Language,
    sample_survey: DeliberationSampleSurveySummary,
    survey_status: SurveyStatus,
    is_login: bool,
    on_process_survey: EventHandler<MouseEvent>,
    start_date: i64,
    end_date: i64,
) -> Element {
    let tr: SampleSurveyTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-10",
            div { class: "flex flex-col w-full h-fit bg-box-gray gap-10 justify-center items-center",
                TabTitle { title: Tab::SampleSurvey.translate(&lang),
                    div { class: "font-medium text-[15px] text-black",
                        {
                            format!(
                                "{} ~ {}",
                                formatted_timestamp(lang, start_date),
                                formatted_timestamp(lang, end_date),
                            )
                        }
                    }
                }
                div { class: "w-full flex flex-col justify-center items-center gap-10 mb-40",
                    Accordion { title: tr.title, default_open: true,
                        div { class: "w-full justify-start mt-15 mb-20 font-bold text-lg",
                            {sample_survey.title}
                        }
                        RichTextViewer {
                            class: "w-full flex justify-start text-[15px]",
                            contenteditable: false,
                            html: sample_survey.description,
                        }
                        div { class: "w-full flex flex-row justify-start gap-20",
                            for member in sample_survey.members {
                                AvatarLabel {
                                    label: member.nickname.unwrap_or(member.email),
                                    //FIXME: use organization name
                                    sub: "DAO",
                                }
                            }
                        }
                    }
                    Button {
                        class: "flex flex-row px-15 py-13 disabled:bg-hint-gray disabled:cursor-not-allowed rounded-lg text-white text-base",
                        disabled: survey_status != SurveyStatus::InProgress || !is_login,
                        onclick: move |e| {
                            on_process_survey.call(e);
                        },
                        {survey_status.translate(&lang)}
                    }
                }
            }
        }
    }
}
