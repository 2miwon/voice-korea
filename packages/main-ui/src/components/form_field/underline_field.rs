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
            class: "flex flex-row w-full h-{height} justify-start items-center bg-white focus:outline-none border-b-[1px] border-[#bfc8d9] px-[15px] py-[15px] font-medium text-[#b4b4b4] text-[15px] leading-[22px]",
            r#type: "text",
            placeholder,
            value,
            oninput,
        }
    }
}
