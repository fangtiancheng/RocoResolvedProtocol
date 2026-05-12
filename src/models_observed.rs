use serde::{Deserialize, Serialize};

use crate::{
    CombatHistoryFieldEffect, CombatHistoryGuardianPetStats, CombatHistoryHpVar,
    CombatHistoryIntimacy, CombatHistoryItem, CombatHistoryNewSpiritInfo,
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
    pub action: CombatHistoryRoundAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryActionAckEvent {
    pub return_code: CombatHistoryReturnCode,
    pub action: CombatHistoryAcknowledgedAction,
    pub pp_left: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryAcknowledgedAction {
    pub kind: CombatHistoryActionKind,
    pub actor_position: u8,
    pub action_slot: Option<u8>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryActionKind {
    Skill,
    ChangeSpirit,
    UseItem,
    Escape,
    Unknown(u8),
}

impl CombatHistoryActionKind {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            1 => Self::Skill,
            2 => Self::ChangeSpirit,
            3 => Self::UseItem,
            4 => Self::Escape,
            value => Self::Unknown(value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryRoundResultEvent {
    pub round: u32,
    pub extra_settlement: bool,
    pub attacks: Vec<CombatHistoryAttackEvent>,
    pub buffs: Vec<CombatHistoryBuffEvent>,
    pub result_info: Option<CombatHistoryResultInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryResultInfo {
    pub finish_reason_code: Option<u8>,
    pub action_availability: CombatHistoryActionAvailability,
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
    #[serde(default = "default_unknown_side_hint")]
    pub actor_side: CombatHistorySideHint,
    #[serde(default)]
    pub action: CombatHistoryRoundAction,
    pub actor_id: u32,
    pub actor_position: u8,
    pub target_id: u32,
    pub target_position: u8,
    pub is_hurt: bool,
    pub is_shaut: bool,
    pub is_miss: bool,
    pub restrain_hint: i8,
    pub affects: Vec<CombatHistoryAttackAffectEvent>,
}

fn default_unknown_side_hint() -> CombatHistorySideHint {
    CombatHistorySideHint::Unknown
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatHistoryRoundAction {
    Skill {
        actor_position: u8,
        skill_id: u32,
    },
    UseItem {
        actor_position: u8,
        item_id: u32,
    },
    ChangeSpirit {
        old_position: u8,
        new_position: u8,
    },
    Escape,
    #[default]
    Unknown,
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
    pub status_changes: Vec<CombatHistoryStatusChange>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryStatusChangeKind {
    Add,
    Remove,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryStatusChange {
    pub status_id: u8,
    pub kind: CombatHistoryStatusChangeKind,
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
    pub change_kind: CombatHistoryChangeSpiritKind,
    pub old_position: u8,
    pub new_position: u8,
    pub buff_ids: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryChangeSpiritKind {
    Normal,
    Silent,
    Unknown(u8),
}

impl CombatHistoryChangeSpiritKind {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::Normal,
            1 => Self::Silent,
            value => Self::Unknown(value),
        }
    }
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
    pub my_action_availability: Option<CombatHistoryActionAvailability>,
    pub spirit_updates: Vec<CombatHistoryObservedSpiritStateDelta>,
    pub participant_display_updates: Vec<CombatHistoryObservedParticipantDisplayStateDelta>,
    pub weather_update: Option<CombatHistoryObservedWeatherDelta>,
    pub finish_reason_code: Option<u8>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryActionAvailability {
    pub can_change_spirit: bool,
    pub can_capture: bool,
    pub can_use_item: bool,
    pub can_escape: bool,
    pub can_use_skill: bool,
}

impl CombatHistoryActionAvailability {
    pub fn from_raw_can_combat(raw_can_combat: u8) -> Self {
        Self {
            can_change_spirit: raw_can_combat_bit(raw_can_combat, 0),
            can_capture: raw_can_combat_bit(raw_can_combat, 1),
            can_use_item: raw_can_combat_bit(raw_can_combat, 2),
            can_escape: raw_can_combat_bit(raw_can_combat, 3),
            can_use_skill: raw_can_combat_bit(raw_can_combat, 4),
        }
    }
}

fn raw_can_combat_bit(raw_can_combat: u8, index: u8) -> bool {
    ((raw_can_combat >> index) & 1) != 0
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
    pub effect: CombatHistoryWeatherEffect,
    pub initial_rounds: Option<u8>,
    pub remaining_rounds: Option<u8>,
    pub effective_round: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatHistoryWeatherEffect {
    None,
    FieldEffect { effect: CombatHistoryFieldEffect },
    Raw { raw_id: u8 },
}

impl CombatHistoryWeatherEffect {
    pub fn from_raw(raw_weather: u8, normalized: Option<CombatHistoryFieldEffect>) -> Self {
        match normalized {
            Some(CombatHistoryFieldEffect::None) => Self::None,
            Some(effect) => Self::FieldEffect { effect },
            None => Self::Raw {
                raw_id: raw_weather,
            },
        }
    }
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
    pub effect: CombatHistoryWeatherEffect,
    pub initial_rounds: Option<u8>,
    pub remaining_rounds: Option<u8>,
}
