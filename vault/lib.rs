#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod vault {
    // use ink::{contract_ref};
    // use psp22::{PSP22Error, PSP22};
    use super::*;
    use openbrush::traits::Storage;
    use ink::storage::Mapping;
    use paymentToken::PaymentTokenRef;
    use perpToken::PerpTokenRef;

    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        from: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Withdraw {
        #[ink(topic)]
        from: AccountId,
        amount: Balance,     
    }

    #[ink(event)]
    pub struct WithdrawMarginFee {
        #[ink(topic)]
        manager: AccountId,
        amount: Balance,
    }

    #[ink(storage)]
    pub struct Vault {
        token_in: AccountId,
        token_out: AccountId,
        admin: AccountId,
        paymentToken: PaymentTokenRef,
        perpToken: PerpTokenRef,
    }

    pub enum Error {}

    impl Vault {
        #[ink(constructor)]
        pub fn new(payment_token_code_hash: Hash, initial_supply: Balance, perp_token_code_hash: Hash, _token_in: AccountId, _token_out: AccountId) -> Self {
            let caller = Self::env().caller();

            let paymentToken: PaymentTokenRef = PaymentTokenRef::new(initial_supply)
                .code_hash(payment_token_code_hash);

            let perpToken: PerpTokenRef = PerpTokenRef::new()
                .code_hash(perp_token_code_hash);

            // let mut token_in: contract_ref!(PSP22) = _token_in.into();
            // let mut token_out: contract_ref!(PSP37) = _token_out.into();

            Self { 
                token_in: _token_in, 
                token_out: _token_out, 
                admin: caller,
                paymentToken: paymentToken,
                perpToken: perpToken,
            }
        }

        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance, calculated_amount: Balance, user: AccountId) {
            let contract = self.env().account_id();
            // let mut token_ref: contract_ref!(PSP22) = self.token_in.into();

            // token_ref.transfer_from(user, contract, amount, Vec::new());

            // self.token_out.transfer(user, calculated_amount);

            self.env().emit_event(Deposit {
                from: user,
                amount,
            });

            //Ok(())
        }

        // #[ink(message)]
        // pub fn withdraw(&mut self, amount: Balance, calculated_amount: Balance, user: AccountId) -> Result<()> {
        //     let contract = self.env().account_id();

        //     self.token_out.transfer_from(user, contract, calculated_amount);

        //     self.token_in.transfer(user, amount);

        //     self.env().emit_event(Withdraw {
        //         from: user,
        //         amount,
        //     });

        //     Ok(())
        // }

        // #[ink(message)]
        // pub fn collect_margin_fee(&mut self, amount: Balance, manager: AccountId) -> Result<()> {
        //     self.token_in.transfer(manager, amount);

        //     self.env().emit_event(WithdrawMarginFee {
        //         manager: manager,
        //         amount,
        //     });

        //     Ok(())
        // }
    }
}
