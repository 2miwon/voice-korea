use bdk::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Translate)]
pub enum Info {
    #[translate(ko = "임시저장되었습니다.", en = "Temporary save completed.")]
    TempSave,
}
