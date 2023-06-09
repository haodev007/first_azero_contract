#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod mytoken {
    use ink::storage::Mapping;

    // storage definition
    #[ink(storage)]
    #[derive(Default)]
    pub struct MyToken {
        total_supply: u32,
        balances: Mapping<AccountId, u32>,
    }

    impl MyToken {
        // constructor definition
        /// Creates a token contract with the given initial supply belonging to the contract creator.
        #[ink(constructor)]
        pub fn new_token(supply: u32) -> Self {
            let caller = Self::env().caller();
            let mut instance = Self::default();

            instance.balances.insert(&caller, &supply);
            instance.total_supply = supply;
            instance
        }

        /// Total supply of the token.
        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
        }

        /// Current balance of chosen account.
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u32 {
            match self.balances.get(&account) {
                Some(value) => value,
                None => 0,
            }
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: u32) -> bool {
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);
            if sender_balance < value {
                false;
            }
            self.balances.insert(sender, &(sender_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mytoken::MyToken;
    use ink::env::{test, DefaultEnvironment};

    #[ink::test]
    fn total_supply_works() {
        let mytoken = MyToken::new_token(1000);
        assert_eq!(mytoken.total_supply(), 1000);
    }

    #[ink::test]
    fn balance_of_works() {
        let accounts = test::default_accounts::<DefaultEnvironment>();
        test::set_caller::<DefaultEnvironment>(accounts.alice);
        let mytoken = MyToken::new_token(1000);
        assert_eq!(mytoken.balance_of(accounts.alice), 1000);
        assert_eq!(mytoken.balance_of(accounts.bob), 0);
    }

    #[ink::test]
    fn transfer_works() {
        let accounts = test::default_accounts::<DefaultEnvironment>();
        test::set_caller::<DefaultEnvironment>(accounts.alice);
        let mut mytoken = MyToken::new_token(1000);
        assert_eq!(mytoken.balance_of(accounts.alice), 1000);
        assert_eq!(mytoken.balance_of(accounts.bob), 0);
        mytoken.transfer(accounts.bob, 100);
        assert_eq!(mytoken.balance_of(accounts.alice), 900);
        assert_eq!(mytoken.balance_of(accounts.bob), 100);
    }
}
