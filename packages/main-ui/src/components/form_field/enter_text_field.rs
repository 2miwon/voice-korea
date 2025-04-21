use bdk::prelude::*;

#[component]
pub fn EnterTextField(
    #[props(default = 248)] height: i64,
    #[props(default = "textfield".to_string())] name: String,
    placeholder: String,
    value: String,
    oninput: EventHandler<FormEvent>,
    onenter: EventHandler<KeyboardEvent>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-row w-full focus:outline-none justify-start items-start bg-background-gray rounded-[4px]",
            style: "height: {height}px",
            div { class: "flex px-15 py-10 w-full h-full justify-start items-start",
                textarea {
                    class: "flex w-full h-full justify-start items-start bg-transparent focus:outline-none my-10 break-words whitespace-normal",
                    placeholder,
                    name,
                    value,
                    oninput,
                    onkeypress: move |e: KeyboardEvent| {
                        let key = e.key();
                        if key == Key::Enter {
                            onenter.call(e);
                        }
                    },
                }
            }
        }
    }
}
