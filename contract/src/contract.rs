use soroban_sdk::{
    contract, contractimpl, Address, Env, Vec, Map, token, panic_with_error
};
use crate::types::*;
use crate::event;

pub trait P2PLendingTrait {
    // create new loan
    fn new_loan(env: Env, loan_key: u64, loan: Loan);
    // cancel loan
    fn cancel_loan(env: Env, loan_key: u64);
    // lend
    fn lend(env: Env, loan_key: u64, lender: Address);
    // borrow
    fn borrow(env: Env, loan_key: u64, borrower: Address);

    // repay loan
    fn repay(env: Env, loan_key: u64, user: Address);
    // seize collateral
    fn seize(env: Env, loan_key: u64);

    // get loan
    fn get_loan(env: Env, loan_key: u64) -> Loan;
    // get loan list
    fn get_loans(env: Env, user: Address) -> Vec<u64>;
    // get current interest
    fn get_interest(env: Env, loan_key: u64) -> i128;
}


pub trait ClaimableBalanceTrait {
    // withdraw unclaimed balance
    fn withdraw(eenv: Env, user: Address, token: Address);

    // get unclaimed balances
    fn get_balances(env: Env, user: Address) -> Map<Address, i128>;
}

#[contract]
pub struct P2PLendingContract;

#[contractimpl]
impl P2PLendingTrait for P2PLendingContract {
    fn new_loan(env: Env, loan_key: u64, loan: Loan) {
        if env.storage().persistent().has(&DataKey::Loan(loan_key.clone())) {
            panic_with_error!(&env, Error::LoanAlreadyExist);
        }

        if loan.collateral.is_some() {
            let collateral = loan.collateral.clone().unwrap();

            if collateral.seize_conditions.len() == 0 {
                panic_with_error!(&env, Error::InvalidCollateral);
            }
        }

        match loan.status { 
            LoanStatus::WaitingForLender => {
                let borrower = loan.borrower.clone().unwrap_or_else(|| panic_with_error!(&env, Error::InvalidBorrower));
                borrower.require_auth();

                // transfering collateral to the smart contract
                if loan.collateral.is_some() {
                    let collateral = loan.collateral.clone().unwrap();
                    _transfer_tokens(&env, &collateral.asset_contract, &borrower, &env.current_contract_address(), collateral.amount);
                }

                _modify_loan_list(&env, &borrower, loan_key, true);
            },
            LoanStatus::WaitingForBorrower => {
                let lender = loan.lender.clone().unwrap_or_else(|| panic_with_error!(&env, Error::InvalidLender));
                lender.require_auth();

                // transfering loan amount to the smart contract
                _transfer_tokens(&env, &loan.loan_asset, &lender, &env.current_contract_address(), loan.loan_amount);

                _modify_loan_list(&env, &lender, loan_key, true);
            },
            LoanStatus::InProgress => {
                // can't create a loan with in progress status
                panic_with_error!(&env, Error::LoanInProgress);
            }
        }

        env.storage().persistent().set(&DataKey::Loan(loan_key.clone()), &loan);

        env.storage().persistent().extend_ttl(
            &DataKey::Loan(loan_key.clone()),
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT
        );

        event::new_loan(&env, loan_key.clone());
    }

    fn cancel_loan(env: Env, loan_key: u64) {
        let loan: Loan = env.storage().persistent().get(&DataKey::Loan(loan_key.clone())).unwrap_or_else(|| panic_with_error!(&env, Error::LoanNotExist));

        match loan.status { 
            LoanStatus::WaitingForLender => {
                let borrower = loan.borrower.clone().unwrap_or_else(|| panic_with_error!(&env, Error::InvalidBorrower));
                borrower.require_auth();

                // returning collateral to borrower
                if loan.collateral.is_some() {
                    let collateral = loan.collateral.unwrap();
                    _transfer_tokens(&env, &collateral.asset_contract, &env.current_contract_address(), &borrower, collateral.amount);
                }

                _modify_loan_list(&env, &borrower, loan_key, false);
            },
            LoanStatus::WaitingForBorrower => {
                let lender = loan.lender.clone().unwrap_or_else(|| panic_with_error!(&env, Error::InvalidLender));
                lender.require_auth();

                // returning loan amount to lender
                _transfer_tokens(&env, &loan.loan_asset, &env.current_contract_address(), &lender, loan.loan_amount);

                _modify_loan_list(&env, &lender, loan_key, false);
            },
            LoanStatus::InProgress => {
                // In progress loans can't be canceled
                panic_with_error!(&env, Error::LoanInProgress);
            }
        }

        env.storage().persistent().remove(&DataKey::Loan(loan_key.clone())); 

        event::loan_canceled(&env, loan_key.clone());
    }

