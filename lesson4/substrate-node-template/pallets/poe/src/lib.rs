#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{storage::{StorageMap}, decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, traits::Get};
use frame_support::traits::{Currency, ExistenceRequirement};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::traits::{StaticLookup};
use pallet_timestamp as timestamp;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait + timestamp::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Currency: Currency<Self::AccountId>;
    type MaxClaimLength: Get<u32>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

decl_storage! {
	trait Store for Module<T: Trait> as PoeModule {
        Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber, T::Moment, Option<Vec<u8>>);
        Prices get(fn price): map hasher(blake2_128_concat) Vec<u8> => BalanceOf<T>;
        Owners get(fn owners): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber, T::Moment, Option<Vec<u8>>);
	}
}

decl_event!(
	pub enum Event<T> where 
        AccountId = <T as system::Trait>::AccountId,
        Balance = BalanceOf<T>,
        Moment = <T as timestamp::Trait>::Moment,
    {
        ClaimCreated(AccountId, Vec<u8>, Balance, Moment, Option<Vec<u8>>),
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
        pub fn create_claim(origin, claim: Vec<u8>, note: Option<Vec<u8>>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            ensure!(claim.len() as u32 <= T::MaxClaimLength::get(), Error::<T>::LengthTooLong);

            let time_stamp = <timestamp::Module<T>>::get();
            Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number(), &time_stamp, &note));
            Owners::<T>::insert(sender.clone(), &claim, (sender.clone(), system::Module::<T>::block_number(), &time_stamp, &note));

            let price: BalanceOf<T> = 0.into();
            Prices::<T>::insert(&claim, &price);
            
            Self::deposit_event(RawEvent::ClaimCreated(sender, claim, price, time_stamp, note));

            Ok(())
        }

        #[weight = 100]
        pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (s, _, _, _) = Proofs::<T>::get(&claim);
            ensure!(s == sender, Error::<T>::NotOwner);

            Proofs::<T>::remove(&claim);
            Owners::<T>::remove(sender.clone(), &claim);
            Prices::<T>::remove(&claim);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

            Ok(())
        }

        #[weight = 100]
        pub fn transfer_claim(origin, claim: Vec<u8>, receiver: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (s, _, time_stamp, note) = Proofs::<T>::get(&claim);
            ensure!(s == sender, Error::<T>::NotOwner);

            let dest = T::Lookup::lookup(receiver)?;

            Proofs::<T>::insert(&claim, (dest.clone(), system::Module::<T>::block_number(), time_stamp, &note));
            Owners::<T>::remove(sender.clone(), &claim);
            Owners::<T>::insert(dest.clone(), &claim, (dest.clone(), system::Module::<T>::block_number(), &time_stamp, &note));

            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

            Ok(())
        }

        #[weight = 100]
        pub fn set_price(origin, claim: Vec<u8>, price: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (s, _, _, _) = Proofs::<T>::get(&claim);
            ensure!(s == sender, Error::<T>::NotOwner);

            Prices::<T>::insert(&claim, &price);

            Self::deposit_event(RawEvent::PriceSet(sender, claim, price));

            Ok(())
        }

        #[weight = 100]
        pub fn buy_claim(origin, claim: Vec<u8>, in_price: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, _, time_stamp, note) = Proofs::<T>::get(&claim);
            ensure!(owner != sender, Error::<T>::BuyOwnClaim);

            let price = Prices::<T>::get(&claim);
            ensure!(in_price > price, Error::<T>::PriceTooLow);

            T::Currency::transfer(&sender, &owner, price, ExistenceRequirement::AllowDeath)?;

            Proofs::<T>::insert(&claim, (&sender, system::Module::<T>::block_number(), time_stamp, &note));
            Owners::<T>::remove(owner.clone(), &claim);
            Owners::<T>::insert(sender.clone(), &claim, (sender.clone(), system::Module::<T>::block_number(), &time_stamp, &note));
            Prices::<T>::insert(&claim, &in_price);

            Self::deposit_event(RawEvent::ClaimBuyed(sender, claim, price));

            Ok(())
        }
	}
}
