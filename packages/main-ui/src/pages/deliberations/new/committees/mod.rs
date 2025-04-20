mod controller;
mod i18n;
mod page;

mod components;

pub(self) use super::controller::Controller as ParentController;
pub use page::*;
