#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatHistoryRawValueError {
    UnknownParticipantType { raw: u8 },
    UnknownFinishReason { raw: u8 },
    UnknownRestrainHint { raw: i8 },
    UnknownAbnormalState { raw_id: u32 },
    UnknownLockedEnhance { raw_bit: u8 },
    UnknownImmunityPropertyEffectOffset { offset: u32 },
    UnknownEquipmentQuality { raw: u8 },
    UnknownEquipmentAttr { raw: u8 },
    UnknownEquipmentType { raw: u8 },
    IntimacyClosenessOutOfRange { closeness: u8 },
    IntimacyInvalidFullLabel { affiliation: u8, closeness: u8 },
    UnknownIntimacyAffiliation { affiliation: u8 },
    UnknownSpiritSex { raw: u8 },
    UnknownSpiritFieldState { raw_bits: u8 },
}

impl std::fmt::Display for CombatHistoryRawValueError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownParticipantType { raw } => {
                write!(formatter, "unknown combat participant type: {raw}")
            }
            Self::UnknownFinishReason { raw } => {
                write!(formatter, "unknown combat finish reason: {raw}")
            }
            Self::UnknownRestrainHint { raw } => {
                write!(formatter, "unknown combat restrain hint: {raw}")
            }
            Self::UnknownAbnormalState { raw_id } => {
                write!(formatter, "unknown combat abnormal state id: {raw_id}")
            }
            Self::UnknownLockedEnhance { raw_bit } => {
                write!(formatter, "unknown combat locked enhance bit: {raw_bit}")
            }
            Self::UnknownImmunityPropertyEffectOffset { offset } => write!(
                formatter,
                "unknown combat immunity property effect offset: {offset}"
            ),
            Self::UnknownEquipmentQuality { raw } => {
                write!(formatter, "unknown combat spirit equipment quality: {raw}")
            }
            Self::UnknownEquipmentAttr { raw } => {
                write!(formatter, "unknown combat spirit equipment attr: {raw}")
            }
            Self::UnknownEquipmentType { raw } => {
                write!(formatter, "unknown combat spirit equipment type: {raw}")
            }
            Self::IntimacyClosenessOutOfRange { closeness } => {
                write!(formatter, "combat intimacy closeness out of range: {closeness}")
            }
            Self::IntimacyInvalidFullLabel {
                affiliation,
                closeness,
            } => write!(
                formatter,
                "combat intimacy invalid full label: affiliation={affiliation}, closeness={closeness}"
            ),
            Self::UnknownIntimacyAffiliation { affiliation } => {
                write!(formatter, "unknown combat intimacy affiliation: {affiliation}")
            }
            Self::UnknownSpiritSex { raw } => {
                write!(formatter, "unknown combat spirit sex: {raw}")
            }
            Self::UnknownSpiritFieldState { raw_bits } => write!(
                formatter,
                "unknown combat spirit field state bits: {raw_bits}"
            ),
        }
    }
}

impl std::error::Error for CombatHistoryRawValueError {}
