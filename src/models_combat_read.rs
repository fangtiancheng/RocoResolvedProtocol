use serde::{Deserialize, Serialize};

use crate::{
    CombatHistoryAbnormalState, CombatHistoryFieldEffect, CombatHistoryGuardianPetStats,
    CombatHistoryIntimacy, CombatHistoryLockedEnhance, CombatHistoryParticipantType,
    CombatHistorySpiritFieldStatus, CombatHistorySpiritPropertyStages, CombatHistorySpiritSex,
    CombatPresentation,
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
    System,
}

impl CombatReadSideKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::My => "我方",
            Self::Rival => "对方",
            Self::System => "系统",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadSnapshot {
    pub phase: CombatPhase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation: Option<CombatPresentation>,
    pub round: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<CombatFinishReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<CombatHistoryFieldEffect>,
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
    pub property_ids: Vec<u32>,
    pub property_names: Vec<String>,
    pub intimacy: CombatHistoryIntimacy,
    pub intimacy_label: String,
    pub bloodline: CombatReadBloodline,
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
pub struct CombatReadEquipment {
    pub equipment_type: u8,
    pub equipment_id: u32,
    pub name: String,
    pub quality: u8,
    pub base_attr: u8,
    pub base_attr_name: String,
    pub base_value: u8,
    pub special_attr: u8,
    pub special_attr_name: String,
    pub special_value: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadStatus {
    pub raw_id: u32,
    pub kind: CombatHistoryAbnormalState,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadFieldStatus {
    pub kind: CombatHistorySpiritFieldStatus,
    pub raw_id: Option<u32>,
    pub name: String,
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
    pub property_id: u32,
    pub property_name: String,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatReadStatusAnimation {
    None,
    Add {
        statuses: Vec<CombatHistoryAbnormalState>,
    },
    RemoveOnly {
        status: CombatHistoryAbnormalState,
    },
    TickThenRemove {
        status: CombatHistoryAbnormalState,
    },
}

impl Default for CombatReadStatusAnimation {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatReadLog {
    pub kind: CombatReadLogKind,
    pub side: CombatReadSideKind,
    pub text: String,
    #[serde(skip_serializing_if = "CombatReadStatusAnimation::is_none")]
    pub status_animation: CombatReadStatusAnimation,
}

impl CombatReadStatusAnimation {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
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
    fn status_animation_none_is_omitted_from_read_log_json() {
        let log = CombatReadLog {
            kind: CombatReadLogKind::State,
            side: CombatReadSideKind::System,
            text: "ready".to_string(),
            status_animation: CombatReadStatusAnimation::None,
        };

        let value = serde_json::to_value(log).unwrap();
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
