use serde::{Deserialize, Serialize};

use crate::models_error::CombatHistoryRawValueError;

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
    NonPlayer8,
    NonPlayer9,
    NonPlayer17,
    NonPlayer21,
}

impl CombatHistoryParticipantType {
    pub fn from_raw(raw: u8) -> Result<Self, CombatHistoryRawValueError> {
        match raw {
            0 => Ok(Self::Player),
            1 => Ok(Self::Object),
            2 => Ok(Self::Boss),
            3 => Ok(Self::NonPlayer),
            8 => Ok(Self::NonPlayer8),
            9 => Ok(Self::NonPlayer9),
            17 => Ok(Self::NonPlayer17),
            21 => Ok(Self::NonPlayer21),
            _ => Err(CombatHistoryRawValueError::UnknownParticipantType { raw }),
        }
    }

    pub fn raw(self) -> u8 {
        match self {
            Self::Player => 0,
            Self::Object => 1,
            Self::Boss => 2,
            Self::NonPlayer => 3,
            Self::NonPlayer8 => 8,
            Self::NonPlayer9 => 9,
            Self::NonPlayer17 => 17,
            Self::NonPlayer21 => 21,
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
    pub fn from_raw(
        id: u32,
        participant_type: u8,
        position: u8,
    ) -> Result<Self, CombatHistoryRawValueError> {
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
#[serde(rename_all = "snake_case")]
pub enum CombatHistorySkillElement {
    /// 普通系。
    Normal,
    /// 火系。
    Fire,
    /// 水系。
    Water,
    /// 草系。
    Grass,
    /// 电系。
    Electric,
    /// 冰系。
    Ice,
    /// 武系。
    Fighting,
    /// 毒系。
    Poison,
    /// 土系。
    Ground,
    /// 翼系。
    Flying,
    /// 萌系。
    Psychic,
    /// 虫系。
    Bug,
    /// 石系。
    Rock,
    /// 幽系。
    Ghost,
    /// 龙系。
    Dragon,
    /// 恶魔系。
    Dark,
    /// 机械系。
    Steel,
    /// 光系。
    Light,
    /// 神火系。
    GodFire,
    /// 神水系。
    GodWater,
    /// 神草系。
    GodGrass,
    /// 未识别的原始属性值。
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistoryCombatantIdentity {
    pub id: u32,
    pub participant_type: CombatHistoryParticipantType,
}

impl CombatHistoryCombatantIdentity {
    pub fn from_raw(id: u32, participant_type: u8) -> Result<Self, CombatHistoryRawValueError> {
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
    ) -> Result<Self, CombatHistoryRawValueError> {
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum CombatHistoryFieldEffect {
    #[default]
    None,
    ScorchingSun {
        rounds_left: u8,
    },
    Rain {
        rounds_left: u8,
    },
    Hail {
        rounds_left: u8,
    },
    Thunderstorm {
        rounds_left: u8,
    },
    DarkCastle {
        rounds_left: u8,
    },
    Gale {
        rounds_left: u8,
    },
    Dreamland {
        rounds_left: u8,
    },
    Miasma {
        rounds_left: u8,
    },
    FertileSoil {
        rounds_left: u8,
    },
    DragonFormation {
        rounds_left: u8,
    },
    MartialRealm {
        rounds_left: u8,
    },
    HolyLight {
        rounds_left: u8,
    },
    BloodMoon {
        rounds_left: u8,
    },
    Labyrinth {
        rounds_left: u8,
    },
    IronWall {
        rounds_left: u8,
    },
    Fragrance {
        rounds_left: u8,
    },
    Fog {
        rounds_left: u8,
    },
    Meteor {
        rounds_left: u8,
    },
    Mirage {
        rounds_left: u8,
    },
    Nebula {
        rounds_left: u8,
    },
    Locked {
        rounds_left: u8,
    },
    Dawn {
        rounds_left: u8,
    },
    WindForestFireMountain {
        rounds_left: u8,
    },
    Paradise {
        rounds_left: u8,
    },
    Unknown25 {
        rounds_left: u8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistoryFieldEffectClass {
    None,
    Weather,
    Environment,
}

impl CombatHistoryFieldEffect {
    pub fn key(self) -> &'static str {
        match self {
            Self::ScorchingSun { .. } => "scorching_sun",
            Self::Rain { .. } => "rain",
            Self::Hail { .. } => "hail",
            Self::Thunderstorm { .. } => "thunderstorm",
            Self::DarkCastle { .. } => "dark_castle",
            Self::Gale { .. } => "gale",
            Self::Dreamland { .. } => "dreamland",
            Self::Miasma { .. } => "miasma",
            Self::FertileSoil { .. } => "fertile_soil",
            Self::DragonFormation { .. } => "dragon_formation",
            Self::MartialRealm { .. } => "martial_realm",
            Self::HolyLight { .. } => "holy_light",
            Self::BloodMoon { .. } => "blood_moon",
            Self::Labyrinth { .. } => "labyrinth",
            Self::IronWall { .. } => "iron_wall",
            Self::Fragrance { .. } => "fragrance",
            Self::Fog { .. } => "fog",
            Self::Meteor { .. } => "meteor",
            Self::Mirage { .. } => "mirage",
            Self::Nebula { .. } => "nebula",
            Self::Locked { .. } => "locked",
            Self::Dawn { .. } => "dawn",
            Self::WindForestFireMountain { .. } => "wind_forest_fire_mountain",
            Self::Paradise { .. } => "paradise",
            Self::Unknown25 { .. } => "unknown_25",
            Self::None => "none",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::ScorchingSun { .. } => "暴晒天气",
            Self::Rain { .. } => "阴雨天气",
            Self::Hail { .. } => "冰雹天气",
            Self::Thunderstorm { .. } => "雷暴天气",
            Self::DarkCastle { .. } => "暗黑城环境",
            Self::Gale { .. } => "疾风天气",
            Self::Dreamland { .. } => "梦境环境",
            Self::Miasma { .. } => "瘴气环境",
            Self::FertileSoil { .. } => "沃土环境",
            Self::DragonFormation { .. } => "龙阵环境",
            Self::MartialRealm { .. } => "化境环境",
            Self::HolyLight { .. } => "圣光天气",
            Self::BloodMoon { .. } => "血月天气",
            Self::Labyrinth { .. } => "迷宫环境",
            Self::IronWall { .. } => "铁壁环境",
            Self::Fragrance { .. } => "芬芳环境",
            Self::Fog { .. } => "迷雾天气",
            Self::Meteor { .. } => "陨石天气",
            Self::Mirage { .. } => "海市蜃楼环境",
            Self::Nebula { .. } => "星云天气",
            Self::Locked { .. } => "锁定",
            Self::Dawn { .. } => "曙光天气",
            Self::WindForestFireMountain { .. } => "风林火山环境",
            Self::Paradise { .. } => "乐园环境",
            Self::Unknown25 { .. } => "未知环境25",
            Self::None => "无",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::ScorchingSun { .. } => {
                "火系与神火系技能威力伤害上升，水系与神水系技能威力伤害下降，电系技能命中下降"
            }
            Self::Rain { .. } => {
                "火系与神火系技能威力伤害下降，水系与神水系技能威力伤害上升，电系技能命中上升"
            }
            Self::Hail { .. } => "冰系技能威力伤害上升",
            Self::Thunderstorm { .. } => {
                "电系技能威力伤害上升，非电系宠物行动时若未免疫麻醉则全技能损失PP"
            }
            Self::DarkCastle { .. } => "恶魔系宠物受到威力伤害下降",
            Self::Gale { .. } => "技能先手值相同时翼系宠物优先行动，翼系宠物先手造成的所有伤害上升",
            Self::Dreamland { .. } => "反转行动顺序",
            Self::Miasma { .. } => {
                "对手处于中毒与剧毒异常状态时毒系宠物技能威力伤害上升，非毒系宠物精力回复效果下降"
            }
            Self::FertileSoil { .. } => "土系宠物双防能力值上升，水系草系与冰系宠物速度能力值下降",
            Self::DragonFormation { .. } => {
                "龙系宠物每回合首次受到威力伤害时，删除龙系对手对应威力技能PP与非龙系攻击者所有威力技能PP"
            }
            Self::MartialRealm { .. } => "武系宠物受到威力伤害下降，武系宠物攻击时不被概率闪避",
            Self::HolyLight { .. } => {
                "光系技能威力伤害上升，光系宠物受到非神系技能威力伤害下降，双方免疫异常状态"
            }
            Self::BloodMoon { .. } => "恶魔系技能威力伤害上升，恶魔系宠物每回合回复精力",
            Self::Labyrinth { .. } => "虫系宠物受到魔法威力伤害下降，非虫系宠物每回合受到伤害",
            Self::IronWall { .. } => "机械系宠物速度能力值和受到克制威力伤害下降",
            Self::Fragrance { .. } => "草系与神草系宠物受到威力伤害下降且每回合回复精力",
            Self::Fog { .. } => "幽灵系技能先手值略微上升，双方每回合大概率被混乱",
            Self::Meteor { .. } => "石系技能威力伤害上升，石系宠物受到物理威力伤害下降",
            Self::Mirage { .. } => {
                "非神水系宠物每回合受到伤害，为神水系宠物回复等量精力，每回合令不处于控制异常的双方睡眠"
            }
            Self::Nebula { .. } => "反转非神系系别的克制抵抗关系",
            Self::Locked { .. } => "当前无天气环境且无法改变",
            Self::Dawn { .. } => "非光系宠物行动后强制重置自身正负面强化",
            Self::WindForestFireMountain { .. } => {
                "双方被攻击时受到额外伤害，精力低于一半的宠物PP回复效果下降，神火系宠物所受额外伤害和精力阈值下降"
            }
            Self::Paradise { .. } => "萌系宠物回复精力效果提升，萌系威力技能无法造成抵抗伤害",
            Self::Unknown25 { .. } => "未知环境25",
            Self::None => "当前无天气环境",
        }
    }

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

    pub fn class(self) -> CombatHistoryFieldEffectClass {
        match self {
            Self::None | Self::Locked { .. } => CombatHistoryFieldEffectClass::None,
            Self::ScorchingSun { .. }
            | Self::Rain { .. }
            | Self::Hail { .. }
            | Self::Thunderstorm { .. }
            | Self::Gale { .. }
            | Self::HolyLight { .. }
            | Self::BloodMoon { .. }
            | Self::Fog { .. }
            | Self::Meteor { .. }
            | Self::Nebula { .. }
            | Self::Dawn { .. } => CombatHistoryFieldEffectClass::Weather,
            Self::DarkCastle { .. }
            | Self::Dreamland { .. }
            | Self::Miasma { .. }
            | Self::FertileSoil { .. }
            | Self::DragonFormation { .. }
            | Self::MartialRealm { .. }
            | Self::Labyrinth { .. }
            | Self::IronWall { .. }
            | Self::Fragrance { .. }
            | Self::Mirage { .. }
            | Self::WindForestFireMountain { .. }
            | Self::Paradise { .. } => CombatHistoryFieldEffectClass::Environment,
            Self::Unknown25 { .. } => CombatHistoryFieldEffectClass::None,
        }
    }

    pub fn is_weather(self) -> bool {
        matches!(self.class(), CombatHistoryFieldEffectClass::Weather)
    }

    pub fn is_environment(self) -> bool {
        matches!(self.class(), CombatHistoryFieldEffectClass::Environment)
    }

    pub fn has_no_weather_environment(self) -> bool {
        matches!(self.class(), CombatHistoryFieldEffectClass::None)
    }

    pub fn same_kind(self, other: Self) -> bool {
        self.raw_id() == other.raw_id()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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
    pub fn from_raw(closeness: u8, affiliation: u8) -> Result<Self, CombatHistoryRawValueError> {
        if closeness > 100 {
            return Err(CombatHistoryRawValueError::IntimacyClosenessOutOfRange { closeness });
        }
        if affiliation != 0 && closeness != 100 {
            return Err(CombatHistoryRawValueError::IntimacyInvalidFullLabel {
                affiliation,
                closeness,
            });
        }
        match affiliation {
            0 => Ok(Self::Progress { closeness }),
            1 => Ok(Self::Close),
            2 => Ok(Self::NeverLeave),
            3 => Ok(Self::Inseparable),
            4 => Ok(Self::Soulmate),
            5 => Ok(Self::Friendly),
            _ => Err(CombatHistoryRawValueError::UnknownIntimacyAffiliation { affiliation }),
        }
    }

    pub fn closeness(self) -> u8 {
        match self {
            Self::Progress { closeness } => closeness,
            _ => 100,
        }
    }

    pub fn label(self) -> String {
        match self {
            Self::Progress { closeness } => closeness.to_string(),
            Self::Close => "亲密".to_string(),
            Self::NeverLeave => "不弃".to_string(),
            Self::Inseparable => "形影".to_string(),
            Self::Soulmate => "灵犀".to_string(),
            Self::Friendly => "友好".to_string(),
        }
    }
}

impl Default for CombatHistoryIntimacy {
    fn default() -> Self {
        Self::Progress { closeness: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistorySpiritSex {
    Male,
    Female,
}

impl CombatHistorySpiritSex {
    pub fn from_raw(raw: u8) -> Result<Self, CombatHistoryRawValueError> {
        match raw {
            1 => Ok(Self::Male),
            2 => Ok(Self::Female),
            _ => Err(CombatHistoryRawValueError::UnknownSpiritSex { raw }),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Male => "[♂]",
            Self::Female => "[♀]",
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

impl CombatHistorySpiritFieldStatus {
    pub fn name(self) -> String {
        match self {
            Self::AllAbnormalImmunity => "免疫异常".to_string(),
            Self::AbnormalImmunity { abnormal_state } => {
                format!("免疫{}", abnormal_state.name())
            }
            Self::NegativeEnhanceImmunity => "免疫负强".to_string(),
            Self::PpDoubleCost => "双损".to_string(),
            Self::ExpelImmunity => "免疫驱逐".to_string(),
        }
    }

    pub fn name_with_abnormal_state_name(self, abnormal_state_name: &str) -> String {
        match self {
            Self::AbnormalImmunity { .. } => format!("免疫{abnormal_state_name}"),
            _ => self.name(),
        }
    }
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
    pub fn from_raw_bit(raw_bit: u8) -> Result<Self, CombatHistoryRawValueError> {
        match raw_bit {
            0 => Ok(Self::PhysicalAttack),
            1 => Ok(Self::PhysicalDefense),
            2 => Ok(Self::MagicAttack),
            3 => Ok(Self::MagicDefense),
            4 => Ok(Self::Speed),
            5 => Ok(Self::Accuracy),
            6 => Ok(Self::Evasion),
            7 => Ok(Self::Critical),
            _ => Err(CombatHistoryRawValueError::UnknownLockedEnhance { raw_bit }),
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

    pub fn name(self) -> &'static str {
        match self {
            Self::PhysicalAttack => "物攻",
            Self::PhysicalDefense => "物防",
            Self::MagicAttack => "魔攻",
            Self::MagicDefense => "魔防",
            Self::Speed => "速度",
            Self::Accuracy => "命中",
            Self::Evasion => "闪避",
            Self::Critical => "暴击",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistorySpiritFieldState {
    #[default]
    Empty,
    Hidden,
    Fainted,
    Ready,
}

impl CombatHistorySpiritFieldState {
    pub fn from_raw_bits(raw_bits: u8) -> Result<Self, CombatHistoryRawValueError> {
        match raw_bits {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Hidden),
            2 => Ok(Self::Fainted),
            3 => Ok(Self::Ready),
            _ => Err(CombatHistoryRawValueError::UnknownSpiritFieldState { raw_bits }),
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
    pub fn from_raw_id(raw_id: u32) -> Result<Self, CombatHistoryRawValueError> {
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
            _ => Err(CombatHistoryRawValueError::UnknownAbnormalState { raw_id }),
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

    pub fn name(self) -> &'static str {
        match self {
            Self::Sleep => "睡眠",
            Self::Paralysis => "麻醉",
            Self::Burn => "烧伤",
            Self::Frozen => "冰冻",
            Self::Poison => "中毒",
            Self::Toxic => "剧毒",
            Self::Confusion => "混乱",
            Self::Fear => "恐惧",
            Self::LeechSeed => "寄生",
            Self::Curse => "诅咒",
            Self::Bewilder => "迷惑",
            Self::Nightmare => "梦魇",
            Self::Bind => "束缚",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySpiritEquipment {
    pub server_id: u32,
    pub item_id: u32,
    pub equipment_type: CombatHistorySpiritEquipmentType,
    pub quality: CombatHistorySpiritEquipmentQuality,
    pub base_effect: CombatHistorySpiritEquipmentEffect,
    pub special_effect: Option<CombatHistorySpiritEquipmentEffect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatHistorySpiritEquipmentEffect {
    pub attr: CombatHistorySpiritEquipmentAttr,
    pub value: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatHistorySpiritEquipmentQuality {
    Orange,
    Green,
    Blue,
    Red,
    Temporary,
}

impl CombatHistorySpiritEquipmentQuality {
    pub fn from_raw(raw: u8) -> Result<Self, CombatHistoryRawValueError> {
        match raw {
            1 => Ok(Self::Orange),
            2 => Ok(Self::Green),
            3 => Ok(Self::Blue),
            4 => Ok(Self::Red),
            5 => Ok(Self::Temporary),
            _ => Err(CombatHistoryRawValueError::UnknownEquipmentQuality { raw }),
        }
    }

    pub fn raw(self) -> u8 {
        match self {
            Self::Orange => 1,
            Self::Green => 2,
            Self::Blue => 3,
            Self::Red => 4,
            Self::Temporary => 5,
        }
    }
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
    pub fn from_raw(raw: u8) -> Result<Self, CombatHistoryRawValueError> {
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
            _ => Err(CombatHistoryRawValueError::UnknownEquipmentAttr { raw }),
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

    pub fn name(self) -> &'static str {
        match self {
            Self::PhysicalAttack => "物攻",
            Self::PhysicalDefense => "物防",
            Self::MagicAttack => "魔攻",
            Self::MagicDefense => "魔防",
            Self::Speed => "速度",
            Self::Energy => "精力",
            Self::Accuracy => "命中",
            Self::Evasion => "闪避",
            Self::Critical => "暴击",
            Self::CriticalResistance => "暴抗",
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
    pub fn from_raw(raw: u8) -> Result<Self, CombatHistoryRawValueError> {
        match raw {
            0 => Ok(Self::Weapon),
            1 => Ok(Self::Armor),
            2 => Ok(Self::Jewelry),
            _ => Err(CombatHistoryRawValueError::UnknownEquipmentType { raw }),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

impl CombatHistorySpiritPropertyStage {
    pub fn name(self) -> &'static str {
        match self {
            Self::PhysicalAttack => "物攻",
            Self::PhysicalDefense => "物防",
            Self::MagicAttack => "魔攻",
            Self::MagicDefense => "魔防",
            Self::Speed => "速度",
            Self::Accuracy => "命中",
            Self::Evasion => "闪避",
            Self::Critical => "暴击",
        }
    }
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
            (8, CombatHistoryParticipantType::NonPlayer8),
            (9, CombatHistoryParticipantType::NonPlayer9),
            (17, CombatHistoryParticipantType::NonPlayer17),
            (21, CombatHistoryParticipantType::NonPlayer21),
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
    fn field_effect_classifies_none_weather_and_environment() {
        assert_eq!(
            CombatHistoryFieldEffect::None.class(),
            CombatHistoryFieldEffectClass::None
        );
        assert_eq!(
            CombatHistoryFieldEffect::Locked { rounds_left: 2 }.class(),
            CombatHistoryFieldEffectClass::None
        );
        assert_eq!(
            CombatHistoryFieldEffect::Rain { rounds_left: 2 }.class(),
            CombatHistoryFieldEffectClass::Weather
        );
        assert_eq!(
            CombatHistoryFieldEffect::Fragrance { rounds_left: 2 }.class(),
            CombatHistoryFieldEffectClass::Environment
        );

        assert!(CombatHistoryFieldEffect::Thunderstorm { rounds_left: 2 }.is_weather());
        assert!(CombatHistoryFieldEffect::BloodMoon { rounds_left: 2 }.is_weather());
        assert!(CombatHistoryFieldEffect::Nebula { rounds_left: 2 }.is_weather());
        assert!(CombatHistoryFieldEffect::HolyLight { rounds_left: 2 }.is_weather());
        assert!(CombatHistoryFieldEffect::Meteor { rounds_left: 2 }.is_weather());
        assert!(CombatHistoryFieldEffect::Dawn { rounds_left: 2 }.is_weather());
        assert!(CombatHistoryFieldEffect::Fragrance { rounds_left: 2 }.is_environment());
        assert!(CombatHistoryFieldEffect::Locked { rounds_left: 2 }.has_no_weather_environment());
        assert!(CombatHistoryFieldEffect::Unknown25 { rounds_left: 2 }.has_no_weather_environment());
    }

    #[test]
    fn combat_domain_values_own_their_display_metadata() {
        let rain = CombatHistoryFieldEffect::Rain { rounds_left: 2 };
        assert_eq!(rain.key(), "rain");
        assert_eq!(rain.name(), "阴雨天气");
        assert!(!rain.description().is_empty());

        assert_eq!(CombatHistoryAbnormalState::Frozen.name(), "冰冻");
        assert_eq!(
            CombatHistorySpiritFieldStatus::AbnormalImmunity {
                abnormal_state: CombatHistoryAbnormalState::Frozen,
            }
            .name(),
            "免疫冰冻"
        );
        assert_eq!(CombatHistorySpiritEquipmentAttr::MagicAttack.name(), "魔攻");
        assert_eq!(CombatHistorySpiritPropertyStage::Evasion.name(), "闪避");
        assert_eq!(CombatHistoryLockedEnhance::Critical.name(), "暴击");
        assert_eq!(CombatHistorySpiritSex::Female.label(), "[♀]");
        assert_eq!(CombatHistoryIntimacy::Soulmate.label(), "灵犀");
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
