#![allow(non_snake_case)]
use by_components::loaders::cube_loader::CubeLoader;
use dioxus::prelude::*;

use dioxus_logger::tracing;
use dioxus_translate::Language;

use crate::components::PopupZone;
use crate::pages::main_footer::MainFooter;
use crate::pages::main_header::MainHeader;
use crate::routes::Route;

#[component]
pub fn MainLayout(lang: Language) -> Element {
    let route = use_route::<Route>();

    let is_main = if let Route::MainPage { .. } = route {
        true
    } else {
        false
    };
    rsx! {
        ErrorBoundary {
            handle_error: move |e| {
                tracing::error!("error: {:?}", e);
                rsx! { "error : " }
            },
            div { class: "flex flex-col w-screen min-h-screen justify-center items-center bg-white text-black",
                div { class: "w-full flex flex-col max-w-desktop px-10",
                    MainHeader { lang }
                }
                div { class: "w-full flex flex-col justify-center items-center",
                    SuspenseBoundary {
                        fallback: |_| rsx! {
                            div { class: "absolute w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                                CubeLoader {}
                            }
                        },
                        div { class: "flex flex-col w-full min-h-lvh pt-(--header-height)",
                            Outlet::<Route> {}
                        }
                        div { class: "flex flex-row w-full",
                            if is_main {
                                MainFooter { lang }
                            }
                        }
                    }
                }
                PopupZone {}
            }
        }
    }
}
