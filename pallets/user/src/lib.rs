#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, OptionQuery, DispatchResult}, Blake2_128Concat};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct UserInfo {
		/// User name
		pub username: Vec<u8>,
		/// Number of id of user
		pub id: i64,
		//Aboutme
		pub about_me: Vec<u8>
	}

	//use storsge macro
	#[pallet::storage]
	#[pallet::getter(fn info)]
	pub type AccountToUserInfo<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, UserInfo, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		UserCreated { user: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	//Handles the business logic
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_user(origin: OriginFor<T>, username: Vec<u8>, id: i64, about_me: Vec<u8>) -> DispatchResult {
			//Gets the caller of the function
			let sender = ensure_signed(origin)?;

			let new_user = UserInfo {
				username,
				id,
				about_me,
			};

			<AccountToUserInfo<T>>::insert(&sender, new_user);

			Self::deposit_event(Event::<T>::UserCreated { user: sender });

			log::info!("Hello from user register.");

			Ok(())
		}
	}
	
}
