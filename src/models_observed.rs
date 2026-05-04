use serde::{Deserialize, Serialize};

use crate::{
    CombatHistoryFieldEffect, CombatHistoryFightRequest, CombatHistoryGuardianPetStats,
    CombatHistoryHpVar, CombatHistoryIntimacy, CombatHistoryItem, CombatHistoryNewSpiritInfo,
    CombatHistoryNormalizedStatus, CombatHistoryParticipantDisplayState,
    CombatHistoryParticipantIdentity, CombatHistoryPerspective, CombatHistoryReturnCode,
    CombatHistorySideHint, CombatHistorySkillState, CombatHistorySpiritEquipment,
    CombatHistorySpiritPanelStats, CombatHistorySpiritProperties, CombatHistorySpiritPropertyVar,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObserved {
    pub schema_version: u32,
    pub battle_id: String,
    pub uploader_uin: u32,
    pub perspective: CombatHistoryPerspective,
    pub battle_started_at_unix_ms: u64,
    pub initial_state: CombatHistoryObservedInitialState,
    pub frames: Vec<CombatHistoryObservedFrame>,
    pub finish_reason_code: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedInitialState {
    pub combat_type: u8,
    pub my_side: CombatHistoryObservedParticipantState,
    pub rival_side: CombatHistoryObservedParticipantState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedParticipantState {
    pub participant: CombatHistoryParticipantIdentity,
    pub guardian_pet: Option<CombatHistoryGuardianPetStats>,
    pub active_spirit_index: u8,
    pub spirits: [Option<CombatHistoryObservedSpiritState>; 6],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedSpiritState {
    pub spirit_id: u32,
    pub level: u8,
    pub sex: u8,
    pub current_hp: u16,
    pub max_hp: u16,
    pub skin_id: u32,
    pub talent_type: u16,
    pub talent_level: u16,
    pub closeness: u8,
    pub affiliation: u8,
    pub intimacy: CombatHistoryIntimacy,
    pub skills: [Option<CombatHistorySkillState>; 4],
    pub equipments: [Option<CombatHistorySpiritEquipment>; 3],
    pub extra_equipment_template_ids: Vec<u16>,
    pub panel_stats: Option<CombatHistorySpiritPanelStats>,
    pub base_properties: Option<CombatHistorySpiritProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedFrame {
    pub seq: u64,
    pub round: Option<u32>,
    pub source: CombatHistoryFrameSource,
    pub event: CombatHistoryObservedFrameEvent,
    pub state_delta: CombatHistoryObservedStateDelta,
    pub state_snapshot: Option<CombatHistoryObservedStateSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryFrameSource {
    pub cmd_id: Option<u32>,
    pub ui_serial_num: Option<u32>,
    pub source_kind: CombatHistoryFrameSourceKind,
    pub packet_summary: Option<CombatHistoryPacketSummaryRef>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryFrameSourceKind {
    StartReply,
    LoadedAck,
    ActionAck,
    FightResult,
    ChangeSpiritNotify,
    MovieEnd,
    LocalSubmitAction,
    LocalSynthetic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryPacketSummaryRef {
    pub body_len: Option<u32>,
    pub decode_ok: bool,
    pub protocol_version: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CombatHistoryObservedFrameEvent {
    Start,
    ActionSubmitted(CombatHistoryActionSubmittedEvent),
    ActionAck(CombatHistoryActionAckEvent),
    RoundResult(CombatHistoryRoundResultEvent),
    ChangeSpirit(CombatHistoryChangeSpiritEvent),
    MovieEnd(CombatHistoryMovieEndEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryActionSubmittedEvent {
    pub request: CombatHistoryFightRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryActionAckEvent {
    pub return_code: CombatHistoryReturnCode,
    pub req_type: u8,
    pub spirit_index: u8,
    pub skill_index: u8,
    pub pp_left: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryRoundResultEvent {
    pub round: u32,
    pub attacks: Vec<CombatHistoryAttackEvent>,
    pub buffs: Vec<CombatHistoryBuffEvent>,
    pub result_info: Option<CombatHistoryResultInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryResultInfo {
    pub can_combat: u8,
    pub finished: u8,
    pub version: u32,
    pub spirit_props_var: Vec<CombatHistorySpiritPropertyVar>,
    pub obtain_items: Vec<CombatHistoryItem>,
    pub spirit_infos: Vec<CombatHistoryNewSpiritInfo>,
    pub trainer_exp: u32,
    pub honour_point: u32,
    pub next_level_trainer_exp: u32,
    pub meet_condition: u8,
    pub exp_add_bits: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryAttackEvent {
    pub offense_id: u32,
    pub offense_type: u8,
    pub offense_index: u8,
    pub defense_id: u32,
    pub defense_type: u8,
    pub defense_index: u8,
    pub skill_type: u8,
    pub skill_id: u32,
    pub is_hurt: bool,
    pub is_shaut: bool,
    pub is_miss: bool,
    pub restrain_hint: i8,
    pub affects: Vec<CombatHistoryAttackAffectEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryAttackAffectEvent {
    pub id: u32,
    pub index: u8,
    pub hp_var: CombatHistoryHpVar,
    pub pro_vars: CombatHistorySpiritProperties,
    pub all_spirits_hp: Vec<u16>,
    pub restrain_type: i8,
    pub immunity_status_ids: Vec<u16>,
    pub buff_status_ids: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryBuffEvent {
    pub id: u32,
    pub index: u8,
    pub buff_id: u8,
    pub hp_var: CombatHistoryHpVar,
    pub pro_vars: CombatHistorySpiritProperties,
    pub is_remove: bool,
    pub is_other_pro: bool,
    pub other_id: u32,
    pub other_index: u8,
    pub other_buff_id: u8,
    pub other_hp_var: Option<CombatHistoryHpVar>,
    pub other_pro_vars: Option<CombatHistorySpiritProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryChangeSpiritEvent {
    pub owner: CombatHistorySideHint,
    pub actor_id: u32,
    pub change_type: u8,
    pub can_combat: u8,
    pub old_index: u8,
    pub new_index: u8,
    pub resolved_old_index: Option<u8>,
    pub resolved_new_index: Option<u8>,
    pub buff_ids: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryMovieEndEvent {
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedStateDelta {
    pub my_active_spirit_index: Option<u8>,
    pub rival_active_spirit_index: Option<u8>,
    pub my_can_combat_mask: Option<u8>,
    pub spirit_updates: Vec<CombatHistoryObservedSpiritStateDelta>,
    pub participant_display_updates: Vec<CombatHistoryObservedParticipantDisplayStateDelta>,
    pub weather_update: Option<CombatHistoryObservedWeatherDelta>,
    pub finish_reason_code: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedParticipantDisplayStateDelta {
    pub owner: CombatHistorySideHint,
    pub display_state: CombatHistoryParticipantDisplayState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedSpiritStateDelta {
    pub owner: CombatHistorySideHint,
    pub spirit_index: u8,
    pub current_hp: Option<u16>,
    pub max_hp: Option<u16>,
    pub hp_var: Option<CombatHistoryHpVar>,
    pub base_properties: Option<CombatHistorySpiritProperties>,
    pub spirit_state_bits_by_slot: Option<[u8; 6]>,
    pub abnormal_state_ids: Option<Vec<u8>>,
    pub normalized_statuses: Option<Vec<CombatHistoryNormalizedStatus>>,
    pub pp_updates: Vec<CombatHistorySkillPpDelta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySkillPpDelta {
    pub slot_index: u8,
    pub skill_id: Option<u32>,
    pub pp_left: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedWeatherDelta {
    pub raw_weather: u8,
    pub raw_round_flag: Option<u8>,
    pub normalized_field_effect: Option<CombatHistoryFieldEffect>,
    pub derived_remaining_rounds: Option<u8>,
    pub effective_round: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedStateSnapshot {
    pub round: u32,
    pub my_side: CombatHistoryObservedParticipantSnapshot,
    pub rival_side: CombatHistoryObservedParticipantSnapshot,
    pub weather: Option<CombatHistoryObservedWeatherSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedParticipantSnapshot {
    pub participant: CombatHistoryParticipantIdentity,
    pub guardian_pet: Option<CombatHistoryGuardianPetStats>,
    pub display_state: Option<CombatHistoryParticipantDisplayState>,
    pub active_spirit_index: u8,
    pub spirits: [Option<CombatHistoryObservedSpiritSnapshot>; 6],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedSpiritSnapshot {
    pub spirit_id: u32,
    pub level: u8,
    pub sex: u8,
    pub current_hp: u16,
    pub max_hp: u16,
    pub closeness: u8,
    pub affiliation: u8,
    pub intimacy: CombatHistoryIntimacy,
    pub talent_type: u16,
    pub talent_level: u16,
    pub skin_id: u32,
    pub skills: [Option<CombatHistorySkillState>; 4],
    pub equipments: [Option<CombatHistorySpiritEquipment>; 3],
    pub extra_equipment_template_ids: Vec<u16>,
    pub base_properties: Option<CombatHistorySpiritProperties>,
    pub abnormal_state_ids: Vec<u32>,
    pub normalized_statuses: Vec<CombatHistoryNormalizedStatus>,
    pub spirit_state_bits: Option<u8>,
    pub capture_ratio: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedWeatherSnapshot {
    pub raw_weather: u8,
    pub raw_round_flag: Option<u8>,
    pub normalized_field_effect: Option<CombatHistoryFieldEffect>,
    pub derived_remaining_rounds: Option<u8>,
}
