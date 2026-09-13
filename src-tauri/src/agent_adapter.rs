use crate::db::DatabaseState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgentProvider {
    Codex,
    Claude,
}

impl AgentProvider {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Codex => "CODEX",
            Self::Claude => "CLAUDE",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterReadiness {
    pub provider: String,
    pub available: bool,
    pub version: Option<String>,
    pub readiness_state: String,
    pub diagnostic_code: Option<String>,
    pub diagnostic_message: Option<String>,
    pub capabilities: Vec<String>,
    pub supports_resume: bool,
    pub checked_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterStartRequest {
    pub project_id: String,
    pub task_id: Option<String>,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterSession {
    pub id: String,
    pub provider: String,
    pub project_id: String,
    pub task_id: Option<String>,
    pub operation_kind: String,
    pub state: String,
    pub cwd: String,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
    pub final_response: Option<String>,
    pub final_response_truncated: bool,
    pub final_response_state: String,
    pub final_response_role: Option<String>,
    pub diagnostic_code: Option<String>,
    pub diagnostic_message: Option<String>,
    pub prompt_body: Option<String>,
}

/// Common provider lifecycle owned by H!veAI. Implementations must keep all
/// provider-specific identities and process policy behind this boundary.
pub trait AgentAdapter {
    fn provider(&self) -> AgentProvider;
    fn readiness(&self) -> AdapterReadiness;
    fn start(
        &self,
        database: &DatabaseState,
        request: AdapterStartRequest,
    ) -> Result<AdapterSession, String>;
    fn list(&self, database: &DatabaseState, project_id: &str)
        -> Result<Vec<AdapterSession>, String>;
    fn stop(&self, database: &DatabaseState, session_id: &str) -> Result<AdapterSession, String>;
    fn resume(&self, database: &DatabaseState, session_id: &str) -> Result<AdapterSession, String>;
    fn reconcile(&self, database: &DatabaseState) -> Result<(), String>;
}
