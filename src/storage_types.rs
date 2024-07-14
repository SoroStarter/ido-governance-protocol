use soroban_sdk::{contracttype, Address, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub struct Proposal {
    pub proposal_id: u32,
    pub creator: Address,
    pub title: String,
    pub description: String,
    pub vote_start_at: u64,
    pub vote_end_at: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct QuorumRequirements {
    pub min_total_votes: u64,
    pub percent_yes: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct Votes {
    pub yes_votes: u64,
    pub total_votes: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct VotesWeight {
    pub staker_weight: u32,
    pub holder_weight: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    GovernanceToken,
    Proposals(u32),
    Quorum,
    ProposalVotes(u32),
    VoteTokenWeight,
    StakedAmount(Address),
    TotalStaked,
    VotersCount,
    HasVoted(Address, u32),
}
