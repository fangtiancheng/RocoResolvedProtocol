use serde::{Deserialize, Serialize};

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
pub struct TestModeErrorMessage {
    pub request_id: Option<String>,
    pub error: String,
}
