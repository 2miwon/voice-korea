use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::deliberation_project::DeliberationProjectSummary;
use num_format::{Locale, ToFormattedString};

use crate::{
    components::{
        button::Button,
        icons::{user::User, vote::Vote},
        label::Label,
    },
    pages::i18n::ProjectBoxTranslate,
    routes::Route,
};

#[component]
pub fn ProjectCard(lang: Language, deliberation: DeliberationProjectSummary) -> Element {
    let project_id = deliberation.id;
    let tr: ProjectBoxTranslate = translate(&lang);
    let nav = use_navigator();
    rsx! {
        div {
            class: "relative w-full h-420 rounded-[12px] shadow-md hover:shadow-xl overflow-hidden group",
            tabindex: 0,
            // Thumbnail
            div {
                background_image: format!("url({})", deliberation.thumbnail_image),
                class: "h-260 bg-cover bg-center",
            }
            // Overlay
            div { class: "flex flex-col transition-all duration-400 ease-[cubic-bezier(.17,.67,.5,1.03)] absolute bottom-0 w-full bg-white rounded-[12px] px-16 py-20 min-h-170 max-h-170 group-hover:max-h-400 group-focus-within:max-h-400",
                div { class: "font-bold text-[16px] leading-normal text-text-black line-clamp-1 group-hover:line-clamp-none group-focus-within:line-clamp-none",
                    {deliberation.title}
                }
                div { class: "font-normal text-text-gray text-sm/22 flex-1 min-h-44 overflow-hidden",
                    div { class: "h-auto line-clamp-2 group-hover:line-clamp-9 group-focus-within:line-clamp-9",
                        {deliberation.description}
                    }
                }

                div { class: "group-hover:hidden group-focus-within:hidden flex flex-row gap-4 shrink-0",
                    for area in deliberation.project_areas.iter() {
                        Label { name: area.project_area.translate(&lang) }
                    }
                }
                Button {
                    class: "hidden group-hover:block group-focus-within:block w-full py-8 border border-bt-grey bg-white rounded-full font-semibold text-black text-[15px]/25",
                    onclick: move |_| {
                        nav.push(Route::ProjectPage {
                            lang,
                            project_id,
                        });
                    },
                    {tr.detail}
                }
                div { class: "flex flex-row w-full justify-between items-center mt-16",
                    div { class: "flex flex-row gap-6",
                        User { width: "18", height: "18" }
                        div { class: "flex flex-row gap-4",
                            div { class: "font-normal text-sm text-text-black leading-17",
                                "{tr.participant}"
                            }
                            div { class: "font-bold text-sm text-text-black leading-17",
                                {deliberation.participants.to_formatted_string(&Locale::en)}
                            }
                        }
                    }

                    div { class: "flex flex-row gap-6",
                        Vote { width: "18", height: "18" }
                        div { class: "flex flex-row gap-4",
                            div { class: "font-normal text-sm text-text-black leading-17",
                                "{tr.vote}"
                            }
                            div { class: "font-bold text-sm text-text-black leading-17",
                                {deliberation.votes.to_formatted_string(&Locale::en)}
                            }
                        }
                    }
                }
            }
        }
    }
}
