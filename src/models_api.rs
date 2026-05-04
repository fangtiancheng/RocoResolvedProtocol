use serde::{Deserialize, Serialize};

use crate::CombatHistoryObserved;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedUploadResponse {
    pub battle_id: String,
    pub schema_version: u32,
    pub uploader_uin: u32,
    pub source_hash: String,
    pub inserted: bool,
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
