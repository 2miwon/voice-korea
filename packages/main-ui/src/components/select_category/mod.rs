use bdk::prelude::*;

#[component]
pub fn SelectCategory(
    #[props(default = 215)] width: i64,
    #[props(default = 54)] height: i64,
    selected_field: Option<String>,
    placeholder: String,
    onchange: EventHandler<Event<FormData>>,
    options: Vec<String>,
) -> Element {
    rsx! {
        select {
            class: "focus:outline-none justify-start items-start p-15 bg-background-gray rounded-[4px] font-medium text-[15px] text-disabled",
            style: "width: {width}px; height: {height}px",
            onchange,
            for question_type in options {
                option {
                    value: question_type.clone(),
                    selected: selected_field == Some(question_type),
                    {question_type.clone()}
                }
            }
        }
    }
}
