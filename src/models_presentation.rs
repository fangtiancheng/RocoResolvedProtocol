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
    pub batches: Vec<CombatPresentationBatch>,
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
    pub start_after: Option<CombatPresentationCueStart>,
    pub operation: CombatPresentationOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatPresentationCueStart {
    pub cue_id: u32,
    pub event: CombatEffectEvent,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatPresentationBarrier {
    pub event_cues: Vec<u32>,
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
    pub complete_events: Vec<CombatPetMotionEvent>,
    pub advance_events: Vec<CombatPetMotionEvent>,
    pub loop_motion: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CombatPetMotionEvent {
    AppearEnd,
    StbEnd,
    BtsEnd,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombatEffectPlayback {
    pub label: Option<String>,
    pub signal_events: Vec<CombatEffectEvent>,
    pub complete_events: Vec<CombatEffectEvent>,
    pub advance_events: Vec<CombatEffectEvent>,
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

impl CombatPresentationOperation {
    pub fn waits_for_event(&self) -> bool {
        match self {
            Self::PetMotion { playback, .. } => !playback.advance_events.is_empty(),
            Self::SkillEffect { playback, .. } | Self::PublicEffect { playback, .. } => {
                !playback.advance_events.is_empty()
            }
            Self::FloatingText { .. } => true,
            Self::HpChange { delta, .. } => *delta != 0,
        }
    }
}
