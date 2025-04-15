use bdk::prelude::*;

#[component]
pub fn StartProjectModal(
    lang: Language,
    onsend: EventHandler<MouseEvent>,
    oncancel: EventHandler<MouseEvent>,
) -> Element {
    let tr: StartProjectModalTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col max-w-[590px] w-full justify-start items-start gap-40",
            div { class: "font-normal text-sm/22 text-text-black", {tr.description} }

            div { class: "flex flex-row w-full justify-start items-center gap-20",
                div {
                    class: "cursor-pointer flex flex-row w-fit px-14 py-8 bg-hover rounded-sm",
                    onclick: move |e: Event<MouseData>| {
                        onsend.call(e);
                    },
                    div { class: "font-semibold text-base/24 text-white", {tr.start} }
                }

                div {
                    class: "cursor-pointer flex flex-row w-fit px-14 py-8 bg-white rounded-sm",
                    onclick: move |e: Event<MouseData>| {
                        oncancel.call(e);
                    },
                    div { class: "font-semibold text-base/24 text-text-black", {tr.cancel} }
                }
            }
        }
    }
}

translate! {
    StartProjectModalTranslate;

    title: {
        ko: "설문 시작 안내",
        en: "Instructions for starting the survey"
    }
    description: {
        ko: "‘설문 시작하기’를 선택하시면 설문이 시작되며, 참여자가 응답할 수 있게 됩니다. 진행하시겠습니까?",
        en: "When you select ‘Start Survey’, the survey will begin and participants will be able to respond. Would you like to proceed?"
    }

    start: {
        ko: "설문 시작하기",
        en: "Start Survey"
    }
    cancel: {
        ko: "취소",
        en: "Cancel"
    }
}
