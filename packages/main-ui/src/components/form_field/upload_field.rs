use bdk::prelude::*;

#[component]
pub fn UploadField(
    placeholder: String,
    description: String,
    value: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full",
            div { class: "flex flex-row gap-20 px-15 w-full h-54 bg-background-gray rounded-sm justify-center items-center",
                {children}
                input {
                    class: "flex flex-row w-full justify-start items-center bg-transparent text-disabled focus:outline-none",
                    r#type: "text",
                    placeholder,
                    readonly: true,
                    value,
                }
            }
            p { class: "text-text-gray text-start w-full text-sm font-normal", {description} }
        }
    }
}
