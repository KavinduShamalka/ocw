#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::offchain::{http,Duration,};

pub use pallet::*;

use sp_core::crypto::KeyTypeId;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
	type RuntimeAppPublic = Public;
	type GenericSignature = sp_core::sr25519::Signature;
	type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::pallet_prelude::{*, DispatchResult};
	use frame_system::pallet_prelude::{*, OriginFor};
	use scale_info::prelude::string::String;
	use frame_support::sp_io::offchain;
	use codec::alloc::string::ToString;
	use sp_std::vec::Vec;
	use sp_std::str;
	use frame_system::offchain::CreateSignedTransaction;
	use frame_system::offchain::AppCrypto;
	use frame_system::offchain::Signer;
	use frame_system::offchain::SendSignedTransaction;
	// const WORD_VEC_LEN: usize = 10;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	//Config
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {

		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	//Word Struct
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct Word {
		pub word: String
	}

	// #[pallet::storage]
	// #[pallet::getter(fn info)]
	// pub type WordSave<T> = StorageValue<_, String>;

	#[pallet::storage]
	#[pallet::getter(fn wordstore)]
	//pub type WordStore<T: Config> = StorageValue<_, VecDeque<String>, ValueQuery>;
	pub type WordStore<T: Config> = StorageValue<_, String, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn wordsave)]
	//pub type WordStore<T: Config> = StorageValue<_, VecDeque<String>, ValueQuery>;
	pub type WordSave<T: Config> = StorageMap<_,Twox64Concat ,T::AccountId, String>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		WordStored { word: String, signer: T::AccountId },
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
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		fn offchain_worker(block_number: BlockNumberFor<T>) {
			
			log::info!("Hello from ⛓️‍💥 offchain worker ⛓️‍💥.");
			log::info!("🌐⛓️ Current block: {:?} 🌐⛓️", block_number);

			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			match Self::fetch_word_and_send_signed() {
				Ok(()) => log::info!("Fetching word successfully....."),
				Err(_) => log::info!("Error fetching word"),

			};

			//get_metadata_uploaded
			signer.send_signed_transaction(|_account| {
				Call::get_word_uploaded {  }
			});
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		//Store word
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn save_word(origin: OriginFor<T>, word: String) -> DispatchResult {

			log::info!("Hello from word Save call func");

			let sender = ensure_signed(origin)?;

			log::info!("Signer id: {:?}",sender);

			// let new_word = Word {
			// 	word,
			// };

			// <WordSave<T>>::put(new_word);

			// Self::deposit_event(Event::WordStored { word: sender });

			
			Self::add_word(sender, word);

			Ok(())

		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn get_word_uploaded(origin: OriginFor<T>) -> DispatchResult {
			log::info!("👋🤓 Hello from get word data upload call.....");
	
			let signer = ensure_signed(origin)?;
	
			//get the file value store onstorage
			//let data_uploaded = <WordStore<T>>::get();

			let data_uploaded = <WordSave<T>>::get(signer);

			log::info!("WORD: {:?}", data_uploaded);
	
			// match data_uploaded {
			// 	Some(data) => {
			// 		log::info!("WORD: {:?}", data);
			// 	},
			// 	None => {
			// 		// File data is not available
			// 		log::info!("❌ Word Not Fetched");
			// 	},
			// }
	
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

			// Self::saved_words(result.clone());

			Ok(result)
		}

		/// A helper function to fetch the word and send signed transaction.
		pub fn fetch_word_and_send_signed() -> Result<(), &'static str> {
		
			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			let word = Self::fetch_word().map_err(|_| "Failed to fetch word")?;

			let results = signer.send_signed_transaction(|_account| {
				Call::save_word { word: word.clone() }
			});

			// let results = signer.send_signed_transaction(|_account| {
			// 	Call::save_word { word: word.clone() }
			// });

			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!("{:?} Word fetch success: {}.", acc.id, word),
					Err(e) => log::error!("{:?}: submit transaction failure. Reason: {:?}", acc.id, e),
				}
			}

			Ok(())

		}

		fn add_word(signer: T::AccountId, word: String) {
			log::info!("Inside adding word to onchain function");

			
			//add word into onchain
			// <WordStore<T>>::set(word.clone());
			<WordSave<T>>::set(signer.clone(), Some(word.clone()));


			// <WordStore<T>>::mutate(|words| {
			// 	words.push_str(&word);
			// });
			
			//call the event 
			Self::deposit_event(Event::WordStored { word: word.clone(), signer: signer });
		}

	}
}