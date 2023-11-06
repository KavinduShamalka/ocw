#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::offchain::{http,Duration,};
use sp_runtime::offchain::http::Method;

use frame_support::{dispatch::GetDispatchInfo, traits::UnfilteredDispatchable};

use sp_core::crypto::KeyTypeId;


// use frame_support::storage_root as root;



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
	use scale_info::prelude::format;
	// const WORD_VEC_LEN: usize = 10;

	// #[cfg(feature = "std")]
	// use std::path::Path;

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

	//File struct
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo, Debug)]
	pub struct File {
		pub file: Vec<u8>,
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

	#[pallet::storage]
	#[pallet::getter(fn file)]
	pub type FileSave<T> = StorageValue<_, Vec<u8>>;

	#[pallet::storage]
	#[pallet::getter(fn delete)]
	pub type ObjectDelete<T> = StorageValue<_, String>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		WordStored { word: T::AccountId },
		BucketCreated { name: T::AccountId },
		FolderCreated { folder: T::AccountId },
		FileFetched { file: T::AccountId },
		FileDeleted { file_delete: T::AccountId },
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

			// match Self::_fetch_word_and_send_signed() {
			// 	Ok(result) => log::info!("Word: {}", result),
			// 	Err(error) => log::info!("Error fetching word: {}", error),
			// }

			// match Self::_bucket_creation() {
			// 	Ok(_) => log::info!("Bucket created"),
			// 	Err(error) => log::info!("Error: {:#?}", error)
			// }

			// match Self::_folder_creation() {
			// 	Ok(_) => log::info!("Folder created"),
			// 	Err(error) => log::info!("Error: {:#?}", error)
			// }

			// match Self::_file_upload() {
			// 	Ok(_) => log::info!("✅️ ✅️ ✅️ File uploaded ✅️ ✅️ ✅️"),
			// 	Err(error) => log::info!(" ❌ ❌ ❌ Error file uploading ===> : {:#?} ❌ ❌ ❌", error)
			// }

			match Self::_delete_object() {
				Ok(code) => log::info!("✅️ ✅️ ✅️ Object deleted succesfully : {} ✅️ ✅️ ✅️", code),
				Err(error) => log::info!(" ❌ ❌ ❌ Error deleting object : {:#?} ❌ ❌ ❌", error)
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


		//Create bucket
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_folder(origin: OriginFor<T>) -> DispatchResult {
		
			log::info!("Hello from word folder_creation. 1");
		
			let sender = ensure_signed(origin)?;
		
			Self::deposit_event(Event::FolderCreated { folder : sender.clone() });
		
			Ok(())
		
		}

		//Upload file
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn upload_file(origin: OriginFor<T>, _file: Vec<u8>) -> DispatchResult {

			log::info!("Hello from word upload_file. 1");

			let sender = ensure_signed(origin)?;

			// let file: File = File {
			// 	file: _file.clone(),
			// };

			// log::info!("Fule {:#?}", file.clone());

			<FileSave<T>>::put(_file);

			Self::deposit_event(Event::FileFetched { file: sender });

			// match Self::_file_upload(_file) {
			// 	Ok(_) => log::info!("file uploaded."),
			// 	Err(error) => log::info!("File not fetched: {:#?}.", error),
			// }

			Ok(())
		}

		//Delete file
		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn delete_object(origin: OriginFor<T>, object_name: String) -> DispatchResult {
		
			let sender = ensure_signed(origin)?;

			<ObjectDelete<T>>::put(object_name);
		
			Self::deposit_event(Event::FileDeleted { file_delete: sender });

			log::info!(" ✅️ ✅️ 👋 👋 🗂🗑 Hello from delete file 🗑🗂 👋 👋 ✅️ ✅️");
		
			Ok(())
		
		}

	}

	impl<T: Config> Pallet<T> {

		//Fetch word from the api
		fn _fetch_word() -> Result<String, http::Error> {

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


		fn _fetch_word_and_send_signed() -> Result<String, &'static str> {

			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			if !signer.can_sign() {
				return Err(
					"No local accounts available. Consider adding one via `author_insertKey` RPC.",
				)
			}

			let word = Self::_fetch_word().map_err(|_| "Failed to fetch word")?;

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
									"name": "dockset-test-2"
								}
							"#;
			
			// Send a POST request
			let request = http::Request::post("https://storage.googleapis.com/storage/v1/b?project=intern-storage-apis", vec![json_body])
			.add_header("Authorization", "Bearer ya29.c.c0AY_VpZh0LRKXb1pdIOrmW7PdEmFCGhI60h8sUlsIVQx0CsWy6MMXDON5DyLU4vi9Qb-6eoxOO4_hhBL4KtB3JuXYbE5PvbsK2Ye3feDzr7_LTeLWWrVlXiur5knMfyqEqU8PvEsuY2Af4ujfWQNd_BsU0GZhGE8r5iiqOXjrPHmRMPjtvTFAbN0_1ylHmSGkw1ly-mZz3TXlcOSExzQw7x3YRqjB-_Xvq-xlUXU7Q1nPJBIT0lZA0MMeic_HBRnqMpyRC4q7ofpF9mQIIxdsHPa6BGx0E3hXWsONwvNjGYs2s7GmsIlhY7VhXVZetmciYC7SfPXb1BhNgCYbFnaIURXJOp79LF0lvXrr7WAi6LnxloII3O2hPEFCuHodN5jRFxzJKgL399DiV8pYfsrV5UgVu4OSIi4jeWFY5baQvZfMF9uw_co46Uo263fVFmMIJw3lIRvcRfRytkykpi5m38UMl1RcMIZ99Uhrarkqxwt9-J7xZcgqYoMOVBMf8UxUq-tf3uXumcBm-oFRjlM92pkutSQrVhu0IJWW6Zovfl0z5gYu4Z1J6Z9n4jQijmzRmb-M2d6r9I0MnhQjkkuiR17_udm4b1SxBc3Fp71t8nmFeJ4sq_4-2MWRkBzgBJvcdzic7QlZJwbM1li5B_70qjIfcOrYsi1ib1Mm1r5sJZ6-qYBk6k2pV4_v0XzpRmmy9ZupSFkvhrqvk-yUd1k61x9x6sl_e8dJ2dopx6-UZ4xvO2n_S6VtOlyqzpizwfXjMh9zjr3UMV0S6Y4o3SB26urW4pQX_i6vcer6FMp5ZXhWsrsSW376eXxlQBsc15S86dOR7zQk_180ir4RpxbXijv5w1VQvgx3I_7zmc4Ydmc0VgBQSf1p8cjXvcf8kMIby8xVgyFvxjBg6qj-w2eIgmwxUgSMhh6b9tRia5md7sXqv1bYe2-uWpWtF00aWom5qIWa1F5coIr6J1ebewMZr5b4ZJascMx6hkZjZxwZRF6px8cBJBdiyBo")
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

		//folder creation feature
		fn _folder_creation() -> Result<(), http::Error> {

			//set deadline
			let deadline = offchain::timestamp().add(Duration::from_millis(2_000));

			//json body
			let json_body = r#"{ "name": "flower" }"#;

			//send post request for folder
			let request = http::Request::post("https://storage.googleapis.com/storage/v1/b/dockset-test-2/managedFolders", vec![json_body])
			.add_header("Authorization", "Bearer ya29.c.c0AY_VpZiZ_mpl0xzhqupVxXdzXbxOkMnMpX_pj3PYo-K9ZwKm7xCLitpnrqoKwsSc-t7C5xLZgH4slDHQo43ize30UfLlAu32y97UDTzlvyGddj_8pjjG94TaHrOfk3lisz0-FtzDu98mq2HkDZui0Lnx4E6lqfyJjK8b8hChuqgQ--IbiIdxWAGDZ_swoJhyWZtYsmAsQ1psNNePvbLJSfg1t1a8RwnvKGmhNaqu7SY0JEbr1s4n3mNzfYKSSqfJAABDdTFks66tq42U4Bl5S0g07PdlW0ymEEqgFN_tNREB1fKAZCRFllOt3U-YLHf_7D8JhNB26GpK5RHxInKb3iv9zjVb8uA-Hhzam-8-7UFWhrufhKlQkP9R0eA6kO4Ddj-fuwG399DVqwucwS9n5frRU8Bwr8cO3U0oR_O5w5gzyzmJ82Be4Sp2oIXnu7XmtOdJSjg1QUWnU7tOY7dtr7f3lrXmtZMi__14trxrvxsOFIZZflyBu-qb3bxJtVzBg0Q86q68Sf5b81ndimmWi5JvscmBcjSwx3J_paFc8SYgVZg0vSxXjfBOdbiaBemv-VicnbO2o2vxIlvX-5mO7FdV3UeJUtlOja64oQYhel8qgvsVFm9qeh6I9qJ-v35Iz-eYylZ5Ibcl8wr3rfbxs5eo__x_Bir66UBnIQnfwehgIamgr725w8MhpZzqZive-jsn_6cYWjXh7Fzelosu3lJJnodfqB7Q_nfjmqihh3U9ibayBuz4_io_RuXc7R-d2t8Z_8538Y-Xdx7Owl9Xnd9jrWl7BJWbJwtyWZZ5lY-8zsIJltjsWq6snBst10Bti2eW-WmRw8JhZOJZQh91Ry4S7ZwRuRqc1h5i4ydk8b9F6jg_ii2fd4rnoU0FOniXFsw84ytbhoY0qmR652l4yjl9-6vfw6ooV1v3W0WZsMSjc71s2FSMWvmvt0tRrOo6wpsd6xS8_Fu1-7tpdi_tq8txX3Mmh8Mga1Y2d1ZXZzSfI0avgM_Z8pj")
			.add_header("Content-Type", "application/json");

			let pending = request
			.deadline(deadline)
			.body(vec![json_body])
			.send()
			.map_err(|_| http::Error::IoError)?;

			// Wait for response 
			let _response = pending
			.try_wait(deadline)
			.map_err(|_| http::Error::DeadlineReached)??;

			log::info!("Status code: {:#?}", _response.code);

			//check response is successfull
			if _response.code != 200 {
				log::warn!("Unexpected status code when creating folder: {}", _response.code);
				return Err(http::Error::Unknown)
			}

			// let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;

			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			signer.send_signed_transaction(|_account| {

				Call::create_folder { }

			});

			Ok(())
		}


		//file upload
		fn _file_upload() -> Result<(), http::Error> {

			//set deadline
			let _deadline = offchain::timestamp().add(Duration::from_millis(2_000));

			let get = FileSave::<T>::try_get();

			let value = match get {
				Ok(result) => result,
				Err(_) => Vec::new(), // Return an empty Vec<u8> on error
			};

			//send post request for file upload
			let request = http::Request::post("https://storage.googleapis.com/upload/storage/v1/b/dockset-test-2/o?uploadType=media&name=fish.png", vec![value.clone()])
			.add_header("Authorization", "Bearer ya29.c.c0AY_VpZjNzR8Io55HxzHXKvq08_Nw82tAbdI2ayjtopXtEdQuZf9dqHb7nc_06enJf66wTMIIZIiRybhU6q8pWzXgG_R283qPCUD2Y8jY-JIOCdz3jCtbCZva_-F_jt-_AEqQRqMYFO9HiiYnP4kPSme4qrDKmvF_w5vPxP05QvSyLWQn9vOgHq4XRRpp1uLeTj_MGacYHJ1pPSz0VwsMqcS6AmndSU5XH08pzxkd2eY2KgR5wGFgDhG2cf8Zr1WvRrTxPUd3CtsshTc180PGBDwlHGCBK4vqWwk57XByCc0l0I2BDghbsTlK4PEdzwXGnIXzW70KdPDseyds_wtmi44zaQTeBjncMQB9LmmEdBL2YML6HNESzWSZp7JAu1jmR2A8BwG399CI3h3445_Blm3-3puid51to7bz0U-z9t19U9hihjsz5dcR4Qdovi5ktBZSrcnWRR9wpUrJdu6yBQ3gOxumzmYgieQgl9ahJRkx0mixs-xJR6BMixnxinxS6hOw88eypyd1Wpqfnbe9uz15O8Jmvh-p8Xvre3xZSvVM71nZlQ-zm52nm303SOFqF9-m_9391dQpmigSusROY1p1zaI7YQnklwU5onSm__SumFJvRlRFgevQbQZy7-ZSM8zfO9x4M28FcRlhWJxYWvx4YZv_SQi7_qgJnwt-3Xd_W7woU0W0WmiiVa2uOpJcnhloFgU6lqZB0exgUs6BrM-ncjdw6Xmj97vfcQ_Qm48JoylmmmfxU-zr-5doxU4qxUf-hj4XO7I_h1smcpSu3ipXamdlR9Oj6gluYbX4MIn5ZkYy4U-Sse0sSldVspMYa97dFB_hBXbpx9jXy1fWI8vesxvWp9i1mWtz_1WyJnjec9_m1OiBtoz75F4B2kic-jjYm3lqvinSqztImymyVU-3RRez6k8Qul7uhwF2BrWZZc0BrpiB9ZM_JOW6kzi6VqS6V-wgxg8jc96gey8O-mQZV_ruluJMwd937w9IramwOBwki5k0v-e")
			.add_header("Content-Type", "image/png");

			let pending = request
			.deadline(_deadline)
			.body(vec![value.clone()])
			.send()
			.map_err(|_| http::Error::IoError)?;

			// Wait for response 
			let _response = pending
			.try_wait(_deadline)
			.map_err(|_| http::Error::DeadlineReached)??;

			log::info!("Status code: {:#?}", _response.code);

			//check response is successfull
			if _response.code != 200 {
				log::warn!("Unexpected status code when uploading file: {}", _response.code);
				return Err(http::Error::Unknown)
			}
			
			Ok(())
		}

		//file delete
		fn _delete_object() -> Result<u16, http::Error> {

			//set deadline
			let _deadline = offchain::timestamp().add(Duration::from_millis(2_000));


			let get = ObjectDelete::<T>::try_get();

			let object_name = match get {
				Ok(result) => result,
				Err(_) => "".to_string(), // Return an empty Vec<u8> on error
			};

			let file_delete_url = format!("https://storage.googleapis.com/storage/v1/b/dockset-test-2/o/{}?key=AIzaSyDALd0Bm-prjVKyNDZROwSPYeZJMIMTm38", object_name.clone());

			//json body
			let json_body = r#"{}"#;

			let _delete_requsest = 	http::Request::default().method(Method::Delete)
			.url(file_delete_url.as_str())
			.add_header("Authorization", "Bearer ya29.c.c0AY_VpZjNzR8Io55HxzHXKvq08_Nw82tAbdI2ayjtopXtEdQuZf9dqHb7nc_06enJf66wTMIIZIiRybhU6q8pWzXgG_R283qPCUD2Y8jY-JIOCdz3jCtbCZva_-F_jt-_AEqQRqMYFO9HiiYnP4kPSme4qrDKmvF_w5vPxP05QvSyLWQn9vOgHq4XRRpp1uLeTj_MGacYHJ1pPSz0VwsMqcS6AmndSU5XH08pzxkd2eY2KgR5wGFgDhG2cf8Zr1WvRrTxPUd3CtsshTc180PGBDwlHGCBK4vqWwk57XByCc0l0I2BDghbsTlK4PEdzwXGnIXzW70KdPDseyds_wtmi44zaQTeBjncMQB9LmmEdBL2YML6HNESzWSZp7JAu1jmR2A8BwG399CI3h3445_Blm3-3puid51to7bz0U-z9t19U9hihjsz5dcR4Qdovi5ktBZSrcnWRR9wpUrJdu6yBQ3gOxumzmYgieQgl9ahJRkx0mixs-xJR6BMixnxinxS6hOw88eypyd1Wpqfnbe9uz15O8Jmvh-p8Xvre3xZSvVM71nZlQ-zm52nm303SOFqF9-m_9391dQpmigSusROY1p1zaI7YQnklwU5onSm__SumFJvRlRFgevQbQZy7-ZSM8zfO9x4M28FcRlhWJxYWvx4YZv_SQi7_qgJnwt-3Xd_W7woU0W0WmiiVa2uOpJcnhloFgU6lqZB0exgUs6BrM-ncjdw6Xmj97vfcQ_Qm48JoylmmmfxU-zr-5doxU4qxUf-hj4XO7I_h1smcpSu3ipXamdlR9Oj6gluYbX4MIn5ZkYy4U-Sse0sSldVspMYa97dFB_hBXbpx9jXy1fWI8vesxvWp9i1mWtz_1WyJnjec9_m1OiBtoz75F4B2kic-jjYm3lqvinSqztImymyVU-3RRez6k8Qul7uhwF2BrWZZc0BrpiB9ZM_JOW6kzi6VqS6V-wgxg8jc96gey8O-mQZV_ruluJMwd937w9IramwOBwki5k0v-e")
			.add_header("Accept", "image/png");

			let pending = _delete_requsest
			.deadline(_deadline)
			.body(vec![json_body])
			.send()
			.map_err(|_| http::Error::IoError)?;


			// Wait for response 
			let _response = pending
			.try_wait(_deadline)
			.map_err(|_| http::Error::DeadlineReached)??;
			
			log::info!("🆔 Delete response status code: {:#?}", _response.code);
			
			//check response is successfull
			if _response.code != 204 {
				log::warn!("🛑 ❌  An unexpected status code occurs when deleting a file: {} ❌ 🛑", _response.code);
				return Err(http::Error::Unknown)
			}

			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			signer.send_signed_transaction(|_account| {

				Call::delete_object { object_name: object_name.clone() }

			});

			Ok(_response.code)
		}
		

		/*
			DELETE https://storage.googleapis.com/storage/v1/b/[BUCKET]/o/[OBJECT]?key=[YOUR_API_KEY] HTTP/1.1

			Authorization: Bearer [YOUR_ACCESS_TOKEN]
			Accept: application/json
		*/

	}
}
