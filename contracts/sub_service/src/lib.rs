#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct SimpleSubscriptionContract;

#[contractimpl]
impl SimpleSubscriptionContract {
    // subscribe for the platform for 1 year for 365 tokens 1token/day
    pub fn subscribe(env: Env, from: Address, amount: i128) -> Symbol {
        if amount != 365 {
            panic!("wrong amount sent");
        }
        // Make sure `from` address authorized the deposit call with all the
        // arguments.
        from.require_auth();

        // Return a message to indicate successful subscription
        Symbol::new(&env, "Subscribed")
    }

    // unsubscribe from the platform
    pub fn unsubscribe(env: Env, from: Address) -> Symbol {
        // Make sure `from` address authorized the deposit call with all the
        // arguments.
        from.require_auth();

        // Return a message to indicate successful unsubscription
        Symbol::new(&env, "UnSubscribed")
    }
}