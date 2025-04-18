use bdk::prelude::*;

use dioxus_translate::{translate, Language};

use crate::{components::Header, routes::Route};

#[component]
pub fn ProjectHeader(lang: Language) -> Element {
    let tr: ProjectHeaderTranslate = translate(&lang);
    let expanded = use_signal(|| false);
    rsx! {
        Header { lang, expanded,
            NavLink { to: Route::ComingSoonPage { lang }, {tr.space} }
            NavLink { to: Route::ComingSoonPage { lang }, {tr.reward} }
            NavLink { to: Route::ComingSoonPage { lang }, {tr.participant} }
            NavLink { to: Route::ComingSoonPage { lang }, {tr.deliberation_committee} }
            NavLink { to: Route::ComingSoonPage { lang }, {tr.data_room} }
            NavLink { to: Route::ComingSoonPage { lang }, {tr.activity_details} }
        }
    }
}
#[component]
fn NavLink(#[props(into)] to: Route, #[props(into)] children: Element) -> Element {
    rsx! {
        Link { class: "hover:text-primary cursor-pointer", to, {children} }
    }
}
translate! {
    ProjectHeaderTranslate;

    login: {
        ko: "로그인",
        en: "Login"
    },

    logout: {
        ko: "로그아웃",
        en: "Logout"
    },

    space: {
        ko: "참여 공간",
        en: "Participation Space"
    }

    reward: {
        ko: "참여 보상",
        en: "Participation Reward"
    }

    participant: {
        ko: "참여자",
        en: "Participant"
    }

    deliberation_committee: {
        ko: "공론 위원회",
        en: "Deliberation Committee"
    }

    data_room: {
        ko: "자료실",
        en: "Data Room"
    }

    activity_details: {
        ko: "활동 내역",
        en: "Activity Details"
    }

    deliberation_design: {
        ko: "공론 조사 설계",
        en: "Deliberation Design"
    }

    my_profile: {
        ko: "나의 프로필",
        en: "My Profile"
    }
}
