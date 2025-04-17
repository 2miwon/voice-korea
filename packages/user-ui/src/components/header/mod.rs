use bdk::prelude::*;
use by_components::icons::{
    self as by_components_icon, alignments::AlignJustify, arrows::ArrowRight,
};
use dioxus_translate::{translate, Language};

mod controller;
mod i18n;

use crate::{
    components::{
        button::Button,
        custom_checkbox::CustomCheckbox,
        icons::{self, Logo},
        input::Input,
    },
    routes::Route,
    service::user_service::{UserEvent, UserService},
};
use dioxus_popup::PopupService;
use i18n::{
    CompletePopupTranslate, GoogleLoginPopupTranslate, SeeDetailButtonTranslate,
    SignupPopupTranslate, Translate,
};

#[component]
pub fn SeeDetailButton(lang: Language) -> Element {
    let tr: SeeDetailButtonTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-row bg-third rounded-[4px] px-[10px] py-[3px] font-semibold text-white text-[14px]",
            {tr.see_detail}
        }
    }
}

#[component]
pub fn CompletePopup(lang: Language, onclose: EventHandler<MouseEvent>) -> Element {
    let tr: CompletePopupTranslate = translate(&lang);
    let mut popup_service: PopupService = use_context();
    rsx! {
        div { class: "flex flex-col min-w-[420px] max-[420px]:min-w-[300px] justify-center items-center gap-[35px]",
            div { class: "flex flex-col w-full justify-center items-center gap-[15px]",
                div { class: "flex flex-row w-[88px] h-[88px] justify-center items-center bg-third rounded-[100px]",
                    Logo { width: "47", height: "47", class: "fill-white" }
                }
                div { class: "flex flex-col w-full justify-center items-center font-semibold text-[16px] text-key-gray leading-[24px]",
                    div { "{tr.complete_message_1}" }
                    div { "{tr.complete_message_2}" }
                }
            }
            div {
                class: "cursor-pointer flex flex-row w-full h-[57px] justify-center items-center rounded-[12px] bg-primary font-extrabold text-[18px] text-white",
                onclick: move |_| {
                    popup_service.close();
                },
                {tr.start}
            }
        }
    }
}

#[component]
pub fn SignupPopup(lang: Language, email: String, profile_url: String) -> Element {
    let user_service: UserService = use_context();
    let mut popup_service: PopupService = use_context();
    let tr: SignupPopupTranslate = translate(&lang);
    let mut nickname: Signal<String> = use_signal(|| "".to_string());
    let mut checked_1: Signal<bool> = use_signal(|| false);
    let mut checked_2: Signal<bool> = use_signal(|| false);

    let mut nickname_error = use_signal(|| "".to_string());
    let mut check_error = use_signal(|| "".to_string());
    rsx! {
        div { class: "flex flex-col min-w-[420px] max-[420px]:min-w-[300px] justify-between items-center gap-[25px]",
            div { class: "flex flex-col w-full justify-start items-start gap-[15px]",
                div { class: "flex flex-col gap-[5px] w-full",
                    div { class: "flex flex-row w-full justify-start items-start gap-[3px]",
                        div { class: "font-bold text-[#ff0004] text-[14px]", "*" }
                        div { class: "font-bold text-[#222222] text-[14px]", {tr.nickname} }
                    }
                    Input {
                        placeholder: tr.nickname_hint,
                        value: nickname(),
                        onchange: move |v: String| {
                            nickname.set(v);
                        },
                    }
                    div { class: "font-normal text-[#7C8292] text-[14px]", {tr.nickname_warning} }
                    if nickname_error() != "" {
                        div { class: "font-normal text-red-400 text-[14px]", {nickname_error()} }
                    }
                }
                div { class: "flex flex-col w-full justify-start items-center gap-[15px]",
                    div { class: "flex flex-row w-full gap-[10px]",

                        CustomCheckbox {
                            checked: checked_1(),
                            onchange: move |v| {
                                checked_1.set(v);
                            },
                        }
                        div { class: "font-medium !text-davy-gray text-[16px]", {tr.agree_1} }
                        SeeDetailButton { lang }
                    }
                    div { class: "flex flex-row w-full gap-[10px]",
                        CustomCheckbox {
                            checked: checked_2(),
                            onchange: move |v| {
                                checked_2.set(v);
                            },
                        }
                        div { class: "font-medium !text-davy-gray text-[16px]", {tr.agree_2} }
                        SeeDetailButton { lang }
                    }
                    if check_error() != "" {
                        div { class: "font-normal text-red-400 text-[14px]", {check_error()} }
                    }
                }
            }
            div {
                class: "cursor-pointer flex flex-row w-full h-[57px] justify-center items-center rounded-[12px] bg-primary font-extrabold text-[18px] text-white",
                onclick: move |_| {
                    let email = email.clone();
                    async move {
                        if nickname() == "" {
                            nickname_error.set(tr.nickname_error.to_string());
                            return;
                        } else if !checked_1() || !checked_2() {
                            check_error.set(tr.check_error.to_string());
                            return;
                        }
                        nickname_error.set("".to_string());
                        check_error.set("".to_string());
                        match user_service.login_or_signup(lang, &email, &nickname()).await {
                            Ok(_) => {
                                popup_service
                                    .open(rsx! {
                                        CompletePopup {
                                            lang,
                                            onclose: move |_| {
                                                popup_service.close();
                                            },
                                        }
                                    })
                                    .with_id("complete")
                                    .with_title("VOICE KOREA");
                            }
                            Err(e) => {
                                tracing::error!("signup failed: {:?}", e);
                            }
                        };
                    }
                },
                {tr.next}
            }
        }
    }
}

