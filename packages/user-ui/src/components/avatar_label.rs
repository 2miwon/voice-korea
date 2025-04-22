use bdk::prelude::*;

#[component]
pub fn AvatarLabel(
    #[props(default = "".to_string())] class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    label: String,
    sub: String,
    #[props(default = None)] img_url: Option<String>,
) -> Element {
    rsx! {
        div { class: "flex flex-row justify-start items-center gap-10 {class}",
            div { class: "w-40 h-40 bg-profile-gray rounded-full shrink-0",
                if let Some(url) = img_url {
                    img { class: "w-full h-full rounded-full", src: "{url}" }
                }
            }
            div { class: "flex flex-col justify-center flex-1 overflow-hidden",
                p { class: "font-semibold text-[15px] justify-start text-text-black truncate",
                    {label}
                }
                p { class: "font-medium text-[12px] justify-start text-text-black truncate",
                    {sub}
                }
            }
        }
    }
}
