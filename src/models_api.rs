use serde::{Deserialize, Serialize};

use crate::CombatHistoryObserved;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedUploadRequest {
    pub experiment_name: Option<String>,
    pub run_id: Option<String>,
    pub source: Option<String>,
    pub history: CombatHistoryObserved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedUploadResponse {
    pub battle_id: String,
    pub schema_version: u32,
    pub uploader_uin: u32,
    pub source_hash: String,
    pub observed_history_id: Option<u64>,
    pub experiment_id: Option<u64>,
    pub inserted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatExperimentSummary {
    pub id: u64,
    pub name: String,
    pub created_by_uin: Option<u32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatExperimentListResponse {
    pub experiments: Vec<CombatExperimentSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatExperimentHistoryIdsResponse {
    pub experiment_id: u64,
    pub name: String,
    pub history_ids: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatExperimentRunSummary {
    pub id: u64,
    pub experiment_id: u64,
    pub experiment_name: String,
    pub request_id: String,
    pub client_uin: Option<u32>,
    pub source: String,
    pub script_hash: Option<String>,
    pub issued_at_unix_ms: Option<u64>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub status: String,
    pub error: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatExperimentRunListResponse {
    pub runs: Vec<CombatExperimentRunSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatExperimentRunHistoriesResponse {
    pub request_id: String,
    pub history_ids: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeDispatchRequest {
    pub target_uin: u32,
    pub request_id: String,
    pub issued_at_unix_ms: u64,
    pub experiment_name: String,
    pub script: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeDispatchResponse {
    pub delivered: bool,
    pub request_id: String,
    pub target_uin: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeStopRequest {
    pub target_uin: u32,
    pub request_id: String,
    pub issued_at_unix_ms: u64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeStopResponse {
    pub delivered: bool,
    pub request_id: String,
    pub target_uin: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TestModeServerMessage {
    RunScript(TestModeRunScriptMessage),
    StopScript(TestModeStopScriptMessage),
    Ping { time_unix_ms: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeRunScriptMessage {
    pub request_id: String,
    pub issued_at_unix_ms: u64,
    pub experiment_name: String,
    pub script: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeStopScriptMessage {
    pub request_id: String,
    pub issued_at_unix_ms: u64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TestModeClientMessage {
    Hello(TestModeHelloMessage),
    Heartbeat(TestModeHeartbeatMessage),
    ScriptStarted(TestModeScriptStartedMessage),
    ScriptFinished(TestModeScriptFinishedMessage),
    ScriptFailed(TestModeScriptFailedMessage),
    ScriptStopped(TestModeScriptStoppedMessage),
    HistoryUploaded(TestModeHistoryUploadedMessage),
    Error(TestModeErrorMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeHelloMessage {
    pub client_id: String,
    pub uin: u32,
    pub app_version: String,
    pub connected_at_unix_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeHeartbeatMessage {
    pub client_id: String,
    pub uin: u32,
    pub time_unix_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeScriptStartedMessage {
    pub request_id: String,
    pub accepted_at_unix_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeScriptFinishedMessage {
    pub request_id: String,
    pub finished_at_unix_ms: u64,
    pub output_tail: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeScriptFailedMessage {
    pub request_id: String,
    pub finished_at_unix_ms: u64,
    pub error: String,
    pub output_tail: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeScriptStoppedMessage {
    pub request_id: String,
    pub stopped_at_unix_ms: u64,
    pub reason: Option<String>,
    pub output_tail: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeHistoryUploadedMessage {
    pub request_id: String,
    pub battle_id: String,
    pub observed_history_id: Option<u64>,
    pub experiment_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestModeErrorMessage {
    pub request_id: Option<String>,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedSummary {
    pub id: u64,
    pub battle_id: String,
    pub schema_version: u32,
    pub uploader_uin: u32,
    pub source_hash: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedListResponse {
    pub records: Vec<CombatHistoryObservedSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedRecord {
    pub id: u64,
    pub battle_id: String,
    pub schema_version: u32,
    pub uploader_uin: u32,
    pub source_hash: String,
    pub created_at: String,
    pub payload: CombatHistoryObserved,
}
