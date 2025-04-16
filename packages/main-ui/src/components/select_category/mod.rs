use bdk::prelude::*;

#[component]
pub fn SelectCategory(
    #[props(default = 215)] width: i64,
    #[props(default = 54)] height: i64,
    selected_field: Option<String>,
    placeholder: String,
    onchange: EventHandler<Event<FormData>>,
    options: Element,
) -> Element {
    rsx! {
        select {
            class: "focus:outline-none justify-start items-start p-15 bg-background-gray rounded-[4px] font-medium text-[15px] text-disabled",
            style: "width: {width}px; height: {height}px",
            value: match selected_field.as_ref() {
                Some(field) => field.as_str(),
                None => "",
            },
            onchange,
            option { value: "", disabled: true, selected: selected_field == None, {placeholder} }
            {options}
        }
    }
}
