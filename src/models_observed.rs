use serde::{Deserialize, Serialize};

use crate::models_error::CombatHistoryRawValueError;
use crate::{
    CombatHistoryAbnormalState, CombatHistoryCombatantRef, CombatHistoryFieldEffect,
    CombatHistoryGuardianPetStats, CombatHistoryHpVar, CombatHistoryIntimacy, CombatHistoryItem,
    CombatHistoryNewSpiritInfo, CombatHistoryParticipantDisplayState,
    CombatHistoryParticipantIdentity, CombatHistoryPerspective, CombatHistoryReturnCode,
    CombatHistorySideHint, CombatHistorySkillState, CombatHistorySpiritEquipment,
    CombatHistorySpiritFieldState, CombatHistorySpiritGrowthResult, CombatHistorySpiritPanelStats,
    CombatHistorySpiritPropertyStage, CombatHistorySpiritPropertyStages, CombatHistorySpiritSex,
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
    pub round: u32,
    pub source: CombatHistoryFrameSource,
    pub event: CombatHistoryObservedFrameEvent,
    pub state_snapshot: Option<CombatHistoryObservedStateSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryFrameSource {
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

impl CombatHistoryFrameSourceKind {
    pub fn cmd_id(self) -> Option<u32> {
        match self {
            Self::StartReply => Some(0xb0001),
            Self::ActionAck => Some(0xb0003),
            Self::FightResult => Some(0xb0004),
            Self::ChangeSpiritNotify => Some(0xb0007),
            Self::MovieEnd => Some(0xb0008),
            Self::LoadedAck | Self::LocalSubmitAction | Self::LocalSynthetic => None,
        }
    }
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
    pub skill_slot: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatHistoryActionKind {
    Skill,
    ChangeSpirit,
    UseItem,
    Escape,
    ServerUnhandled,
    Unknown(u8),
}

impl CombatHistoryActionKind {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            1 => Self::Skill,
            2 => Self::ChangeSpirit,
            3 => Self::UseItem,
            4 => Self::Escape,
            5 => Self::ServerUnhandled,
            value => Self::Unknown(value),
        }
    }

    pub fn raw(self) -> u8 {
        match self {
            Self::Skill => 1,
            Self::ChangeSpirit => 2,
            Self::UseItem => 3,
            Self::Escape => 4,
            Self::ServerUnhandled => 5,
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for CombatHistoryActionKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Skill => serializer.serialize_str("skill"),
            Self::ChangeSpirit => serializer.serialize_str("change_spirit"),
            Self::UseItem => serializer.serialize_str("use_item"),
            Self::Escape => serializer.serialize_str("escape"),
            Self::ServerUnhandled => serializer.serialize_str("server_unhandled"),
            Self::Unknown(value) => {
                use serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct("CombatHistoryActionKind", 2)?;
                state.serialize_field("kind", "unknown")?;
                state.serialize_field("raw", value)?;
                state.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for CombatHistoryActionKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum EncodedActionKind {
            Known(String),
            Unknown { kind: String, raw: u8 },
        }

        match EncodedActionKind::deserialize(deserializer)? {
            EncodedActionKind::Known(kind) => match kind.as_str() {
                "skill" => Ok(Self::Skill),
                "change_spirit" => Ok(Self::ChangeSpirit),
                "use_item" => Ok(Self::UseItem),
                "escape" => Ok(Self::Escape),
                "server_unhandled" => Ok(Self::ServerUnhandled),
                other => Err(serde::de::Error::unknown_variant(
                    other,
                    &[
                        "skill",
                        "change_spirit",
                        "use_item",
                        "escape",
                        "server_unhandled",
                    ],
                )),
            },
            EncodedActionKind::Unknown { kind, raw } if kind == "unknown" => Ok(Self::Unknown(raw)),
            EncodedActionKind::Unknown { kind, .. } => Err(serde::de::Error::unknown_variant(
                kind.as_str(),
                &["unknown"],
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryRoundResultEvent {
    pub round: u32,
    pub extra_settlement: bool,
    pub events: Vec<CombatHistoryRoundEvent>,
    pub settlement: Option<CombatHistoryRoundSettlement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatHistoryRoundEvent {
    Attack(CombatHistoryAttackEvent),
    Buff(CombatHistoryBuffEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryRoundSettlement {
    pub action_availability: CombatHistoryActionAvailability,
    pub finish: Option<CombatHistoryBattleFinish>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryBattleFinish {
    pub reason: CombatHistoryFinishReason,
    pub spirit_growth_results: Vec<CombatHistorySpiritGrowthResult>,
    pub obtain_items: Vec<CombatHistoryItem>,
    pub captured_spirits: Vec<CombatHistoryNewSpiritInfo>,
    pub trainer_exp: u32,
    pub honour_point: u32,
    pub next_level_trainer_exp: u32,
    pub exp_add_flags: CombatHistoryExpAddFlags,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryExpAddFlags {
    pub vip_bonus: bool,
    pub holiday_bonus: bool,
    pub daily_roco_bonus: bool,
    pub tower_activity_bonus: bool,
    pub unknown_bit5: bool,
    pub unknown_bit6: bool,
    pub unknown_bit7: bool,
    pub unknown_bit8: bool,
}

impl CombatHistoryExpAddFlags {
    pub fn from_raw(raw: u8) -> Self {
        Self {
            vip_bonus: raw & (1 << 0) != 0,
            holiday_bonus: raw & (1 << 1) != 0,
            daily_roco_bonus: raw & (1 << 2) != 0,
            tower_activity_bonus: raw & (1 << 3) != 0,
            unknown_bit5: raw & (1 << 4) != 0,
            unknown_bit6: raw & (1 << 5) != 0,
            unknown_bit7: raw & (1 << 6) != 0,
            unknown_bit8: raw & (1 << 7) != 0,
        }
    }

    pub fn to_raw(self) -> u8 {
        [
            self.vip_bonus,
            self.holiday_bonus,
            self.daily_roco_bonus,
            self.tower_activity_bonus,
            self.unknown_bit5,
            self.unknown_bit6,
            self.unknown_bit7,
            self.unknown_bit8,
        ]
        .into_iter()
        .enumerate()
        .fold(
            0_u8,
            |raw, (index, enabled)| {
                if enabled {
                    raw | (1_u8 << index)
                } else {
                    raw
                }
            },
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryFinishReason {
    Lose,
    Win,
    RunAway,
}

impl CombatHistoryFinishReason {
    pub fn from_raw(raw: u8) -> Result<Option<Self>, CombatHistoryRawValueError> {
        match raw {
            0 => Ok(None),
            1 => Ok(Some(Self::Lose)),
            2 => Ok(Some(Self::Win)),
            3 => Ok(Some(Self::RunAway)),
            raw => Err(CombatHistoryRawValueError::UnknownFinishReason { raw }),
        }
    }

    pub fn raw_code(self) -> u8 {
        match self {
            Self::Lose => 1,
            Self::Win => 2,
            Self::RunAway => 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryAttackEvent {
    pub actor: CombatHistoryAttackParticipant,
    pub target: CombatHistoryAttackParticipant,
    pub action: CombatHistoryRoundAction,
    pub outcome: CombatHistoryAttackOutcome,
    pub form_change: Option<CombatHistoryFormChange>,
    pub weather_change: Option<CombatHistoryFieldEffect>,
    pub affects: Vec<CombatHistoryAttackAffectEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryAttackParticipant {
    pub combatant: CombatHistoryCombatantRef,
    pub display_state: CombatHistoryParticipantDisplayState,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryAttackOutcome {
    pub is_critical: bool,
    pub is_miss: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatHistoryFormChange {
    Awaken {
        trigger_skill_id: u32,
        continues_attack: bool,
    },
    RestoreNormal {
        continues_attack: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatHistoryRoundAction {
    Skill {
        skill_id: u32,
    },
    UseItem {
        item_id: u32,
    },
    ChangeSpirit {
        old_position: Option<u8>,
        new_position: u8,
    },
    Escape,
    ServerUnhandled {
        action_value: u32,
    },
    Unknown {
        action_kind: CombatHistoryActionKind,
        action_value: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryAttackAffectEvent {
    pub target: CombatHistoryCombatantRef,
    pub skill_pp_left: [Option<u8>; 4],
    pub hp_var: CombatHistoryHpVar,
    pub property_stages: CombatHistorySpiritPropertyStages,
    pub side_hp: [Option<u16>; 6],
    pub restrain_hint: CombatHistoryRestrainHint,
    pub immunities: Vec<CombatHistoryImmunity>,
    pub abnormal_state_changes: Vec<CombatHistoryAbnormalStateChange>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatHistoryImmunity {
    InflictAbnormalState {
        abnormal_state: CombatHistoryAbnormalState,
    },
    RemoveAbnormalState {
        abnormal_state: CombatHistoryAbnormalState,
    },
    EnhanceProperty {
        property: CombatHistorySpiritPropertyStage,
    },
    WeakenProperty {
        property: CombatHistorySpiritPropertyStage,
    },
    EnhanceAnyProperty,
    WeakenAnyProperty,
    Unknown {
        effect_id: u32,
    },
}

impl CombatHistoryImmunity {
    pub fn from_effect_id(effect_id: u32) -> Result<Self, CombatHistoryRawValueError> {
        Ok(match effect_id {
            10001..=10013 => CombatHistoryAbnormalState::from_raw_id(effect_id - 10000)
                .map(|abnormal_state| Self::InflictAbnormalState { abnormal_state })?,
            20001..=20013 => CombatHistoryAbnormalState::from_raw_id(effect_id - 20000)
                .map(|abnormal_state| Self::RemoveAbnormalState { abnormal_state })?,
            30001..=30007 => Self::EnhanceProperty {
                property: property_stage_from_effect_offset(effect_id - 30000)?,
            },
            40001..=40007 => Self::WeakenProperty {
                property: property_stage_from_effect_offset(effect_id - 40000)?,
            },
            50001 => Self::EnhanceAnyProperty,
            60001 => Self::WeakenAnyProperty,
            _ => Self::Unknown { effect_id },
        })
    }

    pub fn from_raw_type_id(type_id: u16) -> Result<Self, CombatHistoryRawValueError> {
        Self::from_effect_id(10000 + u32::from(type_id))
    }
}

fn property_stage_from_effect_offset(
    offset: u32,
) -> Result<CombatHistorySpiritPropertyStage, CombatHistoryRawValueError> {
    Ok(match offset {
        1 => CombatHistorySpiritPropertyStage::MagicDefense,
        2 => CombatHistorySpiritPropertyStage::PhysicalAttack,
        3 => CombatHistorySpiritPropertyStage::Accuracy,
        4 => CombatHistorySpiritPropertyStage::MagicAttack,
        5 => CombatHistorySpiritPropertyStage::Evasion,
        6 => CombatHistorySpiritPropertyStage::Speed,
        7 => CombatHistorySpiritPropertyStage::PhysicalDefense,
        offset => {
            return Err(CombatHistoryRawValueError::UnknownImmunityPropertyEffectOffset { offset })
        }
    })
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
    pub fn from_raw(raw: i8) -> Result<Self, CombatHistoryRawValueError> {
        match raw {
            0 => Ok(Self::None),
            -2 => Ok(Self::Resisted),
            -3 => Ok(Self::StronglyResisted),
            2 => Ok(Self::Effective),
            3 => Ok(Self::SuperEffective),
            raw => Err(CombatHistoryRawValueError::UnknownRestrainHint { raw }),
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
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryAbnormalStateTransitionSource {
    AttackAffect,
    BuffTick,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryAbnormalStateChange {
    pub abnormal_state: CombatHistoryAbnormalState,
    pub cause: CombatHistoryAbnormalStateChangeCause,
    pub kind: CombatHistoryAbnormalStateChangeKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition_source: Option<CombatHistoryAbnormalStateTransitionSource>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatHistoryAbnormalStateChangeCause {
    Unknown { raw_code: u8 },
}

impl CombatHistoryAbnormalStateChangeCause {
    pub fn from_raw(raw: u8) -> Self {
        Self::Unknown { raw_code: raw }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryBuffEvent {
    pub main_effect: CombatHistoryBuffEffect,
    pub is_remove: bool,
    pub secondary_effect: Option<CombatHistoryBuffEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryBuffEffect {
    pub target: CombatHistoryCombatantRef,
    pub abnormal_state: CombatHistoryAbnormalState,
    pub hp_var: CombatHistoryHpVar,
    pub property_stages: CombatHistorySpiritPropertyStages,
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
    Forced,
    ForcedSilent,
}

impl CombatHistoryChangeSpiritKind {
    pub fn from_raw(raw: u8) -> Result<Self, CombatHistoryRawValueError> {
        match raw {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Silent),
            2 => Ok(Self::Forced),
            3 => Ok(Self::ForcedSilent),
            raw => Err(CombatHistoryRawValueError::UnknownChangeSpiritKind { raw }),
        }
    }

    pub fn raw(self) -> u8 {
        match self {
            Self::Normal => 0,
            Self::Silent => 1,
            Self::Forced => 2,
            Self::ForcedSilent => 3,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_spirit_kind_accepts_observed_values() {
        let cases = [
            (0, CombatHistoryChangeSpiritKind::Normal),
            (1, CombatHistoryChangeSpiritKind::Silent),
            (2, CombatHistoryChangeSpiritKind::Forced),
            (3, CombatHistoryChangeSpiritKind::ForcedSilent),
        ];

        for (raw, kind) in cases {
            assert_eq!(CombatHistoryChangeSpiritKind::from_raw(raw), Ok(kind));
            assert_eq!(kind.raw(), raw);
        }
        assert!(CombatHistoryChangeSpiritKind::from_raw(4).is_err());
    }

    #[test]
    fn combat_action_kind_round_trips_known_and_unknown_values() {
        let cases = [
            (1, CombatHistoryActionKind::Skill),
            (2, CombatHistoryActionKind::ChangeSpirit),
            (3, CombatHistoryActionKind::UseItem),
            (4, CombatHistoryActionKind::Escape),
            (5, CombatHistoryActionKind::ServerUnhandled),
            (99, CombatHistoryActionKind::Unknown(99)),
        ];

        for (raw, kind) in cases {
            assert_eq!(CombatHistoryActionKind::from_raw(raw), kind);
            assert_eq!(kind.raw(), raw);
        }
    }

    #[test]
    fn combat_action_kind_serializes_unknown_as_explicit_object() {
        assert_eq!(
            serde_json::to_value(CombatHistoryActionKind::Skill).unwrap(),
            serde_json::json!("skill")
        );
        assert_eq!(
            serde_json::to_value(CombatHistoryActionKind::Unknown(99)).unwrap(),
            serde_json::json!({ "kind": "unknown", "raw": 99 })
        );
        assert_eq!(
            serde_json::from_value::<CombatHistoryActionKind>(serde_json::json!({
                "kind": "unknown",
                "raw": 99
            }))
            .unwrap(),
            CombatHistoryActionKind::Unknown(99)
        );
    }

    #[test]
    fn immunity_raw_type_id_maps_to_skill_effect_semantics() {
        assert_eq!(
            CombatHistoryImmunity::from_raw_type_id(6),
            Ok(CombatHistoryImmunity::InflictAbnormalState {
                abnormal_state: CombatHistoryAbnormalState::Toxic,
            })
        );
        assert_eq!(
            CombatHistoryImmunity::from_raw_type_id(50000),
            Ok(CombatHistoryImmunity::Unknown { effect_id: 60000 })
        );
    }
}
