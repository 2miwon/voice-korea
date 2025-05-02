use bdk::prelude::*;

use dioxus_translate::{translate, Language};

use crate::{components::Header, routes::Route};

pub static SELECTED_MENU: GlobalSignal<i32> = GlobalSignal::new(|| 0);

#[component]
pub fn MainHeader(lang: Language) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    let mut expanded = use_signal(|| false);

    rsx! {
        Header { lang, expanded,
            A {
                lang,
                href: "#service",
                selected: SELECTED_MENU() == 1,
                onclick: move |_| {
                    expanded.set(false);
                },
                {tr.service}
            }
            A {
                lang,
                href: "#project",
                selected: SELECTED_MENU() == 2,
                onclick: move |_| {
                    expanded.set(false);
                },
                {tr.project}
            }
            A {
                lang,
                href: "#institution",
                selected: SELECTED_MENU() == 3,
                onclick: move |_| {
                    expanded.set(false);
                },
                {tr.organization}
            }
            A {
                lang,
                href: "#price",
                selected: SELECTED_MENU() == 4,
                onclick: move |_| {
                    expanded.set(false);
                },
                {tr.plan}
            }
            A {
                lang,
                href: "#inquiry",
                selected: SELECTED_MENU() == 5,
                onclick: move |_| {
                    expanded.set(false);
                },
                {tr.contact}
            }
            A {
                lang,
                href: "#footer",
                selected: SELECTED_MENU() == 6,
                onclick: move |_| {
                    expanded.set(false);
                },
                {tr.guide}
            }
        }
    }
}

#[component]
pub fn A(
    children: Element,
    lang: Language,
    href: String,
    #[props(default = false)] selected: bool,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let current_path: Route = use_route();
    let is_home = matches!(current_path, Route::MainPage { .. });
    tracing::debug!("{href} {selected}");
    rsx! {
        div {
            class: "cursor-pointer hover:text-secondary w-full text-center desktop:w-fit",
            color: if selected { "var(--color-secondary)" },
            if is_home {
                a {
                    class: "inline-block w-full",
                    href,
                    onclick: move |evt| {
                        if let Some(onclick) = onclick {
                            onclick.call(evt);
                        }
                    },
                    {children}
                }
            } else {
                Link { class: "w-full", to: Route::MainPage { lang }, {children} }
            }
        }
    }
}

translate! {
    HeaderTranslate;

    service: {
        ko: "서비스 소개",
        en: "Main Page"
    },

    organization: {
        ko: "정책 결정 기관",
        en: "Policy Making organization"
    },
    project: {
        ko: "프로젝트",
        en: "Project"
    },

    login: {
        ko: "로그인",
        en: "Login"
    },

    logout: {
        ko: "로그아웃",
        en: "Logout"
    },

    plan: {
        ko: "플랜",
        en: "Plan"
    },

    contact: {
        ko: "문의하기",
        en: "Contact"
    },

    guide: {
        ko: "가이드",
        en: "Guide"
    },

    my_profile: {
        ko: "나의 프로필",
        en: "My Profile"
    }

    deliberation_design: {
        ko: "공론 조사 설계",
        en: "Deliberation Design"
    }
}
