use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryPerspective {
    MySide,
    RivalSide,
    Spectator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistorySideHint {
    My,
    Rival,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryParticipantType {
    Player,
    Object,
    Boss,
    NonPlayer,
    NonPlayer9,
    NonPlayer17,
    NonPlayer21,
    NonPlayer157,
}

impl CombatHistoryParticipantType {
    pub fn from_raw(raw: u8) -> Result<Self, String> {
        match raw {
            0 => Ok(Self::Player),
            1 => Ok(Self::Object),
            2 => Ok(Self::Boss),
            3 => Ok(Self::NonPlayer),
            9 => Ok(Self::NonPlayer9),
            17 => Ok(Self::NonPlayer17),
            21 => Ok(Self::NonPlayer21),
            157 => Ok(Self::NonPlayer157),
            value => Err(format!("unknown combat participant type: {value}")),
        }
    }

    pub fn raw(self) -> u8 {
        match self {
            Self::Player => 0,
            Self::Object => 1,
            Self::Boss => 2,
            Self::NonPlayer => 3,
            Self::NonPlayer9 => 9,
            Self::NonPlayer17 => 17,
            Self::NonPlayer21 => 21,
            Self::NonPlayer157 => 157,
        }
    }

    pub fn is_player(self) -> bool {
        self == Self::Player
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
    if participant_type.is_player() {
        return CombatHistorySideHint::My;
    }

    let matches_my_type = !participant_type.is_player() && participant_type == my_participant_type;
    let matches_rival_type =
        !participant_type.is_player() && participant_type == rival_participant_type;
    match (matches_my_type, matches_rival_type) {
        (true, false) => CombatHistorySideHint::My,
        (false, true) => CombatHistorySideHint::Rival,
        _ => CombatHistorySideHint::Unknown,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySideParticipants {
    pub my_uin: u32,
    pub my_type: CombatHistoryParticipantType,
    pub rival_uin: u32,
    pub rival_type: CombatHistoryParticipantType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySideIdentity {
    pub id: u32,
    pub participant_type: CombatHistoryParticipantType,
    pub position: u8,
}

impl CombatHistorySideIdentity {
    pub fn from_raw(id: u32, participant_type: u8, position: u8) -> Result<Self, String> {
        Ok(Self {
            id,
            participant_type: CombatHistoryParticipantType::from_raw(participant_type)?,
            position,
        })
    }
}

pub fn opposite_combat_history_side(side: CombatHistorySideHint) -> Option<CombatHistorySideHint> {
    match side {
        CombatHistorySideHint::My => Some(CombatHistorySideHint::Rival),
        CombatHistorySideHint::Rival => Some(CombatHistorySideHint::My),
        CombatHistorySideHint::Unknown => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryCombatantIdentity {
    pub id: u32,
    pub participant_type: CombatHistoryParticipantType,
}

impl CombatHistoryCombatantIdentity {
    pub fn from_raw(id: u32, participant_type: u8) -> Result<Self, String> {
        Ok(Self {
            id,
            participant_type: CombatHistoryParticipantType::from_raw(participant_type)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryCombatantRef {
    pub identity: CombatHistoryCombatantIdentity,
    pub side: CombatHistorySideHint,
    pub position: u8,
}

impl CombatHistoryCombatantRef {
    pub fn from_raw(
        id: u32,
        participant_type: u8,
        side: CombatHistorySideHint,
        position: u8,
    ) -> Result<Self, String> {
        Ok(Self {
            identity: CombatHistoryCombatantIdentity::from_raw(id, participant_type)?,
            side,
            position,
        })
    }
}

pub fn resolve_combat_history_side_identity(
    identity: CombatHistorySideIdentity,
    participants: CombatHistorySideParticipants,
    contextual_side: CombatHistorySideHint,
) -> Result<CombatHistorySideHint, String> {
    if identity.id == participants.my_uin {
        return Ok(CombatHistorySideHint::My);
    }
    if identity.id == participants.rival_uin {
        return Ok(CombatHistorySideHint::Rival);
    }

    let side = resolve_combat_history_side(
        identity.id,
        identity.participant_type,
        participants.my_uin,
        participants.my_type,
        participants.rival_uin,
        participants.rival_type,
    );
    if side != CombatHistorySideHint::Unknown {
        return Ok(side);
    }

    if identity.position != 0 && contextual_side != CombatHistorySideHint::Unknown {
        return Ok(contextual_side);
    }

    Err(format!(
        "combat side unresolved id={} participant_type={:?} position={}",
        identity.id, identity.participant_type, identity.position
    ))
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

impl CombatHistoryFieldEffect {
    pub fn raw_id(self) -> u8 {
        match self {
            Self::ScorchingSun { .. } => 1,
            Self::Rain { .. } => 2,
            Self::Hail { .. } => 3,
            Self::Thunderstorm { .. } => 4,
            Self::DarkCastle { .. } => 5,
            Self::Gale { .. } => 6,
            Self::Dreamland { .. } => 7,
            Self::Miasma { .. } => 8,
            Self::FertileSoil { .. } => 9,
            Self::DragonFormation { .. } => 10,
            Self::MartialRealm { .. } => 11,
            Self::HolyLight { .. } => 12,
            Self::BloodMoon { .. } => 13,
            Self::Labyrinth { .. } => 14,
            Self::IronWall { .. } => 15,
            Self::Fragrance { .. } => 16,
            Self::Fog { .. } => 17,
            Self::Meteor { .. } => 18,
            Self::Mirage { .. } => 19,
            Self::Nebula { .. } => 20,
            Self::Locked { .. } => 21,
            Self::Dawn { .. } => 22,
            Self::WindForestFireMountain { .. } => 23,
            Self::Paradise { .. } => 24,
            Self::Unknown25 { .. } => 25,
            Self::None => 100,
        }
    }

    pub fn rounds_left(self) -> u8 {
        match self {
            Self::None => 0,
            Self::ScorchingSun { rounds_left }
            | Self::Rain { rounds_left }
            | Self::Hail { rounds_left }
            | Self::Thunderstorm { rounds_left }
            | Self::DarkCastle { rounds_left }
            | Self::Gale { rounds_left }
            | Self::Dreamland { rounds_left }
            | Self::Miasma { rounds_left }
            | Self::FertileSoil { rounds_left }
            | Self::DragonFormation { rounds_left }
            | Self::MartialRealm { rounds_left }
            | Self::HolyLight { rounds_left }
            | Self::BloodMoon { rounds_left }
            | Self::Labyrinth { rounds_left }
            | Self::IronWall { rounds_left }
            | Self::Fragrance { rounds_left }
            | Self::Fog { rounds_left }
            | Self::Meteor { rounds_left }
            | Self::Mirage { rounds_left }
            | Self::Nebula { rounds_left }
            | Self::Locked { rounds_left }
            | Self::Dawn { rounds_left }
            | Self::WindForestFireMountain { rounds_left }
            | Self::Paradise { rounds_left }
            | Self::Unknown25 { rounds_left } => rounds_left,
        }
    }

    pub fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
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
    pub field_statuses: Vec<CombatHistorySpiritFieldStatus>,
    pub capture_ratio: Option<u32>,
}

impl CombatHistoryParticipantDisplayState {
    pub fn merge_patch(
        base: Option<Self>,
        patch: CombatHistoryParticipantDisplayState,
    ) -> CombatHistoryParticipantDisplayState {
        if patch.is_capture_ratio_patch() {
            let mut merged = base.unwrap_or_default();
            merged.capture_ratio = patch.capture_ratio;
            return merged;
        }

        let previous_capture_ratio = base.and_then(|state| state.capture_ratio);
        let mut merged = patch;
        if merged.capture_ratio.is_none() {
            merged.capture_ratio = previous_capture_ratio;
        }
        merged
    }

    pub fn is_capture_ratio_patch(&self) -> bool {
        self.capture_ratio.is_some()
            && self.shield_value == 0
            && self.recovery_effect_percent == 0
            && self.locked_enhances.is_empty()
            && self.field_statuses.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatHistorySpiritFieldStatus {
    AllAbnormalImmunity,
    AbnormalImmunity {
        abnormal_state: CombatHistoryAbnormalState,
    },
    NegativeEnhanceImmunity,
    PpDoubleCost,
    ExpelImmunity,
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
    Empty,
    Hidden,
    Fainted,
    Ready,
}

impl CombatHistorySpiritFieldState {
    pub fn from_raw_bits(raw_bits: u8) -> Result<Self, String> {
        match raw_bits {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Hidden),
            2 => Ok(Self::Fainted),
            3 => Ok(Self::Ready),
            _ => Err(format!(
                "unknown combat spirit field state bits: {raw_bits}"
            )),
        }
    }

    pub fn raw_bits(self) -> u8 {
        match self {
            Self::Empty => 0,
            Self::Hidden => 1,
            Self::Fainted => 2,
            Self::Ready => 3,
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

impl Default for CombatHistorySpiritPropertyStages {
    fn default() -> Self {
        Self {
            pa: 0,
            pd: 0,
            ma: 0,
            md: 0,
            ve: 0,
            sp: 0,
            dp: 0,
            crit: 0,
        }
    }
}

impl CombatHistorySpiritPropertyStages {
    pub fn merge_delta(&mut self, delta: &Self) {
        self.pa = merge_bounded_property_stage(self.pa, delta.pa, -6, 6);
        self.pd = merge_bounded_property_stage(self.pd, delta.pd, -6, 6);
        self.ma = merge_bounded_property_stage(self.ma, delta.ma, -6, 6);
        self.md = merge_bounded_property_stage(self.md, delta.md, -6, 6);
        self.ve = merge_bounded_property_stage(self.ve, delta.ve, -6, 6);
        self.sp = merge_bounded_property_stage(self.sp, delta.sp, -6, 6);
        self.dp = merge_bounded_property_stage(self.dp, delta.dp, -6, 6);
    }
}

fn merge_bounded_property_stage(current: i8, delta: i8, min: i8, max: i8) -> i8 {
    current.saturating_add(delta).clamp(min, max)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistorySpiritPropertyStage {
    PhysicalAttack,
    PhysicalDefense,
    MagicAttack,
    MagicDefense,
    Speed,
    Accuracy,
    Evasion,
    Critical,
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
pub struct CombatHistorySpiritGrowthResult {
    pub position: u8,
    pub exp: u32,
    pub level_delta: u8,
    pub current_level: u8,
    pub next_exp: u32,
    pub effort: u16,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn participant_type_accepts_observed_boss_and_object_values() {
        let cases = [
            (0, CombatHistoryParticipantType::Player),
            (1, CombatHistoryParticipantType::Object),
            (2, CombatHistoryParticipantType::Boss),
            (3, CombatHistoryParticipantType::NonPlayer),
            (9, CombatHistoryParticipantType::NonPlayer9),
            (17, CombatHistoryParticipantType::NonPlayer17),
            (21, CombatHistoryParticipantType::NonPlayer21),
            (157, CombatHistoryParticipantType::NonPlayer157),
        ];

        for (raw, participant_type) in cases {
            assert_eq!(
                CombatHistoryParticipantType::from_raw(raw),
                Ok(participant_type)
            );
            assert_eq!(participant_type.raw(), raw);
        }
        assert!(CombatHistoryParticipantType::from_raw(22).is_err());
    }

    #[test]
    fn side_identity_uses_context_when_non_player_types_are_ambiguous() {
        let participants = CombatHistorySideParticipants {
            my_uin: 1773701277,
            my_type: CombatHistoryParticipantType::NonPlayer,
            rival_uin: 1974029771,
            rival_type: CombatHistoryParticipantType::NonPlayer,
        };

        assert_eq!(
            resolve_combat_history_side_identity(
                CombatHistorySideIdentity {
                    id: 470926678,
                    participant_type: CombatHistoryParticipantType::NonPlayer,
                    position: 4,
                },
                participants,
                CombatHistorySideHint::Rival,
            ),
            Ok(CombatHistorySideHint::Rival)
        );
    }

    #[test]
    fn side_identity_rejects_ambiguous_non_player_without_context() {
        let participants = CombatHistorySideParticipants {
            my_uin: 1773701277,
            my_type: CombatHistoryParticipantType::NonPlayer,
            rival_uin: 1974029771,
            rival_type: CombatHistoryParticipantType::NonPlayer,
        };

        assert!(resolve_combat_history_side_identity(
            CombatHistorySideIdentity {
                id: 470926678,
                participant_type: CombatHistoryParticipantType::NonPlayer,
                position: 4,
            },
            participants,
            CombatHistorySideHint::Unknown,
        )
        .is_err());
    }

    #[test]
    fn spirit_field_state_accepts_client_display_states() {
        let cases = [
            (0, CombatHistorySpiritFieldState::Empty),
            (1, CombatHistorySpiritFieldState::Hidden),
            (2, CombatHistorySpiritFieldState::Fainted),
            (3, CombatHistorySpiritFieldState::Ready),
        ];

        for (raw, field_state) in cases {
            assert_eq!(
                CombatHistorySpiritFieldState::from_raw_bits(raw),
                Ok(field_state)
            );
            assert_eq!(field_state.raw_bits(), raw);
        }
        assert!(CombatHistorySpiritFieldState::from_raw_bits(4).is_err());
    }
}
