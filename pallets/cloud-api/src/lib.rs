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
	pub struct BucketS {
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

	//Folder structure
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo, Debug)]
	pub struct Folder {
		pub folder_name: String,
		pub bucket_name: String,
	}

	//File structure
	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo, Debug)]
	pub struct FileDown {
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
	pub type FolderCreate<T> = StorageValue<_, Folder>;

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

	#[pallet::storage]
	#[pallet::getter(fn fdown)]
	pub type FileDownload<T> = StorageValue<_, FileDown>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BucketCreated { name: T::AccountId },
		FolderCreated { fname: T::AccountId },
		FileFetched { file: T::AccountId },
		FileDeleted { file: T::AccountId },
		BucketDeleted { name: T::AccountId },
		FileDownloaded { file: T::AccountId },
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
			// 	Ok(word) => log::info!("📁✅ Folder: {:?} created successfully📁✅", word),
			// 	Err(_) => log::info!("📁❌ Error creating folder 📁❌"),
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

			// match Self::file_download() {
			// 	Ok(_) => log::info!(" 📁 ✅ 〰️〰️ File download 〰️〰️ 📁 ✅"),
			// 	Err(error) => log::info!("📁 ➡️ ⭕⭕ ❌❌❌❌ Error file downloading ===> : {:#?} ⭕⭕ ❌❌❌❌", error)
			// }

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
		pub fn create_folder(origin: OriginFor<T>,folder_name: String, bucket_name: String) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let new_name = Folder { folder_name, bucket_name };

			<FolderCreate<T>>::put(new_name);

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

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn download_file(origin: OriginFor<T>,file_name: String, bucket_name: String) -> DispatchResult {
			
			log::info!("Hello from download file");
			
			let sender = ensure_signed(origin)?;

			let new_file = FileDown { file_name, bucket_name };

			<FileDownload<T>>::put(new_file);
			
			// <FileStore<T>>::insert(sender.clone(), filename, _file);

			Self::deposit_event(Event::FileDownloaded { file: sender });

		
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

			let folder_create = FolderCreate::<T>::try_get();

			let value = match folder_create {
				Ok(res) => res,
				Err(_) => Folder{folder_name: String::new(), bucket_name: String::new()}, 
			};

			let bucket_name = value.bucket_name;

			let folder_name = value.folder_name;

			// let json_data = r#"{"name": "text-folder-newest"}"#;

			//let json_data = format!(r#"{{"label": "{}"}}"#, folder_name);

			let json_data = format!(r#"{{"name": "{}", "folder": "{}"}}"#, folder_name, true);

			
			let url = format!("https://www.googleapis.com/storage/v1/b/{}/o", bucket_name);
			
		
			// let request = http::Request::patch(url.as_str(), vec![json_data.clone()])
			// 	.add_header("Authorization", "Bearer ya29.c.c0AY_VpZgoWbKGLPmCzClXU0SWItAji_pxxTAJxiqbXcmxQj70N6n0tGMflsFIhj3J-PSX5bj3JD_yAWymt4b0ymtCDXrvB26TJYwjpd7JWyLYOE3NM4lGxjzswfoeDoDxVpkM6sZi23ZSE_0w8VwQ8ptAodFDv9f9lrBK6if7v3NfZzcgDw480WQIDQtnEcLxAUA52XdWUENcCleym7B6ZS1b9Yd7SvqFIPE0J53ESdMhNEwKctSOlfrcQc9L2JEQCTXZmlCOCUs2TJt2lTcefgmG4ajNCAy-Vxk20N-J9SLIvq-xaWoJsNOWQU_RBheucN6LcEXCEp-jn_7O2QENSKZglW9BE1axgdsrPQVEmRPrtnRxA6H-ys5liVyNVIZFVee-7gIH400AkqsIngOh_Zs7JunbnFd7jOdxi1c5pzm5SB8d9kY6Sn37OpZ4msUMZMMJrshIlupe2rym8haxu4aQbWc1syZdtXJFYrta-Xwbo1VihiVu6Fxy8pQbaR-M6rlr9kMiaR8B2shd9pgpR-rcJdzRJiI-U5mXviupJ3eoRzZwdOWkJecuzOBxap36Zyl4Fefn2403YJQs2d5z5yli-sJg1_87khkh1ghzw-m6ra_Ur-M8VMcZMxYSgOgl70Bukav1njFF_-yV2j8sOYOcnp9YZX1tgYgZfVbBjq3ZFMoc0XqmoX0Mdw9rq25OnSbZMwZUj-nOrvhxypM30YeJ4psQbJ-yRgb9fM6nvdZ_vMvJF-3tRJm0Mart6OmsIJlI3BpcBsV8wxrMuQhXrFe-9atn-Mf05w4ivtoQvQRRrZog8oFonZ83i8YeIqJV5m3fR-6iwI3XVQQJjljj9c2slUx93_ik-cl53W6lqhqVYsinvrtfJFwjiBlc29QUR6uoy2ky5u7W0-kWZIiYk-YnWuJ6Q33qvIaJusov5-SpjnIw2V7BX6n-8BWR9Qs15jXM1f4izuRzfwWeRnSQhoZa1Sxl4hdtMUzwnvmlaq4lm2ZJnRrif3X")
			// 	.add_header("Content-Type", "application/json");


			let request = http::Request::post(url.as_str(), vec![json_data.clone()])
            .add_header("Authorization", "Bearer ya29.c.c0AY_VpZjH01bqq9e5OS39VeB0-mckGh4A_rfUIAuDnJu7HEDOMNhJFR4CEya2uNMU475itsAHx8fZS9aZ1CXk2QbRakb3z8dN7nvY9RoFvKz5-nADXrOaFpEk69yVORbP644DKK9s5tacMUgdbCkzsmqLz-8qRFYpbSyJU2dmmcgffDKhchDUH8Nt3vTKv7rPZnC-2Zrm5EPJSJ2GsIKJb_C8GpmZ1oCAhlpdrx345FDGUicgL0r_xxtQBgJFURcougDaA-mxlAGGkEpAm9PG_N20rEq-0k6RoJxJGr0iCgUEZkjrJUOz6PC3ySSMckmTbJbcRueYOYR8ky1RevVcCrzm2gMJHJ6wahMvRFqy1FMVmeYiQG08MPvwq3hrXA8QXPjtT397P09lI2IxgkRBu5Vz0n_6hWjoSn0-oSk0Vd5R20BqibQtqV46eqxzzqla8_XMYySx5Oy8QQdJr7xIZznXZFuVQZh0_cl4j3FwFud1R3BxyhOzvpx8yRo8hjiW_MdoIid53y4wv_FQSzytZpwaQ8gpZFB1h531zhkvU4liajXsha1Vbu8qestuzeJdVsRtZpnkk_ql3RZYM8hZ8rtRQzxF07czjd2gvi_9IUg1qcuUYkMjweXdRzRm3nZYglZsIZVxyeaM_YeO5QR7ex9rRbud82hslecUXgXe0okzpkUicrjM5r7FqJZ1li3OymklkupRc1JnRXx2o2IoqblqrugJtYXJdISuYJSxQoFakBe0r3nJX88q7BJ7cOn4Qkelw46d9p03lVRQ4m59F4Z3h0u9dBXhxX51h7nRalhjWj60zx1wirRi_rI7iMddSjjB3j8tx9nmRXkS7bg8tcOkap2kSaZxbUV98YFpdbbzgnRR3qylZS5fmgm4zSqMXike1ta4wVe719zbblR3Jik3Zxi4VbbqIyp74fUajF899uyq_YnrBFSB_3jUuI_pdnkr13O-mBwWpbqqajg2MRFeJ73S8Iae7UY0IVcuU32elfSshWi1kF8")
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

			log::info!(" 📁 Status code: {:#?}", response.code);

			if response.code != 200 {
				log::warn!("📁🔴 Unexpected status code: {} 📁🔴", response.code);
				return Err(http::Error::Unknown);
			}
			
		
			Ok(folder_name)
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

		// fn file_download() -> Result<(), http::Error> {
		// 	//set deadline
		// 	let deadline = offchain::timestamp().add(Duration::from_millis(2_000));

		// 	let file_down = FileDownload::<T>::try_get();

		// 	let value = match file_down {
		// 		Ok(res) => res,
		// 		Err(_) => FileDown{file_name: "".to_string(), bucket_name: "".to_string() }, 
		// 	};

		// 	let file_name = value.file_name;
		// 	let bucket_name = value.bucket_name;

		// 	let url = format!("https://storage.googleapis.com/storage/v1/b/{}/o/{}?alt=media", bucket_name, file_name);

		// 	//send post request to file upload
		// 	let request = http::Request::get(url.as_str())
		// 	.add_header("Authorization", "Bearer ya29.c.c0AY_VpZg8bc3C9wddUNwsEobdH4juj-xwHLKQ64jCQOx5HtEs9f7958cotkhYcDely3pTFTCSgrEuKWLPg864In0HPtdvW4ATegVv9M_TaXm9RiAGhtvXdbi11oTn5cpu-bv2pw7yvi-CUdIpkkDfnNgPSTJRUONEVCfQp1PTYfhVytvB1S_qRV5Gg_sBPqPRvPpgJwO4YhUwPDi86uZ4vbGZhDcEHeAgaw-d3kl0bxVLiP98JAasHy7s9LhYbhOg4vOWTvvHhFVuNQbo1d28LGXFNrSossr6mrOfUEy8KR_rt_2Rl9REgan48uyivXh8l5ZFE9kN96xbAtAV_iI8vxzN2xbi9D1jH5k3ZwNKMfw53vEhvXrkrC-8V8kaE_OVAAV0YY0E400Aw1y186S8yh-zZyjJ8gwgf0_89W0kRZz8--oUUigl1OSBaIwV5xe-p0iinft8QaYd__gu48vaFX_3X1q_es7aMaq12pUMBRrS9dnZlqdXl7respzvoBs67dmWd_Mn1809efI9tUmR0jSXax8ZiV_cfY60hn5hmMX1I5IVM4sMdsqWW816VJ8rlRliRnxIedbRB0rU-ewQRbjVkmIsMtM3j9tfvSuUsUov6lr6Qqjq7mU15Z8Je1mg46J8YzYzZ1_1zwqb0eIOlx7etM7IsvUjmydmS_ZJe_BBFZcx-mWfW3j0R4MzW4B00OR0Wjvaa3Os36B5VIsMYsbe56VZkFtVqu5jn9FnSgRM7rOYbx-hO36sBjXiWJ1UBS9XB95QehvmlmSYe8UyyJolrm3gvWB3_Vsi4S6q2l9Q6jwxhuMsnuxb-w32rj3Be88e9ZUQkog6eSpXzt8JZdg628cJZ3jkk1R15ItmhrOf2thXtmei5s2WgzsdnvIYc4UMRqpvylvb_5amFglpsQe8-ZpqQuvm-xiQIgh5Ua6qd_s3OkO5ZzOuxs4vbkoVI8uZkkgRVywZb3by7BQf5IwpBmqp37I91b0k2Yh96odz_e0s20Isbog");

		// 	let pending = request
		// 	.deadline(deadline)
		// 	.send()
		// 	.map_err(|_| http::Error::IoError)?;

		// 	// Wait for response 
		// 	let response = pending
		// 	.try_wait(deadline)
		// 	.map_err(|_| http::Error::DeadlineReached)??;

		// 	log::info!("🖌️Status code: {:#?}", response.code);

		// 	//check response is successfull
		// 	if response.code != 200 {
		// 		log::warn!("🔴 🔴 🔴 🔴 Unexpected status code when downloading file: {} 🔴 🔴 🔴 🔴", response.code);
		// 		return Err(http::Error::Unknown)
		// 	}

		// 	// Save the response body to a file
		// 	let mut file = fs::File::create("/Users/Gimhani/Downloads").map_err(|_| http::Error::IoError)?;
		// 	io::copy(&mut response, &mut file).map_err(|_| http::Error::IoError)?;
		
			
		// 	Ok(())

		// }

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

	}

}
