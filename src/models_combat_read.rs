use serde::{Deserialize, Serialize};

use crate::{
    CombatHistoryAbnormalState, CombatHistoryFieldEffect, CombatHistoryGuardianPetStats,
    CombatHistoryIntimacy, CombatHistoryLockedEnhance, CombatHistoryParticipantType,
    CombatHistorySpiritPropertyStages, CombatHistorySpiritSex, CombatPresentation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatPhase {
    Idle,
    WaitingStartReply,
    WaitingFrontendReady,
    WaitingServerOpeningRelease,
    PlayingOpening,
    WaitingPlayerAction,
    WaitingRoundResult,
    PlayingRoundResult,
    WaitingRoundRelease,
    WaitingMyExtraSwitch,
    WaitingOpponentExtraSwitch,
    Finished,
    Aborted,
}

impl CombatPhase {
    pub fn is_active(self) -> bool {
        !matches!(self, Self::Idle | Self::Finished | Self::Aborted)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatFinishReason {
    Win,
    Lose,
    RunAway,
    Aborted,
    Unresolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatStatusSnapshot {
    pub phase: CombatPhase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation: Option<CombatPresentation>,
    pub round: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<CombatFinishReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatReadSideKind {
    My,
    Rival,
}

impl CombatReadSideKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::My => "我方",
            Self::Rival => "对方",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatReadLogSource {
    My,
    Rival,
    System,
}

impl From<CombatReadSideKind> for CombatReadLogSource {
    fn from(side: CombatReadSideKind) -> Self {
        match side {
            CombatReadSideKind::My => Self::My,
            CombatReadSideKind::Rival => Self::Rival,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadSnapshot {
    #[serde(flatten)]
    pub status: CombatStatusSnapshot,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<CombatReadWeather>,
    pub my_side: CombatReadSide,
    pub rival_side: CombatReadSide,
    pub actions: CombatReadActionState,
    pub logs: Vec<CombatReadLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadSide {
    pub participant_type: CombatHistoryParticipantType,
    pub nickname: String,
    pub active_position: u8,
    pub spirits: [Option<CombatReadSpirit>; 6],
    pub display: CombatReadSideDisplay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadSideDisplay {
    pub guardian_pet: Option<CombatHistoryGuardianPetStats>,
    pub shield_value: u16,
    pub recovery_effect_percent: u8,
    pub capture_ratio: Option<u32>,
    pub locked_enhances: Vec<CombatHistoryLockedEnhance>,
    pub field_statuses: Vec<CombatReadFieldStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadSpirit {
    pub spirit_id: u32,
    pub skin_id: u32,
    pub name: String,
    pub level: u8,
    pub sex: CombatHistorySpiritSex,
    pub current_hp: u16,
    pub max_hp: u16,
    pub properties: Vec<CombatReadProperty>,
    pub intimacy: CombatHistoryIntimacy,
    pub intimacy_label: String,
    pub bloodline: Option<CombatReadBloodline>,
    pub equipments: [Option<CombatReadEquipment>; 3],
    pub abnormal_states: Vec<CombatReadStatus>,
    pub property_stages: Option<CombatHistorySpiritPropertyStages>,
    pub skills: Vec<CombatReadSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadBloodline {
    pub id: u32,
    pub name: String,
    pub awakened: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadProperty {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadEquipment {
    pub equipment_type: u8,
    pub equipment_id: u32,
    pub name: String,
    pub quality: u8,
    pub base_effect: CombatReadEquipmentEffect,
    pub special_effect: Option<CombatReadEquipmentEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadEquipmentEffect {
    pub attr: u8,
    pub attr_name: String,
    pub value: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadStatus {
    pub kind: CombatHistoryAbnormalState,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatReadFieldStatusKind {
    AllAbnormalImmunity,
    AbnormalImmunity,
    NegativeEnhanceImmunity,
    PpDoubleCost,
    ExpelImmunity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadFieldStatus {
    pub kind: CombatReadFieldStatusKind,
    pub abnormal_states: Vec<CombatHistoryAbnormalState>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadWeather {
    pub effect: CombatHistoryFieldEffect,
    pub name: String,
    pub description: String,
    pub remaining_rounds: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadSkill {
    pub skill_id: u32,
    pub name: String,
    pub pp_left: u8,
    pub pp_max: u8,
    pub power: String,
    pub speed: i32,
    pub property: Option<CombatReadProperty>,
    pub damage_type: i32,
    pub description: String,
    pub description2: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadActionState {
    pub can_use_skill: bool,
    pub can_capture: bool,
    pub can_recover: bool,
    pub can_change_spirit: bool,
    pub can_escape: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatReadLogKind {
    State,
    Opening,
    Round,
    Skill,
    Item,
    Damage,
    Heal,
    Status,
    Immunity,
    Weather,
    Escape,
    Switch,
    Faint,
    Finish,
    Impact,
    Miss,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadLog {
    pub kind: CombatReadLogKind,
    pub source: CombatReadLogSource,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_activity_is_owned_by_the_protocol_model() {
        assert!(!CombatPhase::Idle.is_active());
        assert!(CombatPhase::WaitingPlayerAction.is_active());
        assert!(CombatPhase::PlayingRoundResult.is_active());
        assert!(!CombatPhase::Finished.is_active());
        assert!(!CombatPhase::Aborted.is_active());
    }

    #[test]
    fn read_log_serializes_its_source_without_animation_payload() {
        let log = CombatReadLog {
            kind: CombatReadLogKind::State,
            source: CombatReadLogSource::System,
            text: "ready".to_string(),
        };

        let value = serde_json::to_value(log).unwrap();
        assert_eq!(value["source"], "system");
        assert!(value.get("statusAnimation").is_none());
    }

    #[test]
    fn status_snapshot_contains_only_the_command_boundary_state() {
        let status = CombatStatusSnapshot {
            phase: CombatPhase::WaitingPlayerAction,
            presentation: None,
            round: 3,
            finish_reason: None,
        };

        let value = serde_json::to_value(status).unwrap();
        assert_eq!(value["phase"], "waiting_player_action");
        assert_eq!(value["round"], 3);
        assert!(value.get("presentation").is_none());
        assert!(value.get("finishReason").is_none());
        assert!(value.get("protocolTrace").is_none());
        assert!(value.get("observedHistory").is_none());
    }
}
