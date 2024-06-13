#![no_std]

mod types;
mod event;
mod contract;
mod reflector_oracle {
    soroban_sdk::contractimport!(file = "./reflector_oracle.wasm");
}

pub use crate::contract::P2PLendingContractClient;