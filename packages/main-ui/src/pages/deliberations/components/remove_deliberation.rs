use bdk::prelude::*;

#[component]
pub fn RemoveDeliberationModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    onremove: EventHandler<MouseEvent>,
) -> Element {
    let tr: RemoveDeliberationModalTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-text-black font-normal text-sm gap-5",
                div { "{tr.remove_info}" }
                div { "{tr.remove_warning}" }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-20",
                div {
                    class: "flex flex-row w-85 h-40 justify-center items-center bg-primary rounded-md cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onremove.call(e);
                    },
                    div { class: "text-white font-bold text-base", "{tr.remove}" }
                }
                div {
                    class: "flex flex-row w-85 h-40 font-semibold text-base text-text-black justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{tr.cancel}"
                }
            }
        }
    }
}

translate! {
    RemoveDeliberationModalTranslate;

    title: {
        ko: "공론 삭제",
        en: "Remove Deliberation"
    }
    remove_info: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?",
    },
    remove_warning: {
        ko: "삭제된 프로젝트는 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요.",
        en: "Deleted projects cannot be restored. Please check again before deleting.",
    },
    remove: {
        ko: "삭제하기",
        en: "Remove",
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel",
    },
}
