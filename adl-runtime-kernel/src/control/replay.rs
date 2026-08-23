use std::collections::{BTreeMap, BTreeSet};

const ACIP_MAX_SEQUENCE_ADVANCE: u64 = 1_000_000;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct AcipReplayDomain {
    pub(super) runtime_id: String,
    pub(super) source: String,
}

#[derive(Default)]
pub(super) struct AcipReplayDomainState {
    committed_sequence: u64,
    pending_sequences: BTreeSet<u64>,
}

pub(super) struct AcipReplayState {
    pub(super) sequences_by_principal:
        BTreeMap<[u8; 32], BTreeMap<AcipReplayDomain, AcipReplayDomainState>>,
}

pub(super) struct AcipSequenceReservation {
    pub(super) principal: [u8; 32],
    domain: AcipReplayDomain,
    sequence: u64,
}

pub(super) fn reserve_replay_sequence(
    state: &mut AcipReplayState,
    max_records: usize,
    principal: [u8; 32],
    domain: AcipReplayDomain,
    sequence: u64,
) -> Option<AcipSequenceReservation> {
    let domains = state.sequences_by_principal.entry(principal).or_default();
    let high_water = domains
        .get(&domain)
        .map(|domain_state| {
            domain_state
                .pending_sequences
                .last()
                .copied()
                .unwrap_or(domain_state.committed_sequence)
                .max(domain_state.committed_sequence)
        })
        .unwrap_or(0);
    if sequence <= high_water || sequence - high_water > ACIP_MAX_SEQUENCE_ADVANCE {
        return None;
    }
    if !domains.contains_key(&domain) && domains.len() >= max_records {
        return None;
    }
    let domain_state = domains.entry(domain.clone()).or_default();
    domain_state.pending_sequences.insert(sequence);
    Some(AcipSequenceReservation {
        principal,
        domain,
        sequence,
    })
}

pub(super) fn commit_replay_sequence(
    state: &mut AcipReplayState,
    reservation: &AcipSequenceReservation,
) {
    if let Some(domain) = state
        .sequences_by_principal
        .get_mut(&reservation.principal)
        .and_then(|domains| domains.get_mut(&reservation.domain))
    {
        domain.pending_sequences.remove(&reservation.sequence);
        domain.committed_sequence = domain.committed_sequence.max(reservation.sequence);
    }
}

pub(super) fn rollback_replay_sequence(
    state: &mut AcipReplayState,
    reservation: AcipSequenceReservation,
) {
    let mut remove_principal = false;
    if let Some(domains) = state.sequences_by_principal.get_mut(&reservation.principal) {
        let remove_domain = domains.get_mut(&reservation.domain).is_some_and(|domain| {
            domain.pending_sequences.remove(&reservation.sequence);
            domain.committed_sequence == 0 && domain.pending_sequences.is_empty()
        });
        if remove_domain {
            domains.remove(&reservation.domain);
        }
        remove_principal = domains.is_empty();
    }
    if remove_principal {
        state.sequences_by_principal.remove(&reservation.principal);
    }
}
