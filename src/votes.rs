use crate::balances::read_user_stake;
use crate::proposal::read_proposal;
use crate::storage_types::{DataKey, QuorumRequirements, Votes, VotesWeight};
use soroban_sdk::{Address, Env};

pub fn read_quorum_requirement(e: &Env) -> QuorumRequirements {
    let key = DataKey::Quorum;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_quorum_requirement(e: &Env, min_total_votes: u64, percent_yes: u64) {
    let key = DataKey::Quorum;

    let quorum_requirements = QuorumRequirements {
        min_total_votes,
        percent_yes,
    };
    e.storage().instance().set(&key, &quorum_requirements);
}

pub fn read_vote_token_weight(e: &Env) -> VotesWeight {
    let key = DataKey::VoteTokenWeight;
    if let Some(vote_weight) = e.storage().instance().get::<_, VotesWeight>(&key) {
        {
            vote_weight
        }
    } else {
        VotesWeight {
            staker_weight: 0,
            holder_weight: 0,
        }
    }
}

pub fn write_vote_token_weight(e: &Env, staker_weight: u32, holder_weight: u32) {
    let key = DataKey::VoteTokenWeight;
    let vote_weight = VotesWeight {
        staker_weight,
        holder_weight,
    };
    e.storage().instance().set(&key, &vote_weight);
}

pub fn read_can_vote(e: &Env, voter: Address) -> bool {
    let staked_amount = read_user_stake(e, voter);
    let stake_weight = read_vote_token_weight(e).staker_weight as i128;
    staked_amount >= stake_weight
}

pub fn read_has_voted(e: &Env, voter: Address, proposal_id: u32) -> bool {
    let key = DataKey::HasVoted(voter, proposal_id);

    e.storage().instance().get(&key).unwrap_or(false)
}

pub fn write_has_voted(e: &Env, voter: Address, proposal_id: u32) {
    let key = DataKey::HasVoted(voter, proposal_id);

    e.storage().instance().set(&key, &true);
}

pub fn read_proposal_votes(e: &Env, proposal_id: u32) -> Votes {
    let key = DataKey::ProposalVotes(proposal_id);
    if let Some(proposal_votes) = e.storage().instance().get::<_, Votes>(&key) {
        {
            proposal_votes
        }
    } else {
        Votes {
            yes_votes: 0,
            total_votes: 0,
        }
    }
}

pub fn write_proposal_votes(
    e: &Env,
    voter: Address,
    vote_count: u64,
    yes_or_no: bool,
    proposal_id: u32,
) {
    let can_vote = read_can_vote(e, voter.clone());
    if !can_vote {
        panic!("you don't meet the voting requirements")
    }
    let has_voted = read_has_voted(e, voter.clone(), proposal_id);
    if has_voted {
        panic!("you have voted on this proposal already")
    }

    let key = DataKey::ProposalVotes(proposal_id);
    let cur_votes = read_proposal_votes(e, proposal_id);
    let new_total_votes = cur_votes.total_votes + vote_count;
    let new_yes_votes = if yes_or_no {
        cur_votes.yes_votes + vote_count
    } else {
        cur_votes.yes_votes
    };

    let new_votes_out = Votes {
        yes_votes: new_yes_votes,
        total_votes: new_total_votes,
    };
    write_has_voted(e, voter, proposal_id);

    e.storage().instance().set(&key, &new_votes_out);
}

pub fn read_is_proposal_passed(e: &Env, proposal_id: u32) -> bool {
    let voting_has_ended = read_proposal(e, proposal_id).vote_end_at >= e.ledger().timestamp();
    if !voting_has_ended {
        panic!("Voting is still ongoing")
    };

    let quorum_requirement = read_quorum_requirement(e);
    let voting_outcome = read_proposal_votes(e, proposal_id);
    if voting_outcome.total_votes > quorum_requirement.min_total_votes
        && voting_outcome.yes_votes > voting_outcome.total_votes * quorum_requirement.percent_yes
    {
        true
    } else {
        false
    }
}
