#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::offchain::{http, Duration};
// use sp_runtime::offchain::http::Error;

// use frame_support::{dispatch::GetDispatchInfo, traits::UnfilteredDispatchable};

pub use pallet::*;

use sp_core::crypto::KeyTypeId;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
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
	use codec::alloc::string::ToString;
	use frame_support::{
		pallet_prelude::{DispatchResult, *},
		sp_io::offchain,
	};
	use frame_system::{
		offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer},
		pallet_prelude::{OriginFor, *},
	};
	use scale_info::prelude::string::String;
	use sp_std::{collections::vec_deque::VecDeque, str, vec};
	use sp_std::vec::Vec;
	use sp_runtime::offchain::http::Method;
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

	//Bucket name Struct
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct BucketName {
		pub name: String,
	}

	//Folder name structure
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct FolderName {
		pub fname: String,
	}

	//File name structure
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo, Debug)]
	pub struct File {
		pub file: Vec<u8>,
	}

	#[pallet::storage]
	#[pallet::getter(fn info)]
	pub type BucketNameSave<T> = StorageValue<_, BucketName>;

	#[pallet::storage]
	#[pallet::getter(fn fstore)]
	pub type FolderNameSave<T> = StorageValue<_, FolderName>;

	#[pallet::storage]
	#[pallet::getter(fn store)]
	pub type BucketNameStore<T> = StorageValue<_, VecDeque<String>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn file)]
	pub type FileSave<T> = StorageValue<_, Vec<u8>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BucketCreated { name: T::AccountId },
		FolderCreated { fname: T::AccountId },
		FileFetched { file: T::AccountId },
		FileDeleted { file: T::AccountId },
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
			log::info!("Hello from ⛓️‍💥 cloud offchain worker ⛓️‍💥.");
			log::info!("🌐⛓️ Current block: {:?} 🌐⛓️", block_number);

			// match Self::create_bucket_request() {
			// 	Ok(word) => log::info!("Bucket: {:?} created successfully", word),
			// 	Err(_) => log::info!("Error creating bucket"),
			// }

			// match Self::create_folder_in_bucket() {
			// 	Ok(word) => log::info!("Folder: {:?} created successfully", word),
			// 	Err(_) => log::info!("Error creating folder"),
			// }

			// match Self::create_bucket_request() {
            //     Ok(bucket_name) => {
            //         <BucketNameSave<T>>::put(BucketName { name: bucket_name.clone() });
            //         Self::deposit_event(Event::BucketCreated(bucket_name));
            //     }
            //     Err(_) => log::info!("Error creating bucket"),
            // }

			// match Self::_file_upload() {
			// 	Ok(_) => log::info!(" 📁 ✅ 〰️〰️ File uploaded 〰️〰️ 📁 ✅"),
			// 	Err(error) => log::info!("📁 ➡️ ⭕⭕ ❌❌❌❌ Error file uploading ===> : {:#?} ⭕⭕ ❌❌❌❌", error)
			// }

			// match Self::file_delete() {
			// 	Ok(_) => log::info!("File deleted"),
			// 	Err(error) => log::info!("Error file deleting ===> : {:#?}", error)
			// }

			
			match Self::file_delete() {
				Ok(code) => log::info!("✅✅✅✅ File deleted : {} ✅✅✅✅", code),
				Err(error) => log::info!("❌❌❌❌ Error file deleting ===> : {:#?} ❌❌❌❌", error)
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//Store word
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn save_bucket_name(origin: OriginFor<T>, name: String) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let new_name = BucketName { name };

			<BucketNameSave<T>>::put(new_name);

			Self::deposit_event(Event::BucketCreated { name: sender });

			log::info!("Hello from bucket name save.");

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn save_folder_name(origin: OriginFor<T>, fname: String) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let new_name = FolderName { fname };

			<FolderNameSave<T>>::put(new_name);

			Self::deposit_event(Event::FolderCreated { fname: sender });

			log::info!("Hello from folder name save.");

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn upload_file(origin: OriginFor<T>, _file: Vec<u8>) -> DispatchResult {
			
			log::info!("Hello from upload file");
			
			let sender = ensure_signed(origin)?;

			// let new_file = FileName { filename };

			// <FileNameSave<T>>::put(new_file);
			
			<FileSave<T>>::put(_file);


			Self::deposit_event(Event::FileFetched { file: sender });

		
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn delete_file(origin: OriginFor<T>) -> DispatchResult {
			
			log::info!("Hello from delete file");
			
			let sender = ensure_signed(origin)?;

			// let new_file = FileName { filename };

			// <FileNameSave<T>>::put(new_file);


			Self::deposit_event(Event::FileDeleted { file: sender });

		
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		//fetch data from the url
		fn _create_bucket_request() -> Result<String, http::Error> {

			let url = "https://storage.googleapis.com/storage/v1/b?project=intern-storage-apis";
			
			let json_data = r#"{"name": "fcx-text-bucket1"}"#;

			let request =
				http::Request::post(url, vec![json_data])
					.add_header("Authorization", "Bearer ya29.c.c0AY_VpZhPswE6MeSUeJ-FEjK66VCMEzwX05e1XseG-8UDlXsy_fuJgZH-ifOOYelgWk2s6Yztv4CVYrOptg6rJy31q0qocB9YgXuyo2aBptIsS4VVu2O_7q0D5gydjfcA-89bYaRIhsui6pkoXbXQNtp0hrRSOeENHphRRvkpYJhpsTDTxpxb1CiAZlQ_7PLza6RpSBnvBz1dl6oDcIsLqL8uzhDbBsgyb7WyeG6AdpuCiOxSiHOaDLODRJdntbV7p6l3IFQnoEimBmw7IzjTWuvaJYiXqHfNnXqJax5uOOnVpGINiFWlQM1Ii9SzOnt5j2qDYdzE1vcuW354wT9v1hnOiwpnH3UNfoSEMFm3l3fYCNGmNoxfzdKjyExwcUSUYewmL397Di30mhV9pV0OwsQdg2aX4FSui8bRaz5eo6mkW9Y5FmseZgkWwfw0Z5hJIWunRitcWcV23hFkoI_qsUmdlbM23QppdqZ4XBRe55VUl5sv2yJf70YjoRzMdytiQ5WMnaVYMQFWhqQ-eizd0l-FF37x9Y7aOjS0nqMeU2vQioMBou3-vBycRQZS3140J-rtZBnWBcdk-7sdJqnOtU98hdpVjU7S9U2lOj605koke8hqBVq1YXju0BWIBSsOgdIv3RtSgyMZVJm4yr8VjdmjqoxifdwpomhRQmVU_Q0oZvjwe7507YcXyV5M023IWMZSXmZoeI5JIzgzujce1JedgBqbv_hv-XBUF8YYYQBIt3mvbOqtZw_W_cQpoxSvnVMYmdal4or39avw1Mpgsot9gcealV5YnkFb1UiFBy-mqJlUp_b1JSo43XmQkYOSx-p206_b-XWsIwrx3d0Zwb2n41uarmq2j2qou5fr2zuZb81cQjln3r0IcOgaBmt_RwQOmgjp036xIV-ejczOuOiFFJQZq7y1ZRSYBzqjYSsg7kzMfymiaU0iRR6JWfer74Z-lm21R60Vw0_O_YY81k_V4h_X0jrBe6BQmxeVer-ppYk2IOmBlmj")
					.add_header("Content-Type", "application/json")
					.add_header("Accept", "application/json");

			let deadline = offchain::timestamp().add(Duration::from_millis(2_000));
			
			let pending = request
				.deadline(deadline)
				.body(vec![json_data])
				.send()
				.map_err(|_| http::Error::IoError)?;

			// Wait for response
			let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;


			if response.code != 200 {
                log::warn!("Unexpected status code: {}", response.code);
                return Err(http::Error::Unknown)
            }


			Self::fetch_word_and_send_signed("fcx-text-bucket1".to_string());

    		Ok("fcx-text-bucket1".to_string())
		}

		fn _create_folder_in_bucket() -> Result<String, http::Error> {
		

			let json_data = r#"{"name": "text-folder-newest"}"#;
			
			let url = "https://storage.googleapis.com/storage/v1/b/fcx-text-bucket1/managedFolders";
			
		
			let request = http::Request::post(url, vec![json_data])
				.add_header("Authorization", "Bearer ya29.c.c0AY_VpZgI-kTUuQuRlxeTf0T8g5XIQcBYmkkq9xdAQ-1e6s3jFxaLGtlHXy-6oQ4HRFf54ep7ErrldPkYi5CFoEnzCZI6mO_zdRFusiDlsccgMn8NmA8ZDtWZKdPZVmzGDYcJTTAfIQn4tfPaLN_XcSZIwWYqaMaJANEzxfom7ZkF0OYxSLxZSMxLaNxcwRqGWiJ_BVbBCW6VM-f9SKgbeTB-fcr_MpXOsgDhw6rghuN5X4nNxC-x_WtgJ5xClPzZ6wHpmm0s3rBu7MuDPocfHslToe_ZQzKOEmsAy3TvaSBD_Zqw8KfHBUTxbVbWzq231wwMuaa8otcRIedhQLsLcTdrdVQ4KKlTmfRZxzyAq_S-Fqk4YPDl4admUv9xver_ipgP-AL399DfQds0-53-ZBhMQcYoUw99ggbixSesU-SvU3nkJd7ojXXoItWhOSR5hQi4pSpwIbo4vqIZQYnmRsOIgj091M5agz6g9_hpi58vgleYrF6eqoX1_rtm-g0QjZy5nQ2nliVWUjlyo78t2seuMwB6sV9otfFo9drcqXVwU3R_V1_FFfff2MsIru-94Qn-9-_gh-Vho_9zM88cdb6t8-qsO9tyigpUQWuul_sUJa8U61RV_twbWnzaXretYxlQhBmorphnWfVYhQnXF-h0zip_yYOXrj7jWQRryFeXMgXruxfxBS7ZISotR33XsleoJV228XjzmMXs-Wab36Uw4_nZ642IeiBW2bQ9FWS5es6-xfaFUFfmxdncozMkWfnnFiul-4XRcVQaoSi2vadkmcJeVZ9Qlcf78JOju86UXUroeZ5ZZZzFVepWMpf1IYquadI41p9pSjXBkhlfwWd2z7ykSca6oxvb7MoUYiz94cmeun_k760O0Rebw9hV6VzVnk-vVx4_tRkVwoyuUSRsQFMSdq7zrzh4nc2-ZZuMVm11yo_ekSdyp5Y1Z11JeonibzJmOObZiU5r0R4sZ_bloR7caSZe6nyJimy5ulrQpfQmbe4JItJ")
				.add_header("Content-Type", "application/json");
		
			let deadline = offchain::timestamp().add(Duration::from_millis(2_000));
		
			let pending = request
				.deadline(deadline)
				.body(vec![json_data])
				.send()
				.map_err(|_| http::Error::IoError)?;
		
			// Wait for response
			let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
		
			// match response {
			// 	Ok(res) => log::info!("Successfull status code: {:?}", res.code),
			// 	Err(err) => log::warn!("Error status code: {:?}", err)
			// };

			if response.code != 200 {
				log::warn!("Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown);
			}

			Self::fetch_folder_name_and_send_signed("text-folder-newest".to_string());
		
			Ok("text-folder-newest".to_string())
		}
		
		fn _file_upload() -> Result<(), http::Error> {
			//set deadline
			let deadline = offchain::timestamp().add(Duration::from_millis(2_000));

			let file_fetch = FileSave::<T>::try_get();

			let value = match file_fetch {
				Ok(res) => res,
				Err(_) => Vec::new(), //retuen an empty vec
			};

			//send post request to file upload
			let request = http::Request::post("https://storage.googleapis.com/upload/storage/v1/b/fcx-text-bucket1/o?uploadType=media&name=pic.jpeg", vec![value.clone()])
			.add_header("Authorization", "Bearer ya29.c.c0AY_VpZjH50kvwF1pDwMtucqZZqvbfd9BmcD8tnjk7QO7fhAxzLTKwBHtq_2pxMdWqXWpvOuPFrXDEpw1osAlMOZpAL_RBWOdlj3XJeonXrjgjB-yupc4N5DGwx_-0Ip8yuCEUjOu1xsV4ezcfCwPc03oKDfbcFYO1rStdjwXyyUmnUplL-YriIYQJIfU1nYV33K0EXWTCTmxb8raRNMOUNABOx0wk5nzsyRXJ7cIUO6rL557JhtLv2oEdMG_Jf-OPGtFHlNjTsMdWN7vRmEUxKuZ7c_Kn3cIGI6veV6M3ke7dLUhxe7BSwHqIXPxNaMoeCRHDfASB52mgIiDNpEE-b7JIwlIyYKhFXy32tCobZN1jhH6UsPRV8JDwc_nZ7uxTdoJDgL399DarQ1Wd9-yyUlt7xiuvWp4U5p3lJj8Ws0fsqXXI7aeu01Qlhz5cY7f2yJs_bp5oMwlk_0r01MgMVa4hI_-W5nB-hr8eOrw6MMq2wjerUslrIuOzkzsR8ln6zxlIxJ_yYOvuXRkBilR2nsZYIMzXWFph5k3Z7hzYa6whis27f7Y18txiYJ89tgaqS8iXOnOB_pyyecbFUdFpzIMySrId6c2-jx8XkO2tyfpOzM7yMz0WenIlfleRsXZo8v2MsqRdrxIYsn7rb-owuVB2RJ4luOQc0_vXcYXrUjx1hhbO6q_Ongz8vBa23itahiypzMF_byoIYmld80npR466tRVRyhlpUSvc56bV248y7bFQROWoZgMFMQVUM5pkFOB0Rx2f20Jv4nJcazQIjqhW8hyFamu6MMBdZebV1z2wWekhyXkidxY9sa01zhspbc-R2vXWBooimn4_WcjSOiVqX__zrveiJ3dMoXgU31Ofeagzw2UluakZ-BjbwzydQgaSrJ-O4U2aRI1e3y5xJJ-_3_nR5skslakQXz9zouqrsjQgccI12OfZwSeZi2boeXx9oxstYIzOOjf6vkp1y3rOXlUnj2y8SqIQxsImO7zdf_agMQgfU8")
			.add_header("Content-Type", "image/jpeg");

			let pending = request
			.deadline(deadline)
			.body(vec![value.clone()])
			.send()
			.map_err(|_| http::Error::IoError)?;

			// Wait for response 
			let response = pending
			.try_wait(deadline)
			.map_err(|_| http::Error::DeadlineReached)??;

			log::info!("Status code: {:#?}", response.code);

			//check response is successfull
			if response.code != 200 {
				log::warn!("🔴 🔴 🔴 🔴 Unexpected status code when uploading file: {} 🔴 🔴 🔴 🔴", response.code);
				return Err(http::Error::Unknown)
			}
			
			Ok(())


		}

		fn file_delete() -> Result<u16, http::Error> {
			//set deadline
			let deadline = offchain::timestamp().add(Duration::from_millis(2_000));

			// let file_fetch = FileSave::<T>::try_get();

			// let value = match file_fetch {
			// 	Ok(res) => res,
			// 	Err(_) => Vec::new(), //retuen an empty vec
			// };

			let json_body = r#"{}"#;

			//send post request to file upload
			let delete_requsest = http::Request::default().method(Method::Delete)
            .url("https://storage.googleapis.com/storage/v1/b/fcx-text-bucket1/o/cat.jpeg?key=AIzaSyA6f3dyDiLIkizS6PB3UZY-a1CcDksVEAc")
            .add_header("Authorization", "Bearer ya29.c.c0AY_VpZgXFFw88RhSsBxDP7A0I73sXVGeCMouArPdH6UDIHs2alJm9vs0hWOent_5XamcvpNTB1YJ8GWvWcHrfez6MumFMDbNrTvZk2P22EHNpv_Ino7FGwd377-fMweT5jqsdwEDaM7BdidmR8bU1vLqbMZy29PPLRRsVKSRCgvQ0fU6T0e4A6nfp1h7hhCs6foyPiXaRpC4xwcgI713Ayu0np3PgI53FxOUWcbor9vUABkP4cWdLdNBINKmeAA2h9VEYkkMh1oVXD4t1M6dJ7WP394IhahXy8pRBOO8609ZxRKeh48Ew4SXANMqRYWMjITe5OxBlAC8SjJEv7GrMGZLhchfoXppIeEJS9FHX5MOJqKJb9pMMcXWhnb9O95KACWNN397Pr53jJ-qVz8g-s0ikUVu-5msMYsMmZf4O91IyjalrmwX2zV2a550vfkUZmxXpOFi5ISFgQj00cWrQuWqtOb4sb7m_RsZan8O3Qweax6QpSsl9nbrfSbVs4p5W-tJ5aF4y4fWiW2_1mmScoqv224_yXBkuQVfY8s1UBs_5FjjIbduXMjueshuiiZS5Xkek9OuM0-vqOilqO3rlum_hsfursnfcycQvegexaf80JRvma32c_dQJSO23lsIu348W6dx0o0pybbvz7B5bQ6VWchM9fgghc90I9BWV2xSvMdXur16dd_8uh9xSotlo8qi888O0Vtahq5M1qY7XqbfscSuJgV7dlB0RwX72oWt9tgMFnpzntxtWrVRRXdqVWapYik8c1jm21-uptwBmZnQUmS4W1ZVqhkWBW6RXyMylQ3BwWvZ6o7Qlv-ilXuqkcg6qgpnm85FUVro_wvr-Foq8OyjUcyjnk4_a_Fsh4ydR3olbgtsruMrSlV5wmkUY1SaF3rs6ffz2cb1QI16yn_aib_vF8q19ekg2M5eRS6wjuMfhROy_eOctfj9rWuOka7b2nzxOUvUyRFlZUMFQSkmciVuaaY7-dBs4J8Vd6dZp9tlafpyqem")
            .add_header("Accept", "image/jpeg");


			let pending = delete_requsest
			.deadline(deadline)
			.body(vec![json_body])
			.send()
			.map_err(|_| http::Error::IoError)?;

			// Wait for response 
			let response = pending
			.try_wait(deadline)
			.map_err(|_| http::Error::DeadlineReached)??;

			log::info!("Status code: {:#?}", response.code);

			//check response is successfull
			if response.code != 204 {
				log::warn!("❌ ❌ ❌ ❌Unexpected status code when deleting file: {} ❌ ❌ ❌ ❌", response.code);
				return Err(http::Error::Unknown)
			}

			//call signed transaction
			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			signer.send_signed_transaction(|_account| {
				Call::delete_file { }
			});

			
			Ok(response.code)


		}



		/// A helper function to fetch the word and send signed transaction.
		pub fn fetch_word_and_send_signed(word: String) {
			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			let results = signer.send_signed_transaction(|_account| {
				Call::save_bucket_name { name: word.clone() }
			});

			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!("{:?} Word fetch success: {}.", acc.id, word),
					Err(e) =>
						log::error!("{:?}: submit transaction failure. Reason: {:?}", acc.id, e),
				}
			}
		}

		pub fn fetch_folder_name_and_send_signed(word: String) {
			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			let results = signer.send_signed_transaction(|_account| {
				Call::save_folder_name { fname: word.clone() }
			});

			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!("{:?} Word fetch success: {}.", acc.id, word),
					Err(e) =>
						log::error!("{:?}: submit transaction failure. Reason: {:?}", acc.id, e),
				}
			}
		}

		//get word from the user
		

	}
}
