use codec::Decode;
use frame_system::offchain::{SubmitSignedTransaction, SubmitUnsignedTransaction};
use node_runtime::{Call, Executive, Indices, Runtime, SubmitTransaction, UncheckedExtrinsic};
use pallet_im_online::sr25519::AuthorityPair as Key;
use sp_application_crypto::AppKey;
use sp_core::offchain::{testing::TestTransactionPoolExt, TransactionPoolExt};
use sp_core::testing::KeyStore;
use sp_core::traits::KeystoreExt;

pub mod common;
use self::common::*;

#[test]
fn should_submit_unsigned_transaction() {
	let mut t = new_test_ext(COMPACT_CODE, false);
	let (pool, state) = TestTransactionPoolExt::new();
	t.register_extension(TransactionPoolExt::new(pool));

	t.execute_with(|| {
		let signature = Default::default();
		let heartbeat_data = pallet_im_online::Heartbeat {
			block_number: 1,
			network_state: Default::default(),
			session_index: 1,
			authority_index: 0,
		};

		let call = pallet_im_online::Call::heartbeat(heartbeat_data, signature);
		<SubmitTransaction as SubmitUnsignedTransaction<Runtime, Call>>::submit_unsigned(call).unwrap();

		assert_eq!(state.read().transactions.len(), 1)
	});
}

const PHRASE: &str = "news slush supreme milk chapter athlete soap sausage put clutch what kitten";

#[test]
fn should_submit_signed_transaction() {
	let mut t = new_test_ext(COMPACT_CODE, false);
	let (pool, state) = TestTransactionPoolExt::new();
	t.register_extension(TransactionPoolExt::new(pool));

	let keystore = KeyStore::new();
	keystore
		.write()
		.sr25519_generate_new(Key::ID, Some(&format!("{}/hunter1", PHRASE)))
		.unwrap();
	keystore
		.write()
		.sr25519_generate_new(Key::ID, Some(&format!("{}/hunter2", PHRASE)))
		.unwrap();
	keystore
		.write()
		.sr25519_generate_new(Key::ID, Some(&format!("{}/hunter3", PHRASE)))
		.unwrap();
	t.register_extension(KeystoreExt(keystore));

	t.execute_with(|| {
		let keys = <SubmitTransaction as SubmitSignedTransaction<Runtime, Call>>::find_all_local_keys();
		assert_eq!(keys.len(), 3, "Missing keys: {:?}", keys);

		let can_sign = <SubmitTransaction as SubmitSignedTransaction<Runtime, Call>>::can_sign();
		assert!(can_sign, "Since there are keys, `can_sign` should return true");

		let call = pallet_balances::Call::transfer(Default::default(), Default::default());
		let results = <SubmitTransaction as SubmitSignedTransaction<Runtime, Call>>::submit_signed(call);

		let len = results.len();
		assert_eq!(len, 3);
		assert_eq!(results.into_iter().filter_map(|x| x.1.ok()).count(), len);
		assert_eq!(state.read().transactions.len(), len);
	});
}

#[test]
fn should_submit_signed_twice_from_the_same_account() {
	let mut t = new_test_ext(COMPACT_CODE, false);
	let (pool, state) = TestTransactionPoolExt::new();
	t.register_extension(TransactionPoolExt::new(pool));

	let keystore = KeyStore::new();
	keystore
		.write()
		.sr25519_generate_new(Key::ID, Some(&format!("{}/hunter1", PHRASE)))
		.unwrap();
	t.register_extension(KeystoreExt(keystore));

	t.execute_with(|| {
		let call = pallet_balances::Call::transfer(Default::default(), Default::default());
		let results = <SubmitTransaction as SubmitSignedTransaction<Runtime, Call>>::submit_signed(call);

		let len = results.len();
		assert_eq!(len, 1);
		assert_eq!(results.into_iter().filter_map(|x| x.1.ok()).count(), len);
		assert_eq!(state.read().transactions.len(), 1);

		// submit another one from the same account. The nonce should be incremented.
		let call = pallet_balances::Call::transfer(Default::default(), Default::default());
		let results = <SubmitTransaction as SubmitSignedTransaction<Runtime, Call>>::submit_signed(call);

		let len = results.len();
		assert_eq!(len, 1);
		assert_eq!(results.into_iter().filter_map(|x| x.1.ok()).count(), len);
		assert_eq!(state.read().transactions.len(), 2);

		// now check that the transaction nonces are not equal
		let s = state.read();
		fn nonce(tx: UncheckedExtrinsic) -> frame_system::CheckNonce<Runtime> {
			let extra = tx.signature.unwrap().2;
			extra.3
		}
		let nonce1 = nonce(UncheckedExtrinsic::decode(&mut &*s.transactions[0]).unwrap());
		let nonce2 = nonce(UncheckedExtrinsic::decode(&mut &*s.transactions[1]).unwrap());
		assert!(
			nonce1 != nonce2,
			"Transactions should have different nonces. Got: {:?}",
			nonce1
		);
	});
}

#[test]
fn submitted_transaction_should_be_valid() {
	use codec::Encode;
	use frame_support::storage::StorageMap;
	use sp_runtime::traits::StaticLookup;
	use sp_runtime::transaction_validity::ValidTransaction;

	let mut t = new_test_ext(COMPACT_CODE, false);
	let (pool, state) = TestTransactionPoolExt::new();
	t.register_extension(TransactionPoolExt::new(pool));

	let keystore = KeyStore::new();
	keystore
		.write()
		.sr25519_generate_new(Key::ID, Some(&format!("{}/hunter1", PHRASE)))
		.unwrap();
	t.register_extension(KeystoreExt(keystore));

	t.execute_with(|| {
		let call = pallet_balances::Call::transfer(Default::default(), Default::default());
		let results = <SubmitTransaction as SubmitSignedTransaction<Runtime, Call>>::submit_signed(call);
		let len = results.len();
		assert_eq!(len, 1);
		assert_eq!(results.into_iter().filter_map(|x| x.1.ok()).count(), len);
	});

	// check that transaction is valid, but reset environment storage,
	// since CreateTransaction increments the nonce
	let tx0 = state.read().transactions[0].clone();
	let mut t = new_test_ext(COMPACT_CODE, false);
	t.execute_with(|| {
		let extrinsic = UncheckedExtrinsic::decode(&mut &*tx0).unwrap();
		// add balance to the account
		let author = extrinsic.signature.clone().unwrap().0;
		let address = Indices::lookup(author).unwrap();
		let data = pallet_balances::AccountData {
			free: 5_000_000_000_000,
			..Default::default()
		};
		let account = frame_system::AccountInfo {
			nonce: 0u32,
			refcount: 0u8,
			data,
		};
		<frame_system::Account<Runtime>>::insert(&address, account);

		// check validity
		let res = Executive::validate_transaction(extrinsic);

		assert_eq!(
			res.unwrap(),
			ValidTransaction {
				priority: 2_411_002_000_000,
				requires: vec![],
				provides: vec![(address, 0).encode()],
				longevity: 127,
				propagate: true,
			}
		);
	});
}
