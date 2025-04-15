use bdk::prelude::*;

#[component]
pub fn UnderlineField(
    #[props(default = 54)] height: i64,
    placeholder: String,
    value: String,
    oninput: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        input {
            class: "flex flex-row w-full justify-start items-center bg-white focus:outline-none border-b-[1px] border-label-border-gray px-15 py-15 font-medium text-disabled text-[15px]/22",
            style: "height: {height}px",
            r#type: "text",
            placeholder,
            value,
            oninput,
        }
    }
}