#[component]
pub fn GoogleLoginPopup(lang: Language, onclose: EventHandler<MouseEvent>) -> Element {
    let mut user_service: UserService = use_context();
    let mut popup_service: PopupService = use_context();
    let tr: GoogleLoginPopupTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col min-w-[420px] max-[420px]:min-w-[300px] justify-between items-center",
            div {
                class: "cursor-pointer flex flex-row w-full bg-primary rounded-[8px] p-[8px] gap-[15px] justify-start items-center",
                onclick: move |e: Event<MouseData>| {
                    let onclose = onclose.clone();
                    async move {
                        let v: UserEvent = user_service.google_login().await;
                        match v {
                            UserEvent::Signup(email, _, profile_url) => {
                                popup_service
                                    .open(rsx! {
                                        SignupPopup { lang, email, profile_url }
                                    })
                                    .with_id("signup")
                                    .with_title(tr.signup);
                            }
                            UserEvent::Login => {
                                onclose.call(e);
                            }
                            UserEvent::Logout => {
                                onclose.call(e);
                            }
                        };
                    }
                },
                div { class: "flex flex-row w-[62px] h-[62px] bg-white rounded-[8px] justify-center items-center",
                    div {
                        by_components_icon::logo::Google { size: 31 }
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start gap-[3px]",
                    div { class: "text-white font-extrabold text-[16px]", "Continue with Google" }
                    div { class: "text-white font-normal text-[14px]", "Quick Sign-in" }
                }
            }

            div { class: "flex flex-row w-full justify-center items-center gap-[20px] font-semibold text-[#A3A3A3] text-[14px] mt-[45px]",
                div { {tr.privacy} }
                div { {tr.usage} }
            }
        }
    }
}

#[component]
pub fn Header(lang: Language, children: Element, expanded: Signal<bool>) -> Element {
    let translates: Translate = translate(&lang);

    let mut ctrl = controller::Controller::new(lang)?;

    // let mut expanded = use_signal(|| false);

    let mobile_menu = use_memo(move || {
        let expanded = expanded();

        if expanded {
            "max-desktop:right-0"
        } else {
            "max-desktop:-right-full"
        }
    });

    rsx! {
        header { class: "fixed top-0 left-0 w-screen h-(--mobile-header-height) tablet:h-(--header-height) overflow-hidden flex items-center justify-center z-100 bg-white",
            div { class: "flex justify-between my-25 h-30 items-center w-full max-w-desktop max-desktop:px-20",
                Link {
                    class: "flex flex-row items-center justify-around gap-4 h-full",
                    to: Route::MainPage { lang },
                    icons::Logo {}
                    div { class: "font-extrabold text-base text-logo", "VOICE KOREA" }
                }
                div { class: "block desktop:hidden",
                    button {
                        class: "desktop:hidden",
                        display: if expanded() { "none" },
                        onclick: move |_| {
                            expanded.set(true);
                        },
                        AlignJustify { width: 32, height: 32 }
                    }
                    button {
                        display: if !expanded() { "none" },
                        onclick: move |_| {
                            expanded.set(false);
                        },
                        ArrowRight { width: 32, height: 32 }
                    }
                }

                //Deskto1
                div { class: "flex-1 flex fixed transition-all max-desktop:w-screen max-desktop:h-[calc(100vh_-_var(--mobile-header-height))] max-desktop:top-(--mobile-header-height) desktop:static bg-white overflow-y-scroll z-10 {mobile_menu}",
                    div { class: "flex-1 flex font-bold justify-start desktop:justify-end items-center text-key-gray text-[15px]/19 gap-30 desktop:gap-45 flex-col desktop:flex-row h-fit",

                        {children}

                        if !ctrl.user.is_login() {
                            div {
                                onclick: move |_| {
                                    ctrl.google_login();
                                },
                                {translates.login}
                            }
                        }
                        Button {
                            class: "flex flex-row w-fit h-fit justify-center items-center rounded-lg p-5 bg-transparent border border-key-gray",
                            onclick: move |_| {
                                ctrl.move_to_console();
                            },
                            {translates.deliberation_design}
                        }
                        if ctrl.user.is_login() {
                            Profile { lang, name: ctrl.user.get_nicename() }
                        }
                    
                    }
                }
            }
        }
    }
}

#[component]
fn Profile(lang: Language, name: String, image_url: Option<String>) -> Element {
    rsx! {
        Link {
            class: "gap-10 flex flex-row justify-center items-center",
            to: Route::ProfilePage { lang },
            div { class: "w-28 h-28 rounded-full bg-profile-gray overflow-hidden",

                if image_url.is_some() {
                    img { src: image_url }
                }
            }
            span { class: "text-[15px]/18 font-bold text-black", {name} }
            img {
                class: "w-24 h-24",
                src: asset!("/public/images/cert_badge.png"),
            }
        }
    }
}
