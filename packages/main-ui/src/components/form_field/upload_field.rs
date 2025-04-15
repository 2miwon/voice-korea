use bdk::prelude::*;

#[component]
pub fn UploadField(
    lang: Language,
    description: String,
    value: String,
    children: Element,
) -> Element {
    let tr: UploadFieldTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full",
            div { class: "flex flex-row gap-20 px-15 w-full h-54 bg-background-gray rounded-sm justify-center items-center",
                {children}
                input {
                    class: "flex flex-row w-full justify-start items-center bg-transparent text-disabled focus:outline-none",
                    r#type: "text",
                    placeholder: tr.placeholder,
                    readonly: true,
                    value,
                }
            }
            p { class: "text-text-gray text-start w-full text-sm font-normal", {description} }
        }
    }
}

translate! {
    UploadFieldTranslate;

    placeholder: {
        ko: "파일 없음",
        en: "No file",
    }
}
