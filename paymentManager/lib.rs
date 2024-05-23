#![cfg_attr(not(feature = "std"), no_std, no_main)]


pub use self::paymentManager::{
    PaymentManagerRef,
};

#[ink::contract]
mod paymentManager {
    use super::*;
    pub type TokenId = u128;
    // use amm::AmmPoolRef;

    #[ink(storage)]
    pub struct PaymentManager {
        value: bool,
        // amm_pool: AmmPoolRef,
    }

    impl PaymentManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            // let amm_pool: AmmPoolRef = AmmPoolRef::new();

            Self{value: true, /* amm_pool */}
        }

        #[ink(message)]
        pub fn liquidation(&mut self) {
            //call pool
            //Update mapping in manager
            //
        }

        #[ink(message)]
        pub fn add_to_pool(&self, token_id: TokenId, value: Balance, user: AccountId) {
            //call add_liquidity on AMM/Pool contract
        }

        #[ink(message)]
        pub fn remove_from_pool(&mut self, user: AccountId, amount: Balance) {
            //amount from manager contract
        }

        #[ink(message)]
        pub fn collect_margin_fee(&self, user: AccountId) {
            //ping oracle for amount
            //call collect_fee on amm
        }

        #[ink(message)]
        pub fn withdraw_funds(&self, user: AccountId) {
            //ping oracle for amount to withdraw
            //call withdrawFunds on amm
        }
    }

}
