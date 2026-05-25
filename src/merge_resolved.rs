use crate::{
    CombatHistoryFieldEffect, CombatHistoryObserved, CombatHistoryObservedFrame,
    CombatHistoryObservedInitialState, CombatHistoryObservedParticipantSnapshot,
    CombatHistoryObservedParticipantState, CombatHistoryObservedSpiritSnapshot,
    CombatHistoryObservedSpiritState, CombatHistoryObservedStateSnapshot,
    CombatHistoryParticipantDisplayState, CombatHistoryResolved, CombatHistoryResolvedInitialState,
    CombatHistoryResolvedParticipantSnapshot, CombatHistoryResolvedParticipantState,
    CombatHistoryResolvedSnapshot, CombatHistoryResolvedSpiritSnapshot,
    CombatHistoryResolvedSpiritState, CombatHistorySkillState,
};

/// Merge two observed histories (from both participants) into a resolved history
///
/// # Arguments
/// * `my_observed` - History from participant A's perspective
/// * `rival_observed` - History from participant B's perspective
///
/// # Returns
/// A resolved history with complete information from both sides
pub fn merge_observed_to_resolved(
    my_observed: &CombatHistoryObserved,
    rival_observed: &CombatHistoryObserved,
) -> CombatHistoryResolved {
    // Merge initial states
    let initial_state =
        merge_initial_states(&my_observed.initial_state, &rival_observed.initial_state);

    // Extract snapshots from frames and merge
    let my_snapshots = extract_snapshots(&my_observed.frames);
    let rival_snapshots = extract_snapshots(&rival_observed.frames);
    let snapshots = merge_snapshots(&my_snapshots, &rival_snapshots);

    CombatHistoryResolved {
        initial_state,
        snapshots,
    }
}

fn extract_snapshots(
    frames: &[CombatHistoryObservedFrame],
) -> Vec<&CombatHistoryObservedStateSnapshot> {
    frames
        .iter()
        .filter_map(|frame| frame.state_snapshot.as_ref())
        .collect()
}

fn merge_initial_states(
    my_state: &CombatHistoryObservedInitialState,
    rival_state: &CombatHistoryObservedInitialState,
) -> CombatHistoryResolvedInitialState {
    // my_state.my_side is complete, my_state.rival_side is incomplete
    // rival_state.rival_side is complete, rival_state.my_side is incomplete
    // So we use my_state.my_side + rival_state.my_side (which is actually my rival)

    let my_side = merge_participant_state(&my_state.my_side, &rival_state.rival_side);
    let rival_side = merge_participant_state(&my_state.rival_side, &rival_state.my_side);

    CombatHistoryResolvedInitialState {
        my_side,
        rival_side,
    }
}

fn merge_participant_state(
    observed_complete: &CombatHistoryObservedParticipantState,
    _observed_incomplete: &CombatHistoryObservedParticipantState,
) -> CombatHistoryResolvedParticipantState {
    let mut spirits: [Option<CombatHistoryResolvedSpiritState>; 6] = Default::default();

    for i in 0..6 {
        if let Some(complete_spirit) = &observed_complete.spirits[i] {
            spirits[i] = Some(merge_spirit_state(complete_spirit));
        }
    }

    CombatHistoryResolvedParticipantState {
        participant: observed_complete.participant.clone(),
        guardian_pet: observed_complete.guardian_pet,
        active_spirit_index: observed_complete.active_spirit_index,
        spirits,
    }
}

fn merge_spirit_state(
    complete: &CombatHistoryObservedSpiritState,
) -> CombatHistoryResolvedSpiritState {
    // Complete side has all data including PP and panel_stats
    let mut skills: [Option<CombatHistorySkillState>; 4] = Default::default();

    for i in 0..4 {
        if let Some(skill) = &complete.skills[i] {
            skills[i] = Some(CombatHistorySkillState {
                skill_id: skill.skill_id,
                pp_left: skill.pp_left,
                pp_max: skill.pp_max,
                inherited: skill.inherited,
            });
        }
    }

    CombatHistoryResolvedSpiritState {
        spirit_id: complete.spirit_id,
        level: complete.level,
        sex: complete.sex,
        current_hp: complete.current_hp,
        max_hp: complete.max_hp,
        skin_id: complete.skin_id,
        talent_type: complete.talent_type,
        talent_level: complete.talent_level,
        intimacy: complete.intimacy,
        skills,
        equipments: complete.equipments.clone(),
        panel_stats: complete.panel_stats.clone(),
        property_stages: complete.property_stages.clone(),
        field_state: complete.field_state,
    }
}

fn merge_snapshots(
    my_snapshots: &[&CombatHistoryObservedStateSnapshot],
    rival_snapshots: &[&CombatHistoryObservedStateSnapshot],
) -> Vec<CombatHistoryResolvedSnapshot> {
    use std::collections::HashMap;

    // Index snapshots by round
    let mut my_map: HashMap<u32, &CombatHistoryObservedStateSnapshot> = HashMap::new();
    let mut rival_map: HashMap<u32, &CombatHistoryObservedStateSnapshot> = HashMap::new();

    for snapshot in my_snapshots {
        my_map.insert(snapshot.round, snapshot);
    }

    for snapshot in rival_snapshots {
        rival_map.insert(snapshot.round, snapshot);
    }

    // Collect all rounds
    let mut rounds: Vec<u32> = my_map.keys().chain(rival_map.keys()).copied().collect();
    rounds.sort_unstable();
    rounds.dedup();

    // Merge snapshots for each round
    rounds
        .into_iter()
        .filter_map(|round| {
            let my_snapshot = my_map.get(&round);
            let rival_snapshot = rival_map.get(&round);

            match (my_snapshot, rival_snapshot) {
                (Some(my), Some(rival)) => Some(merge_snapshot_pair(round, my, rival)),
                _ => None, // Skip rounds where we don't have both snapshots
            }
        })
        .collect()
}

