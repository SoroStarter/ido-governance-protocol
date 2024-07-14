use crate::access::{has_administrator, read_administrator, write_administrator};
use crate::balances::{read_total_staked, read_user_stake, write_total_staked, write_user_stake};
use crate::governance_token::{read_token, send_token, take_token, write_token};

use crate::proposal::{read_proposal, write_proposal};

use crate::storage_types::{Proposal, QuorumRequirements, Votes, VotesWeight};
use crate::votes::{
    read_can_vote, read_has_voted, read_is_proposal_passed, read_proposal_votes,
    read_quorum_requirement, read_vote_token_weight, write_proposal_votes,
    write_quorum_requirement, write_vote_token_weight,
};

use soroban_sdk::{contract, contractimpl, Address, Env, String};

pub trait SaleTrait {
    fn initialize(e: Env, admin: Address);
    fn set_governance_token(e: Env, token_addres: Address);
    fn get_governance_token(e: Env) -> Address;
    fn set_token_vote_weight(e: Env, staker_weight: u32, holder_weight: u32);
    fn get_token_vote_weight(e: Env) -> VotesWeight;
    fn set_quorum_requirements(e: Env, min_total_votes: u64, percent_yes: u64);
    fn get_quorum_requirements(e: Env) -> QuorumRequirements;
    fn create_proposal(
        e: Env,
        proposal_id: u32,
        creator: Address,
        title: String,
        description: String,
        vote_start_at: u64,
        vote_end_at: u64,
    );
    fn get_proposal(e: Env, proposal_id: u32) -> Proposal;
    fn cast_vote(e: Env, voter: Address, yes_or_no: bool, proposal_id: u32);
    fn get_proposal_votes(e: Env, requester: Address, proposal_id: u32) -> Votes;
    fn get_user_can_vote(e: Env, voter: Address) -> bool;
    fn get_user_has_voted(e: Env, voter: Address, proposal_id: u32) -> bool;
    fn get_is_proposal_passed(e: Env, proposal_id: u32) -> bool;
    fn stake(e: Env, staker: Address, amount: i128);
    fn unstake(e: Env, staker: Address, amount: i128);
    fn get_user_stake(e: Env, staker: Address) -> i128;
    fn get_total_staked(e: Env) -> i128;

    fn get_admin(e: &Env) -> Address;
}

#[contract]
pub struct TokenSale;

#[contractimpl]
impl SaleTrait for TokenSale {
    fn initialize(e: Env, admin: Address) {
        if has_administrator(&e) {
            panic!("already has an admin")
        }
        write_administrator(&e, &admin);
    }

    fn set_governance_token(e: Env, token_addres: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();
        write_token(&e, &token_addres);
    }

    fn get_governance_token(e: Env) -> Address {
        read_token(&e)
    }

    fn set_token_vote_weight(e: Env, staker_weight: u32, holder_weight: u32) {
        write_vote_token_weight(&e, staker_weight, holder_weight)
    }

    fn get_token_vote_weight(e: Env) -> VotesWeight {
        read_vote_token_weight(&e)
    }

    fn set_quorum_requirements(e: Env, min_total_votes: u64, percent_yes: u64) {
        let admin = read_administrator(&e);
        admin.require_auth();
        write_quorum_requirement(&e, min_total_votes, percent_yes)
    }

    fn get_quorum_requirements(e: Env) -> QuorumRequirements {
        read_quorum_requirement(&e)
    }

    fn create_proposal(
        e: Env,
        proposal_id: u32,
        creator: Address,
        title: String,
        description: String,
        vote_start_at: u64,
        vote_end_at: u64,
    ) {
        let can_vote = read_can_vote(&e, creator.clone());
        if !can_vote {
            panic!("you are not eligible to create a proposal")
        }
        creator.require_auth();
        write_proposal(
            &e,
            proposal_id,
            creator,
            title,
            description,
            vote_start_at,
            vote_end_at,
        )
    }

    fn get_proposal(e: Env, proposal_id: u32) -> Proposal {
        read_proposal(&e, proposal_id)
    }

    fn cast_vote(e: Env, voter: Address, yes_or_no: bool, proposal_id: u32) {
        write_proposal_votes(&e, voter, 1, yes_or_no, proposal_id)
    }

    fn get_proposal_votes(e: Env, requester: Address, proposal_id: u32) -> Votes {
        let requester_has_voted = read_has_voted(&e, requester.clone(), proposal_id);
        let admin = read_administrator(&e);
        let voting_is_ongoing =
            read_proposal(&e, proposal_id).vote_end_at >= e.ledger().timestamp();

        if requester != admin {
            if !requester_has_voted && voting_is_ongoing {
                panic!("you need to vote before you can see the result");
            }
        }

        read_proposal_votes(&e, proposal_id)
    }

    fn get_user_can_vote(e: Env, voter: Address) -> bool {
        read_can_vote(&e, voter)
    }

    fn get_user_has_voted(e: Env, voter: Address, proposal_id: u32) -> bool {
        read_has_voted(&e, voter, proposal_id)
    }

    fn get_is_proposal_passed(e: Env, proposal_id: u32) -> bool {
        read_is_proposal_passed(&e, proposal_id)
    }

    fn stake(e: Env, staker: Address, amount: i128) {
        staker.require_auth();
        let governance_token = read_token(&e);
        let cur_stake_amount = read_user_stake(&e, staker.clone());
        let cur_stake_total = read_total_staked(&e);
        take_token(&e, &governance_token, &staker, amount as i128);
        let new_stake_amount = cur_stake_amount + amount;
        let new_total_staked = cur_stake_total + amount;
        write_user_stake(&e, staker, new_stake_amount);
        write_total_staked(&e, new_total_staked)
    }

    fn unstake(e: Env, staker: Address, amount: i128) {
        staker.require_auth();
        let cur_stake_amount = read_user_stake(&e, staker.clone());
        let cur_stake_total = read_total_staked(&e);
        if amount > cur_stake_amount {
            panic!("you cannot unstake more than you have staked")
        }
        let governance_token = read_token(&e);

        send_token(&e, &governance_token, &staker, amount as i128);
        let new_staked_amount = cur_stake_amount - amount;
        let new_total_staked = cur_stake_total - amount;
        write_total_staked(&e, new_total_staked);
        write_user_stake(&e, staker, new_staked_amount);
    }

    fn get_user_stake(e: Env, staker: Address) -> i128 {
        read_user_stake(&e, staker)
    }

    fn get_total_staked(e: Env) -> i128 {
        read_total_staked(&e)
    }

    fn get_admin(e: &Env) -> Address {
        read_administrator(e)
    }
}
