#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{storage::{StorageMap}, decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, traits::Get};
use frame_support::traits::{Currency, ExistenceRequirement};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::traits::{StaticLookup};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Currency: Currency<Self::AccountId>;
    type MaxClaimLength: Get<u32>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

decl_storage! {
	trait Store for Module<T: Trait> as PoeModule {
        Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
        Prices get(fn price): map hasher(blake2_128_concat) Vec<u8> => BalanceOf<T>;
	}
}

decl_event!(
	pub enum Event<T> where 
        AccountId = <T as system::Trait>::AccountId,
        Balance = BalanceOf<T>,
    {
        ClaimCreated(AccountId, Vec<u8>, Balance),
        ClaimRevoked(AccountId, Vec<u8>),
        ClaimTransfered(AccountId, Vec<u8>),
        ClaimBuyed(AccountId, Vec<u8>, Balance),
        PriceSet(AccountId, Vec<u8>, Balance),
    }
);

decl_error! {
	pub enum Error for Module<T: Trait> {
        ProofAlreadyExist,
        ClaimNotExist,
        LengthTooLong,
        NotOwner,
        BuyOwnClaim,
        PriceIsZero,
        PriceTooLow,
    }
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

        #[weight = 100]
        pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            ensure!(claim.len() as u32 <= T::MaxClaimLength::get(), Error::<T>::LengthTooLong);

            Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));
            
            let price: BalanceOf<T> = 0.into();
            Prices::<T>::insert(&claim, &price);

            Self::deposit_event(RawEvent::ClaimCreated(sender, claim, price));

            Ok(())
        }

        #[weight = 100]
        pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (s, _) = Proofs::<T>::get(&claim);
            ensure!(s == sender, Error::<T>::NotOwner);

            Proofs::<T>::remove(&claim);
            Prices::<T>::remove(&claim);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

            Ok(())
        }

        #[weight = 100]
        pub fn transfer_claim(origin, claim: Vec<u8>, receiver: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (s, _) = Proofs::<T>::get(&claim);
            ensure!(s == sender, Error::<T>::NotOwner);

            let dest = T::Lookup::lookup(receiver)?;

            Proofs::<T>::insert(&claim, (dest, system::Module::<T>::block_number()));

            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

            Ok(())
        }

        #[weight = 100]
        pub fn set_price(origin, claim: Vec<u8>, price: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (s, _) = Proofs::<T>::get(&claim);
            ensure!(s == sender, Error::<T>::NotOwner);

            Prices::<T>::insert(&claim, &price);

            Self::deposit_event(RawEvent::PriceSet(sender, claim, price));

            Ok(())
        }

        #[weight = 100]
        pub fn buy_claim(origin, claim: Vec<u8>, in_price: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, _) = Proofs::<T>::get(&claim);
            ensure!(owner != sender, Error::<T>::BuyOwnClaim);

            let price = Prices::<T>::get(&claim);
            ensure!(in_price > price, Error::<T>::PriceTooLow);

            T::Currency::transfer(&sender, &owner, price, ExistenceRequirement::AllowDeath)?;

            Proofs::<T>::insert(&claim, (&sender, system::Module::<T>::block_number()));
            Prices::<T>::insert(&claim, &in_price);

            Self::deposit_event(RawEvent::ClaimBuyed(sender, claim, price));

            Ok(())
        }
	}
}
