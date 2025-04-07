#![allow(unused_variables)]
#![allow(unused_mut)]
use bdk::prelude::*;
use models::DeliberationDiscussionCreateRequest;

use crate::{
    components::{expandable_card::ExpandableCard, upload_button::UploadButton},
    pages::deliberations::new::details::discussions::i18n::DocumentTranslate,
    service::metadata_api::MetadataApi,
};

#[cfg(feature = "web")]
use crate::components::drop_zone::handle_file_upload;

use models::File;

//FIXME: fix to connect api
#[component]
pub fn Document(
    lang: Language,
    discussion: DeliberationDiscussionCreateRequest,
    set_discussion: EventHandler<DeliberationDiscussionCreateRequest>,
) -> Element {
    let tr: DocumentTranslate = translate(&lang);
    let api: MetadataApi = use_context();
    let mut documents: Signal<Vec<File>> = use_signal(|| vec![]);

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            div { class: "flex flex-row w-full justify-start items-center gap-50",
                div { class: "font-medium text-[15px] text-black w-fit min-w-100", {tr.schedule} }

                div { class: "flex flex-row gap-20 px-15 w-full h-54 bg-background-gray rounded-sm justify-center items-center",
                    UploadButton {
                        class: "flex min-w-130 h-40 border bg-white border-active rounded-sm text-active text-center font-semibold text-sm justify-center items-center",
                        text: tr.direct_upload,
                        accept: ".pdf,.xls,.xlsx,.csv".to_string(),
                        onuploaded: move |event: FormEvent| {
                            spawn(async move {
                                #[cfg(feature = "web")]
                                if let Some(file_engine) = event.files() {
                                    let result = handle_file_upload(file_engine, api).await;
                                    documents.push(result[0].clone());
                                }
                            });
                        },
                    }

                    div { class: "flex flex-row w-full justify-start items-center gap-5",
                        for document in documents() {
                            div { {document.name.clone()} }
                        }
                    }
                }
            }
        }
    }
}
