use serde::{Deserialize, Serialize};

use crate::{
    CombatHistoryAbnormalState, CombatHistoryFieldEffect, CombatHistoryGuardianPetStats,
    CombatHistoryHpVar, CombatHistoryIntimacy, CombatHistoryItem, CombatHistoryNewSpiritInfo,
    CombatHistoryParticipantDisplayState, CombatHistoryParticipantIdentity,
    CombatHistoryPerspective, CombatHistoryReturnCode, CombatHistorySideHint,
    CombatHistorySkillState, CombatHistorySpiritEquipment, CombatHistorySpiritFieldState,
    CombatHistorySpiritPanelStats, CombatHistorySpiritPropertyStages,
    CombatHistorySpiritPropertyVar, CombatHistorySpiritSex,
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
    pub sex: CombatHistorySpiritSex,
    pub current_hp: u16,
    pub max_hp: u16,
    pub skin_id: u32,
    pub talent_type: u16,
    pub talent_level: u16,
    pub intimacy: CombatHistoryIntimacy,
    pub skills: [Option<CombatHistorySkillState>; 4],
    pub equipments: [Option<CombatHistorySpiritEquipment>; 3],
    pub panel_stats: Option<CombatHistorySpiritPanelStats>,
    pub property_stages: Option<CombatHistorySpiritPropertyStages>,
    pub field_state: CombatHistorySpiritFieldState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedFrame {
    pub seq: u64,
    pub round: Option<u32>,
    pub source: CombatHistoryFrameSource,
    pub event: CombatHistoryObservedFrameEvent,
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
}

impl CombatHistoryActionKind {
    pub fn from_raw(raw: u8) -> Result<Self, String> {
        match raw {
            1 => Ok(Self::Skill),
            2 => Ok(Self::ChangeSpirit),
            3 => Ok(Self::UseItem),
            4 => Ok(Self::Escape),
            value => Err(format!("unknown combat history action kind: {value}")),
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
    pub action: CombatHistoryRoundAction,
    pub actor_id: u32,
    pub actor_position: u8,
    pub target_id: u32,
    pub target_position: u8,
    pub is_critical: bool,
    pub is_miss: bool,
    pub weather_change: Option<CombatHistoryFieldEffect>,
    pub affects: Vec<CombatHistoryAttackAffectEvent>,
}

fn default_unknown_side_hint() -> CombatHistorySideHint {
    CombatHistorySideHint::Unknown
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        old_position: Option<u8>,
        new_position: u8,
    },
    Escape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryAttackAffectEvent {
    pub target_id: u32,
    pub target_side: CombatHistorySideHint,
    pub target_position: u8,
    pub skill_pp_left: [Option<u8>; 4],
    pub hp_var: CombatHistoryHpVar,
    pub property_stages: CombatHistorySpiritPropertyStages,
    pub side_hp: [Option<u16>; 6],
    pub restrain_hint: CombatHistoryRestrainHint,
    pub immunities: Vec<CombatHistoryAbnormalState>,
    pub abnormal_state_changes: Vec<CombatHistoryAbnormalStateChange>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryRestrainHint {
    None,
    Resisted,
    StronglyResisted,
    Effective,
    SuperEffective,
}

impl CombatHistoryRestrainHint {
    pub fn from_raw(raw: i8) -> Result<Self, String> {
        match raw {
            0 => Ok(Self::None),
            -2 => Ok(Self::Resisted),
            -3 => Ok(Self::StronglyResisted),
            2 => Ok(Self::Effective),
            3 => Ok(Self::SuperEffective),
            value => Err(format!("unknown combat restrain hint: {value}")),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryAbnormalStateChangeKind {
    Add,
    Remove,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryAbnormalStateChange {
    pub abnormal_state: CombatHistoryAbnormalState,
    pub cause: u8,
    pub kind: CombatHistoryAbnormalStateChangeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryBuffEvent {
    pub id: u32,
    pub target_side: CombatHistorySideHint,
    pub index: u8,
    pub abnormal_state: CombatHistoryAbnormalState,
    pub hp_var: CombatHistoryHpVar,
    pub pro_vars: CombatHistorySpiritPropertyStages,
    pub is_remove: bool,
    pub other: Option<CombatHistoryBuffOtherEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryBuffOtherEvent {
    pub other_id: u32,
    pub target_side: CombatHistorySideHint,
    pub other_index: u8,
    pub abnormal_state: CombatHistoryAbnormalState,
    pub other_hp_var: CombatHistoryHpVar,
    pub other_pro_vars: CombatHistorySpiritPropertyStages,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryChangeSpiritEvent {
    pub owner: CombatHistorySideHint,
    pub actor_id: u32,
    pub change_kind: CombatHistoryChangeSpiritKind,
    pub old_position: u8,
    pub new_position: u8,
    pub abnormal_state_changes: Vec<CombatHistoryAbnormalStateChange>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryChangeSpiritKind {
    Normal,
    Silent,
}

impl CombatHistoryChangeSpiritKind {
    pub fn from_raw(raw: u8) -> Result<Self, String> {
        match raw {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Silent),
            value => Err(format!("unknown combat change spirit kind: {value}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryMovieEndEvent {
    pub value: u32,
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
pub struct CombatHistoryObservedParticipantDisplayStatePatch {
    pub owner: CombatHistorySideHint,
    pub display_state: CombatHistoryParticipantDisplayState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryObservedStateSnapshot {
    pub round: u32,
    pub action_availability: Option<CombatHistoryActionAvailability>,
    pub my_side: CombatHistoryObservedParticipantSnapshot,
    pub rival_side: CombatHistoryObservedParticipantSnapshot,
    pub weather: CombatHistoryFieldEffect,
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
    pub sex: CombatHistorySpiritSex,
    pub current_hp: u16,
    pub max_hp: u16,
    pub intimacy: CombatHistoryIntimacy,
    pub talent_type: u16,
    pub talent_level: u16,
    pub skin_id: u32,
    pub skills: [Option<CombatHistorySkillState>; 4],
    pub equipments: [Option<CombatHistorySpiritEquipment>; 3],
    pub property_stages: Option<CombatHistorySpiritPropertyStages>,
    pub field_state: CombatHistorySpiritFieldState,
    pub abnormal_states: Vec<CombatHistoryAbnormalState>,
}
