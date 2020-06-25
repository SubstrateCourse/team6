#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, StorageMap,
					traits::{Get, Currency, ExistenceRequirement::AllowDeath},
};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


type BalanceOf<T> =<<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Currency :Currency<Self::AccountId>;
	type MaxClaimLength:Get<u32>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as poeMoudle {
		Prices get(fn price) :map hasher(blake2_128_concat) Vec<u8> =>BalanceOf<T>;
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber ,Vec<u8>);
		submitDocInfo get(fn get_info): map hasher(blake2_128_concat) T::AccountId => Vec<Vec<u8>>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> 
	where AccountId = <T as system::Trait>::AccountId, 
		  Balance = BalanceOf<T>, 
		  BlockNumber = <T as system::Trait>::BlockNumber{
		ClaimCreated(AccountId, Vec<u8>,BlockNumber),
		ClaimRevoked(AccountId, Vec<u8>),
		ProofTransferto(AccountId, Vec<u8>),
		ClaimForSale(AccountId, Vec<u8>, Balance),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		NoSuchProof,
		NotProofOwner,
		TooLongHash,//limit size for proof
		LowerThanPrice,
		AccountFreeBalanceNotEnough,
		NotForSale,
		NoTransaction,
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

		/// This fuction is for claim a proof 
		#[weight = 10_000]
		pub fn crate_claim(origin, proof: Vec<u8> ,note:Vec<u8>) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&proof),Error::<T>::ProofAlreadyExist);//ensure there are no same proof alreay exist
			ensure!(T::MaxClaimLength::get() >= proof.len() as u32, Error::<T>::TooLongHash);//limit size for proof 
			let current_block = <system::Module<T>>::block_number();
			Proofs::<T>::insert(&proof, (&sender,current_block.clone(),note.clone()));
			let mut info = Self::get_info(&sender);
			info.push(proof.clone());
			info.push(note.clone());
			submitDocInfo::<T>::insert(&sender,info.clone());
			Self::deposit_event(RawEvent::ClaimCreated(sender, proof,current_block));

			
			Ok(())
		}

		/// This function can revoke the proof you have claimed  
		#[weight = 10_000]
		pub fn revoke_claim(origin, proof:Vec<u8>) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);//ensure the proof already exist 
			let (owner, _,_) = Proofs::<T>::get(&proof);
			ensure!(sender == owner, Error::<T>::NotProofOwner);//ensure only the owner of proof can revoke itorigin
			Proofs::<T>::remove(&proof);

			let mut account = submitDocInfo::<T>::get(&sender);
			
			if let Ok(index) = account.binary_search(&proof) {
				account.remove(index);
			}
			submitDocInfo::<T>::insert(&sender,account);

			Self::deposit_event(RawEvent::ClaimRevoked(sender,proof)); 
			Ok(())
		}

		///This function can transfer one proof from one account to another
		#[weight = 10_000]
		pub fn transfer_claim(origin, proof:Vec<u8>, new_owner:T::AccountId,note:Vec<u8>) -> dispatch::DispatchResult{
			
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
			let (owner, _,_) = Proofs::<T>::get(&proof);
			ensure!(sender == owner, Error::<T>::NotProofOwner);
			Proofs::<T>::insert(&proof, (&new_owner,<system::Module<T>>::block_number(),note.clone()));//cover the old proof cliam
			Self::deposit_event(RawEvent::ProofTransferto(new_owner,proof));

			
			
			Ok(())
		}


		#[weight = 10_000]
		pub fn set_price(origin, claim: Vec<u8>, price: BalanceOf<T>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::NoSuchProof);

			let (owner, _block_number,_) = Proofs::<T>::get(&claim);
			ensure!(owner == sender, Error::<T>::NotProofOwner);

			Prices::<T>::insert(&claim, price);
			Self::deposit_event(RawEvent::ClaimForSale(sender, claim, price));

			Ok(())
		}

		#[weight = 0]
		pub fn buy_claim(origin, claim: Vec<u8>, amount: BalanceOf<T>,note:Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::NoSuchProof);
			ensure!(Prices::<T>::contains_key(&claim), Error::<T>::NotForSale);
			ensure!(amount >= Prices::<T>::get(&claim), Error::<T>::LowerThanPrice);

			let (owner, _block_number,_) = Proofs::<T>::get(&claim);
			T::Currency::transfer(&sender, &owner, amount, AllowDeath)
				.map_err(|_| Error::<T>::AccountFreeBalanceNotEnough)?;

			Proofs::<T>::insert(&claim, (sender, system::Module::<T>::block_number(),note.clone()));

			Ok(())
		}


	}
}