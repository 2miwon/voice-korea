use bdk::prelude::*;
use by_components::charts::{
    horizontal_bar::HorizontalBar,
    pie_chart::{PieChart, PieChartData},
};
use models::ParsedQuestion;

use crate::pages::projects::_id::components::tab_title::TabTitleWithPrev;

use super::i18n::SurveyStatisticsTranslate;
#[component]
pub fn SurveyStatistics(
    lang: Language,
    grouped_answers: Vec<(String, ParsedQuestion)>,
    onprev: EventHandler<MouseEvent>,
) -> Element {
    let tr: SurveyStatisticsTranslate = translate(&lang);
    rsx! {
        TabTitleWithPrev {
            title: tr.response_per_question,
            onprev: move |e: Event<MouseData>| {
                onprev.call(e);
            },
        }
        Statistics { lang, grouped_answers }
    }
}

#[component]
pub fn Statistics(lang: Language, grouped_answers: Vec<(String, ParsedQuestion)>) -> Element {
    rsx! {
        for (i , (title , parsed_question)) in grouped_answers.iter().enumerate() {
            match parsed_question {
                ParsedQuestion::SingleChoice { answers, response_count } => {
                    rsx! {
                        div { class: "flex flex-col w-full",
                            ObjectiveBox {
                                lang,
                                title,
                                answers: answers.clone(),
                                answer_count: response_count.clone(),
                                index: i,
                                is_single: true,
                            }
                        }
                    }
                }
                ParsedQuestion::MultipleChoice { answers, response_count } => {
                    rsx! {
                        div { class: "flex flex-col w-full",
                            ObjectiveBox {
                                lang,
                                title,
                                answers: answers.clone(),
                                answer_count: response_count.clone(),
                                index: i,
                            }
                        }
                    }
                }
                ParsedQuestion::ShortAnswer { answers } => {
                    rsx! {
                        div { class: "flex flex-col w-full",
                            SubjectiveBox { lang, title, answers: answers.clone() }
                        }
                    }
                }
                ParsedQuestion::Subjective { answers } => {
                    rsx! {
                        div { class: "flex flex-col w-full",
                            SubjectiveBox { lang, title, answers: answers.clone() }
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn ObjectiveBox(
    lang: Language,
    index: usize,
    title: String,
    answers: Vec<String>,
    answer_count: Vec<i64>,
    #[props(default = false)] is_single: bool,
) -> Element {
    let tr: SurveyStatisticsTranslate = translate(&lang);
    let mut pie_charts: Signal<Vec<PieChartData>> = use_signal(|| vec![]);
    let mut total_answers: Signal<i32> = use_signal(|| 0);

    use_effect(use_reactive(&answer_count, {
        let answers = answers.clone();
        move |answer_count| {
            let mut pies = vec![];
            let mut totals = 0;

            for (i, answer) in answers.iter().enumerate() {
                pies.push(PieChartData::new(answer.clone(), answer_count[i] as i32));
                totals += answer_count[i] as i32;
            }

            pie_charts.set(pies);
            total_answers.set(totals);
        }
    }));

    rsx! {
        div { class: "flex flex-col w-full  bg-white px-40 py-20 rounded-lg gap-20",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "flex flex-row justify-start items-center gap-20",
                        div { class: "flex flex-row justify-start items-center gap-5",
                            if is_single {
                                div { class: "font-semibold text-base text-necessary-red",
                                    {tr.necessary}
                                }
                            } else {
                                div { class: "font-semibold text-base text-optional-blue",
                                    {tr.plural}
                                }
                            }
                            div { class: "font-semibold text-text-black text-16 leading-22",
                                {title}
                            }
                        }
                    }
                }
                div { class: "flex flex-row w-full h-1 justify-start items-start bg-quiz-border my-7" }
            }

            div { class: "flex max-[1100px]:flex-col max-[1100px]:gap-20 flex-row w-full justify-between items-start",
                div { class: "flex flex-col flex-1 justify-start items-start gap-20",
                    for (i , answer) in answers.clone().iter().enumerate() {
                        div { class: "flex flex-col w-full justify-start items-start gap-5",
                            div { class: "font-medium text-text-quiz-black text-[15px] leading-22",
                                "{answer}"
                            }

                            div { class: "flex flex-row w-full justify-start items-center gap-20",
                                if total_answers() != 0 {
                                    HorizontalBar {
                                        id: format!("horizontal_bar_{}{}", index, i),
                                        value: answer_count[i],
                                        height: "23px",
                                        max_value: total_answers() as i64,
                                        class: "flex flex-row flex-1 bg-line-gray rounded-md",
                                    }
                                }

                                div { class: "w-200 font-medium text-text-quiz-black text-[15px] leading-22",
                                    {
                                        format!(
                                            "{:?}{} ({:.2}%)",
                                            answer_count[i],
                                            tr.unit,
                                            if total_answers() != 0 {
                                                answer_count[i] as f64 * 100.0 / total_answers() as f64
                                            } else {
                                                0.0
                                            },
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
                PieChart {
                    id: format!("pie_chart_{index}"),
                    width: "500px",
                    height: "500px",
                    class: "max-[1300px]:min-w-300 max-[800px]:hidden sm:block",
                    data: pie_charts(),
                }
            }
        }
    }
}

#[component]
pub fn SubjectiveBox(lang: Language, title: String, answers: Vec<String>) -> Element {
    let tr: SurveyStatisticsTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full  bg-white px-40 py-20 rounded-lg gap-20",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "flex flex-row justify-start items-center gap-20",
                        div { class: "font-semibold text-text-black text-base leading-22",
                            "{title}"
                        }
                    }
                }
                div { class: "flex flex-row w-full h-1 justify-start items-start bg-quiz-border my-7" }
            }

            div { class: "flex flex-col w-full justify-start items-start gap-5",
                div { class: "font-medium text-text-quiz-black text-15", {tr.subjective_answer} }

                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    for answer in answers.clone() {
                        div { class: "flex flex-row w-full justify-start items-center px-15 py-10 rounded-4 bg-box-gray",
                            div { class: "font-medium text-text-black text-[15px] leading-22",
                                "{answer}"
                            }
                        }
                    }
                }
            }
        }
    }
}
