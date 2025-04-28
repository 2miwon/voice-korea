use bdk::prelude::*;

use models::{elearning::Elearning, Tab};

use crate::{
    components::AvatarLabel,
    pages::projects::_id::components::{
        accordion::Accordion, consideration::i18n::ProgressBarTranslate,
        response_files::ResourcesComponent, rich_text_viewer::RichTextViewer, section::Section,
        tab_title::TabTitle,
    },
    utils::time::formatted_timestamp,
};

use super::controller::Controller;
use super::i18n::ConsiderationTranslate;

#[component]
pub fn Consideration(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let content = ctrl.content()?;

    let tr: ConsiderationTranslate = translate(&lang);
    let tab_title: &str = Tab::Deliberation.translate(&lang);
    rsx! {
        Section { id: "basic-info",
            TabTitle { title: tab_title,
                div { class: "font-medium text-[15px] text-black",
                    {
                        format!(
                            "{} ~ {}",
                            formatted_timestamp(lang, content.started_at),
                            formatted_timestamp(lang, content.ended_at),
                        )
                    }
                }
            }
            // information section
            div { class: "flex flex-col gap-10",
                Accordion { title: tr.main_title, default_open: true,
                    div { class: "w-full flex flex-col gap-20",
                        div { class: "font-bold text-lg", {content.title} }
                        RichTextViewer { contenteditable: false, html: content.description }
                        div { class: "w-full flex flex-row justify-start gap-20",
                            for roles in content.roles {
                                AvatarLabel {
                                    label: roles.email,
                                    //FIXME: use organization name
                                    sub: "DAO",
                                }
                            }
                        }
                    }
                }

                //FIXME: fix to pdf reader
                ResourcesComponent {
                    title: tr.e_learning_title,
                    resources: content
                        .elearnings
                        .iter()
                        .flat_map(|elearning| elearning.resources.clone())
                        .collect(),
                }

            // Accordion { title: tr.e_learning_title, default_open: false,
            //     div { class: "w-full flex flex-col gap-44 [&>:last-child]:hidden",
            //         for elearning in content.elearnings {
            //             ElearningComponent { lang, elearning }
            //             hr { class: "w-full border-b-1 border-line-gray" }
            //         }
            //     }
            // }
            }
        }
    }
}

#[component]
fn ElearningComponent(lang: Language, elearning: Elearning) -> Element {
    rsx! {
        div { class: "@container w-full flex flex-col tablet:flex-row gap-20",
            div { class: "w-240 h-150 bg-profile-gray rounded-lg shrink-0" }
            div { class: "w-full flex flex-col justify-between items-start",
                div { class: "w-full flex flex-col gap-10",
                    div { class: "w-full flex flex-col gap-4",
                        h2 { class: "text-[14px] font-normal", "e-Book" }
                        h1 { class: "text-[18px] font-bold", {elearning.title} }
                    }
                    ProgressBar { lang, current_page: 50, total_page: 100 }
                }
                AvatarLabel { label: "Speaker", sub: "DAO" }
            }
        }
    }
}

#[component]
fn ProgressBar(lang: Language, current_page: u32, total_page: u32) -> Element {
    let tr: ProgressBarTranslate = translate(&lang);

    let percent = if total_page != 0 {
        (current_page as f64 / total_page as f64) * 100.0
    } else {
        0.0
    };
    rsx! {
        div { class: "flex flex-row w-full max-w-500 justify-start items-center gap-10 max-tablet:flex-col max-tablet:justify-start max-tablet:items-start",
            div { class: "relative h-8 w-full",
                div { class: "absolute left-0 w-full top-2 h-4 bg-disabled rounded-full" }
                div {
                    class: "absolute left-0 h-8 bg-progress rounded-full",
                    style: "width: {percent}%",
                }
            }
            span { class: "text-review-gray text-[14px] font-normal text-nowrap",
                "{percent}% ({current_page}/{total_page}) {tr.page}"
            }
        }
    }
}
