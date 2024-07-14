use crate::storage_types::{DataKey, BUMP_AMOUNT, LIFETIME_THRESHOLD};
use soroban_sdk::{Address, Env};

pub fn read_user_stake(e: &Env, staker: Address) -> i128 {
    let key = DataKey::StakedAmount(staker);
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        balance
    } else {
        0
    }
}

pub fn write_user_stake(e: &Env, staker: Address, amount: i128) {
    let key = DataKey::StakedAmount(staker);
    e.storage().persistent().set(&key, &amount);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn read_total_staked(e: &Env) -> i128 {
    let key = DataKey::TotalStaked;
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        balance
    } else {
        0
    }
}

pub fn write_total_staked(e: &Env, amount: i128) {
    let key = DataKey::TotalStaked;
    let cur_total_staked = read_total_staked(e);
    let new_total_staked = amount + cur_total_staked;
    e.storage().persistent().set(&key, &new_total_staked);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}
