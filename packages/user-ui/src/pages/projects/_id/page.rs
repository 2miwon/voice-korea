use bdk::prelude::*;

use crate::pages::projects::_id::components::skeleton::Skeleton;
use dioxus_translate::Language;

use crate::pages::projects::_id::{
    components::{
        basic_info::BasicInfo, comments::Comment, consideration::Consideration,
        discussion::DiscussionTab, final_recommendation::FinalRecommendation,
        final_survey::FinalSurvey, project_header::ProjectHeader, sample_survey::SampleSurvey,
    },
    controller,
};

#[component]
pub fn ProjectPage(lang: Language, project_id: ReadOnlySignal<i64>) -> Element {
    let mut ctrl = controller::Controller::init(lang, project_id)?;
    let comments = ctrl.comment_trees();
    let deliberation = ctrl.summary()?;
    let active_tab = use_signal(|| Tab::SampleSurvey);

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center",
            ProjectHeader { lang, deliberation, active_tab: active_tab.clone() }
            div { class: "flex flex-col w-full justify-center items-center bg-box-gray",
                div { class: "flex flex-col max-w-desktop w-full max-desktop:px-20",
                    SuspenseBoundary {
                        fallback: |_| rsx! {
                            Skeleton {}
                        },
                        ProjectDetails {
                            lang,
                            active_tab: active_tab.clone(),
                            project_id,
                        }
                    }
                }
            }
            Comment {
                lang,
                comments,
                send_comment: move |comment: String| async move {
                    let _ = ctrl.send_comment(comment).await;
                },
                like_comment: move |id: i64| async move {
                    let _ = ctrl.like_comment(id).await;
                },
                send_reply: move |(id, reply): (i64, String)| async move {
                    let _ = ctrl.send_reply(id, reply).await;
                },
                is_login: ctrl.user.is_login(),
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Translate, Copy)]
pub enum Tab {
    #[default]
    #[translate(ko = "기본 정보", en = "Basic Info")]
    BasicInfo = 0,
    #[translate(ko = "표본 조사", en = "Sample Survey")]
    SampleSurvey = 1,
    #[translate(ko = "숙의", en = "Consideration")]
    Consideration = 2,
    #[translate(ko = "토론", en = "Discussion")]
    Discussion = 3,
    #[translate(ko = "투표", en = "Final Survey")]
    FinalSurvey = 4,
    #[translate(ko = "최종 권고안", en = "Final Recommendation")]
    FinalRecommendation = 5,
}

#[component]
pub fn ProjectDetails(
    lang: Language,
    active_tab: Signal<Tab>,
    project_id: ReadOnlySignal<i64>,
) -> Element {
    rsx! {
        div { class: "flex flex-col max-w-desktop w-full max-desktop:px-20",
            match active_tab() {
                Tab::BasicInfo => rsx! {
                    BasicInfo { lang, project_id }
                },
                Tab::SampleSurvey => rsx! {
                    SampleSurvey { lang, project_id }
                },
                Tab::Consideration => rsx! {
                    Consideration { lang, project_id }
                },
                Tab::Discussion => rsx! {
                    DiscussionTab { lang, project_id }
                },
                Tab::FinalSurvey => rsx! {
                    FinalSurvey { lang, project_id }
                },
                Tab::FinalRecommendation => rsx! {
                    FinalRecommendation { lang, project_id }
                },
            }
        }
    }
}
