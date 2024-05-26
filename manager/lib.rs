#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub type TokenId = u128;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum Error {}

#[ink::contract]
mod manager {
    use super::*;
    use ink::storage::Mapping;
    use paymentManager::PaymentManagerRef;

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub enum Position {
        LONG,
        SHORT,
    }

    #[ink(event)]
    pub struct PositionOpened {
        #[ink(topic)]
        from: Option<AccountId>,
        token_id: TokenId,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct PositionClosed {
        #[ink(topic)]
        from: Option<AccountId>,
        token_id: TokenId,      
    }

    #[ink(storage)]
    //#[derive(Default)]
    pub struct Manager {
        balances: Mapping<(AccountId, TokenId), Balance>,
        deposits: Mapping<(AccountId, TokenId), Balance>,
        positions: Mapping<(AccountId, TokenId), Position>,
        entry_position_prices: Mapping<(AccountId, TokenId), Balance>,
        paymentManager: PaymentManagerRef,
        oracle: AccountId,
        leverage: u32,
    }

    impl Manager {

        #[ink(constructor, payable)]
        pub fn new(payment_manager_code_hash: Hash, _oracle: AccountId, _leverage: u32) -> Self {
            let balances = Mapping::default();
            let entry_position_prices = Mapping::default();
            let deposits = Mapping::default();
            let positions = Mapping::default();
            let leverage = _leverage;
            let paymentManager: PaymentManagerRef = PaymentManagerRef::new()
            .code_hash(payment_manager_code_hash)
            .endowment(0)
            .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
            .instantiate();
            let oracle = _oracle;
            Self { balances, deposits, positions, entry_position_prices, paymentManager, oracle, leverage }
        }

        #[ink(message)]
        pub fn open_position(&mut self, token_id: TokenId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            self.balances.insert((caller, token_id), &value);
            self.paymentManager.liquidation();

            self.env().emit_event(PositionOpened {
                from: Some(caller),
                token_id,
                value,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn close_position(&mut self, token_id: TokenId) -> Result<()> {
            let caller = self.env().caller();
            self.balances.remove((caller, token_id));
            self.paymentManager.withdraw_funds(caller);

            self.env().emit_event(PositionClosed {
                from: Some(caller),
                token_id,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn check_position(&self, user: AccountId, token_id: TokenId) -> Balance {
            // let spot_price = self.oracle.get_price();
            let spot_price: u128 = 123;
            let entry_price = self.entry_position_prices.get((user, token_id)).unwrap();
            //let position = self.positions.get((user, token_id));

            let liquidation_price = (spot_price.abs_diff(entry_price)).saturating_mul(self.leverage as u128);

            liquidation_price
        }
    }
}
