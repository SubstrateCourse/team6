#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod erc20 {
    use ink_core::storage;

    #[ink(storage)]
    struct Erc20 {
        total_supply: storage::Value<Balance>,
        balances: storage::HashMap<AccountId, Balance>,
        allowance: storage::HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Option<Balance>,
    }

    #[ink(event)]
    struct Allowance {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Option<Balance>,
    }

    impl Erc20 {
        #[ink(constructor)]
        fn new(&mut self, initial_supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(initial_supply);
            self.balances.insert(caller, initial_supply);
            self.env().emit_event(Transfer {
                from: None,
                to: caller,
                value: initial_supply,
            });
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        fn balance_of_or_zero(&self, owner: AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller;
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
        fn allowance_quota(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller;
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false
            }
            self.allowance.insert((from, to), value);
            self.env().emit_event(Allowance {
                from: Some(from),
                to: Some(to),
                value: Some(value),
            });
            ture
        }

        #[ink(message)]
        fn allowance_check(&mut self, to: AccountId) -> Balance {
            let from = self.env().caller;
            *self.allowance.get((from, to)).unwrap_or(&0)
        }

        #[ink(message)]
        fn transfer_from(&mut self, Investor: AccountId, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller;     

            //金主允许from花多少钱
            let allowance_balance = self.allowance_check(&Investor);
            if allowance_balance < value {
                return false
            }

            //金主目前有多少钱
            let Investor_balance = self.balance_of_or_zero(&Investor);
            if Investor_balance < value {
                return false
            }

            //付款
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(Investor, Investor_balance - value);
            self.balances.insert(to, to_balance + value);

            //额度减少
            self.allowance.insert((Investor, from), allowance_balance - value);
            
            true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_works() {
            let erc20 = Erc20::new(666);
            assert_eq!(erc20.total_supply(), 666);
        }
    }
}