use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CodeSubmission {
    #[serde(default = "default_id", skip_deserializing)]
    pub id: String,
    pub language: Language,
    pub code: String,
    pub stdin: String,
    pub expected_stdout: String,
    #[serde(default = "default_time_limit")]
    pub time_limit: u32,
    #[serde(default = "default_memory_limit")]
    pub memory_limit: u32,
    #[serde(default)]
    pub fields: Vec<Field>,
}

fn default_id() -> String {
    Uuid::new_v4().to_string()
}

const DEFAULT_TIME_LIMIT: u32 = 10;
fn default_time_limit() -> u32 {
    DEFAULT_TIME_LIMIT
}

const DEFAULT_MEMORY_LIMIT: u32 = 128 * 1024;
fn default_memory_limit() -> u32 {
    DEFAULT_MEMORY_LIMIT
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum Language {
    #[serde(rename = "c")]
    C,
    #[serde(rename = "c++")]
    Cpp,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "javascript")]
    Javascript,
    #[serde(rename = "python")]
    Python,
}

impl Into<hodu_core::Language> for Language {
    fn into(self) -> hodu_core::Language {
        match self {
            Language::C => hodu_core::Language::C,
            Language::Cpp => hodu_core::Language::Cpp,
            Language::Java => hodu_core::Language::Java,
            Language::Javascript => hodu_core::Language::JavaScript,
            Language::Python => hodu_core::Language::Python,
        }
    }
}

#[derive(Serialize)]
struct MarkResponseFields {
    time: Option<f64>,
    memory: Option<u32>,
    stdout: Option<String>,
    stderr: Option<String>,
}

#[derive(Serialize)]
pub struct MarkResponse {
    status: String,
    fields: MarkResponseFields,
}

impl MarkResponse {
    pub fn new(result: &hodu_core::MarkResult, fields: &Vec<Field>) -> Self {
        MarkResponse {
            status: match result.status {
                hodu_core::MarkResultStatus::Correct => "correct".to_string(),
                hodu_core::MarkResultStatus::Wrong => "wrong".to_string(),
                hodu_core::MarkResultStatus::CompileError => "compile_error".to_string(),
                hodu_core::MarkResultStatus::RuntimeError => "runtime_error".to_string(),
                hodu_core::MarkResultStatus::TimeLimitExceeded => "time_limit_exceeded".to_string(),
                hodu_core::MarkResultStatus::MemoryLimitExceeded => {
                    "memory_limit_exceeded".to_string()
                }
            },
            fields: if fields.contains(&Field::WildCard) {
                MarkResponseFields {
                    time: Some(result.time),
                    memory: Some(result.memory),
                    stdout: Some(result.stdout.clone()),
                    stderr: Some(result.stderr.clone()),
                }
            } else {
                MarkResponseFields {
                    time: if fields.contains(&Field::Time) {
                        Some(result.time)
                    } else {
                        None
                    },
                    memory: if fields.contains(&Field::Memory) {
                        Some(result.memory)
                    } else {
                        None
                    },
                    stdout: if fields.contains(&Field::Stdout) {
                        Some(result.stdout.clone())
                    } else {
                        None
                    },
                    stderr: if fields.contains(&Field::Stderr) {
                        Some(result.stderr.clone())
                    } else {
                        None
                    },
                }
            },
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Field {
    #[serde(rename = "*")]
    WildCard,
    Time,
    Memory,
    Stdout,
    Stderr,
}
