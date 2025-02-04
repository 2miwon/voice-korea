#![allow(unused_variables, unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use by_types::QueryResponse;
use chrono::{TimeZone, Utc};
use dioxus_translate::{Language, Translate};
use validator::ValidationError;

// If you want to know how to use Y macro, refer to https://github.com/biyard/rust-sdk/tree/main/packages/by-macros
#[api_model(base = "/surveys/v2", table = surveys, iter_type=QueryResponse)]
pub struct SurveyV2 {
    #[api_model(summary, primary_key, read_action = find_by_id)]
    pub id: String,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create)]
    pub name: String,

    #[api_model(summary, type = INTEGER)]
    pub project_type: ProjectType,

    #[api_model(summary, action = create, type = INTEGER)]
    pub project_area: ProjectArea,

    #[api_model(summary)]
    pub status: ProjectStatus,

    #[api_model(summary, action = create)]
    pub started_at: i64,

    #[api_model(summary, action = create)]
    pub ended_at: i64,

    #[api_model(action = create)]
    pub description: String,
    #[api_model(summary, action = create)]
    pub quotes: i64,

    #[api_model(summary, action = create, many_to_one = organizations)]
    pub org_id: String,
    #[api_model(action = create, type = JSONB, version = v0.1)]
    pub questions: Vec<Question>,
    // #[api_model(summary, one_to_many= responses, aggregator = count)]
    // pub response_count: i64,

    // #[api_model(summary, many_to_many = attrs, foreign_table_name = attributes, foreign_primary_key = attr_id, foreign_reference_key = survey_id)]
    // pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum Question {
    SingleChoice(ChoiceQuestion),
    MultipleChoice(ChoiceQuestion),
    ShortAnswer(SubjectiveQuestion),
    Subjective(SubjectiveQuestion),
}

impl Default for Question {
    fn default() -> Self {
        Question::ShortAnswer(SubjectiveQuestion::default())
    }
}

impl Question {
    pub fn new(answer_type: &str) -> Self {
        match answer_type {
            "Single Choice" | "객관식(단일선택)" => {
                Question::SingleChoice(ChoiceQuestion::default())
            }

            "Multiple Choice" | "객관식(다중선택)" => {
                Question::MultipleChoice(ChoiceQuestion::default())
            }

            "Short Answer" | "주관식(단답형)" => {
                Question::ShortAnswer(SubjectiveQuestion::default())
            }

            "Subjective" | "주관식(서술형)" => {
                Question::Subjective(SubjectiveQuestion::default())
            }
            _ => {
                panic!("Invalid answer type: {}", answer_type);
            }
        }
    }

    pub fn set_title(&mut self, title: &str) {
        match self {
            Question::SingleChoice(q) => {
                q.title = title.to_string();
            }
            Question::MultipleChoice(q) => {
                q.title = title.to_string();
            }
            Question::ShortAnswer(q) => {
                q.title = title.to_string();
            }
            Question::Subjective(q) => {
                q.title = title.to_string();
            }
        }
    }

