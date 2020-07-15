#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use core::{fmt};
use frame_support::{
	debug, decl_module, decl_storage, decl_event, decl_error,
};
use frame_system::{
	self as system,
	offchain::{
		AppCrypto, CreateSignedTransaction,
	},
};
use sp_core::crypto::KeyTypeId;
use sp_std::prelude::*;
use sp_std::str;
use codec::{Decode, Encode};
use sp_runtime::{
	offchain as rt_offchain,
	offchain::storage::StorageValueRef, RuntimeDebug,
};

// We use `alt_serde`, and Xanewok-modified `serde_json` so that we can compile the program
//   with serde(features `std`) and alt_serde(features `no_std`).
use alt_serde::{Deserialize, Deserializer};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"0987");
pub const ETH_PRICE_API1: &str = "https://api.coincap.io/v2/assets/ethereum";
pub const ETH_PRICE_API2: &str = "https://min-api.cryptocompare.com/data/pricemulti?fsyms=ETH&tsyms=USD";

#[allow(non_snake_case)]
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default, RuntimeDebug)]
struct EthPrice2 { // {"ETH":{"USD":236.5}}
	ETH: EthPriceData2,
}
#[allow(non_snake_case)]
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct EthPriceData2 {
	#[serde(deserialize_with = "de_float_to_bytes")]
	USD: Vec<u8>,
}
pub fn de_float_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error> where D: Deserializer<'de> {
	let s: f64 = Deserialize::deserialize(de)?;
	Ok(alt_serde::export::ToString::to_string(&s).as_bytes().to_vec())
}
impl fmt::Debug for EthPriceData2 {
	// `fmt` converts the vector of bytes inside the struct back to string for
	//   more friendly display.
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{{ USD: {} }}", str::from_utf8(&self.USD).map_err(|_| fmt::Error)?, )
	}
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default, RuntimeDebug)]
struct EthPrice {
	data: EthPriceData,
	timestamp: u64,
}

#[serde(crate = "alt_serde", rename_all = "camelCase")]
#[derive(Deserialize, Encode, Decode, Default)]
struct EthPriceData { /* { "data": { "id": "ethereum",
							"rank": "2",
							"symbol": "ETH",
							"name": "Ethereum",
							"supply": "111654972.9990000000000000",
							"maxSupply": null,
							"marketCapUsd": "26466495488.5116816562184791",
							"volumeUsd24Hr": "2371766485.3484418957654448",
							"priceUsd": "237.0382149369085412",
							"changePercent24Hr": "1.7676464681359239",
							"vwap24Hr": "237.6740946608588550"
						}, "timestamp": 1594105650525 } */
	// Specify our own deserializing function to convert JSON string to vector of bytes
	#[serde(deserialize_with = "de_string_to_bytes")]
	id: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	symbol: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	name: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	price_usd: Vec<u8>,
}

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error> where D: Deserializer<'de> {
	let s: &str = Deserialize::deserialize(de)?;
	Ok(s.as_bytes().to_vec())
}
impl fmt::Debug for EthPriceData {
	// `fmt` converts the vector of bytes inside the struct back to string for
	//   more friendly display.
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{{ id: {}, symbol: {}, name: {}, priceUsd: {} }}",
			str::from_utf8(&self.id).map_err(|_| fmt::Error)?,
			str::from_utf8(&self.symbol).map_err(|_| fmt::Error)?,
			str::from_utf8(&self.name).map_err(|_| fmt::Error)?,
			str::from_utf8(&self.price_usd).map_err(|_| fmt::Error)?,
		)
	}
}

/// The pallet's configuration trait.
//noinspection ALL
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
	// Add other types and constants required to configure this pallet.
	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	//noinspection ALL
	/// The overarching dispatch call type.
	type Call: From<Call<Self>>;
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

