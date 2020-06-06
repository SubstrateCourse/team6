#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet poe with necessary imports


use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
// use sp_std::vec::Vec;
// use frame_support::inherent::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.  约束
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items. 存储单元
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
    Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8>  => (T::AccountId, T::BlockNumber);
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
    ClaimCreated(AccountId, Vec<u8>),
    ClaimRevoked(AccountId, Vec<u8>),
    ClaimTransfered(AccountId, AccountId, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
    ClaimOverflow,
    ProofAlreadyExist,
    ProofNotExist,
    NotProofOwner,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

      ensure!(claim.len() <= 128, Error::<T>::ClaimOverflow);
      ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

      Proofs::<T>::insert(&claim, (who.clone(), system::Module::<T>::block_number()));

			Self::deposit_event(RawEvent::ClaimCreated(who, claim));
			Ok(())
		}

		#[weight = 10_000]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
      
      ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ProofNotExist);

      let(owner, _) = Proofs::<T>::get(&claim);
      ensure!(who == owner, Error::<T>::NotProofOwner);

      Proofs::<T>::remove(&claim);

      Self::deposit_event(RawEvent::ClaimRevoked(who, claim));
      Ok(())
    }
    
    
		#[weight = 10_000]
		pub fn transfer_claim(origin, claim: Vec<u8>, account: T::AccountId) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
      
      ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ProofNotExist);

      let(owner, _) = Proofs::<T>::get(&claim);
      ensure!(who == owner, Error::<T>::NotProofOwner);

      Proofs::<T>::insert(&claim, (account.clone(), system::Module::<T>::block_number()));

      Self::deposit_event(RawEvent::ClaimTransfered(who, account, claim));
      Ok(())
    }

	}
}
