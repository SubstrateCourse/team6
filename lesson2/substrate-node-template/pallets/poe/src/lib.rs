#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch,ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(fn something): Option<u32>;

		Proofs get(fn proof): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId,T::BlockNumber);
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		SomethingStored(u32, AccountId),
		ClaimCreated(AccountId,Vec<u8>),
		ClaimRevoked(AccountId,Vec<u8>),
		ClaimTransfered(AccountId,Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,

		ProofAlreadyExist,

		ClaimNotExist,

		NotClaimOwner,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		/// Just a dummy entry point.
		/// function that can be called by the external world as an extrinsics call
		/// takes a parameter of the type `AccountId`, stores it, and emits an event
		#[weight = 10_000]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			// Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// Here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		/// Another dummy entry point.
		/// takes no parameters, attempts to increment storage value, and possibly throws an error
		#[weight = 10_000]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let _who = ensure_signed(origin)?;

			match Something::get() {
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					Something::put(new);
					Ok(())
				},
			}
		}

		//创建存证
		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);
			Proofs::<T>::insert(&claim,(sender.clone(),system::Module::<T>::block_number()));
			Self::deposit_event(RawEvent::ClaimCreated(sender,claim));
			Ok(())
		}

		//撤销存证
		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);
			let (owner,_block_number) = Proofs::<T>::get(&claim);
			ensure!(owner == sender,Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&claim);
			Self::deposit_event(RawEvent::ClaimRevoked(sender,claim));
			Ok(())
		}
		//转移存证
		#[weight = 0]
		pub fn transfer_claim(origin, claim: Vec<u8>,new_owner: T::AccountId) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);
			let (owner,_block_number) = Proofs::<T>::get(&claim);
			ensure!(owner == sender,Error::<T>::NotClaimOwner);
			Proofs::<T>::insert(&claim,(new_owner.clone(),system::Module::<T>::block_number()));
			Self::deposit_event(RawEvent::ClaimTransfered(sender,claim));
			Ok(())
		}

	}
}
