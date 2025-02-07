#![allow(unused)]
use std::str::FromStr;

use dioxus::prelude::*;

use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{AccessLevel, ProjectArea, ResourceSummary, ResourceType, Source, UsagePurpose};

use crate::{
    components::{
        icons::{Navigation, RowOption, Search as SearchIcon, Switch, Upload},
        pagination::Pagination,
    },
    pages::resources::i18n::ResourceTranslate,
};

#[cfg(feature = "web")]
use dioxus::html::HasFileData;

#[cfg(feature = "web")]
use dioxus::web::WebEventExt;

#[cfg(feature = "web")]
use web_sys::window;

use super::controller::{Controller, OrderBy, SortOrder, UpdateResource};

#[component]
pub fn Badge(value: String, #[props(default = "".to_string())] class: String) -> Element {
    rsx! {
        div { class: "inline-block whitespace-nowrap rounded-full bg-black text-white font-semibold {class}",
            div { class: "px-2.5 py-0.5 flex justify-center items-center", {value} }
        }
    }
}

#[component]
pub fn TableRow(
    resource_index: usize,
    lang: Language,
    resource: ResourceSummary,
    is_editing: bool,
    onedit: EventHandler<bool>,
    onupdate: EventHandler<UpdateResource>,
    ondownload: EventHandler<i64>,
) -> Element {
    let translate: ResourceTranslate = translate(&lang);
    let no_selection_text = translate.no_selection;
    let ctrl: Controller = use_context();
    let resource_type = match resource.resource_type {
        Some(v) => v.translate(&lang),
        None => no_selection_text,
    };
    let mut resource_type_options = ResourceType::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    resource_type_options.insert(0, no_selection_text.to_string());

    let project_area = match resource.project_area {
        Some(v) => v.translate(&lang),
        None => no_selection_text,
    };
    let mut project_area_options = ProjectArea::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    project_area_options.insert(0, no_selection_text.to_string());

    let usage_purpose = match resource.usage_purpose {
        Some(v) => v.translate(&lang),
        None => no_selection_text,
    };
    let mut usage_purpose_options = UsagePurpose::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    usage_purpose_options.insert(0, no_selection_text.to_string());

    let source = match resource.source {
        Some(v) => v.translate(&lang),
        None => no_selection_text,
    };
    let mut source_options = Source::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    source_options.insert(0, no_selection_text.to_string());

    let access_level = match resource.access_level {
        Some(v) => v.translate(&lang),
        None => no_selection_text,
    };
    let mut access_level_options = AccessLevel::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    access_level_options.insert(0, no_selection_text.to_string());

    rsx! {
        tr {
            tabindex: "0",
            class: "[&>*]:px-6 [&>*]:py-3 [&>*]:text-center h-[56px] [&>*]:w-full [&>*]:text-sm [&>*]:font-semibold [&>*]:text-[#34333e]",
            style: if is_editing { "background: #F7F7F7;" } else { "" },
            onclick: move |evt: MouseEvent| {
                if !is_editing {
                    onedit.call(true);
                    evt.stop_propagation();
                }
            },

            EditableTableBodyCell {
                edit_mode: is_editing,
                default_value: resource_type,
                options: resource_type_options,
                onchange: move |v: String| {
                    let resource_type = ResourceType::from_str(&v).ok();
                    onupdate.call(UpdateResource::ResourceType(resource_type));
                },
            }
            EditableTableBodyCell {
                edit_mode: is_editing,
                default_value: project_area,
                options: project_area_options,
                onchange: move |v: String| {
                    let project_area = ProjectArea::from_str(&v).ok();
                    onupdate.call(UpdateResource::ProjectArea(project_area));
                },
            }
            EditableTableBodyCell {
                edit_mode: is_editing,
                default_value: usage_purpose,

                options: usage_purpose_options,
                onchange: move |v: String| {
                    let purpose = UsagePurpose::from_str(&v).ok();
                    onupdate.call(UpdateResource::UsagePurpose(purpose));
                },
            }
            td { "{resource.title}" }
            //TODO: Use Resource Data
            td {
                Badge {
                    class: "text-white bg-[#2a60d3] rounded-[4px] px-[5px] py-[2px]",
                    value: "공론명",
                }
            }
            EditableTableBodyCell {
                default_value: source,
                edit_mode: is_editing,
                options: source_options,
                onchange: move |v: String| {
                    let source = Source::from_str(&v).ok();
                    onupdate.call(UpdateResource::Source(source));
                },
            }
            EditableTableBodyCell {
                default_value: access_level,
                edit_mode: is_editing,
                options: access_level_options,
                onchange: move |v: String| {
                    let access_level = AccessLevel::from_str(&v).ok();
                    onupdate.call(UpdateResource::AccessLevel(access_level));
                },
            }
            td { class: "font-semibold",
                "{Controller::convert_timestamp_to_date(resource.updated_at)}"
            }
            td {
                onclick: move |event: Event<MouseData>| {
                    event.stop_propagation();
                    event.prevent_default();
                },
                button {
                    class: "text-[#2a60d3] font-semibold text-[14px]",
                    onclick: move |_| {
                        ondownload.call(resource.id);
                    },
                    "{translate.download}"
                }
            }
            td {
                More {
                    options: vec![
                        translate.more_option_update_resource.to_string(),
                        translate.more_option_remove_resource.to_string(),
                    ],
                    onclick: move |option_index| {
                        match option_index {
                            0 => {
                                ctrl.open_modify_resource_modal(resource_index);
                            }
                            1 => {
                                ctrl.open_remove_resource_modal(resource_index);
                            }
                            _ => {
                                tracing::error!("Invalid Option Index: {}", option_index);
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn EditableTableBodyCell(
    edit_mode: bool,
    options: Vec<String>,
    default_value: String,
    onchange: EventHandler<String>,
) -> Element {
    let mut value: Signal<String> = use_signal(|| default_value);
    rsx! {
        td {
            onclick: move |evt| {
                if edit_mode {
                    evt.stop_propagation();
                }
            },
            if edit_mode {
                select {
                    class: "focus:outline-none w-full text-inherit bg-transparent",
                    value: value(),
                    onchange: move |evt: FormEvent| {
                        tracing::debug!("updated value : {}", evt.value());
                        value.set(evt.value());
                        onchange.call(evt.value());
                    },
                    for option in options {
                        option {
                            class: "text-center",
                            value: option.clone(),
                            selected: value() == option,
                            div { class: "flex justify-between items-center", "{option}" }
                        }
                    }
                }
            } else {
                div { class: "text-[#222222] text-sm font-semibold text", "{value}" }
            }
        }
    }
}

#[component]
pub fn TableHeaderCell(
    #[props(default = "".to_string())] class: String,
    value: String,
    #[props(default = None)] order: Option<SortOrder>, // None: default, true: asc, false: desc
    onclick: EventHandler<MouseEvent>,
    #[props(default = false)] disabled: bool,
) -> Element {
    let v = order.clone();
    rsx! {
        th {
            class: "py-[18px] text-[#7C8291] font-semibold text-sm {class} cursor-pointer",
            onclick: move |evt| {
                onclick.call(evt);
            },
            div { class: "flex flex-row gap-[10px] justify-center items-center",
                "{value}"
                if !disabled {
                    if order.is_none() {
                        Switch { width: "18px", height: "18px" }
                    } else if order.unwrap() == SortOrder::Asc {
                        Navigation {}
                    } else {
                        div { class: "rotate-180", Navigation {} }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Search(placeholder: String, onsearch: EventHandler<String>) -> Element {
    let mut value = use_signal(String::default);
    rsx! {
        div { class: "text-sm gap-2 flex flex-row flex-[0_1_590px] justify-between items-center rounded-lg pl-[18px] pr-[15px] py-[10px] bg-[#f7f7f7] border border-[#7c8292] focus:bg-[#ffffff] focus:border-[#2a60d3]",
            input {
                class: "flex flex-row bg-transparent focus:outline-none flex-1",
                r#type: "text",
                placeholder,
                value: value(),
                oninput: move |event| {
                    value.set(event.value());
                },
            }
            SearchIcon { width: "18", height: "18", color: "#7c8292" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ResourceProps {
    lang: Language,
}

#[component]
pub fn More(options: Vec<String>, onclick: EventHandler<usize>) -> Element {
    rsx! {
        div { class: "group relative",
            div {
                class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                onclick: move |event: Event<MouseData>| {
                    event.stop_propagation();
                    event.prevent_default();
                },
                button {
                    RowOption { width: "24", height: "24" }
                }
                nav { class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                    ul { class: "py-1",
                        for (index , option) in options.iter().enumerate() {
                            li {
                                class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                onclick: move |_| {
                                    onclick.call(index);
                                },
                                "{option}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ResourcePage(props: ResourceProps) -> Element {
    let translate: ResourceTranslate = translate(&props.lang);
    let mut ctrl = Controller::new(props.lang)?;
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start mb-6",
                div { class: "text-[#b3b3b3] font-medium text-sm mb-2", "{translate.title}" }
                div { class: "text-[#222222] text-[28px] font-semibold leading-[42px] mb-10",
                    "{translate.title}"
                }
            }
            div { class: "text-[#35343f] font-normal text-sm mb-10", "{translate.description}" }
            div { class: "flex flex-col w-full p-5 bg-white rounded-lg",
                div { class: "flex-1 flex justify-between",
                    Search {
                        placeholder: translate.placeholder.to_string(),
                        onsearch: move |p| {
                            tracing::debug!("Params: {:?}", p);
                        },
                    }
                    button {
                        onclick: move |_| {
                            ctrl.open_create_resource_modal();
                        },
                        div { class: "flex flex-row h-[40px] justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] gap-[5px]",
                            Upload { width: "24", height: "24" }
                            div { class: "text-white font-semibold text-[#16px]",
                                "{translate.upload_resource}"
                            }
                        }
                    }
                }
                div { class: "overflow-x-scroll my-[30px] border border-[#bfc8d9] rounded",
                    table { class: "border-collapse w-full table-fixed",
                        colgroup {
                            col { class: "min-w-[150px]" }
                            col { class: "min-w-[120px]" }
                            col { class: "min-w-[180px]" }
                            col { class: "min-w-[200px]" }
                            col { class: "min-w-[200px]" }
                            col { class: "min-w-[200px]" }
                            col { class: "min-w-[150px]" }
                            col { class: "min-w-[150px]" }
                            col { class: "min-w-[150px]" }
                            col { class: "min-w-[90px] max-w-7xl" }
                        }
                        thead {
                            tr {
                                TableHeaderCell {
                                    value: translate.metadata_type,
                                    order: ctrl.is_sorted_by(OrderBy::ResourceType),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::ResourceType);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.field,
                                    order: ctrl.is_sorted_by(OrderBy::ProjectArea),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::ProjectArea);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.purpose,
                                    order: ctrl.is_sorted_by(OrderBy::UsagePurpose),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::UsagePurpose);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.header_title,
                                    order: ctrl.is_sorted_by(OrderBy::Title),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::Title);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.linked_deliberation_survey,
                                    order: ctrl.is_sorted_by(OrderBy::LinkedDeliberationSurvey),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::LinkedDeliberationSurvey);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.source,
                                    order: ctrl.is_sorted_by(OrderBy::Source),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::Source);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.authority,
                                    order: ctrl.is_sorted_by(OrderBy::AccessLevel),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::AccessLevel);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.last_modified_date,
                                    order: ctrl.is_sorted_by(OrderBy::LastModifiedDate),
                                    onclick: move |v| {
                                        ctrl.handle_sorting_order(OrderBy::LastModifiedDate);
                                    },
                                }
                                TableHeaderCell {
                                    value: translate.function,
                                    disabled: true,
                                    onclick: move |v| {},
                                }
                                th {
                                }
                            }
                        }
                        tbody {
                            for (index , resource) in ctrl.get_resources().into_iter().enumerate() {
                                TableRow {
                                    key: format!("resource-{}", resource.id),
                                    resource_index: index,
                                    lang: props.lang,
                                    is_editing: ctrl.is_editing(index as i32),
                                    resource: resource.clone(),
                                    onedit: move |v: bool| {
                                        if !v {
                                            ctrl.handle_change_editing_row(-1);
                                        } else {
                                            ctrl.handle_change_editing_row(index as i32);
                                        }
                                    },
                                    ondownload: move |id| {
                                        tracing::debug!("Download Button Clicked: {}", id);
                                    },
                                    onupdate: move |update_field| {
                                        ctrl.handle_update_resource(index, update_field);
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
        Pagination {
            total_page: (ctrl.total_count)() as usize,
            current_page: (ctrl.page)(),
            size: ctrl.size,
            onclick: move |page| {
                ctrl.change_page(page);
            },
        }
    }
}
