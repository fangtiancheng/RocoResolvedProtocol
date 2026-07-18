use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CombatPresentationTrigger {
    BattleOpening,
    RoundResolution { round: u32 },
    ExtraSwitch { side: CombatPresentationSide },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatPresentation {
    pub serial: u32,
    pub trigger: CombatPresentationTrigger,
    pub preloads: CombatPresentationPreloads,
    pub batches: Vec<CombatPresentationBatch>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatPresentationPreloads {
    pub my_spirit_ids: Vec<u32>,
    pub rival_spirit_ids: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatPresentationBatch {
    pub cues: Vec<CombatPresentationCue>,
    pub barrier: CombatPresentationBarrier,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatPresentationCue {
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after: Option<CombatPresentationEventRef>,
    pub operation: CombatPresentationOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatPresentationEventRef {
    pub cue_id: u32,
    pub event: CombatPresentationEvent,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatPresentationBarrier {
    pub waits: Vec<CombatPresentationEventRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatPresentationSide {
    My,
    Rival,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatPresentationPetActor {
    pub side: CombatPresentationSide,
    pub position: u8,
    pub spirit_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatPetMotion {
    Blank,
    Idle,
    Appear,
    SwitchIn,
    SwitchOut,
    Transform,
    AntiTransform,
    Attack,
    MagicStart,
    MagicFocus,
    MagicEnd,
    UnderAttack,
    BeatDown,
    Miss,
    Dead,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatPetMotionPlayback {
    pub label: String,
    pub signal_events: Vec<CombatPetMotionEvent>,
    pub complete_event: Option<CombatPetMotionEvent>,
    pub loop_motion: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CombatPetMotionEvent {
    AppearEnd,
    StbEnd,
    BtsEnd,
    TransformEnd,
    AntiTransformEnd,
    AttackHit,
    AttackEnd,
    MagicStart,
    MagicEnd,
    UnderAttackEnd,
    BeatDownEnd,
    MissEnd,
    DeadEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatEffectEvent {
    #[serde(rename = "EATTACK_HIT")]
    AttackHit,
    #[serde(rename = "EATTACK_END")]
    AttackEnd,
    #[serde(rename = "ESCAES_END")]
    MagicEscapeEnd,
    #[serde(rename = "EMAGIC_HIT")]
    MagicHit,
    #[serde(rename = "EMAGIC_FOCUS_END")]
    MagicFocusEnd,
    #[serde(rename = "EMAGIC_END")]
    MagicEnd,
    #[serde(rename = "EFFECT_HIT")]
    EffectHit,
    #[serde(rename = "EFFECT_END")]
    EffectEnd,
    #[serde(rename = "EBALL_LIGHT")]
    BallLight,
    #[serde(rename = "EBALL_END")]
    BallEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatPresentationEvent {
    #[serde(rename = "APPEAR_END")]
    PetAppearEnd,
    #[serde(rename = "STB_END")]
    PetStbEnd,
    #[serde(rename = "BTS_END")]
    PetBtsEnd,
    #[serde(rename = "TRANSFORM_END")]
    PetTransformEnd,
    #[serde(rename = "ANTITRANSFORM_END")]
    PetAntiTransformEnd,
    #[serde(rename = "ATTACK_HIT")]
    PetAttackHit,
    #[serde(rename = "ATTACK_END")]
    PetAttackEnd,
    #[serde(rename = "MAGIC_START")]
    PetMagicStart,
    #[serde(rename = "MAGIC_END")]
    PetMagicEnd,
    #[serde(rename = "UNDER_ATTACK_END")]
    PetUnderAttackEnd,
    #[serde(rename = "BEAT_DOWN_END")]
    PetBeatDownEnd,
    #[serde(rename = "MISS_END")]
    PetMissEnd,
    #[serde(rename = "DEAD_END")]
    PetDeadEnd,
    #[serde(rename = "EATTACK_HIT")]
    EffectAttackHit,
    #[serde(rename = "EATTACK_END")]
    EffectAttackEnd,
    #[serde(rename = "ESCAES_END")]
    EffectMagicEscapeEnd,
    #[serde(rename = "EMAGIC_HIT")]
    EffectMagicHit,
    #[serde(rename = "EMAGIC_FOCUS_END")]
    EffectMagicFocusEnd,
    #[serde(rename = "EMAGIC_END")]
    EffectMagicEnd,
    #[serde(rename = "EFFECT_HIT")]
    EffectHit,
    #[serde(rename = "EFFECT_END")]
    EffectEnd,
    #[serde(rename = "EBALL_LIGHT")]
    EffectBallLight,
    #[serde(rename = "EBALL_END")]
    EffectBallEnd,
}

impl From<CombatPetMotionEvent> for CombatPresentationEvent {
    fn from(event: CombatPetMotionEvent) -> Self {
        match event {
            CombatPetMotionEvent::AppearEnd => Self::PetAppearEnd,
            CombatPetMotionEvent::StbEnd => Self::PetStbEnd,
            CombatPetMotionEvent::BtsEnd => Self::PetBtsEnd,
            CombatPetMotionEvent::TransformEnd => Self::PetTransformEnd,
            CombatPetMotionEvent::AntiTransformEnd => Self::PetAntiTransformEnd,
            CombatPetMotionEvent::AttackHit => Self::PetAttackHit,
            CombatPetMotionEvent::AttackEnd => Self::PetAttackEnd,
            CombatPetMotionEvent::MagicStart => Self::PetMagicStart,
            CombatPetMotionEvent::MagicEnd => Self::PetMagicEnd,
            CombatPetMotionEvent::UnderAttackEnd => Self::PetUnderAttackEnd,
            CombatPetMotionEvent::BeatDownEnd => Self::PetBeatDownEnd,
            CombatPetMotionEvent::MissEnd => Self::PetMissEnd,
            CombatPetMotionEvent::DeadEnd => Self::PetDeadEnd,
        }
    }
}

impl From<CombatEffectEvent> for CombatPresentationEvent {
    fn from(event: CombatEffectEvent) -> Self {
        match event {
            CombatEffectEvent::AttackHit => Self::EffectAttackHit,
            CombatEffectEvent::AttackEnd => Self::EffectAttackEnd,
            CombatEffectEvent::MagicEscapeEnd => Self::EffectMagicEscapeEnd,
            CombatEffectEvent::MagicHit => Self::EffectMagicHit,
            CombatEffectEvent::MagicFocusEnd => Self::EffectMagicFocusEnd,
            CombatEffectEvent::MagicEnd => Self::EffectMagicEnd,
            CombatEffectEvent::EffectHit => Self::EffectHit,
            CombatEffectEvent::EffectEnd => Self::EffectEnd,
            CombatEffectEvent::BallLight => Self::EffectBallLight,
            CombatEffectEvent::BallEnd => Self::EffectBallEnd,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatEffectPlayback {
    pub label: Option<String>,
    pub signal_events: Vec<CombatEffectEvent>,
    pub complete_event: CombatEffectEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CombatFloatingText {
    Miss,
    Immune,
    Command { text: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "snake_case",
    rename_all_fields = "camelCase"
)]
pub enum CombatPresentationOperation {
    PetMotion {
        actor: CombatPresentationPetActor,
        #[serde(skip_serializing_if = "Option::is_none")]
        asset_spirit_id: Option<u32>,
        motion: CombatPetMotion,
        playback: CombatPetMotionPlayback,
    },
    SkillEffect {
        skill_id: u32,
        from: CombatPresentationPetActor,
        to: CombatPresentationPetActor,
        playback: CombatEffectPlayback,
    },
    FloatingText {
        actor: CombatPresentationPetActor,
        content: CombatFloatingText,
    },
    PublicEffect {
        actor: CombatPresentationPetActor,
        effect_id: u32,
        playback: CombatEffectPlayback,
    },
    HpChange {
        actor: CombatPresentationPetActor,
        to: u16,
        delta: i32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cue_events_preserve_pet_and_effect_signal_names() {
        assert_eq!(
            serde_json::to_value(CombatPresentationEvent::from(
                CombatPetMotionEvent::AttackHit
            ))
            .unwrap(),
            serde_json::json!("ATTACK_HIT")
        );
        assert_eq!(
            serde_json::to_value(CombatPresentationEvent::from(CombatEffectEvent::AttackHit))
                .unwrap(),
            serde_json::json!("EATTACK_HIT")
        );
    }

    #[test]
    fn presentation_contract_fixture_round_trips_without_schema_loss() {
        let source = include_str!("../tests/fixtures/combat_presentation.json");
        let expected: serde_json::Value = serde_json::from_str(source).unwrap();
        let presentation: CombatPresentation = serde_json::from_value(expected.clone()).unwrap();
        assert_eq!(serde_json::to_value(presentation).unwrap(), expected);
    }
}
