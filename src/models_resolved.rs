use serde::{Deserialize, Serialize};

use crate::{
    CombatHistoryGuardianPetStats, CombatHistoryIntimacy, CombatHistoryNormalizedStatus,
    CombatHistoryParticipantDisplayState, CombatHistoryParticipantIdentity,
    CombatHistorySkillState, CombatHistorySpiritEquipment, CombatHistorySpiritPanelStats,
    CombatHistorySpiritProperties, CombatHistoryWeatherEffect,
};

/// Resolved history contains complete information from all participants
/// This is the server-side merged view with no information hiding
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryResolved {
    pub initial_state: CombatHistoryResolvedInitialState,
    pub snapshots: Vec<CombatHistoryResolvedSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryResolvedInitialState {
    pub my_side: CombatHistoryResolvedParticipantState,
    pub rival_side: CombatHistoryResolvedParticipantState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryResolvedParticipantState {
    pub participant: CombatHistoryParticipantIdentity,
    pub guardian_pet: Option<CombatHistoryGuardianPetStats>,
    pub active_spirit_index: u8,
    pub spirits: [Option<CombatHistoryResolvedSpiritState>; 6],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryResolvedSpiritState {
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
pub struct CombatHistoryResolvedSnapshot {
    pub round: u32,
    pub my_side: CombatHistoryResolvedParticipantSnapshot,
    pub rival_side: CombatHistoryResolvedParticipantSnapshot,
    pub weather: Option<CombatHistoryResolvedWeatherSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryResolvedParticipantSnapshot {
    pub participant: CombatHistoryParticipantIdentity,
    pub guardian_pet: Option<CombatHistoryGuardianPetStats>,
    pub display_state: Option<CombatHistoryParticipantDisplayState>,
    pub active_spirit_index: u8,
    pub spirits: [Option<CombatHistoryResolvedSpiritSnapshot>; 6],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryResolvedSpiritSnapshot {
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
pub struct CombatHistoryResolvedWeatherSnapshot {
    pub effect: CombatHistoryWeatherEffect,
    pub initial_rounds: Option<u8>,
    pub remaining_rounds: Option<u8>,
}