pub mod crypto {
	use crate::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct AuthId;
	// implemented for ocw-runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for AuthId {
		//noinspection ALL
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for AuthId {
		//noinspection ALL
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		NewNumber(Option<AccountId>, u64),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		// Error returned when making signed transactions in off-chain worker
		SignedSubmitNumberError,
		// Error returned when making unsigned transactions in off-chain worker
		UnsignedSubmitNumberError,
		// Error returned when making remote http fetching
		HttpFetchingError,
		// Error returned when gh-info has already been fetched
		AlreadyFetched,
		DeserializeError,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");
			let result = Self::fetch(false);
			if let Err(e) = result {
				debug::error!("Error: {:?}", e);
			}
		}
	}
}

impl<T: Trait> Module<T> {
	/// Check if we have fetched github info before. If yes, we use the cached version that is
	///   stored in off-chain worker storage `storage`. If no, we fetch the remote info and then
	///   write the info into the storage for future retrieval.
	fn fetch(is_test: bool) -> Result<(), Error<T>> {
		// Start off by creating a reference to Local Storage value.
		// Since the local storage is common for all offchain workers, it's a good practice
		// to prepend our entry with the pallet name.
		let s_info = StorageValueRef::persistent(b"offchain-demo::prices");
		let s_lock = StorageValueRef::persistent(b"offchain-demo::lock");

		// We are implementing a mutex lock here with `s_lock`
		let res: Result<Result<bool, bool>, Error<T>> = s_lock.mutate(|s: Option<Option<bool>>| {
			match s {
				// `s` can be one of the following:
				//   `None`: the lock has never been set. Treated as the lock is free
				//   `Some(None)`: unexpected case, treated it as AlreadyFetch
				//   `Some(Some(false))`: the lock is free
				//   `Some(Some(true))`: the lock is held

				// If the lock has never been set or is free (false), return true to execute `fetch_n_parse`
				None | Some(Some(false)) => Ok(true),

				// Otherwise, someone already hold the lock (true), we want to skip `fetch_n_parse`.
				// Covering cases: `Some(None)` and `Some(Some(true))`
				_ => Err(<Error<T>>::AlreadyFetched),
			}
		});

		// Cases of `res` returned result:
		//   `Err(<Error<T>>)` - lock is held, so we want to skip `fetch_n_parse` function.
		//   `Ok(Err(true))` - Another ocw is writing to the storage while we set it,
		//                     we also skip `fetch_n_parse` in this case.
		//   `Ok(Ok(true))` - successfully acquire the lock, so we run `fetch_n_parse`
		if let Ok(Ok(true)) = res {
			match Self::fetch_n_parse(is_test) {
				Ok((gh_info1, gh_info2)) => {
					let price1 = str::from_utf8(&gh_info2.ETH.USD).map_err(|_| <Error<T>>::HttpFetchingError)?;
					let price1 = price1.parse::<f64>().map_err(|_| <Error<T>>::HttpFetchingError)?;

					let price2 = str::from_utf8(&gh_info1.data.price_usd).map_err(|_| <Error<T>>::HttpFetchingError)?;
					let price2 = price2.parse::<f64>().map_err(|_| <Error<T>>::HttpFetchingError)?;

					let price = ((price1 + price2) * 1_000_000.0 / 2.0) as i64;

					if let Some(Some(mut prices)) = s_info.get::<Vec<i64>>() {
						prices.push(price);
						s_info.set(&prices);
						for p in prices {

							#[cfg(test)]
							println!("***** cached price: {:?}", (p as f64) / 1_000_000.0);

							#[cfg(not(test))]
							debug::info!("***** cached price: {:?}", (p as f64) / 1_000_000.0);
						}
					} else {
						s_info.set(&vec![price]);
					}
					s_lock.set(&false);
				}
				Err(err) => {
					// release the lock
					s_lock.set(&false);
					return Err(err);
				}
			}
		}
		Ok(())
	}

