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
        ko: "프로젝트 시작",
        en: "Start Project"
    }
    description: {
        ko: "프로젝트를 시작 후에는 일부 정보가 참여자에게 즉시 노출될 수 있으니, 설정을 다시 한 번 확인해주세요. 공개 여부를 변경하고 싶으실 경우, 공론 관리 페이지 또는 해당 프로젝트 상세 페이지에서 수정하실 수 있습니다.",
        en: "After you start the project, some information may be immediately exposed to participants, so please check the settings again. If you want to change whether it is public or not, you can edit it on the public management page or the project details page."
    }

    start: {
        ko: "시작하기",
        en: "Start"
    }
    cancel: {
        ko: "취소",
        en: "Cancel"
    }
}
