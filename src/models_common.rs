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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryParticipantType {
    Player,
    NonPlayer,
}

impl CombatHistoryParticipantType {
    pub fn from_raw(raw: u8) -> Result<Self, String> {
        match raw {
            0 => Ok(Self::Player),
            3 => Ok(Self::NonPlayer),
            value => Err(format!("unknown combat participant type: {value}")),
        }
    }

    pub fn raw(self) -> u8 {
        match self {
            Self::Player => 0,
            Self::NonPlayer => 3,
        }
    }
}

pub fn resolve_combat_history_side(
    id: u32,
    participant_type: CombatHistoryParticipantType,
    my_uin: u32,
    my_participant_type: CombatHistoryParticipantType,
    rival_uin: u32,
    rival_participant_type: CombatHistoryParticipantType,
) -> CombatHistorySideHint {
    if id == my_uin {
        return CombatHistorySideHint::My;
    }
    if id == rival_uin {
        return CombatHistorySideHint::Rival;
    }
    if participant_type == CombatHistoryParticipantType::Player {
        return CombatHistorySideHint::My;
    }

    let matches_my_type = participant_type != CombatHistoryParticipantType::Player
        && participant_type == my_participant_type;
    let matches_rival_type = participant_type != CombatHistoryParticipantType::Player
        && participant_type == rival_participant_type;
    match (matches_my_type, matches_rival_type) {
        (true, false) => CombatHistorySideHint::My,
        (false, true) => CombatHistorySideHint::Rival,
        _ => CombatHistorySideHint::Unknown,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatHistoryFieldEffect {
    None,
    ScorchingSun { rounds_left: u8 },
    Rain { rounds_left: u8 },
    Hail { rounds_left: u8 },
    Thunderstorm { rounds_left: u8 },
    DarkCastle { rounds_left: u8 },
    Gale { rounds_left: u8 },
    Dreamland { rounds_left: u8 },
    Miasma { rounds_left: u8 },
    FertileSoil { rounds_left: u8 },
    DragonFormation { rounds_left: u8 },
    MartialRealm { rounds_left: u8 },
    HolyLight { rounds_left: u8 },
    BloodMoon { rounds_left: u8 },
    Labyrinth { rounds_left: u8 },
    IronWall { rounds_left: u8 },
    Fragrance { rounds_left: u8 },
    Fog { rounds_left: u8 },
    Meteor { rounds_left: u8 },
    Mirage { rounds_left: u8 },
    Nebula { rounds_left: u8 },
    Locked { rounds_left: u8 },
    Dawn { rounds_left: u8 },
    WindForestFireMountain { rounds_left: u8 },
    Paradise { rounds_left: u8 },
    Unknown25 { rounds_left: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatHistoryIntimacy {
    Progress { closeness: u8 },
    Close,
    NeverLeave,
    Inseparable,
    Soulmate,
    Friendly,
}

impl CombatHistoryIntimacy {
    pub fn from_raw(closeness: u8, affiliation: u8) -> Result<Self, String> {
        if closeness > 100 {
            return Err(format!(
                "combat intimacy closeness out of range: {closeness}"
            ));
        }
        if affiliation != 0 && closeness != 100 {
            return Err(format!(
                "combat intimacy invalid full label: affiliation={affiliation}, closeness={closeness}"
            ));
        }
        match affiliation {
            0 => Ok(Self::Progress { closeness }),
            1 => Ok(Self::Close),
            2 => Ok(Self::NeverLeave),
            3 => Ok(Self::Inseparable),
            4 => Ok(Self::Soulmate),
            5 => Ok(Self::Friendly),
            _ => Err(format!(
                "unknown combat intimacy affiliation: {affiliation}"
            )),
        }
    }

    pub fn closeness(self) -> u8 {
        match self {
            Self::Progress { closeness } => closeness,
            _ => 100,
        }
    }
}

impl Default for CombatHistoryIntimacy {
    fn default() -> Self {
        Self::Progress { closeness: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistorySpiritSex {
    Male,
    Female,
}

impl CombatHistorySpiritSex {
    pub fn from_raw(raw: u8) -> Result<Self, String> {
        match raw {
            1 => Ok(Self::Male),
            2 => Ok(Self::Female),
            _ => Err(format!("unknown combat spirit sex: {raw}")),
        }
    }
}

impl Default for CombatHistorySpiritSex {
    fn default() -> Self {
        Self::Male
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
    pub participant_type: CombatHistoryParticipantType,
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
    pub locked_enhances: Vec<CombatHistoryLockedEnhance>,
    pub immune_negative_enhance: bool,
    pub extra_pp_cost: bool,
    pub immune_expel: bool,
    pub immunities: Vec<CombatHistoryAbnormalState>,
    pub capture_ratio: Option<u32>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryLockedEnhance {
    PhysicalAttack,
    PhysicalDefense,
    MagicAttack,
    MagicDefense,
    Speed,
    Accuracy,
    Evasion,
    Critical,
}

impl CombatHistoryLockedEnhance {
    pub fn from_raw_bit(raw_bit: u8) -> Result<Self, String> {
        match raw_bit {
            0 => Ok(Self::PhysicalAttack),
            1 => Ok(Self::PhysicalDefense),
            2 => Ok(Self::MagicAttack),
            3 => Ok(Self::MagicDefense),
            4 => Ok(Self::Speed),
            5 => Ok(Self::Accuracy),
            6 => Ok(Self::Evasion),
            7 => Ok(Self::Critical),
            _ => Err(format!("unknown combat locked enhance bit: {raw_bit}")),
        }
    }

    pub fn raw_bit(self) -> u8 {
        match self {
            Self::PhysicalAttack => 0,
            Self::PhysicalDefense => 1,
            Self::MagicAttack => 2,
            Self::MagicDefense => 3,
            Self::Speed => 4,
            Self::Accuracy => 5,
            Self::Evasion => 6,
            Self::Critical => 7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryAbnormalState {
    Sleep,
    Paralysis,
    Burn,
    Frozen,
    Poison,
    Toxic,
    Confusion,
    Fear,
    LeechSeed,
    Curse,
    Bewilder,
    Nightmare,
    Bind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistorySpiritFieldState {
    #[default]
    Normal,
    Fainted,
}

impl CombatHistorySpiritFieldState {
    pub fn from_raw_bits(raw_bits: u8) -> Result<Self, String> {
        match raw_bits {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Fainted),
            _ => Err(format!(
                "unknown combat spirit field state bits: {raw_bits}"
            )),
        }
    }

    pub fn raw_bits(self) -> u8 {
        match self {
            Self::Normal => 0,
            Self::Fainted => 1,
        }
    }
}

impl CombatHistoryAbnormalState {
    pub fn from_raw_id(raw_id: u32) -> Result<Self, String> {
        match raw_id {
            1 => Ok(Self::Sleep),
            2 => Ok(Self::Paralysis),
            3 => Ok(Self::Burn),
            4 => Ok(Self::Frozen),
            5 => Ok(Self::Poison),
            6 => Ok(Self::Toxic),
            7 => Ok(Self::Confusion),
            8 => Ok(Self::Fear),
            9 => Ok(Self::LeechSeed),
            10 => Ok(Self::Curse),
            11 => Ok(Self::Bewilder),
            12 => Ok(Self::Nightmare),
            13 => Ok(Self::Bind),
            _ => Err(format!("unknown combat abnormal state id: {raw_id}")),
        }
    }

    pub fn raw_id(self) -> u32 {
        match self {
            Self::Sleep => 1,
            Self::Paralysis => 2,
            Self::Burn => 3,
            Self::Frozen => 4,
            Self::Poison => 5,
            Self::Toxic => 6,
            Self::Confusion => 7,
            Self::Fear => 8,
            Self::LeechSeed => 9,
            Self::Curse => 10,
            Self::Bewilder => 11,
            Self::Nightmare => 12,
            Self::Bind => 13,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySpiritEquipment {
    pub server_id: u32,
    pub item_id: u32,
    pub equipment_type: CombatHistorySpiritEquipmentType,
    pub quality: u8,
    pub base_attr: CombatHistorySpiritEquipmentAttr,
    pub base_value: u8,
    pub special_attr: Option<CombatHistorySpiritEquipmentAttr>,
    pub special_value: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistorySpiritEquipmentAttr {
    PhysicalAttack,
    PhysicalDefense,
    MagicAttack,
    MagicDefense,
    Speed,
    Energy,
    Accuracy,
    Evasion,
    Critical,
    CriticalResistance,
}

impl CombatHistorySpiritEquipmentAttr {
    pub fn from_raw(raw: u8) -> Result<Self, String> {
        match raw {
            1 => Ok(Self::PhysicalAttack),
            2 => Ok(Self::PhysicalDefense),
            3 => Ok(Self::MagicAttack),
            4 => Ok(Self::MagicDefense),
            5 => Ok(Self::Speed),
            6 => Ok(Self::Energy),
            7 => Ok(Self::Accuracy),
            8 => Ok(Self::Evasion),
            9 => Ok(Self::Critical),
            10 => Ok(Self::CriticalResistance),
            _ => Err(format!("unknown combat spirit equipment attr: {raw}")),
        }
    }

    pub fn raw(self) -> u8 {
        match self {
            Self::PhysicalAttack => 1,
            Self::PhysicalDefense => 2,
            Self::MagicAttack => 3,
            Self::MagicDefense => 4,
            Self::Speed => 5,
            Self::Energy => 6,
            Self::Accuracy => 7,
            Self::Evasion => 8,
            Self::Critical => 9,
            Self::CriticalResistance => 10,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistorySpiritEquipmentType {
    Weapon,
    Armor,
    Jewelry,
}

impl CombatHistorySpiritEquipmentType {
    pub fn from_raw(raw: u8) -> Result<Self, String> {
        match raw {
            0 => Ok(Self::Weapon),
            1 => Ok(Self::Armor),
            2 => Ok(Self::Jewelry),
            _ => Err(format!("unknown combat spirit equipment type: {raw}")),
        }
    }

    pub fn raw(self) -> u8 {
        match self {
            Self::Weapon => 0,
            Self::Armor => 1,
            Self::Jewelry => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySpiritPropertyStages {
    pub pa: i8,
    pub pd: i8,
    pub ma: i8,
    pub md: i8,
    pub ve: i8,
    pub sp: i8,
    pub dp: i8,
    pub crit: i8,
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
