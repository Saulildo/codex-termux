use std::collections::HashMap;

use codex_protocol::ToolName;
use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::description::CodeModeToolKind;
use crate::FunctionCallOutputContentItem;
use crate::ToolDefinition;

pub const DEFAULT_EXEC_YIELD_TIME_MS: u64 = 10_000;
pub const DEFAULT_WAIT_YIELD_TIME_MS: u64 = 10_000;
pub const DEFAULT_MAX_OUTPUT_TOKENS_PER_EXEC_CALL: usize = 10_000;

#[derive(Clone, Debug)]
pub struct ExecuteRequest {
    pub cell_id: String,
    pub tool_call_id: String,
    pub enabled_tools: Vec<ToolDefinition>,
    pub source: String,
    pub stored_values: HashMap<String, JsonValue>,
    pub yield_time_ms: Option<u64>,
    pub max_output_tokens: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct WaitRequest {
    pub cell_id: String,
    pub yield_time_ms: u64,
    pub terminate: bool,
}

#[derive(Clone, Debug)]
pub struct WaitToPendingRequest {
    pub cell_id: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum RuntimeResponse {
    Yielded {
        cell_id: String,
        content_items: Vec<FunctionCallOutputContentItem>,
    },
    Terminated {
        cell_id: String,
        content_items: Vec<FunctionCallOutputContentItem>,
    },
    Result {
        cell_id: String,
        content_items: Vec<FunctionCallOutputContentItem>,
        stored_values: HashMap<String, JsonValue>,
        error_text: Option<String>,
    },
}

#[derive(Debug, PartialEq)]
pub enum WaitOutcome {
    LiveCell(RuntimeResponse),
    MissingCell(RuntimeResponse),
}

#[derive(Debug, PartialEq)]
pub enum ExecuteToPendingOutcome {
    Pending {
        cell_id: String,
        content_items: Vec<FunctionCallOutputContentItem>,
        pending_tool_call_ids: Vec<String>,
    },
    Completed(RuntimeResponse),
}

#[derive(Debug, PartialEq)]
pub enum WaitToPendingOutcome {
    LiveCell(ExecuteToPendingOutcome),
    MissingCell(RuntimeResponse),
}

impl From<WaitOutcome> for RuntimeResponse {
    fn from(outcome: WaitOutcome) -> Self {
        match outcome {
            WaitOutcome::LiveCell(response) | WaitOutcome::MissingCell(response) => response,
        }
    }
}

#[derive(Debug)]
pub struct CodeModeNestedToolCall {
    pub cell_id: String,
    pub runtime_tool_call_id: String,
    pub tool_name: ToolName,
    pub tool_kind: CodeModeToolKind,
    pub input: Option<JsonValue>,
}
