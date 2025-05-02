use bdk::prelude::*;
use by_components::icons::upload_download::Download2;
use models::ResourceFile;

#[component]
pub fn ResourcesComponent(title: String, resources: Vec<ResourceFile>) -> Element {
    rsx! {
        div { class: "w-full flex flex-col rounded-lg mb-40 bg-white justify-start items-center py-14 px-20",
            // title and button
            div { class: "w-full flex justify-start items-center gap-13",
                div { class: "w-180 flex flex-row items-center text-base font-bold",
                    span { {title} }
                }
                //file
                div { class: "flex flex-wrap flex-1 justify-start items-center gap-8",
                    for resource in resources {
                        div {
                            class: "cursor-pointer flex flex-row justify-center items-center rounded-full bg-third gap-4 px-12 py-4",
                            onclick: {
                                let files = resource.files.clone();
                                move |_| {
                                    let files = files.clone();
                                    async move {
                                        for file in files.clone() {
                                            #[allow(unused)]
                                            let name = file.name;
                                            #[allow(unused)]
                                            let link = file.url;
                                            #[cfg(feature = "web")]
                                            {
                                                if let Some(link) = link {
                                                    dioxus::document::eval(
                                                        &format!(r#"
                                                            const a = document.createElement("a");
                                                            a.href = "{link}";
                                                            a.download = "{name}";
                                                            a.click();

                                                        "#),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            Download2 {
                                width: "18",
                                height: "18",
                                class: " [&>path]:fill-white",
                            }
                            div { class: "font-medium text-sm/18 text-white", {resource.title} }
                        }
                    }
                }
            }
        }
    }
}