    pub fn title(&self) -> String {
        match self {
            Question::SingleChoice(q) => q.title.clone(),
            Question::MultipleChoice(q) => q.title.clone(),
            Question::ShortAnswer(q) => q.title.clone(),
            Question::Subjective(q) => q.title.clone(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Question::SingleChoice(q) => q.description.clone().unwrap_or_default(),
            Question::MultipleChoice(q) => q.description.clone().unwrap_or_default(),
            Question::ShortAnswer(q) => q.description.clone(),
            Question::Subjective(q) => q.description.clone(),
        }
    }

    pub fn set_description(&mut self, description: &str) {
        match self {
            Question::SingleChoice(q) => {
                q.description = Some(description.to_string());
            }
            Question::MultipleChoice(q) => {
                q.description = Some(description.to_string());
            }
            Question::ShortAnswer(q) => {
                q.description = description.to_string();
            }
            Question::Subjective(q) => {
                q.description = description.to_string();
            }
        }
    }

    pub fn add_option(&mut self, option: &str) {
        match self {
            Question::SingleChoice(q) => {
                q.options.push(option.to_string());
            }
            Question::MultipleChoice(q) => {
                q.options.push(option.to_string());
            }
            _ => {
                panic!("Invalid question type for adding option: {:?}", self);
            }
        }
    }

    pub fn options(&self) -> Vec<String> {
        match self {
            Question::SingleChoice(q) => q.options.clone(),
            Question::MultipleChoice(q) => q.options.clone(),
            _ => vec![],
        }
    }

    pub fn to_type(&self, lang: &Language) -> String {
        match (self, lang) {
            (&Question::SingleChoice(_), &Language::En) => "Single Choice".to_string(),
            (&Question::SingleChoice(_), &Language::Ko) => "객관식(단일선택)".to_string(),

            (&Question::MultipleChoice(_), &Language::En) => "Multiple Choice".to_string(),
            (&Question::MultipleChoice(_), &Language::Ko) => "객관식(다중선택)".to_string(),

            (&Question::ShortAnswer(_), &Language::En) => "Short Answer".to_string(),
            (&Question::ShortAnswer(_), &Language::Ko) => "주관식(단답형)".to_string(),

            (&Question::Subjective(_), &Language::En) => "Subjective".to_string(),
            (&Question::Subjective(_), &Language::Ko) => "주관식(서술형)".to_string(),
        }
    }

    pub fn types(lang: &Language) -> Vec<String> {
        match lang {
            Language::En => vec![
                "Single Choice".to_string(),
                "Multiple Choice".to_string(),
                "Short Answer".to_string(),
                "Subjective".to_string(),
            ],
            Language::Ko => vec![
                "객관식(단일선택)".to_string(),
                "객관식(다중선택)".to_string(),
                "주관식(단답형)".to_string(),
                "주관식(서술형)".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubjectiveQuestion {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ChoiceQuestion {
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<String>,
}

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    ApiModel,
    Translate,
    Copy,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectArea {
    #[default]
    #[translate(ko = "경제")]
    Economy = 1,
    #[translate(ko = "사회")]
    Society = 2,
    #[translate(ko = "환경")]
    Environment = 3,
    #[translate(ko = "교육")]
    Education = 4,
    #[translate(ko = "문화")]
    Culture = 5,
    #[translate(ko = "노동")]
    Labor = 6,
    #[translate(ko = "도시")]
    City = 7,
    #[translate(ko = "기술")]
    Technology = 8,
    #[translate(ko = "보건")]
    Health = 9,
    #[translate(ko = "정치")]
    Politics = 10,
}

impl ProjectArea {
    pub fn all() -> Vec<ProjectArea> {
        vec![
            ProjectArea::Economy,
            ProjectArea::Society,
            ProjectArea::Environment,
            ProjectArea::Education,
            ProjectArea::Culture,
            ProjectArea::Labor,
            ProjectArea::City,
            ProjectArea::Technology,
            ProjectArea::Health,
            ProjectArea::Politics,
        ]
    }
}

#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel, Translate,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectType {
    #[default]
    #[translate(ko = "설문조사")]
    Survey = 1,
    #[translate(ko = "공론조사")]
    Deliberation = 2,
}

// FIXME: rename to ProjectStatus after finishing migration from public_opinion.
#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel, Translate,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    #[default]
    #[translate(ko = "준비")]
    Ready = 1,
    #[translate(ko = "진행")]
    InProgress = 2,
    #[translate(ko = "마감")]
    Finish = 3,
}

impl SurveyV2Summary {
    pub fn start_date(&self) -> String {
        let datetime = Utc.timestamp_opt(self.started_at, 0).unwrap();
        let formatted_date = datetime.format("%Y.%m.%d").to_string();
        formatted_date
    }

    pub fn end_date(&self) -> String {
        let datetime = Utc.timestamp_opt(self.ended_at, 0).unwrap();
        let formatted_date = datetime.format("%Y.%m.%d").to_string();
        formatted_date
    }

    pub fn period(&self) -> String {
        format!("{} ~ {}", self.start_date(), self.end_date())
    }

    pub fn response_rate(&self) -> String {
        // TODO: implement real logic for calculation of response rate.
        let responses = 0;

        format!(
            "{}% ({}/{})",
            responses / self.quotes * 100,
            responses,
            self.quotes
        )
    }
}