    fn lend(env: Env, loan_key: u64, lender: Address) {
        let mut loan: Loan = env.storage().persistent().get(&DataKey::Loan(loan_key.clone())).unwrap_or_else(|| panic_with_error!(&env, Error::LoanNotExist));

        lender.require_auth();

        if loan.status != LoanStatus::WaitingForLender {
            panic_with_error!(&env, Error::LendingError);
        }

        // checking if the loan reserved for specific lender
        if loan.lender.is_some() {
            if lender != loan.lender.clone().unwrap() {
                panic_with_error!(&env, Error::InvalidLender);
            }
        }

        let borrower = loan.borrower.clone().unwrap_or_else(|| panic_with_error!(&env, Error::InvalidBorrower));

        if lender == borrower {
            panic_with_error!(&env, Error::LendingError);
        }

        // transfering loan amount from lender to borrower
        _transfer_tokens(&env, &loan.loan_asset, &lender, &borrower, loan.loan_amount);

        // updating loan data
        loan.lender = Some(lender.clone());
        let ledger = env.ledger();
        loan.timestamp = ledger.timestamp();
        loan.status = LoanStatus::InProgress;

        env.storage().persistent().set(&DataKey::Loan(loan_key.clone()), &loan);

        env.storage().persistent().extend_ttl(
            &DataKey::Loan(loan_key.clone()),
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT
        );

        _modify_loan_list(&env, &lender, loan_key, true);

        event::new_loan(&env, loan_key.clone());
    }

    fn borrow(env: Env, loan_key: u64, borrower: Address) {
        let mut loan: Loan = env.storage().persistent().get(&DataKey::Loan(loan_key.clone())).unwrap_or_else(|| panic_with_error!(&env, Error::LoanNotExist));

        borrower.require_auth();

        if loan.status != LoanStatus::WaitingForBorrower {
            panic_with_error!(&env, Error::BorrowingError);
        }

        // checking if the loan reserved for specific borrower
        if loan.borrower.is_some() {
            if borrower != loan.borrower.clone().unwrap() {
                panic_with_error!(&env, Error::InvalidBorrower);
            }
        }

        if loan.lender.clone().unwrap() == borrower {
            panic_with_error!(&env, Error::BorrowingError);
        }

        // transfering loan amount from smart contract to borrower
        _transfer_tokens(&env, &loan.loan_asset, &env.current_contract_address(), &borrower, loan.loan_amount);

        // transfering collateral from borrower to smart contract
        if loan.collateral.is_some() {
            let collateral = loan.collateral.clone().unwrap();
            _transfer_tokens(&env, &collateral.asset_contract, &borrower, &env.current_contract_address(), collateral.amount); 
        }
        

        // updating loan data
        loan.borrower = Some(borrower.clone());
        let ledger = env.ledger();
        loan.timestamp = ledger.timestamp();
        loan.status = LoanStatus::InProgress;

        env.storage().persistent().set(&DataKey::Loan(loan_key.clone()), &loan);

        env.storage().persistent().extend_ttl(
            &DataKey::Loan(loan_key.clone()),
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT
        );

        _modify_loan_list(&env, &borrower, loan_key, true);

        event::new_loan(&env, loan_key.clone());
    }

    fn repay(env: Env, loan_key: u64, user: Address) {
        let loan: Loan = env.storage().persistent().get(&DataKey::Loan(loan_key.clone())).unwrap_or_else(|| panic_with_error!(&env, Error::LoanNotExist));

        user.require_auth();

        if loan.status != LoanStatus::InProgress {
            panic_with_error!(&env, Error::LoanNotInProgress);
        }

        let interest = _calculate_interest(&env, &loan);
        let total_amount = interest + loan.loan_amount;

        // transfering loan amount + interest from user to lender
        let lender = loan.lender.clone().unwrap_or_else(|| panic_with_error!(&env, Error::InvalidLender));
        _create_claimable_balance(&env, &loan.loan_asset, &user, &lender, total_amount);

        // returning collateral from smart contract to borrower
        let borrower = loan.borrower.clone().unwrap_or_else(|| panic_with_error!(&env, Error::InvalidBorrower));
        if loan.collateral.is_some() {
            let collateral = loan.collateral.clone().unwrap();
            _create_claimable_balance(&env, &collateral.asset_contract, &env.current_contract_address(), &borrower, collateral.amount);   
        }

        _modify_loan_list(&env, &lender, loan_key, false);
        _modify_loan_list(&env, &borrower, loan_key, false);

        env.storage().persistent().remove(&DataKey::Loan(loan_key.clone())); 

        event::loan_repaid(&env, loan_key.clone());
    }

