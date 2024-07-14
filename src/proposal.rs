use crate::storage_types::{DataKey, Proposal};
use soroban_sdk::{Address, Env, String};

pub fn read_proposal(e: &Env, proposal_id: u32) -> Proposal {
    let key = DataKey::Proposals(proposal_id);
    if let Some(proposal) = e.storage().instance().get::<_, Proposal>(&key) {
        {
            proposal
        }
    } else {
        panic!("no the proposal does not exist")
    }
}

pub fn write_proposal(
    e: &Env,
    proposal_id: u32,
    creator: Address,
    title: String,
    description: String,
    vote_start_at: u64,
    vote_end_at: u64,
) {
    if vote_end_at <= e.ledger().timestamp()
        || vote_end_at < vote_start_at
        || proposal_id <= 0
        || title.len() == 0
    {
        panic!("invalid parameter(s) entered!")
    }

    let proposal = Proposal {
        proposal_id,
        creator,
        title,
        description,
        vote_start_at,
        vote_end_at,
    };

    let key = DataKey::Proposals(proposal_id);
    e.storage().instance().set(&key.clone(), &proposal);
}
