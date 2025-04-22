use bdk::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectType {
    #[default]
    #[translate(ko = "설문조사")]
    Survey = 1,
    #[translate(ko = "표본조사", en = "Sample Survey")]
    SampleSurvey = 2,
    #[translate(ko = "최종설문", en = "Final Survey")]
    FinalSurvey = 3,
}
