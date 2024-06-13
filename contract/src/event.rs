use soroban_sdk::{Env, Symbol};

pub(crate) fn new_loan(env: &Env, loan_key: u64) {
    let topics = (Symbol::new(env, "new_loan"),);
    env.events().publish(topics, loan_key);
}

pub(crate) fn loan_canceled(env: &Env, loan_key: u64) {
    let topics = (Symbol::new(env, "loan_canceled"),);
    env.events().publish(topics, loan_key);
}

pub(crate) fn loan_repaid(env: &Env, loan_key: u64) {
    let topics = (Symbol::new(env, "loan_repaid"),);
    env.events().publish(topics, loan_key);
}

pub(crate) fn collateral_seized(env: &Env, loan_key: u64) {
    let topics = (Symbol::new(env, "collateral_seized"),);
    env.events().publish(topics, loan_key);
}