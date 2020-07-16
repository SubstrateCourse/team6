#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod erc20 {
    use ink_core::storage;
    //use ink_core::env::AccountId;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct Erc20 {
        total_supple: storage::Value<Balance>,
        balances: storage::HashMap<AccountId, Balance>,
        allowances: storage::HashMap<(AccountId, AccountId), Balance>
    }

    #[ink(event)]
    struct Transfer{
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        fn new(&mut self, initial_supple: Balance) {
            let caller = self.env().caller();
            self.total_supple.set(initial_supple);
            self.balances.insert(caller, initial_supple);
            self.env().emit_event(Transfer{
                from:None,
                to: Some(caller),
                value: initial_supple,
            });
        }
        
        #[ink(message)]
        fn total_supple(&self)->Balance {
            *self.total_supple
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId)-> Balance{
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        fn approve(&mut self, to: AccountId, value: Balance) -> bool {
            let owner = self.env().caller();
            self.allowances.insert((owner, to), value);
            self.env().emit_event(Approval{
                owner: owner,
                spender: to,
                value: value,
            });
            true
        }

        #[ink(message)]
        fn approval(&self, to: AccountId) -> Balance {
            let owner = self.env().caller();
            *self.allowances.get(&(owner, to)).unwrap_or(&0)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false
            }

            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(from, from_balance - value);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer{
                from: Some(from),
                to: Some(to),
                value,
            });
            true

        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> bool {
            let caller = self.env().caller();
            let allowance = self.allowance_of_or_zero(&from, &caller);
            if allowance < value {
                return false
            }
            self.allowances.insert((from, caller), allowance - value);
            self.transfer_from_to(from, to, value)
        }

        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> bool {
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            true
        }


        fn balance_of_or_zero(&self, owner:&AccountId) -> Balance{
            *self.balances.get(owner).unwrap_or(&0)
        }

        fn allowance_of_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            // ACTION: `get` the `allowances` of `(owner, spender)` and `unwrap_or` return `0`.
            *self.allowances.get(&(*owner, *spender)).unwrap_or(&0)
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test a simple use case of our contract.
        #[test]
        fn new_works() {
            let mut erc20 = Erc20::new(666);
            assert_eq!(erc20.total_supple(), 666);

            // let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
            //         .expect("Cannot get accounts");
            // assert_eq!(erc20.transfer_from(accounts.alice, accounts.eve, 10), false);
            // assert_eq!(erc20.approve(accounts.bob, 10), true);
            // assert_eq!(erc20.approval(accounts.bob), 10);
        }
    }
}