fn merge_snapshot_pair(
    round: u32,
    my_snapshot: &CombatHistoryObservedStateSnapshot,
    rival_snapshot: &CombatHistoryObservedStateSnapshot,
) -> CombatHistoryResolvedSnapshot {
    let my_side = merge_participant_snapshot(
        &my_snapshot.my_side,
        &rival_snapshot.rival_side,
        my_snapshot.my_side.display_state.clone(),
    );
    let rival_side = merge_participant_snapshot(
        &my_snapshot.rival_side,
        &rival_snapshot.my_side,
        my_snapshot.rival_side.display_state.clone(),
    );

    // Use weather from either side (should be the same)
    let weather = match my_snapshot.weather {
        CombatHistoryFieldEffect::None => rival_snapshot.weather,
        value => value,
    };

    CombatHistoryResolvedSnapshot {
        round,
        my_side,
        rival_side,
        weather,
    }
}

fn merge_participant_snapshot(
    observed_complete: &CombatHistoryObservedParticipantSnapshot,
    _observed_incomplete: &CombatHistoryObservedParticipantSnapshot,
    display_state: Option<CombatHistoryParticipantDisplayState>,
) -> CombatHistoryResolvedParticipantSnapshot {
    let mut spirits: [Option<CombatHistoryResolvedSpiritSnapshot>; 6] = Default::default();

    for i in 0..6 {
        if let Some(complete_spirit) = &observed_complete.spirits[i] {
            spirits[i] = Some(merge_spirit_snapshot(complete_spirit));
        }
    }

    CombatHistoryResolvedParticipantSnapshot {
        participant: observed_complete.participant.clone(),
        guardian_pet: observed_complete.guardian_pet,
        display_state,
        active_spirit_index: observed_complete.active_spirit_index,
        spirits,
    }
}

fn merge_spirit_snapshot(
    complete: &CombatHistoryObservedSpiritSnapshot,
) -> CombatHistoryResolvedSpiritSnapshot {
    let mut skills: [Option<CombatHistorySkillState>; 4] = Default::default();

    for i in 0..4 {
        if let Some(skill) = &complete.skills[i] {
            skills[i] = Some(CombatHistorySkillState {
                skill_id: skill.skill_id,
                pp_left: skill.pp_left,
                pp_max: skill.pp_max,
                inherited: skill.inherited,
            });
        }
    }

    CombatHistoryResolvedSpiritSnapshot {
        spirit_id: complete.spirit_id,
        level: complete.level,
        sex: complete.sex,
        current_hp: complete.current_hp,
        max_hp: complete.max_hp,
        intimacy: complete.intimacy,
        talent_type: complete.talent_type,
        talent_level: complete.talent_level,
        skin_id: complete.skin_id,
        skills,
        equipments: complete.equipments.clone(),
        property_stages: complete.property_stages.clone(),
        field_state: complete.field_state,
        abnormal_states: complete.abnormal_states.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CombatHistoryFrameSource, CombatHistoryFrameSourceKind, CombatHistoryObservedFrame,
        CombatHistoryObservedFrameEvent, CombatHistoryObservedInitialState,
        CombatHistoryObservedParticipantSnapshot, CombatHistoryObservedParticipantState,
        CombatHistoryObservedStateSnapshot, CombatHistoryParticipantIdentity,
        CombatHistoryPerspective, CombatHistorySideHint,
    };

    fn observed_with_snapshot(battle_id: &str, round: u32) -> CombatHistoryObserved {
        let my_side = participant(CombatHistorySideHint::My, 1);
        let rival_side = participant(CombatHistorySideHint::Rival, 2);
        CombatHistoryObserved {
            schema_version: 1,
            battle_id: battle_id.to_string(),
            uploader_uin: 1,
            perspective: CombatHistoryPerspective::MySide,
            battle_started_at_unix_ms: 0,
            initial_state: CombatHistoryObservedInitialState {
                combat_type: 1,
                my_side: my_side.clone(),
                rival_side: rival_side.clone(),
            },
            frames: vec![CombatHistoryObservedFrame {
                seq: 1,
                round: Some(round),
                source: CombatHistoryFrameSource {
                    cmd_id: None,
                    ui_serial_num: None,
                    source_kind: CombatHistoryFrameSourceKind::LocalSynthetic,
                    packet_summary: None,
                },
                event: CombatHistoryObservedFrameEvent::Start,
                state_snapshot: Some(CombatHistoryObservedStateSnapshot {
                    round,
                    action_availability: None,
                    my_side: snapshot_participant(&my_side),
                    rival_side: snapshot_participant(&rival_side),
                    weather: CombatHistoryFieldEffect::None,
                }),
            }],
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

    fn snapshot_participant(
        participant: &CombatHistoryObservedParticipantState,
    ) -> CombatHistoryObservedParticipantSnapshot {
        CombatHistoryObservedParticipantSnapshot {
            participant: participant.participant.clone(),
            guardian_pet: None,
            display_state: None,
            active_spirit_index: participant.active_spirit_index,
            spirits: Default::default(),
        }
    }

    #[test]
    fn merges_common_round_snapshots() {
        let merged = merge_observed_to_resolved(
            &observed_with_snapshot("battle", 3),
            &observed_with_snapshot("battle", 3),
        );

        assert_eq!(merged.snapshots.len(), 1);
        assert_eq!(merged.snapshots[0].round, 3);
    }
}
