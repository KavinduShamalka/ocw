#![cfg_attr(not(feature = "std"), no_std)]

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
	pub struct UserInfo{
		///username stored as an array of bytes
		pub username: Vec<u8>,
		///Number id of the user
		pub id: i64,
		///The "About me " section of the user
		pub about_me: Vec<u8>
	}

	#[pallet::storage]
	#[pallet::getter(fn info)]
	pub type AccountToUserInfo<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, UserInfo, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		UserCreated { user: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}
 
	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		//Dispachable calls goes here
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_user(
			origin: OriginFor<T>, //information about who call this function
			username: Vec<u8>, 
			id: i64, 
			about_me: Vec<u8>
		) -> DispatchResult {
			//Gets thw caller of the function
			let sender = ensure_signed(origin)?;
			//Define new_user
			let new_user = UserInfo { username, id, about_me};
			//Change the state of our storage mapping by adding user info to our sender AccountId.
			<AccountToUserInfo<T>>::insert(&sender, new_user);
			//Emit an event indicating the user is now created and registered
			Self::deposit_event(Event::<T>::UserCreated { user: sender });

			Ok(())
		}
		
	}
}
