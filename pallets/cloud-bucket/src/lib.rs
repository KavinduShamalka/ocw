#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::offchain::{http,Duration,};

use frame_support::{dispatch::GetDispatchInfo, traits::UnfilteredDispatchable};

use sp_core::crypto::KeyTypeId;
// use lite_json::json;
// use lite_json::json::JsonValue;

// extern crate std;
// ...

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

// ...

pub mod crypto {
	use super::KEY_TYPE;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519}, MultiSignature, MultiSigner
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

		// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// use core::f64::consts::E;

use super::*;
	use frame_support::pallet_prelude::{*, DispatchResult};
	use frame_system::ensure_signed;
	use frame_system::pallet_prelude::{*, OriginFor};
	use frame_system::offchain::{ 
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer,
	};
	use scale_info::prelude::string::String;
	use frame_support::sp_io::offchain;
	use codec::alloc::string::ToString;
	// use sp_runtime::offchain::http::Response;
	use sp_std::vec::Vec;
	use scale_info::prelude::vec;
	// use sp_std::{collections::vec_deque::VecDeque, str};
	use sp_std::str;
	// const WORD_VEC_LEN: usize = 10;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	//Config
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {

		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		type RuntimeCall: Parameter + UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin> + GetDispatchInfo;
	}

	//Word Struct
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct Word {
		pub word: String
	}

	//Bucket name Struct
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct Bucket {
		pub name: String
	}

	#[pallet::storage]
	#[pallet::getter(fn info)]
	pub type WordSave<T> = StorageValue<_, Word>;

	#[pallet::storage]
	#[pallet::getter(fn store)]
	pub type WordStore<T> = StorageValue<_, Word>;

	#[pallet::storage]
	#[pallet::getter(fn bucket)]
	pub type BucketNameSave<T> = StorageValue<_, Bucket>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		WordStored { word: T::AccountId },
		BucketCreated { name: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
	
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		/// Error returned when fetching github info
		HttpFetchingError,

		UnknownOffchainMux,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		fn offchain_worker(block_number: BlockNumberFor<T>) {
			
			log::info!("Hello from ⛓️‍💥 offchain worker ⛓️‍💥.");
			log::info!("🌐⛓️ Current block: {:?} 🌐⛓️", block_number.clone());

			match Self::fetch_word_and_send_signed() {
				Ok(result) => log::info!("Word: {}", result),
				Err(error) => log::info!("Error fetching word: {}", error),
			}

			match Self::_bucket_creation() {
				Ok(_) => log::info!("Bucket created"),
				Err(error) => log::info!("Error: {:#?}", error)
			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		//Store word
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn save_word(origin: OriginFor<T>, word: String) -> DispatchResult {

			let sender = ensure_signed(origin)?;

			let new_word = Word {
				word,
			};

			<WordSave<T>>::put(new_word);

			Self::deposit_event(Event::WordStored { word: sender });

			log::info!("Hello from word Save.");

			Ok(())

		}

		//Create bucket
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_bucket(origin: OriginFor<T>) -> DispatchResult {

			log::info!("Hello from word bucket_creation. 1");

			let sender = ensure_signed(origin)?;

			// let new_bucket = Bucket {
			// 	name: bucket_name.clone(),
			// };

			// <BucketNameSave<T>>::put(new_bucket);

			Self::deposit_event(Event::BucketCreated { name : sender.clone() });

			// let _ = Self::_bucket_creation();

			Ok(())

		}
	}

	impl<T: Config> Pallet<T> {

		//Fetch word from the api
		fn fetch_word() -> Result<String, http::Error> {

			//set deadline
			let deadline = offchain::timestamp().add(Duration::from_millis(2_000));

			//set get request
			let request = http::Request::get("https://random-word-api.herokuapp.com/word");

			let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;

			let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
			
			//check response is successfull
			if response.code != 200 {
				log::warn!("Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown)
			}
			
			let body = response.body().collect::<Vec<u8>>();

			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
					log::warn!("No UTF8 body");
					http::Error::Unknown
			})?;

			let result = body_str.to_string();

			Ok(result)
		}


		fn fetch_word_and_send_signed() -> Result<String, &'static str> {

			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			if !signer.can_sign() {
				return Err(
					"No local accounts available. Consider adding one via `author_insertKey` RPC.",
				)
			}

			let word = Self::fetch_word().map_err(|_| "Failed to fetch word")?;

			let results = signer.send_signed_transaction(|_account| {

				Call::save_word { word: word.clone() }
			});

			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!("[{:?}] Submitted word of {}", acc.id, word.clone()),
					Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
				}
			}

			Ok(word)
		}


		//Create bucket
		fn _bucket_creation() -> Result<(), http::Error> {

			//set deadline
			let deadline = offchain::timestamp().add(Duration::from_millis(2_000));

			// let name = bucket_name.clone();

			let json_body = r#" {
									"name": "dockset-test-1"
								}
							"#;
			
			// Send a POST request
			let request = http::Request::post("https://storage.googleapis.com/storage/v1/b?project=intern-storage-apis", vec![json_body])
			.add_header("Authorization", "Bearer ya29.c.c0AY_VpZgxwdU-4f2nWkmzFB4K1bkuixXLqLwI6FZTEWMh7Eh5mOtEZ9PrQHbgHbxjQv1bOIuOJOCUQQLmIN6iIRrA8jZORope8Qs1u80Gm-UmG0CaMevIkWFNmoMuYTF3xOFiR0XFRFMm7DeGKo8C6CzEPw-HAlmU0c0MsNduHQBSjJ0Tx27n5ghoj6KYloItpmBjLvMVSHaTnubUqCE0XURjA65nk3ug8vuF81f85G-dInbiXOSn5SJAqhIUXRMB2mFraZUBJXYFwKhPUE8q1aT7a-uC5qTevQ3AoOxgK4qOTBW16A7UJbK9EHJSJXokBqY7ltMdzEbeE4DPlWoVdIpYkeQWtiJUckUC0fvjxRXGYuiQQeQxOYb6D-eiA8tNixGXKhUH400Az-VXYMjgMkqkR259uewn1o_rcVwFum-rv5h0FgX82g-y-WQZyWxyZyhBwYR0tgOoMbQxe8UqikXzYr1ajYQlQ7ewhe6mR7iQZsdg-VyefWjv3ivMixdJvctuhYF2VMUBRfx1fRsyuQtjysvRU62ikRBXcBd453i19vmjrU64_4Wyzrgkl3V147bU7pWWj0QBfqjyIkeUUw5hJQUORO66aRwbrxBW3Rpc4dtyb2e3r4dRi4uQJYY_WFrg6BhSY0Mxtu0zYSl2vOMf5Qp0Ih_3Rhar83Zxbtkj7oUnzRIjM57ixBse3tkjchlYxw1R0SadjpZgVririwRB0YYYJtcB__lmFSIWwwibpZfolZo2y1m_qyiVvXfs5_khzmx0lR1_7kchWa5z1YyQUfybdtJ5Qfwyzi181u0jFiegFcXgegfvt18iIi8VQ1qyhcMSBWgYWmOVgFMB32szZBUvuq5QdU7ai3tXBSktOeJeMx6o5xBIhMOlXi2FzxoQ2u1O9U3SfmM1747B7ctUr228t38-7isSlV87ZBaJc6B9ZXgsqO-yc5YVSBlhgMpxpbWgRk2zIeBZ7vt2tQkRjV6YRS8F7eJ0XtyfiBUYUwjy_-O26sZ")
			.add_header("Content-Type", "application/json")
			.add_header("Accept", "application/json");

			let pending = request
			.deadline(deadline)
			.body(vec![json_body])
			.send()
			.map_err(|_| http::Error::IoError)?;

			// Wait for response 
			let _response = pending
			.try_wait(deadline)
			.map_err(|_| http::Error::DeadlineReached)??;

			//check response is successfull
			if _response.code != 200 {
				log::warn!("Unexpected status code: {}", _response.code);
				return Err(http::Error::Unknown)
			}

			// let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;

			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			signer.send_signed_transaction(|_account| {

				Call::create_bucket {  }

			});

			// for (acc, res) in &results {
			// 	match res {
			// 		Ok(()) => log::info!("[{:?}] Submitted bucket", acc.id),
			// 		Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
			// 	}
			// }

			Ok(())
		}

		// fn create_folder()
	}
}
