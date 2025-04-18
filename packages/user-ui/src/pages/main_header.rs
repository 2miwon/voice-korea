#![allow(unused)]
use bdk::prelude::*;

use by_components::icons::alignments::AlignJustify;
use dioxus_translate::{translate, Language};

use crate::{
    components::{
        header::GoogleLoginPopup,
        icons::{self},
        Header,
    },
    routes::Route,
    service::user_service::UserService,
};
use dioxus_popup::PopupService;

pub static SELECTED_MENU: GlobalSignal<i32> = GlobalSignal::new(|| 0);

// #[component]
// pub fn MainHeader(lang: Language) -> Element {
//     let tr: HeaderTranslate = translate(&lang);

//     let user_service: UserService = use_context();
//     let mut popup_service: PopupService = use_context();

//     let email = (user_service.email)();

//     let onclick = {
//         let email = email.clone();
//         move |_| {
//             tracing::debug!("signup button clicked");

//             if email != "" {
//                 return;
//             }

//             popup_service
//                 .open(rsx! {
//                     GoogleLoginPopup {
//                         lang: lang.clone(),
//                         onclose: move |_| {
//                             popup_service.close();
//                         },
//                     }
//                 })
//                 .with_id("google_login")
//                 .with_title(tr.login);
//         }
//     };

//     rsx! {
//         div { class: "block max-[1300px]:!hidden",
//             MainDesktopHeader { lang, email: email.clone(), onclick: onclick.clone() }
//         }
//         div { class: "hidden max-[1300px]:!block",
//             MainMobileHeader { lang, email, onclick }
//         }
//     }
// }

#[component]
pub fn MainMobileHeader(
    lang: Language,
    email: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let nav = use_navigator();
    let tr: HeaderTranslate = translate(&lang);
    let mut expanded = use_signal(|| false);
    let custom_class = "fixed top-0 left-0 z-100";

    rsx! {
        div { class: "{custom_class} w-full h-70 flex flex-row items-center justify-between bg-white px-[20px]",
            button {
                class: "cursor-pointer flex flex-row items-center justify-around gap-4 h-full w-fit",
                onclick: move |_| {
                    nav.push(Route::MainPage { lang });
                    expanded.set(false);
                },
                icons::Logo {}
                div { class: "font-extrabold text-base text-logo", "VOICE KOREA" }
            }
            button {
                onclick: move |_| {
                    expanded.set(!expanded());
                },
                AlignJustify { class: "cursor-pointer w-[30px] h-[30px] text-black" }
            }
        }

        if expanded() {
            div { class: "fixed top-70 left-0 w-full h-full grow bg-white flex flex-col items-start text-black z-100 px-20 py-[20px]",
                div { class: "flex flex-col font-bold justify-start items-start text-key-gray text-15 leading-19",
                    A { lang, href: "#service",
                        button {
                            class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                            onclick: move |_| {
                                expanded.set(false);
                            },
                            {tr.service}
                        }
                    }
                    A { lang, href: "#project",
                        button {
                            class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                            onclick: move |_| {
                                expanded.set(false);
                            },
                            {tr.project}
                        }
                    }
                    A { lang, href: "#institution",
                        button {
                            class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                            onclick: move |_| {
                                expanded.set(false);
                            },
                            {tr.organization}
                        }
                    }
                    A { lang, href: "#price",
                        button {
                            class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                            onclick: move |_| {
                                expanded.set(false);
                            },
                            {tr.plan}
                        }
                    }
                    A { lang, href: "#inquiry",
                        button {
                            class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                            onclick: move |_| {
                                expanded.set(false);
                            },
                            {tr.contact}
                        }
                    }
                    A { lang, href: "#footer",
                        button {
                            class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                            onclick: move |_| {
                                expanded.set(false);
                            },
                            {tr.guide}
                        }
                    }

                    button {
                        class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                        onclick: move |e: Event<MouseData>| {
                            onclick.call(e);
                            expanded.set(false);
                        },
                        div {
                            if email == "" {
                                "{tr.login}"
                            } else {
                                "{tr.logout}"
                            }
                        }
                    }

                    if email != "" {
                        button {
                            class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                            onclick: move |_| {
                                nav.push(Route::ProfilePage { lang });
                                expanded.set(false);
                            },
                            {tr.my_profile}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn MainHeader(lang: Language) -> Element {
    let nav = use_navigator();
    let tr: HeaderTranslate = translate(&lang);
    let user = use_context::<UserService>();

    rsx! {
        Header { lang,
            div { class: "flex font-bold justify-end items-center text-key-gray text-15 leading-19 gap-45",
                A {
                    lang,
                    href: "#service",
                    selected: SELECTED_MENU == 1,
                    {tr.service}
                }
                A {
                    lang,
                    href: "#project",
                    selected: SELECTED_MENU == 2,
                    {tr.project}
                }
                A {
                    lang,
                    href: "#institution",
                    selected: SELECTED_MENU == 3,
                    {tr.organization}
                }
                A { lang, href: "#price", selected: SELECTED_MENU == 4, {tr.plan} }
                A {
                    lang,
                    href: "#inquiry",
                    selected: SELECTED_MENU == 5,
                    {tr.contact}
                }
                A { lang, href: "#footer", selected: SELECTED_MENU == 6, {tr.guide} }
            

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
) -> Element {
    let current_path: Route = use_route();
    let is_home = matches!(current_path, Route::MainPage { .. });
    tracing::debug!("{href} {selected}");
    rsx! {
        div {
            class: "cursor-pointer hover:text-secondary",
            color: if selected { "var(--color-secondary)" },
            if is_home {
                a { href, {children} }
            } else {
                Link { to: Route::MainPage { lang }, {children} }
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
