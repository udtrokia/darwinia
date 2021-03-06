//! Tests for treasury.

// use frame_support::{assert_noop, assert_ok};
// use sp_runtime::traits::OnFinalize;
//
// use crate::{mock::*, *};
//
// #[test]
// fn genesis_config_works() {
// 	new_test_ext().execute_with(|| {
// 		assert_eq!(Treasury::pot::<Ring>(), 0);
// 		assert_eq!(Treasury::pot::<Kton>(), 0);
// 		assert_eq!(Treasury::proposal_count(), 0);
// 	});
// }
//
// #[test]
// fn minting_works() {
// 	new_test_ext().execute_with(|| {
// 		// Check that accumulate works when we have Some value in Dummy already.
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		assert_eq!(Treasury::pot::<Ring>(), 100);
// 		assert_eq!(Treasury::pot::<Kton>(), 0);
//
// 		// Make sure kton and ring have different storages
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
// 		assert_eq!(Treasury::pot::<Kton>(), 100);
// 	});
// }
//
// #[test]
// fn spend_proposal_takes_min_deposit() {
// 	new_test_ext().execute_with(|| {
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 1, 1, 3));
// 		assert_eq!(Ring::free_balance(&0), 99);
// 		assert_eq!(Kton::free_balance(&0), 99);
// 		assert_eq!(Ring::reserved_balance(&0), 1);
// 		assert_eq!(Kton::reserved_balance(&0), 1);
// 	});
// }
//
// #[test]
// fn spend_proposal_takes_proportional_deposit() {
// 	new_test_ext().execute_with(|| {
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 100, 100, 3));
// 		assert_eq!(Ring::free_balance(&0), 95);
// 		assert_eq!(Kton::free_balance(&0), 95);
// 		assert_eq!(Ring::reserved_balance(&0), 5);
// 		assert_eq!(Kton::reserved_balance(&0), 5);
// 	});
// }
//
// #[test]
// fn spend_proposal_fails_when_proposer_poor() {
// 	new_test_ext().execute_with(|| {
// 		assert_noop!(
// 			Treasury::propose_spend(Origin::signed(2), 100, 100, 3),
// 			Error::<Test>::InsufficientProposersBalance,
// 		);
// 	});
// }
//
// #[test]
// fn accepted_spend_proposal_ignored_outside_spend_period() {
// 	new_test_ext().execute_with(|| {
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
//
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 100, 100, 3));
// 		assert_ok!(Treasury::approve_proposal(Origin::ROOT, 0));
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(1);
// 		assert_eq!(Ring::free_balance(&3), 0);
// 		assert_eq!(Kton::free_balance(&3), 0);
// 		assert_eq!(Treasury::pot::<Ring>(), 100);
// 		assert_eq!(Treasury::pot::<Kton>(), 100);
// 	});
// }
//
// #[test]
// fn unused_pot_should_diminish() {
// 	new_test_ext().execute_with(|| {
// 		let init_total_ring_issuance = Ring::total_issuance();
// 		let init_total_kton_issuance = Kton::total_issuance();
//
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
//
// 		assert_eq!(Ring::total_issuance(), init_total_ring_issuance + 100);
// 		assert_eq!(Kton::total_issuance(), init_total_kton_issuance + 100);
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(2);
// 		assert_eq!(Treasury::pot::<Ring>(), 50);
// 		assert_eq!(Treasury::pot::<Kton>(), 50);
// 		assert_eq!(Ring::total_issuance(), init_total_ring_issuance + 50);
// 		assert_eq!(Kton::total_issuance(), init_total_kton_issuance + 50);
// 	});
// }
//
// #[test]
// fn rejected_spend_proposal_ignored_on_spend_period() {
// 	new_test_ext().execute_with(|| {
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
//
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 100, 100, 3));
// 		assert_ok!(Treasury::reject_proposal(Origin::ROOT, 0));
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(2);
// 		assert_eq!(Ring::free_balance(&3), 0);
// 		assert_eq!(Kton::free_balance(&3), 0);
// 		assert_eq!(Treasury::pot::<Ring>(), 50);
// 		assert_eq!(Treasury::pot::<Kton>(), 50);
// 	});
// }
//
// #[test]
// fn reject_already_rejected_spend_proposal_fails() {
// 	new_test_ext().execute_with(|| {
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
//
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 100, 100, 3));
// 		assert_ok!(Treasury::reject_proposal(Origin::ROOT, 0));
// 		assert_noop!(
// 			Treasury::reject_proposal(Origin::ROOT, 0),
// 			Error::<Test>::InvalidProposalIndex
// 		);
// 	});
// }
//
// #[test]
// fn reject_non_existant_spend_proposal_fails() {
// 	new_test_ext().execute_with(|| {
// 		assert_noop!(
// 			Treasury::reject_proposal(Origin::ROOT, 0),
// 			Error::<Test>::InvalidProposalIndex
// 		);
// 	});
// }
//
// #[test]
// fn accept_already_rejected_spend_proposal_fails() {
// 	new_test_ext().execute_with(|| {
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
//
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 100, 100, 3));
// 		assert_ok!(Treasury::reject_proposal(Origin::ROOT, 0));
// 		assert_noop!(
// 			Treasury::approve_proposal(Origin::ROOT, 0),
// 			Error::<Test>::InvalidProposalIndex
// 		);
// 	});
// }
//
// #[test]
// fn accepted_spend_proposal_enacted_on_spend_period() {
// 	new_test_ext().execute_with(|| {
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
// 		assert_eq!(Treasury::pot::<Ring>(), 100);
// 		assert_eq!(Treasury::pot::<Kton>(), 100);
//
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 100, 100, 3));
// 		assert_ok!(Treasury::approve_proposal(Origin::ROOT, 0));
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(2);
// 		assert_eq!(Ring::free_balance(&3), 100);
// 		assert_eq!(Kton::free_balance(&3), 100);
// 		assert_eq!(Treasury::pot::<Ring>(), 0);
// 		assert_eq!(Treasury::pot::<Kton>(), 0);
// 	});
// }
//
// #[test]
// fn pot_underflow_should_not_diminish() {
// 	new_test_ext().execute_with(|| {
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
// 		assert_eq!(Treasury::pot::<Ring>(), 100);
// 		assert_eq!(Treasury::pot::<Kton>(), 100);
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 150, 150, 3));
// 		assert_ok!(Treasury::approve_proposal(Origin::ROOT, 0));
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(2);
// 		assert_eq!(Treasury::pot::<Ring>(), 100); // Pot hasn't changed
// 		assert_eq!(Treasury::pot::<Kton>(), 100); // Pot hasn't changed
//
// 		let _ = Ring::deposit_into_existing(&Treasury::account_id(), 100).unwrap();
// 		let _ = Kton::deposit_into_existing(&Treasury::account_id(), 100).unwrap();
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(4);
// 		assert_eq!(Ring::free_balance(&3), 150); // Fund has been spent
// 		assert_eq!(Kton::free_balance(&3), 150); // Fund has been spent
// 		assert_eq!(Treasury::pot::<Ring>(), 25); // Pot has finally changed
// 		assert_eq!(Treasury::pot::<Kton>(), 25); // Pot has finally changed
// 	});
// }
//
// // Treasury account doesn't get deleted if amount approved to spend is all its free balance.
// // i.e. pot should not include existential deposit needed for account survival.
// #[test]
// fn treasury_account_doesnt_get_deleted() {
// 	new_test_ext().execute_with(|| {
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
//
// 		assert_eq!(Treasury::pot::<Ring>(), 100);
// 		assert_eq!(Treasury::pot::<Kton>(), 100);
//
// 		let ring_treasury_balance = Ring::free_balance(&Treasury::account_id());
// 		let kton_treasury_balance = Kton::free_balance(&Treasury::account_id());
//
// 		assert_ok!(Treasury::propose_spend(
// 			Origin::signed(0),
// 			ring_treasury_balance,
// 			kton_treasury_balance,
// 			3
// 		));
// 		assert_ok!(Treasury::approve_proposal(Origin::ROOT, 0));
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(2);
// 		assert_eq!(Treasury::pot::<Ring>(), 100); // Pot hasn't changed
// 		assert_eq!(Treasury::pot::<Kton>(), 100); // Pot hasn't changed
//
// 		assert_ok!(Treasury::propose_spend(
// 			Origin::signed(0),
// 			Treasury::pot::<Ring>(),
// 			Treasury::pot::<Kton>(),
// 			3
// 		));
// 		assert_ok!(Treasury::approve_proposal(Origin::ROOT, 1));
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(4);
// 		assert_eq!(Treasury::pot::<Ring>(), 0); // Pot is emptied
// 		assert_eq!(Treasury::pot::<Kton>(), 0); // Pot is emptied
// 		assert_eq!(Ring::free_balance(&Treasury::account_id()), 1); // but the account is still there
// 		assert_eq!(Kton::free_balance(&Treasury::account_id()), 1); // but the account is still there
// 	});
// }
//
// // In case treasury account is not existing then it works fine.
// // This is usefull for chain that will just update runtime.
// #[test]
// fn inexisting_account_works() {
// 	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
// 	pallet_ring::GenesisConfig::<Test> {
// 		balances: vec![(0, 100), (1, 99), (2, 1)],
// 	}
// 	.assimilate_storage(&mut t)
// 	.unwrap();
//
// 	pallet_kton::GenesisConfig::<Test> {
// 		balances: vec![(0, 100), (1, 99), (2, 1)],
// 	}
// 	.assimilate_storage(&mut t)
// 	.unwrap();
//
// 	// Treasury genesis config is not build thus treasury account does not exist
// 	let mut t: sp_io::TestExternalities = t.into();
// 	t.execute_with(|| {
// 		// Account does not exist
// 		assert_eq!(Ring::free_balance(&Treasury::account_id()), 0);
// 		assert_eq!(Kton::free_balance(&Treasury::account_id()), 0);
//
// 		// Pot is empty
// 		assert_eq!(Treasury::pot::<Ring>(), 0);
// 		assert_eq!(Treasury::pot::<Kton>(), 0);
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 99, 99, 3));
// 		assert_ok!(Treasury::approve_proposal(Origin::ROOT, 0));
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 1, 1, 3));
// 		assert_ok!(Treasury::approve_proposal(Origin::ROOT, 1));
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(2);
// 		// Pot hasn't changed
// 		assert_eq!(Treasury::pot::<Ring>(), 0);
// 		assert_eq!(Treasury::pot::<Kton>(), 0);
//
// 		// Balance of `3` hasn't changed
// 		assert_eq!(Ring::free_balance(&3), 0);
// 		assert_eq!(Kton::free_balance(&3), 0);
//
// 		Ring::make_free_balance_be(&Treasury::account_id(), 100);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 100);
// 		assert_eq!(Treasury::pot::<Ring>(), 99); // Pot now contains funds
// 		assert_eq!(Treasury::pot::<Kton>(), 99); // Pot now contains funds
// 		assert_eq!(Ring::free_balance(&Treasury::account_id()), 100); // Account does exist
// 		assert_eq!(Kton::free_balance(&Treasury::account_id()), 100); // Account does exist
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(4);
//
// 		assert_eq!(Treasury::pot::<Ring>(), 0); // Pot has changed
// 		assert_eq!(Treasury::pot::<Kton>(), 0); // Pot has changed
// 		assert_eq!(Ring::free_balance(&3), 99); // Balance of `3` has changed
// 		assert_eq!(Kton::free_balance(&3), 99); // Balance of `3` has changed
// 	});
// }
//
// #[test]
// fn no_spent_no_burn() {
// 	// ring
// 	new_test_ext().execute_with(|| {
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
//
// 		assert_eq!(Treasury::pot::<Ring>(), 100);
// 		assert_eq!(Treasury::pot::<Kton>(), 100);
//
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 0, 100, 3));
// 		assert_ok!(Treasury::approve_proposal(Origin::ROOT, 0));
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(2);
// 		assert_eq!(Ring::free_balance(&3), 0);
// 		assert_eq!(Kton::free_balance(&3), 100);
// 		assert_eq!(Treasury::pot::<Ring>(), 50);
// 		assert_eq!(Treasury::pot::<Kton>(), 0);
// 	});
//
// 	// kton
// 	new_test_ext().execute_with(|| {
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
//
// 		assert_eq!(Treasury::pot::<Ring>(), 100);
// 		assert_eq!(Treasury::pot::<Kton>(), 100);
//
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 100, 0, 3));
// 		assert_ok!(Treasury::approve_proposal(Origin::ROOT, 0));
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(2);
// 		assert_eq!(Ring::free_balance(&3), 100);
// 		assert_eq!(Kton::free_balance(&3), 0);
// 		assert_eq!(Treasury::pot::<Ring>(), 0);
// 		assert_eq!(Treasury::pot::<Kton>(), 50);
// 	});
//
// 	// both
// 	new_test_ext().execute_with(|| {
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
//
// 		assert_eq!(Treasury::pot::<Ring>(), 100);
// 		assert_eq!(Treasury::pot::<Kton>(), 100);
//
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 0, 0, 3));
// 		assert_ok!(Treasury::approve_proposal(Origin::ROOT, 0));
//
// 		<Treasury as OnFinalize<u64>>::on_finalize(2);
// 		assert_eq!(Ring::free_balance(&3), 0);
// 		assert_eq!(Kton::free_balance(&3), 0);
// 		assert_eq!(Treasury::pot::<Ring>(), 50);
// 		assert_eq!(Treasury::pot::<Kton>(), 50);
// 	});
// }
//
// /// # Logic Tests.
// ///
// /// **FIXME**: If some logic went wrong.
// ///
// /// + Proposal: A suggestion to allocate funds from the pot to a beneficiary.
// /// + Beneficiary: An account who will receive the funds from a proposal iff the proposal is approved.
// /// + Deposit: Funds that a proposer must lock when making a proposal.
// /// The deposit will be returned or slashed if the proposal is approved or rejected respectively.
// /// + Pot: Unspent funds accumulated by the treasury module.
// #[test]
// fn approve_proposal_no_keep_burning() {
// 	new_test_ext().execute_with(|| {
// 		// backtrace init configs.
// 		assert_eq!(Ring::free_balance(&0), 100);
// 		assert_eq!(Kton::free_balance(&0), 100);
// 		assert_eq!(Ring::free_balance(&1), 98);
// 		assert_eq!(Kton::free_balance(&1), 98);
// 		assert_eq!(Ring::free_balance(&2), 1);
// 		assert_eq!(Kton::free_balance(&2), 1);
// 		assert_eq!(Ring::free_balance(&3), 0);
// 		assert_eq!(Kton::free_balance(&3), 0);
// 		assert_eq!(Treasury::pot::<Ring>(), 0);
// 		assert_eq!(Treasury::pot::<Kton>(), 0);
//
// 		// Ensure an account's free balance equals some value; this will create the account if needed.
// 		// Returns a signed imbalance and status to indicate if the account was successfully updated
// 		// or update has led to killing of the account.
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
// 		assert_eq!(Treasury::pot::<Ring>(), 100);
// 		assert_eq!(Treasury::pot::<Kton>(), 100);
//
// 		// Put forward a suggestion for spending, burn treasury balances to AccontID-3
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 100, 100, 3));
// 		assert_ok!(Treasury::approve_proposal(Origin::ROOT, 0)); // Accept proposal
//
// 		// @0-1: Check balances after `propose_spend`
// 		<Treasury as OnFinalize<u64>>::on_finalize(1);
// 		assert_eq!(Ring::free_balance(&0), 95); // ProposalBond: Permill::from_percent(5);
// 		assert_eq!(Kton::free_balance(&0), 95); // ProposalBond: Permill::from_percent(5);
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 0); // No changes
// 		assert_eq!(Kton::free_balance(&3), 0); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 100); // No changes
// 		assert_eq!(Treasury::pot::<Kton>(), 100); // No changes
//
// 		// @2: On the first spend perid
// 		<Treasury as OnFinalize<u64>>::on_finalize(2); // SpendPeriod: u64 = 2;
// 		assert_eq!(Ring::free_balance(&0), 100); // ProposalBond: Permill::from_percent(5); **return bond**
// 		assert_eq!(Kton::free_balance(&0), 100); // ProposalBond: Permill::from_percent(5); **return bond**
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 100); // No changes
// 		assert_eq!(Kton::free_balance(&3), 100); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 0); // Burn: Permill::from_percent(50); **Burn 100 if approve**
// 		assert_eq!(Treasury::pot::<Kton>(), 0); // Burn: Permill::from_percent(50); **Burn 100 if approve**
//
// 		// @3: Check balances on the perid after spend perid
// 		<Treasury as OnFinalize<u64>>::on_finalize(3);
// 		assert_eq!(Ring::free_balance(&0), 100); // No changes from last perid
// 		assert_eq!(Kton::free_balance(&0), 100); // No changes from last perid
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 100); // No changes
// 		assert_eq!(Kton::free_balance(&3), 100); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 0); // No changes from last perid
// 		assert_eq!(Treasury::pot::<Kton>(), 0); // No changes from last perid
//
// 		// @4: The second spend perid
// 		<Treasury as OnFinalize<u64>>::on_finalize(4);
// 		assert_eq!(Ring::free_balance(&0), 100); // No changes from last perid
// 		assert_eq!(Kton::free_balance(&0), 100); // No changes from last perid
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 100); // No changes
// 		assert_eq!(Kton::free_balance(&3), 100); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 0); // No changes from last perid
// 		assert_eq!(Treasury::pot::<Kton>(), 0); // No changes from last perid
// 	});
// }
//
// #[test]
// fn reject_proposal_keep_burning() {
// 	new_test_ext().execute_with(|| {
// 		// backtrace init configs.
// 		assert_eq!(Ring::free_balance(&0), 100);
// 		assert_eq!(Kton::free_balance(&0), 100);
// 		assert_eq!(Ring::free_balance(&1), 98);
// 		assert_eq!(Kton::free_balance(&1), 98);
// 		assert_eq!(Ring::free_balance(&2), 1);
// 		assert_eq!(Kton::free_balance(&2), 1);
// 		assert_eq!(Ring::free_balance(&3), 0);
// 		assert_eq!(Kton::free_balance(&3), 0);
// 		assert_eq!(Treasury::pot::<Ring>(), 0);
// 		assert_eq!(Treasury::pot::<Kton>(), 0);
//
// 		// Ensure an account's free balance equals some value; this will create the account if needed.
// 		// Returns a signed imbalance and status to indicate if the account was successfully updated
// 		// or update has led to killing of the account.
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
// 		assert_eq!(Treasury::pot::<Ring>(), 100);
// 		assert_eq!(Treasury::pot::<Kton>(), 100);
//
// 		// Put forward a suggestion for spending, burn treasury balances to AccontID-3
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 100, 100, 3));
// 		assert_ok!(Treasury::reject_proposal(Origin::ROOT, 0));
//
// 		// @0-1: Check balances after `propose_spend`
// 		<Treasury as OnFinalize<u64>>::on_finalize(1);
// 		assert_eq!(Ring::free_balance(&0), 95); // ProposalBond: Permill::from_percent(5);
// 		assert_eq!(Kton::free_balance(&0), 95); // ProposalBond: Permill::from_percent(5);
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 0); // No changes
// 		assert_eq!(Kton::free_balance(&3), 0); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 100); // No changes
// 		assert_eq!(Treasury::pot::<Kton>(), 100); // No changes
//
// 		// @2: On the first spend perid
// 		<Treasury as OnFinalize<u64>>::on_finalize(2); // SpendPeriod: u64 = 2;
// 		assert_eq!(Ring::free_balance(&0), 95); // ProposalBond: Permill::from_percent(5);
// 		assert_eq!(Kton::free_balance(&0), 95); // ProposalBond: Permill::from_percent(5);
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 0); // No changes
// 		assert_eq!(Kton::free_balance(&3), 0); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 50); // Burn: Permill::from_percent(50); **The Burned Balances just burned?**
// 		assert_eq!(Treasury::pot::<Kton>(), 50); // Burn: Permill::from_percent(50); **The Burned Balances just burned?**
//
// 		// @3: Check balances on the perid after spend perid
// 		<Treasury as OnFinalize<u64>>::on_finalize(3);
// 		assert_eq!(Ring::free_balance(&0), 95); // No changes from last perid
// 		assert_eq!(Kton::free_balance(&0), 95); // No changes from last perid
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 0); // No changes
// 		assert_eq!(Kton::free_balance(&3), 0); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 50); // No changes from last perid
// 		assert_eq!(Treasury::pot::<Kton>(), 50); // No changes from last perid
//
// 		// @4: The second spend perid
// 		<Treasury as OnFinalize<u64>>::on_finalize(4);
// 		assert_eq!(Ring::free_balance(&0), 95); // No changes from last perid
// 		assert_eq!(Kton::free_balance(&0), 95); // No changes from last perid
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 0); // No changes
// 		assert_eq!(Kton::free_balance(&3), 0); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 25); // No changes from last perid
// 		assert_eq!(Treasury::pot::<Kton>(), 25); // No changes from last perid
// 	});
// }
//
// #[test]
// fn no_accept_no_reject_keep_burning() {
// 	new_test_ext().execute_with(|| {
// 		// backtrace init configs.
// 		assert_eq!(Ring::free_balance(&0), 100);
// 		assert_eq!(Kton::free_balance(&0), 100);
// 		assert_eq!(Ring::free_balance(&1), 98);
// 		assert_eq!(Kton::free_balance(&1), 98);
// 		assert_eq!(Ring::free_balance(&2), 1);
// 		assert_eq!(Kton::free_balance(&2), 1);
// 		assert_eq!(Ring::free_balance(&3), 0);
// 		assert_eq!(Kton::free_balance(&3), 0);
// 		assert_eq!(Treasury::pot::<Ring>(), 0);
// 		assert_eq!(Treasury::pot::<Kton>(), 0);
//
// 		// Ensure an account's free balance equals some value; this will create the account if needed.
// 		// Returns a signed imbalance and status to indicate if the account was successfully updated
// 		// or update has led to killing of the account.
// 		Ring::make_free_balance_be(&Treasury::account_id(), 101);
// 		Kton::make_free_balance_be(&Treasury::account_id(), 101);
// 		assert_eq!(Treasury::pot::<Ring>(), 100);
// 		assert_eq!(Treasury::pot::<Kton>(), 100);
//
// 		// Put forward a suggestion for spending, burn treasury balances to AccontID-3
// 		assert_ok!(Treasury::propose_spend(Origin::signed(0), 100, 100, 3));
//
// 		// @0-1: Check balances after `propose_spend`
// 		<Treasury as OnFinalize<u64>>::on_finalize(1);
// 		assert_eq!(Ring::free_balance(&0), 95); // ProposalBond: Permill::from_percent(5);
// 		assert_eq!(Kton::free_balance(&0), 95); // ProposalBond: Permill::from_percent(5);
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 0); // No changes
// 		assert_eq!(Kton::free_balance(&3), 0); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 100); // No changes
// 		assert_eq!(Treasury::pot::<Kton>(), 100); // No changes
//
// 		// @2: On the first spend perid
// 		<Treasury as OnFinalize<u64>>::on_finalize(2); // SpendPeriod: u64 = 2;
// 		assert_eq!(Ring::free_balance(&0), 95); // ProposalBond: Permill::from_percent(5);
// 		assert_eq!(Kton::free_balance(&0), 95); // ProposalBond: Permill::from_percent(5);
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 0); // No changes
// 		assert_eq!(Kton::free_balance(&3), 0); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 50); // Burn: Permill::from_percent(50); **The Burned Balances just burned?**
// 		assert_eq!(Treasury::pot::<Kton>(), 50); // Burn: Permill::from_percent(50); **The Burned Balances just burned?**
//
// 		// @3: Check balances on the perid after spend perid
// 		<Treasury as OnFinalize<u64>>::on_finalize(3);
// 		assert_eq!(Ring::free_balance(&0), 95); // No changes from last perid
// 		assert_eq!(Kton::free_balance(&0), 95); // No changes from last perid
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 0); // No changes
// 		assert_eq!(Kton::free_balance(&3), 0); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 50); // No changes from last perid
// 		assert_eq!(Treasury::pot::<Kton>(), 50); // No changes from last perid
//
// 		// @4: The second spend perid
// 		<Treasury as OnFinalize<u64>>::on_finalize(4);
// 		assert_eq!(Ring::free_balance(&0), 95); // No changes from last perid
// 		assert_eq!(Kton::free_balance(&0), 95); // No changes from last perid
// 		assert_eq!(Ring::free_balance(&1), 98); // No changes
// 		assert_eq!(Kton::free_balance(&1), 98); // No changes
// 		assert_eq!(Ring::free_balance(&2), 1); // No changes
// 		assert_eq!(Kton::free_balance(&2), 1); // No changes
// 		assert_eq!(Ring::free_balance(&3), 0); // No changes
// 		assert_eq!(Kton::free_balance(&3), 0); // No changes
// 		assert_eq!(Treasury::pot::<Ring>(), 25); // No changes from last perid
// 		assert_eq!(Treasury::pot::<Kton>(), 25); // No changes from last perid
// 	});
// }
