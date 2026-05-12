use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryPerspective {
    MySide,
    RivalSide,
    Spectator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistorySideHint {
    My,
    Rival,
    Unknown,
}

pub fn resolve_combat_history_side(
    id: u32,
    participant_type: u8,
    my_uin: u32,
    my_participant_type: u8,
    rival_uin: u32,
    rival_participant_type: u8,
) -> CombatHistorySideHint {
    if id == my_uin {
        return CombatHistorySideHint::My;
    }
    if id == rival_uin {
        return CombatHistorySideHint::Rival;
    }
    if participant_type == 0 {
        return CombatHistorySideHint::My;
    }

    let matches_my_type = participant_type != 0 && participant_type == my_participant_type;
    let matches_rival_type = participant_type != 0 && participant_type == rival_participant_type;
    match (matches_my_type, matches_rival_type) {
        (true, false) => CombatHistorySideHint::My,
        (false, true) => CombatHistorySideHint::Rival,
        _ => CombatHistorySideHint::Unknown,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryFieldEffect {
    ScorchingSun,
    Rain,
    Hail,
    Thunderstorm,
    DarkCastle,
    Gale,
    Dreamland,
    Miasma,
    FertileSoil,
    DragonFormation,
    MartialRealm,
    HolyLight,
    BloodMoon,
    Labyrinth,
    IronWall,
    Fragrance,
    Fog,
    Meteor,
    Mirage,
    Nebula,
    Locked,
    Dawn,
    WindForestFireMountain,
    Paradise,
    Unknown25,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryNormalizedStatus {
    Sleep,
    Numb,
    Fear,
    Burn,
    Freeze,
    Poison,
    ToxicPoison,
    Confusion,
    Parasite,
    Curse,
    Bewilder,
    Nightmare,
    Bind,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryIntimacyKind {
    Progress,
    Friendly,
    Lingxi,
    Affectionate,
    NeverLeave,
    Intimate,
    UnknownFull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryIntimacy {
    pub kind: CombatHistoryIntimacyKind,
    pub closeness: u8,
    pub affiliation: u8,
}

impl CombatHistoryIntimacy {
    pub fn from_raw(closeness: u8, affiliation: u8) -> Self {
        let kind = match affiliation {
            0 => CombatHistoryIntimacyKind::Progress,
            1 => CombatHistoryIntimacyKind::Friendly,
            2 => CombatHistoryIntimacyKind::Lingxi,
            3 => CombatHistoryIntimacyKind::Affectionate,
            4 => CombatHistoryIntimacyKind::NeverLeave,
            5 => CombatHistoryIntimacyKind::Intimate,
            _ => CombatHistoryIntimacyKind::UnknownFull,
        };
        Self {
            kind,
            closeness,
            affiliation,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryReturnCode {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryParticipantIdentity {
    pub side_hint: CombatHistorySideHint,
    pub uin: u32,
    pub participant_type: u8,
    pub nickname: String,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryGuardianPetStats {
    pub energy: u16,
    pub attack: u16,
    pub defend: u16,
    pub ma: u16,
    pub md: u16,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryParticipantDisplayState {
    pub shield_value: u16,
    pub recovery_effect_percent: u8,
    pub locked_enhance_bits: u8,
    pub immune_negative_enhance: bool,
    pub extra_pp_cost: bool,
    pub immune_expel: bool,
    pub immunity_ids: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryItem {
    pub id: u32,
    pub count: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySkillState {
    pub skill_id: u32,
    pub pp_left: u8,
    pub inherited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySpiritEquipment {
    pub server_id: u32,
    pub item_id: u32,
    pub equipment_type: u8,
    pub quality: u8,
    pub base_attr: u8,
    pub base_value: u8,
    pub special_attr: u8,
    pub special_value: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySpiritProperties {
    pub pa: i16,
    pub pd: i16,
    pub ma: i16,
    pub md: i16,
    pub ve: i16,
    pub sp: i16,
    pub dp: i16,
    pub crit: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryHpVar {
    pub hit_times: u8,
    pub hp_v: i16,
    pub hp_left: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryNewSpiritInfo {
    pub id: u32,
    pub level: u8,
    pub disposition: u8,
    pub property_list: Vec<u16>,
    pub flair_list: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySpiritPropertyVar {
    pub index: u8,
    pub exp: u32,
    pub d_level: u8,
    pub c_level: u8,
    pub next_exp: u32,
    pub deffort: u16,
    pub pa: u16,
    pub dpa: u16,
    pub pd: u16,
    pub dpd: u16,
    pub ma: u16,
    pub dma: u16,
    pub md: u16,
    pub dmd: u16,
    pub ve: u16,
    pub dve: u16,
    pub sp: u16,
    pub dsp: u16,
    pub dp: u16,
    pub ddp: u16,
    pub current_skills: Vec<CombatHistorySkillState>,
    pub new_skills: Vec<CombatHistorySkillState>,
    pub evolve_spirit_id: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySpiritPanelStats {
    pub hp: u16,
    pub max_hp: u16,
    pub pa: u16,
    pub pd: u16,
    pub ma: u16,
    pub md: u16,
    pub sp: u16,
}
