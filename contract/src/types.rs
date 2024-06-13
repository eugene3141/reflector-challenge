use soroban_sdk::{contracttype, contracterror, Address, Symbol, Vec, Env, panic_with_error, I256};
use crate::reflector_oracle;

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const PERSISTENT_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const PERSISTENT_LIFETIME_THRESHOLD: u32 = PERSISTENT_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone, Debug)]
#[contracttype]
pub enum DataKey {
    Loan(u64),
    Loans(Address),
    Balances(Address)
}


#[derive(Clone, Debug)]
#[contracttype]
pub struct Loan {
    pub borrower: Option<Address>,
    pub lender: Option<Address>,
    pub collateral: Option<Collateral>,
    pub status: LoanStatus,
    pub loan_asset: Address,
    pub loan_amount: i128,
    pub daily_interest_rate: u32,
    pub max_loan_term: u32,
    pub timestamp: u64
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Collateral {
    pub asset_contract: Address,
    pub amount: i128,
    pub seize_conditions: Vec<SeizeCondition>
}

impl Collateral {
    pub fn seizable(&self, env: &Env, loan: Loan) -> bool {
        let mut seizable = false;

        for seize_condition in self.seize_conditions.iter() {
            if seize_condition.check(env, loan.clone()) {
                seizable = true;
            }
        }

        seizable
    }
}

#[derive(Clone, Debug)]
#[contracttype]
pub enum SeizeCondition {
    LoanDefault,
    ReflectorOracle(OracleAsset, i128, OracleAsset, i128, bool) // Asset_A, Amount_A, Asset_B, Amount_B, A>B/A<B
}

impl SeizeCondition {
    pub fn check(&self, env: &Env, loan: Loan) -> bool {
        let ledger = env.ledger();
        match self {
            SeizeCondition::LoanDefault => {
                let loan_duration = ((ledger.timestamp() - loan.timestamp) / 86400) + 1;
                loan_duration > loan.max_loan_term as u64
            },
            SeizeCondition::ReflectorOracle(asset_a, amount_a, asset_b, amount_b, greater) => {
                let asset_a_price = asset_a.lastprice(&env);
                let asset_b_price = asset_b.lastprice(&env);

                if asset_b_price == 0 || *amount_b == 0 {
                    return false;
                }

                let asset_a_value = I256::from_i128(env, asset_a_price)
                                            .mul(&I256::from_i128(env, *amount_a));

                let asset_b_value = I256::from_i128(env, asset_b_price)
                                            .mul(&I256::from_i128(env, *amount_b));

                let diff = asset_a_value.div(&asset_b_value).to_i128().unwrap_or(0);

                if *greater {
                    return diff > 0;
                }

                diff == 0
            }
        }
    }
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct OracleAsset {
    pub oracle_contract: Address,
    pub asset_contract: Option<Address>,
    pub oracle_symbol: Option<Symbol>
}

impl OracleAsset {
    pub fn lastprice(&self, env: &Env) -> i128 {
        let ledger = env.ledger();

        let reflector_contract = reflector_oracle::Client::new(&env, &self.oracle_contract);

        let oracle_asset = match self.oracle_symbol.clone() {
            Some(symbol) => reflector_oracle::Asset::Other(symbol),
            None => reflector_oracle::Asset::Stellar(self.asset_contract.clone().unwrap())
        };
        
        let asset_price_data = reflector_contract.lastprice(&oracle_asset).unwrap_or_else(|| panic_with_error!(&env,Error::OracleError));
        
        // checking if the price is not stale (5 minutes sampling + 10 sec)
        if (ledger.timestamp() - 310) > asset_price_data.timestamp {
            panic_with_error!(&env, Error::OracleError);
        }

        asset_price_data.price
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[contracttype]
pub enum LoanStatus {
    WaitingForLender,
    WaitingForBorrower,
    InProgress
}


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotAuthorized = 0,

    LoanNotExist = 100,
    LoanAlreadyExist = 101,
    InvalidCollateral = 102,
    InvalidBorrower = 103,
    InvalidLender = 104,
    LoanInProgress = 105,
    LoanNotInProgress = 106,
    LendingError = 107,
    BorrowingError = 108,
    CollateralNotSeizable = 109,

    OracleError = 500
}