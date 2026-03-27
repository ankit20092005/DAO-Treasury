#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct SimpleTreasury;

#[contractimpl]
impl SimpleTreasury {

    // Store balance
    pub fn deposit(env: Env, amount: i128) {
        let mut balance: i128 = env.storage().instance().get(&"BAL").unwrap_or(0);
        balance += amount;
        env.storage().instance().set(&"BAL", &balance);
    }

    // Withdraw balance
    pub fn withdraw(env: Env, amount: i128) {
        let mut balance: i128 = env.storage().instance().get(&"BAL").unwrap_or(0);

        if amount > balance {
            panic!("Not enough balance");
        }

        balance -= amount;
        env.storage().instance().set(&"BAL", &balance);
    }

    // Get balance
    pub fn get_balance(env: Env) -> i128 {
        env.storage().instance().get(&"BAL").unwrap_or(0)
    }
}