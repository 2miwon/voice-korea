use bdk::prelude::*;

#[component]
pub fn InputEnterField(
    #[props(default = 54)] height: i64,
    #[props(default = "inputfield".to_string())] name: String,
    placeholder: String,
    value: String,
    oninput: EventHandler<FormEvent>,
    onenter: EventHandler<KeyboardEvent>,
) -> Element {
    let mut is_focusing_title: Signal<bool> = use_signal(|| false);
    rsx! {
        input {
            class: "flex flex-row w-full justify-start items-center bg-background-gray aria-active:!bg-white aria-active:!border aria-active:!border-active focus:outline-none px-15 py-10 font-medium text-[15px]/22 rounded-[4px]",
            style: "height: {height}px",
            "aria-active": is_focusing_title(),
            r#type: "text",
            placeholder,
            value,
            onfocus: move |_| {
                is_focusing_title.set(true);
            },
            onblur: move |_| {
                is_focusing_title.set(false);
            },
            onkeypress: move |e: KeyboardEvent| {
                let key = e.key();
                if key == Key::Enter {
                    onenter.call(e);
                }
            },
            oninput,
        }
    }
}
