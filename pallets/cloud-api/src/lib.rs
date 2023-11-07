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
	use sp_std::{str, vec};
	use sp_std::vec::Vec;
	use sp_runtime::offchain::http::Method;
	use scale_info::prelude::format;
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
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo, Debug)]
	pub struct BucketName {
		pub name: String,
	}

	//Folder name structure
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct FolderName {
		pub fname: String,
	}

	//File structure
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo, Debug)]
	pub struct File {
		pub file: Vec<u8>,
		pub file_name: String,
		pub bucket_name: String,
	}

	// //File name structure
	// #[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo, Debug)]
	// pub struct FileName {
	// 	pub filename: String,
	// }


	#[pallet::storage]
	#[pallet::getter(fn info)]
	pub type BucketSave<T> = StorageValue<_, String>;

	#[pallet::storage]
	#[pallet::getter(fn fstore)]
	pub type FolderNameSave<T> = StorageValue<_, FolderName>;

	// #[pallet::storage]
	// #[pallet::getter(fn store)]
	// pub type BucketNameStore<T> = StorageValue<_, VecDeque<String>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn file)]
	pub type FileSave<T> = StorageValue<_, File>;

	#[pallet::storage]
	#[pallet::getter(fn bdelete)]
	pub type BucketDelete<T> = StorageValue<_, String>;

	// #[pallet::storage]
	// #[pallet::getter(fn filefetch)]
	// pub type FileStore<T: Config> = StorageDoubleMap<
    //     Hasher1 = Blake2_128Concat,
    //     Key1 = T::AccountId,
    //     Hasher2 = Twox64Concat,
    //     Key2 = String,
    //     Value = Vec<u8>,
    //     QueryKind = ValueQuery
    // >;

	#[pallet::storage]
	#[pallet::getter(fn fnamestore)]
	pub type FileDelete<T> = StorageValue<_, String>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BucketCreated { name: T::AccountId },
		FolderCreated { fname: T::AccountId },
		FileFetched { file: T::AccountId },
		FileDeleted { file: T::AccountId },
		BucketDeleted { name: T::AccountId },
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

			match Self::create_bucket_request() {
				Ok(word) => log::info!("✅ 🤟🤟🤟 Bucket: {:?} created successfully 🤟🤟🤟 ✅", word),
				Err(_) => log::info!("⚫⭕Error creating bucket⭕⚫"),
			}

			// match Self::create_folder_in_bucket() {
			// 	Ok(word) => log::info!("Folder: {:?} created successfully", word),
			// 	Err(_) => log::info!("Error creating folder"),
			// }

			match Self::file_upload() {
					Ok(_) => log::info!(" 📁 ✅ 〰️〰️ File uploaded 〰️〰️ 📁 ✅"),
					Err(error) => log::info!("📁 ➡️ ⭕⭕ ❌❌❌❌ Error file uploading ===> : {:#?} ⭕⭕ ❌❌❌❌", error)
			}
			
			match Self::file_delete() {
				Ok(code) => log::info!("✅✅✅✅ File deleted : {} ✅✅✅✅", code),
				Err(error) => log::info!("❌❌❌❌ Error file deleting ===> : {:#?} ❌❌❌❌", error)
			}

			match Self::bucket_delete() {
				Ok(code) => log::info!("✅✅✅✅  Bucket deleted : {} ✅✅✅✅", code),
				Err(error) => log::info!("❌❌❌❌ Error bucket deleting ===> : {:#?} ❌❌❌❌", error)
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//Store word
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_bucket(origin: OriginFor<T>, name: String) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// let new_name = BucketName { name };

			<BucketSave<T>>::put(name);

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
		pub fn upload_file(origin: OriginFor<T>,file_name: String, file: Vec<u8>, bucket_name: String) -> DispatchResult {
			
			log::info!("Hello from upload file");
			
			let sender = ensure_signed(origin)?;

			let new_file = File { file, file_name, bucket_name };

			<FileSave<T>>::put(new_file);
			
			// <FileStore<T>>::insert(sender.clone(), filename, _file);

			Self::deposit_event(Event::FileFetched { file: sender });

		
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn delete_file(origin: OriginFor<T>, filename: String) -> DispatchResult {
			
			log::info!("Hello from delete file");
			
			let sender = ensure_signed(origin)?;


			<FileDelete<T>>::put(filename);


			Self::deposit_event(Event::FileDeleted { file: sender });

		
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn delete_bucket(origin: OriginFor<T>, bucketname: String) -> DispatchResult {
			
			log::info!("Hello from delete bucket");
			
			let sender = ensure_signed(origin)?;

			<BucketDelete<T>>::put(bucketname);


			Self::deposit_event(Event::BucketDeleted { name: sender });

		
			Ok(())
		}


	}

	impl<T: Config> Pallet<T> {
		//fetch data from the url
		fn create_bucket_request() -> Result<String, http::Error> {

			let url = "https://storage.googleapis.com/storage/v1/b?project=intern-storage-apis";

			let bucket_store = BucketSave::<T>::try_get();

			let value = match bucket_store {
				Ok(res) => res,
				Err(_) => String::new(), 
			};
			
			let json_data = format!(r#"{{"name": "{}"}}"#, value);


			log::info!("JSON_DATA:  {:?}", json_data);

			let request =
				http::Request::post(url, vec![json_data.clone()])
					.add_header("Authorization", "Bearer ya29.c.c0AY_VpZhFDtmRP48p82xlqwJTgVKTaDC0QhHdRZrx3r_tK6r0aiGzMZhNJ-cKBEFpzgi5SpT3a3-FCXxSI4MTNcJgByaDx82GWBic9aGj7r5s7-zo1cmXm2QBWnmxWg5-5xHyrVWEX7ypaeOXfbazkeC-v4DLOTxodFgAMr8N3fmBhCBmeGqgaP21Sqa7is8s3ArGX3CeEIFvdZE4V3AuEoWT0e-wVOYPh5Gd9agQK_tvMcC1Mu_QLkvfiNCN4udbr-eK2yIcGtYZgDhuwxaS1tIzUIeA9KwlexDcE7t5fwIT-mJnzYS-YNE2qCd1yaIuMNJNoUP368hyi4dSIdT9sdmHywhVKADrmcL2mWEAGoQCWmSUTnMEOgE4wYdhiaNxlz5KJAL399Kk75Wl70Yjr5suIlpimsFy6dV-wvwV8j6qrXsk8XXggIbz2Vm-6lI_UscI5ROpvxcxJggJXvjlyo07tYzp2ilkfnQZwOg9g4yyiwzplU8YBXr1-x8BunO1_cX9p-iIWksZMOfUzSbp8J77MWFvx-W8OU1QIUa5Qi49aOmZfnX7uBVflltcvkBZiXc2hYFdsj9hmiRszd0665WS4210Bnpz9SRJ5jR2SBb9JII8JMhX1jhZyFhJhOpZ4qtkIXXcIa3-Oe6_3-in-ijjceUjoZw83aaYv9RtFWuyV7nzQe3Mybu8phFuOu2jcp0V7B-sVq_vOwn-rZn5JBJagtcbli-eYUahR0XRJcv1f_8MrMxvYyOJQU5lzuvxfXmo1b0i6J7rBz3008YIi02sV3gbmoiY-qrW44S7m6dZetVQVgRbI8sgqM7m4SobYbanMzd7v3Yd_tbWW_aYgdQ651m86RVcyt91ofbWv1_UmiV-mYfomgevm4Z0Y-sVSVcO4raUfx_cB20ap4qgFSu3jYbYFodtOsx_1WrVInRlaWZ8WqWyfZpuFFbVew2pBfFWzzebk_jBIk_8h3mZ_jFMjVV5wXya1msrfeiJdtptzn2viOQ9RUp")
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
                log::warn!("❌⚫ Unexpected status code: {} ❌⚫", response.code);
                return Err(http::Error::Unknown)
            }


			// Self::fetch_word_and_send_signed("fcx-text-bucket1".to_string());

			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			signer.send_signed_transaction(|_account| {
				Call::create_bucket { name: value.clone()}
			});

    		Ok(value)
		}

		fn _create_folder_in_bucket() -> Result<String, http::Error> {
		

			let json_data = r#"{"name": "text-folder-newest"}"#;
			
			let url = "https://storage.googleapis.com/storage/v1/b/fcx-text-bucket1/managedFolders";
			
		
			let request = http::Request::post(url, vec![json_data])
				.add_header("Authorization", "Bearer ya29.c.c0AY_VpZgaiaxV_kIuHiFgyhIJT9RYVvaYKcZM3zFIPe5LVrPLg7G0_m_U89a16j6WKAykpt7OIqcthKvzcrw0p1S7Rpz2uTUpMJSnQ7dTf0PAbQWH3z4Hsvqzt735wup4OeWJvx8m17yJALOrqzh6V56KhY0ffgpbUbhB2fgYe_J4jFO5MLhbGCA-A_0oa8Gw3EtpBMWEmbrDGuyAiDzTdsHVYLBp30I3ZZdUsC7NV-KRs9HLjSUHrzoyvzPMVmVKPPO0j06vDvF2TQGqZUAzsPyH3RruOOHh8F1xgs8jiI7qGV7Op6qISlYhDXdqyPtzpcCCTEMtbODDa8P9wW3yttZQLqpn1YKIC8eTx1Ep7weXn3BN8hXXpHDy92tnguMRbGBfL397KUyZza3Rpn2Bk4_7jcvl_J17ew56y-fvkl9Qv3QWatyBrBIIBV820_4eFfW3X3V-p0b6OWlrpfnQgt_xSzp5c2sxb2YzShucefYZqsOwrY-fQ-oVhIWjsU_6Zu8_VZfkeQ0Szktu7zzV_bIYp1cV4hfcOamoojbkj5F90V7bY3xu4bc8wQXiM-Rok3w7OZytb6Y5dbh-Vl8W5-5rt8y41Fs_MehzznMnBfabWrrYZ4yyjxu_8kjOwYJfdVRM_o5FatO89x0ju7fdIg7OghR0FvrY41leuV9rbm9Zjf0fq7n8iZ427kQZWXWQJMzgsWYbBRM5287SuFt5r7wWZfxody5lrilopjlfV8dzkgtsafo48eo9SokBp9BOQzwV-8Qazsk6pyj4wygM99V0q7v53ge3q8x2qwlbgxr1uM6kW83m2WIW0iQXxozc9QwtMzzmxh4Mb_tgZthFt2nZZ7dqxZwkwjs8o_92atbyZtUerrQd1oe3lnVg2-caXM33y0iZ-Fj-wIJvrWe3jX4_uJ6061xobYR6uOQtQOt67M8ebr42eU_taum-ylQ2XYBSxJVI6W4gopze0Xid53IojWfgeupleoozZ4pQwrlgfbFct-squth")
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

		fn bucket_delete() -> Result<u16, http::Error> {
			let deadline = offchain::timestamp().add(Duration::from_millis(2_000));

			let bucket_delete = BucketDelete::<T>::try_get();

			let value = match bucket_delete {
				Ok(res) => res,
				Err(_) => String::new(), 
			};

			log::info!("bucket name to be deleted: {:?}",value);
			// let key = "AIzaSyBzfdSX2mR2P5_4PidvjH6zWqNON_t-eXE";

			let url = format!("https://storage.googleapis.com/storage/v1/b/{}?key=AIzaSyDuOP9OCj8TWIFfRauNW6KEMQa9H332GhU", value.clone());

			let json_body = r#"{}"#;

			//send post request to file upload
			let delete_requsest = http::Request::default().method(Method::Delete)
            .url(url.as_str())
            .add_header("Authorization", "Bearer ya29.c.c0AY_VpZhFDtmRP48p82xlqwJTgVKTaDC0QhHdRZrx3r_tK6r0aiGzMZhNJ-cKBEFpzgi5SpT3a3-FCXxSI4MTNcJgByaDx82GWBic9aGj7r5s7-zo1cmXm2QBWnmxWg5-5xHyrVWEX7ypaeOXfbazkeC-v4DLOTxodFgAMr8N3fmBhCBmeGqgaP21Sqa7is8s3ArGX3CeEIFvdZE4V3AuEoWT0e-wVOYPh5Gd9agQK_tvMcC1Mu_QLkvfiNCN4udbr-eK2yIcGtYZgDhuwxaS1tIzUIeA9KwlexDcE7t5fwIT-mJnzYS-YNE2qCd1yaIuMNJNoUP368hyi4dSIdT9sdmHywhVKADrmcL2mWEAGoQCWmSUTnMEOgE4wYdhiaNxlz5KJAL399Kk75Wl70Yjr5suIlpimsFy6dV-wvwV8j6qrXsk8XXggIbz2Vm-6lI_UscI5ROpvxcxJggJXvjlyo07tYzp2ilkfnQZwOg9g4yyiwzplU8YBXr1-x8BunO1_cX9p-iIWksZMOfUzSbp8J77MWFvx-W8OU1QIUa5Qi49aOmZfnX7uBVflltcvkBZiXc2hYFdsj9hmiRszd0665WS4210Bnpz9SRJ5jR2SBb9JII8JMhX1jhZyFhJhOpZ4qtkIXXcIa3-Oe6_3-in-ijjceUjoZw83aaYv9RtFWuyV7nzQe3Mybu8phFuOu2jcp0V7B-sVq_vOwn-rZn5JBJagtcbli-eYUahR0XRJcv1f_8MrMxvYyOJQU5lzuvxfXmo1b0i6J7rBz3008YIi02sV3gbmoiY-qrW44S7m6dZetVQVgRbI8sgqM7m4SobYbanMzd7v3Yd_tbWW_aYgdQ651m86RVcyt91ofbWv1_UmiV-mYfomgevm4Z0Y-sVSVcO4raUfx_cB20ap4qgFSu3jYbYFodtOsx_1WrVInRlaWZ8WqWyfZpuFFbVew2pBfFWzzebk_jBIk_8h3mZ_jFMjVV5wXya1msrfeiJdtptzn2viOQ9RUp");

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
				log::warn!("❌ ❌ ❌ ❌Unexpected status code when deleting bucket: {} ❌ ❌ ❌ ❌", response.code);
				return Err(http::Error::Unknown)
			}

			//call signed transaction
			let signer = Signer::<T, T::AuthorityId>::all_accounts();

			signer.send_signed_transaction(|_account| {
				Call::delete_bucket { bucketname: value.clone()}
			});

			
			Ok(response.code)


		}
		
		fn file_upload() -> Result<(), http::Error> {
			//set deadline
			let deadline = offchain::timestamp().add(Duration::from_millis(2_000));

			let file_fetch = FileSave::<T>::try_get();

			let value = match file_fetch {
				Ok(res) => res,
				Err(_) => File{file: Vec::new(), file_name: "".to_string(), bucket_name: "".to_string() }, //retuen an empty vec
			};

			let file_name = value.file_name;
			let file = value.file;
			let bucket_name = value.bucket_name;

			let url = format!("https://storage.googleapis.com/upload/storage/v1/b/{}/o?uploadType=media&name={}", bucket_name, file_name);

			//send post request to file upload
			let request = http::Request::post(url.as_str(), vec![file.clone()])
			.add_header("Authorization", "Bearer ya29.c.c0AY_VpZiW7QTteGyxMKTLkr7i0LhiZgM_ExQLl1K-JRFC531Tnm4RMehdSHnKOHbCdxuAp63vnnftWRn3FJb9b0rt-qNo9q7LdGf2s9rxmOoG70A-ilb0VyzPe1K0m1xAq7LAtGvLDF8u0DT5aFf2Pp1VodKAYt0PlURP7ZSyyDxbJOoVtjCjT1c2CxoYNGtgSkERfVOw2Mye2jMM9RDlIgdvTL-V8M3D_lYcqnDDXJrwWpkzHWwAosC3F_ctTsJkijy2dH5ufmio49oyDJrRdQMgHXRjDRf1HveJlmNdjNoBjsvoQeKtOGd2JfXQDDe_oewQEVAC_YHNmL71tk5cy2OKbcKjSjg5vvGrWkFHo7Q_1TBaOp4dnqAsYlgziO0lxya3AgH399PbSqX25gbZ52vVopv0ecikhSZfV5V4goWxvYuhyxFX2-Q2rrXM9jgcSlShQjJuB85eBI6j2YgF73ardrrlVkQgwBmSUpShJ64ItiaheRM-f9tmkJk_vq1Vi0IszxqgcVsxSRsaYRuuaUJ7Q0Qrtci2v7cd1BUXcwlhzmgvsrxtXuuJm4Bz409VsV-Ylo08edn40k6J33gvY08q7bvFOwOgkorMt5FJbx8mtXM4hdSU10VnZgnuOd9SFiZQk-pvhgnt2nnzdQO1F96qlV08R1fv3cFmFw1Z_tjqi3xJbfWMZvunadYrcrdBtxvjk099vWbMYWUp1vkJcqX86i6o8F39uefppXg-qYYfh5jUUpxcOi01iuoOUrVrhMuSi209Xx9mpnzRjcMt3Mlx8mIdvlQnSJnrc2qM8raIXhi1Oqa-R5jluUqbW-lXgpFgrVBtXBY0fjZjntQj0wZxZnmwzfMaRs9aMO6gb1oFkvbz9rpvVZ9kzv6ftmqO7rctwJk-oY-_-YbYRRUZ5IiVoQxbfU84_U4Zn6JRX_-gn59ju2Byq75trio73S2y4F1_hSe_Yl_B8IiM0yubUi-Q5eJujJxgv5ts7hntI0ms_7OawgYdiBJ")
			.add_header("Content-Type", "image/jpeg");

			let pending = request
			.deadline(deadline)
			.body(vec![file.clone()])
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

			let file_delete = FileDelete::<T>::try_get();

			let value = match file_delete {
				Ok(res) => res,
				Err(_) => String::new(), //retuen an empty vec
			};

			let api_key = "AIzaSyBmwFlnn_aqaPBSkMntWe40qEn4yrY9BIQ";

			let url = format!("https://storage.googleapis.com/storage/v1/b/fcx-text-bucket1/o/{}?key={}", value.clone(), api_key);


			let json_body = r#"{}"#;

			//send post request to file upload
			let delete_requsest = http::Request::default().method(Method::Delete)
            .url(url.as_str())
            .add_header("Authorization", "Bearer ya29.c.c0AY_VpZiW7QTteGyxMKTLkr7i0LhiZgM_ExQLl1K-JRFC531Tnm4RMehdSHnKOHbCdxuAp63vnnftWRn3FJb9b0rt-qNo9q7LdGf2s9rxmOoG70A-ilb0VyzPe1K0m1xAq7LAtGvLDF8u0DT5aFf2Pp1VodKAYt0PlURP7ZSyyDxbJOoVtjCjT1c2CxoYNGtgSkERfVOw2Mye2jMM9RDlIgdvTL-V8M3D_lYcqnDDXJrwWpkzHWwAosC3F_ctTsJkijy2dH5ufmio49oyDJrRdQMgHXRjDRf1HveJlmNdjNoBjsvoQeKtOGd2JfXQDDe_oewQEVAC_YHNmL71tk5cy2OKbcKjSjg5vvGrWkFHo7Q_1TBaOp4dnqAsYlgziO0lxya3AgH399PbSqX25gbZ52vVopv0ecikhSZfV5V4goWxvYuhyxFX2-Q2rrXM9jgcSlShQjJuB85eBI6j2YgF73ardrrlVkQgwBmSUpShJ64ItiaheRM-f9tmkJk_vq1Vi0IszxqgcVsxSRsaYRuuaUJ7Q0Qrtci2v7cd1BUXcwlhzmgvsrxtXuuJm4Bz409VsV-Ylo08edn40k6J33gvY08q7bvFOwOgkorMt5FJbx8mtXM4hdSU10VnZgnuOd9SFiZQk-pvhgnt2nnzdQO1F96qlV08R1fv3cFmFw1Z_tjqi3xJbfWMZvunadYrcrdBtxvjk099vWbMYWUp1vkJcqX86i6o8F39uefppXg-qYYfh5jUUpxcOi01iuoOUrVrhMuSi209Xx9mpnzRjcMt3Mlx8mIdvlQnSJnrc2qM8raIXhi1Oqa-R5jluUqbW-lXgpFgrVBtXBY0fjZjntQj0wZxZnmwzfMaRs9aMO6gb1oFkvbz9rpvVZ9kzv6ftmqO7rctwJk-oY-_-YbYRRUZ5IiVoQxbfU84_U4Zn6JRX_-gn59ju2Byq75trio73S2y4F1_hSe_Yl_B8IiM0yubUi-Q5eJujJxgv5ts7hntI0ms_7OawgYdiBJ")
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
				Call::delete_file { filename: value.clone()}
			});

			
			Ok(response.code)


		}



		/// A helper function to fetch the word and send signed transaction.
		// pub fn fetch_word_and_send_signed(word: String) {
		// 	let signer = Signer::<T, T::AuthorityId>::all_accounts();

		// 	let results = signer.send_signed_transaction(|_account| {
		// 		Call::save_bucket_name { name: word.clone() }
		// 	});

		// 	for (acc, res) in &results {
		// 		match res {
		// 			Ok(()) => log::info!("{:?} Word fetch success: {}.", acc.id, word),
		// 			Err(e) =>
		// 				log::error!("{:?}: submit transaction failure. Reason: {:?}", acc.id, e),
		// 		}
		// 	}
		// }

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
