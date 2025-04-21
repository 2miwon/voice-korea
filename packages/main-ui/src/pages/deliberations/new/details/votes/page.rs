use super::super::components::introduction_card::IntroductionCard;
use super::*;
use crate::pages::deliberations::new::details::votes::components::{
    member::Member, question::QuestionList, reward::Reward,
};
use bdk::prelude::*;
use controller::*;
use i18n::*;
use models::Question;

#[component]
pub fn DeliberationVoteSettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: VoteTranslate = translate(&lang);
    let final_survey = ctrl.final_survey();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    div { class: "flex flex-col w-full justify-start items-start gap-10",
                        div { class: "font-medium text-base text-text-black", {tr.vote_setting} }
                        div { class: "flex flex-col w-full justify-start items-start gap-20",
                            IntroductionCard {
                                lang,
                                description: tr.introduction_description.to_string(),
                                rich_text_id: "final_survey_rich_text",
                                start_date_id: "final_survey_start_date",
                                end_date_id: "final_survey_end_date",
                                text_value: final_survey.clone().title,
                                started_at: final_survey.clone().started_at,
                                ended_at: final_survey.clone().ended_at,
                                content: final_survey.clone().description,
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

                            Reward {
                                lang,
                                final_survey: final_survey.clone(),
                                set_estimate_time: move |estimate_time: i64| {
                                    ctrl.set_estimate_time(estimate_time);
                                },
                                set_point: move |point: i64| {
                                    ctrl.set_point(point);
                                },
                            }

                            Member {
                                lang,
                                total_committees: ctrl.committee_members(),
                                selected_committees: ctrl.get_selected_committee(),
                                add_committee: move |email: String| {
                                    ctrl.add_committee(email);
                                },
                                remove_committee: move |email: String| {
                                    ctrl.remove_committee(email);
                                },
                                clear_committee: move |_| {
                                    ctrl.clear_committee();
                                },
                            }
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start gap-10 mt-20",
                    div { class: "font-medium text-base text-text-black", {tr.voting_items} }
                    QuestionList {
                        lang,

                        final_survey: final_survey.clone(),

                        add_question: move |_| {
                            ctrl.add_question();
                        },
                        remove_question: move |index: usize| {
                            ctrl.remove_question(index);
                        },
                        update_question: move |(index, question): (usize, Question)| {
                            ctrl.update_question(index, question);
                        },
                    }
                }
            }
            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                button {
                    class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20 hover:!bg-primary hover:!text-white",
                    onclick: move |_| {
                        ctrl.back();
                    },
                    {tr.backward}
                }
                button {
                    class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20 hover:!bg-primary hover:!text-white",
                    onclick: move |_| async move {
                        ctrl.temp_save().await;
                    },
                    {tr.temporary_save}
                }
                button {
                    class: "aria-active:cursor-pointer cursor-not-allowed flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-disabled aria-active:!bg-hover font-semibold text-base text-white",
                    "aria-active": ctrl.is_valid(),
                    onclick: move |_| ctrl.next(),
                    {tr.next}
                }
            }
        }
    }
}
