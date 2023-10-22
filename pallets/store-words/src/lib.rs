#![cfg_attr(not(feature = "std"), no_std)]

// use sp_runtime::offchain::{http,Duration,};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::pallet_prelude::{*, DispatchResult};
	use frame_system::pallet_prelude::{*, OriginFor};
	use scale_info::prelude::string::String;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	//Config
	#[pallet::config]
	pub trait Config: frame_system::Config {

		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

	}

	//Word Struct
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct Word {
		pub word: String
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type WordSave<T> = StorageValue<_, Word>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		WordStored { word: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		// Error returned when fetching github info
		HttpFetchingError,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		fn offchain_worker(block_number: BlockNumberFor<T>) {
			
			log::info!("Hello from ⛓️‍💥 offchain worker ⛓️‍💥.");
			log::info!("🌐⛓️ Current block: {:?} 🌐⛓️", block_number);

			// match Self::fetch_word() {
			// 	Ok(word) => log::info!("Word: {}", word),
			// 	Err(e) => log::info!("Error: {:?}", e) 
			// };

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
	}

	// impl<T: Config> Pallet<T> {

	// 	//Fetch word from the api
	// 	fn fetch_word() -> Result<String, http::Error> {

	// 		let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));

	// 		let request = http::Request::get("https://random-word-api.herokuapp.com/word");

	// 		let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;

	// 		let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
			
	// 		if response.code != 200 {
	// 			log::warn!("Unexpected status code: {}", response.code);
	// 			return Err(http::Error::Unknown)
	// 		}
			
	// 		let body = response.body().collect::<Vec<u8>>();

	// 		let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
	// 				log::warn!("No UTF8 body");
	// 				http::Error::Unknown
	// 		})?;

	// 		log::warn!("Word: {}", body_str);

	// 		Ok(body_str.to_string())
	// 	}
	// }
}