    fn seize(env: Env, loan_key: u64) {
        let loan: Loan = env.storage().persistent().get(&DataKey::Loan(loan_key.clone())).unwrap_or_else(|| panic_with_error!(&env, Error::LoanNotExist));

        if loan.status != LoanStatus::InProgress {
            panic_with_error!(&env, Error::LoanNotInProgress);
        }

        if loan.collateral.is_none() {
            panic_with_error!(&env, Error::InvalidCollateral);
        }

        let collateral = loan.collateral.clone().unwrap();

        let lender = loan.lender.clone().unwrap_or_else(|| panic_with_error!(&env, Error::InvalidLender));

        let borrower = loan.borrower.clone().unwrap_or_else(|| panic_with_error!(&env, Error::InvalidBorrower));

        // only lender can seize collateral
        lender.require_auth();

        // checking if the collateral can be seized
        if !collateral.seizable(&env, loan) {
            panic_with_error!(&env, Error::CollateralNotSeizable);
        }

        // transfering collateral from smart contract to lender
        _create_claimable_balance(&env, &collateral.asset_contract, &env.current_contract_address(), &lender, collateral.amount);

        _modify_loan_list(&env, &lender, loan_key, false);
        _modify_loan_list(&env, &borrower, loan_key, false);

        env.storage().persistent().remove(&DataKey::Loan(loan_key.clone())); 

        event::collateral_seized(&env, loan_key.clone());
    }


    fn get_loan(env: Env, loan_key: u64) -> Loan {
        env.storage().persistent().get(&DataKey::Loan(loan_key.clone())).unwrap_or_else(|| panic_with_error!(&env, Error::LoanNotExist))
    }

    fn get_loans(env: Env, user: Address) -> Vec<u64> {
        env.storage().persistent().get(&DataKey::Loans(user.clone())).unwrap_or(Vec::new(&env))
    }

    fn get_interest(env: Env, loan_key: u64) -> i128 {
        let loan: Loan = env.storage().persistent().get(&DataKey::Loan(loan_key.clone())).unwrap_or_else(|| panic_with_error!(&env, Error::LoanNotExist));

        _calculate_interest(&env, &loan)
    }
}

#[contractimpl]
impl ClaimableBalanceTrait for P2PLendingContract {
    fn withdraw(env: Env, user: Address, token: Address) {
        let mut balances: Map<Address, i128> = env.storage().persistent().get(&DataKey::Balances(user.clone())).unwrap_or(Map::new(&env));

        let balance = balances.get(token.clone()).unwrap_or(0);

        if balance == 0 {
            panic_with_error!(&env, Error::NotAuthorized);
        }

        _transfer_tokens(&env, &token, &env.current_contract_address(), &user, balance);

        balances.remove(token.clone());

        env.storage().persistent().set(&DataKey::Balances(user.clone()), &balances);
        env.storage().persistent().extend_ttl(&DataKey::Balances(user.clone()), PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    }

    fn get_balances(env: Env, user: Address) -> Map<Address, i128> {
        env.storage().persistent().get(&DataKey::Balances(user.clone())).unwrap_or(Map::new(&env))
    }
}

fn _modify_loan_list(
    env: &Env,
    user: &Address,
    loan_key: u64,
    add: bool
) {
    let mut loan_list: Vec<u64> = env.storage().persistent().get(&DataKey::Loans(user.clone())).unwrap_or(Vec::new(env));

    if add {
        if loan_list.first_index_of(loan_key).is_none() {
            loan_list.push_front(loan_key);
        }
    } else {
        let index = loan_list.first_index_of(loan_key);
        if index.is_some() {
            loan_list.remove(index.unwrap());
        }
    }

    env.storage().persistent().set(&DataKey::Loans(user.clone()), &loan_list);

    env.storage().persistent().extend_ttl(
        &DataKey::Loans(user.clone()),
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT
    );
}

fn _calculate_interest(
    env: &Env,
    loan: &Loan
) -> i128 {
    if loan.status != LoanStatus::InProgress {
        return 0;
    }

    let ledger = env.ledger();
    let loan_duration = ((ledger.timestamp() - loan.timestamp) / 86400) + 1;

    let interest = (loan.loan_amount * loan_duration as i128 * loan.daily_interest_rate as i128) / 10000;

    interest
}

fn _transfer_tokens(
    env: &Env,
    token_address: &Address,
    from: &Address,
    to: &Address,
    transfer_amount: i128,
) {
    let token = token::Client::new(env, token_address);
    token.transfer(from, to, &transfer_amount);
}

fn _create_claimable_balance(
    env: &Env,
    token_address: &Address,
    from: &Address,
    to: &Address,
    transfer_amount: i128,
) {
    if from != &env.current_contract_address() {
        let token = token::Client::new(env, token_address);
        token.transfer(from, &env.current_contract_address(), &transfer_amount);    
    }

    let mut balances: Map<Address, i128> = env.storage().persistent().get(&DataKey::Balances(to.clone())).unwrap_or(Map::new(&env));

    let mut balance = balances.get(token_address.clone()).unwrap_or(0);

    balance += transfer_amount;

    balances.set(token_address.clone(), balance);

    env.storage().persistent().set(&DataKey::Balances(to.clone()), &balances);
    env.storage().persistent().extend_ttl(&DataKey::Balances(to.clone()), PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}