	/// Fetch from remote and deserialize the JSON to a struct
	fn fetch_n_parse(is_test: bool) -> Result<(EthPrice, EthPrice2), Error<T>> {
		let (resp_bytes1, resp_bytes2) = if !is_test {
			Self::fetch_from_remote().map_err(|e| {
				debug::error!("fetch_from_remote error: {:?}", e);
				<Error<T>>::HttpFetchingError
			})?
		} else {
			// just for testing ...
			let resp_bytes1 = b"{ \"data\": { \"id\": \"ethereum\",
							\"symbol\": \"ETH\",
							\"name\": \"Ethereum\",
							\"priceUsd\": \"237.0382149369085412\"
						}, \"timestamp\": 1594105650525 }";
			let resp_bytes2 = b"{\"ETH\":{\"USD\":236.5}}";
			(resp_bytes1.to_vec(), resp_bytes2.to_vec())
		};

		let resp_str1 = str::from_utf8(&resp_bytes1).map_err(|_| <Error<T>>::HttpFetchingError)?;
		// Print out our fetched JSON string
		debug::info!("resp1: {}", resp_str1);

		let resp_str2 = str::from_utf8(&resp_bytes2).map_err(|_| <Error<T>>::HttpFetchingError)?;
		// Print out our fetched JSON string
		debug::info!("resp2: {}", resp_str2);

		// Deserializing JSON to struct, thanks to `serde` and `serde_derive`
		let gh_info1: EthPrice =
			serde_json::from_str(&resp_str1).map_err(|e| {
				debug::info!("deserializeError1: {}", e);
				<Error<T>>::DeserializeError
			})?;
		let gh_info2: EthPrice2 =
			serde_json::from_str(&resp_str2).map_err(|e| {
				debug::info!("deserializeError2: {}", e);
				<Error<T>>::DeserializeError
			})?;
		Ok((gh_info1, gh_info2))
	}

	/// This function uses the `offchain::http` API to query the remote github information,
	///   and returns the JSON response as vector of bytes.
	fn fetch_from_remote() -> Result<(Vec<u8>, Vec<u8>), Error<T>> {
		let request1 = rt_offchain::http::Request::get(ETH_PRICE_API1);
		let request2 = rt_offchain::http::Request::get(ETH_PRICE_API2);

		// Keeping the offchain worker execution time reasonable, so limiting the call to be within 20s.
		let timeout = sp_io::offchain::timestamp().add(rt_offchain::Duration::from_millis(20_000));

		// For github API request, we also need to specify `user-agent` in http request header.
		//   See: https://developer.github.com/v3/#user-agent-required
		let pending1 = request1
			.deadline(timeout) // Setting the timeout time
			.send() // Sending the request out by the host
			.map_err(|_| <Error<T>>::HttpFetchingError)?;

		let pending2 = request2
			.deadline(timeout) // Setting the timeout time
			.send() // Sending the request out by the host
			.map_err(|_| <Error<T>>::HttpFetchingError)?;

		// By default, the http request is async from the runtime perspective. So we are asking the
		//   runtime to wait here.
		// The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
		//   ref: https://substrate.dev/rustdocs/v2.0.0-rc3/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
		let response1 = pending1
			.try_wait(timeout)
			.map_err(|_| <Error<T>>::HttpFetchingError)?
			.map_err(|_| <Error<T>>::HttpFetchingError)?;
		let response2 = pending2
			.try_wait(timeout)
			.map_err(|_| <Error<T>>::HttpFetchingError)?
			.map_err(|_| <Error<T>>::HttpFetchingError)?;

		if response1.code != 200 {
			debug::error!("Unexpected http request status code: {}", response1.code);
			return Err(<Error<T>>::HttpFetchingError);
		}
		if response2.code != 200 {
			debug::error!("Unexpected http request status code: {}", response2.code);
			return Err(<Error<T>>::HttpFetchingError);
		}

		// Next we fully read the response body and collect it to a vector of bytes.
		Ok((response1.body().collect::<Vec<u8>>(), response2.body().collect::<Vec<u8>>()))
	}
}
