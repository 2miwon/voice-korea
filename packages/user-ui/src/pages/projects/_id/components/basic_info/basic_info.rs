use std::collections::HashSet;

use bdk::prelude::*;

use crate::{
    components::AvatarLabel,
    pages::projects::_id::components::{
        accordion::Accordion, response_files::ResourcesComponent, rich_text_viewer::RichTextViewer,
        section::Section, tab_title::TabTitle,
    },
    utils::time::formatted_timestamp,
};
use models::{deliberation_role::DeliberationRole, tab::Tab};

use super::controllers::Controller;
use super::i18n::BasicInfoTranslate;

#[component]
pub fn BasicInfo(lang: Language, project_id: ReadOnlySignal<i64>, children: Element) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let basic_info = ctrl.basic_info()?;

    let tr: BasicInfoTranslate = translate(&lang);
    let tab_title: &str = Tab::BasicInfo.translate(&lang);

    let mut emails: HashSet<String> = HashSet::new();

    rsx! {
        Section { id: "basic-info",
            TabTitle { title: tab_title,
                div { class: "font-medium text-[15px] text-black",
                    {
                        format!(
                            "{} ~ {}",
                            formatted_timestamp(lang, basic_info.started_at),
                            formatted_timestamp(lang, basic_info.ended_at),
                        )
                    }
                }
            }
            // information section
            div { class: "flex flex-col gap-10",
                Accordion { title: tr.main_title, default_open: true,
                    div { class: "w-full flex flex-col gap-20",
                        div { class: "font-bold text-lg", {basic_info.title} }
                        RichTextViewer {
                            contenteditable: false,
                            html: basic_info.description,
                        }
                        div { class: "w-full flex flex-row justify-start gap-20",
                            for role in basic_info.roles {
                                if emails.insert(role.email.clone()) {
                                    AvatarLabel { label: role.email.clone(), sub: "DAO" }
                                }
                            }
                        }
                    }
                }

                Accordion { title: tr.committee_title,
                    div { class: "w-full flex flex-col gap-12 [&>:last-child]:hidden",
                        for (title , members) in ctrl.committees() {
                            Committee { title, members }
                        }
                    }
                }
                //Related Data
                ResourcesComponent {
                    title: tr.related_materials_title,
                    resources: basic_info.resources.clone(),
                }
            }
        }
    }
}

#[component]
fn Committee(title: String, members: Vec<DeliberationRole>) -> Element {
    rsx! {
        div { class: "w-full flex flex-col gap-10",
            h1 { class: "text-18 font-bold w-full", {title} }
            div { class: "grid grid-cols-5 gap-20 w-full",
                for member in members {
                    AvatarLabel { label: member.email, sub: "DAO" }
                }
            }
        }
        hr { class: "w-full border-b-1 border-line-gray" }
    }
}
