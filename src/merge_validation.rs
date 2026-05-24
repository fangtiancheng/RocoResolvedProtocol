use crate::{
    CombatHistoryObserved, CombatHistoryObservedParticipantSnapshot,
    CombatHistoryObservedParticipantState, CombatHistoryObservedSpiritSnapshot,
    CombatHistoryObservedSpiritState, CombatHistoryObservedStateSnapshot,
};

#[derive(Debug, Clone)]
pub struct MergeValidationError {
    pub kind: MergeValidationErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeValidationErrorKind {
    BattleIdMismatch,
    InitialStateMismatch,
    SnapshotMismatch,
    SpiritStateMismatch,
    SpiritSnapshotMismatch,
}

/// Validate that two observed histories can be merged
///
/// This checks that both participants observed the same battle state
pub fn validate_merge_compatibility(
    my_observed: &CombatHistoryObserved,
    rival_observed: &CombatHistoryObserved,
) -> Result<(), Vec<MergeValidationError>> {
    let mut errors = Vec::new();

    // Check battle IDs match
    if my_observed.battle_id != rival_observed.battle_id {
        errors.push(MergeValidationError {
            kind: MergeValidationErrorKind::BattleIdMismatch,
            message: format!(
                "Battle ID mismatch: {} vs {}",
                my_observed.battle_id, rival_observed.battle_id
            ),
        });
    }

    // Validate initial states
    validate_initial_states(
        &my_observed.initial_state,
        &rival_observed.initial_state,
        &mut errors,
    );

    // Validate snapshots
    validate_snapshots(&my_observed.frames, &rival_observed.frames, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_initial_states(
    my_state: &crate::CombatHistoryObservedInitialState,
    rival_state: &crate::CombatHistoryObservedInitialState,
    errors: &mut Vec<MergeValidationError>,
) {
    // my_state.rival_side should match rival_state.my_side (cross-validation)
    validate_participant_state_cross(
        &my_state.rival_side,
        &rival_state.my_side,
        "initial_state.rival_side",
        errors,
    );

    // rival_state.rival_side should match my_state.my_side (cross-validation)
    validate_participant_state_cross(
        &rival_state.rival_side,
        &my_state.my_side,
        "initial_state.my_side",
        errors,
    );
}

fn validate_participant_state_cross(
    observed_incomplete: &CombatHistoryObservedParticipantState,
    observed_complete: &CombatHistoryObservedParticipantState,
    context: &str,
    errors: &mut Vec<MergeValidationError>,
) {
    // Validate participant identity
    if observed_incomplete.participant.uin != observed_complete.participant.uin {
        errors.push(MergeValidationError {
            kind: MergeValidationErrorKind::InitialStateMismatch,
            message: format!(
                "{}: UIN mismatch {} vs {}",
                context, observed_incomplete.participant.uin, observed_complete.participant.uin
            ),
        });
    }

    // Validate spirits (cross-check visible data)
    for i in 0..6 {
        if let (Some(incomplete_spirit), Some(complete_spirit)) = (
            &observed_incomplete.spirits[i],
            &observed_complete.spirits[i],
        ) {
            validate_spirit_state_cross(incomplete_spirit, complete_spirit, context, i, errors);
        }
    }
}

fn validate_spirit_state_cross(
    observed_incomplete: &CombatHistoryObservedSpiritState,
    observed_complete: &CombatHistoryObservedSpiritState,
    context: &str,
    position: usize,
    errors: &mut Vec<MergeValidationError>,
) {
    let spirit_context = format!("{}.spirits[{}]", context, position);

    // Validate basic fields
    if observed_incomplete.spirit_id != observed_complete.spirit_id {
        errors.push(MergeValidationError {
            kind: MergeValidationErrorKind::SpiritStateMismatch,
            message: format!(
                "{}: spirit_id mismatch {} vs {}",
                spirit_context, observed_incomplete.spirit_id, observed_complete.spirit_id
            ),
        });
    }

    if observed_incomplete.level != observed_complete.level {
        errors.push(MergeValidationError {
            kind: MergeValidationErrorKind::SpiritStateMismatch,
            message: format!(
                "{}: level mismatch {} vs {}",
                spirit_context, observed_incomplete.level, observed_complete.level
            ),
        });
    }

    // Validate HP (critical for battle state consistency)
    if observed_incomplete.current_hp != observed_complete.current_hp {
        errors.push(MergeValidationError {
            kind: MergeValidationErrorKind::SpiritStateMismatch,
            message: format!(
                "{}: current_hp mismatch {} vs {}",
                spirit_context, observed_incomplete.current_hp, observed_complete.current_hp
            ),
        });
    }

    if observed_incomplete.max_hp != observed_complete.max_hp {
        errors.push(MergeValidationError {
            kind: MergeValidationErrorKind::SpiritStateMismatch,
            message: format!(
                "{}: max_hp mismatch {} vs {}",
                spirit_context, observed_incomplete.max_hp, observed_complete.max_hp
            ),
        });
    }

    // Validate panel stats if both sides have them
    if let (Some(incomplete_stats), Some(complete_stats)) = (
        &observed_incomplete.panel_stats,
        &observed_complete.panel_stats,
    ) {
        if incomplete_stats.hp != complete_stats.hp {
            errors.push(MergeValidationError {
                kind: MergeValidationErrorKind::SpiritStateMismatch,
                message: format!(
                    "{}: panel_stats.hp mismatch {} vs {}",
                    spirit_context, incomplete_stats.hp, complete_stats.hp
                ),
            });
        }

        if incomplete_stats.pa != complete_stats.pa {
            errors.push(MergeValidationError {
                kind: MergeValidationErrorKind::SpiritStateMismatch,
                message: format!(
                    "{}: panel_stats.pa mismatch {} vs {}",
                    spirit_context, incomplete_stats.pa, complete_stats.pa
                ),
            });
        }

        if incomplete_stats.pd != complete_stats.pd {
            errors.push(MergeValidationError {
                kind: MergeValidationErrorKind::SpiritStateMismatch,
                message: format!(
                    "{}: panel_stats.pd mismatch {} vs {}",
                    spirit_context, incomplete_stats.pd, complete_stats.pd
                ),
            });
        }

        if incomplete_stats.ma != complete_stats.ma {
            errors.push(MergeValidationError {
                kind: MergeValidationErrorKind::SpiritStateMismatch,
                message: format!(
                    "{}: panel_stats.ma mismatch {} vs {}",
                    spirit_context, incomplete_stats.ma, complete_stats.ma
                ),
            });
        }

        if incomplete_stats.md != complete_stats.md {
            errors.push(MergeValidationError {
                kind: MergeValidationErrorKind::SpiritStateMismatch,
                message: format!(
                    "{}: panel_stats.md mismatch {} vs {}",
                    spirit_context, incomplete_stats.md, complete_stats.md
                ),
            });
        }

        if incomplete_stats.sp != complete_stats.sp {
            errors.push(MergeValidationError {
                kind: MergeValidationErrorKind::SpiritStateMismatch,
                message: format!(
                    "{}: panel_stats.sp mismatch {} vs {}",
                    spirit_context, incomplete_stats.sp, complete_stats.sp
                ),
            });
        }
    }
}

fn validate_snapshots(
    my_frames: &[crate::CombatHistoryObservedFrame],
    rival_frames: &[crate::CombatHistoryObservedFrame],
    errors: &mut Vec<MergeValidationError>,
) {
    use std::collections::HashMap;

    // Extract snapshots
    let my_snapshots: HashMap<u32, &CombatHistoryObservedStateSnapshot> = my_frames
        .iter()
        .filter_map(|f| f.state_snapshot.as_ref().map(|s| (s.round, s)))
        .collect();

    let rival_snapshots: HashMap<u32, &CombatHistoryObservedStateSnapshot> = rival_frames
        .iter()
        .filter_map(|f| f.state_snapshot.as_ref().map(|s| (s.round, s)))
        .collect();

    // Validate common rounds
    for (&round, my_snapshot) in &my_snapshots {
        if let Some(rival_snapshot) = rival_snapshots.get(&round) {
            validate_snapshot_pair(my_snapshot, rival_snapshot, round, errors);
        }
    }
}

fn validate_snapshot_pair(
    my_snapshot: &CombatHistoryObservedStateSnapshot,
    rival_snapshot: &CombatHistoryObservedStateSnapshot,
    round: u32,
    errors: &mut Vec<MergeValidationError>,
) {
    let context = format!("snapshot[round={}]", round);

    // Cross-validate: my_snapshot.rival_side vs rival_snapshot.my_side
    validate_participant_snapshot_cross(
        &my_snapshot.rival_side,
        &rival_snapshot.my_side,
        &format!("{}.rival_side", context),
        errors,
    );

    // Cross-validate: rival_snapshot.rival_side vs my_snapshot.my_side
    validate_participant_snapshot_cross(
        &rival_snapshot.rival_side,
        &my_snapshot.my_side,
        &format!("{}.my_side", context),
        errors,
    );
}

fn validate_participant_snapshot_cross(
    observed_incomplete: &CombatHistoryObservedParticipantSnapshot,
    observed_complete: &CombatHistoryObservedParticipantSnapshot,
    context: &str,
    errors: &mut Vec<MergeValidationError>,
) {
    // Validate spirits
    for i in 0..6 {
        if let (Some(incomplete_spirit), Some(complete_spirit)) = (
            &observed_incomplete.spirits[i],
            &observed_complete.spirits[i],
        ) {
            validate_spirit_snapshot_cross(incomplete_spirit, complete_spirit, context, i, errors);
        }
    }
}

fn validate_spirit_snapshot_cross(
    observed_incomplete: &CombatHistoryObservedSpiritSnapshot,
    observed_complete: &CombatHistoryObservedSpiritSnapshot,
    context: &str,
    position: usize,
    errors: &mut Vec<MergeValidationError>,
) {
    let spirit_context = format!("{}.spirits[{}]", context, position);

    // Validate HP
    if observed_incomplete.current_hp != observed_complete.current_hp {
        errors.push(MergeValidationError {
            kind: MergeValidationErrorKind::SpiritSnapshotMismatch,
            message: format!(
                "{}: current_hp mismatch {} vs {}",
                spirit_context, observed_incomplete.current_hp, observed_complete.current_hp
            ),
        });
    }

    if observed_incomplete.max_hp != observed_complete.max_hp {
        errors.push(MergeValidationError {
            kind: MergeValidationErrorKind::SpiritSnapshotMismatch,
            message: format!(
                "{}: max_hp mismatch {} vs {}",
                spirit_context, observed_incomplete.max_hp, observed_complete.max_hp
            ),
        });
    }

    // Validate abnormal states
    if observed_incomplete.abnormal_states != observed_complete.abnormal_states {
        errors.push(MergeValidationError {
            kind: MergeValidationErrorKind::SpiritSnapshotMismatch,
            message: format!(
                "{}: abnormal_states mismatch {:?} vs {:?}",
                spirit_context,
                observed_incomplete.abnormal_states,
                observed_complete.abnormal_states
            ),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CombatHistoryObserved, CombatHistoryObservedInitialState,
        CombatHistoryObservedParticipantState, CombatHistoryParticipantIdentity,
        CombatHistoryPerspective, CombatHistorySideHint,
    };

    fn empty_observed(battle_id: &str) -> CombatHistoryObserved {
        CombatHistoryObserved {
            schema_version: 1,
            battle_id: battle_id.to_string(),
            uploader_uin: 1,
            perspective: CombatHistoryPerspective::MySide,
            battle_started_at_unix_ms: 0,
            initial_state: CombatHistoryObservedInitialState {
                combat_type: 1,
                my_side: participant(CombatHistorySideHint::My, 1),
                rival_side: participant(CombatHistorySideHint::Rival, 2),
            },
            frames: Vec::new(),
            finish_reason_code: None,
        }
    }

    fn participant(
        side_hint: CombatHistorySideHint,
        uin: u32,
    ) -> CombatHistoryObservedParticipantState {
        CombatHistoryObservedParticipantState {
            participant: CombatHistoryParticipantIdentity {
                side_hint,
                uin,
                participant_type: crate::CombatHistoryParticipantType::Player,
                nickname: format!("role-{uin}"),
            },
            guardian_pet: None,
            active_spirit_index: 1,
            spirits: Default::default(),
        }
    }

    #[test]
    fn rejects_battle_id_mismatch() {
        let errors = validate_merge_compatibility(&empty_observed("a"), &empty_observed("b"))
            .expect_err("battle id mismatch should be rejected");

        assert_eq!(errors[0].kind, MergeValidationErrorKind::BattleIdMismatch);
    }
}
