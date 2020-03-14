//! Tests for the module.
//!
//! These tests are migrated from substrate `v2.0.0-alpha.3` for upgrade
//! usages, do not **add** functions to this file unless you are doing the
//! upgrading work :-P
//!
//! + If you want to add some external tests for this crate, please gather
//! them into `tests_local.rs`.
//!
//! + If you want to delete some functions, please left some comments
//! explaining why you delete them.
use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, ReservableCurrency},
	StorageMap,
};
use sp_runtime::{
	assert_eq_error_rate,
	traits::{BadOrigin, OnInitialize},
};
use sp_staking::offence::OffenceDetails;
use substrate_test_utils::assert_eq_uvec;

use crate::{mock::*, *};
use darwinia_support::balance::lock::*;
use pallet_ring::Error as RingError;

// #[test]
// fn force_unstake_works() {
// 	// Verifies initial conditions of mock
// 	ExtBuilder::default().build().execute_with(|| {
// 		// Account 11 is stashed and locked, and account 10 is the controller
// 		assert_eq!(Staking::bonded(&11), Some(10));
// 		// Cant transfer
// 		assert_noop!(
// 			Ring::transfer(Origin::signed(11), 1, 10),
// 			RingError::<Test, _>::LiquidityRestrictions,
// 		);
// 		// Force unstake requires root.
// 		assert_noop!(Staking::force_unstake(Origin::signed(11), 11), BadOrigin);
// 		// We now force them to unstake
// 		assert_ok!(Staking::force_unstake(Origin::ROOT, 11));
// 		// No longer bonded.
// 		assert_eq!(Staking::bonded(&11), None);
// 		// Transfer works.
// 		assert_ok!(Ring::transfer(Origin::signed(11), 1, 10));
// 	});
// }
//
// #[test]
// fn basic_setup_works() {
// 	// Verifies initial conditions of mock
// 	ExtBuilder::default().build().execute_with(|| {
// 		// Account 11 is stashed and locked, and account 10 is the controller
// 		assert_eq!(Staking::bonded(&11), Some(10));
// 		// Account 21 is stashed and locked, and account 20 is the controller
// 		assert_eq!(Staking::bonded(&21), Some(20));
// 		// Account 1 is not a stashed
// 		assert_eq!(Staking::bonded(&1), None);
//
// 		// Account 10 controls the stash from account 11, which is 100 * balance_factor units
// 		assert_eq!(
// 			Staking::ledger(&10),
// 			Some(StakingLedger {
// 				stash: 11,
// 				active_ring: 1000,
// 				active_deposit_ring: 0,
// 				active_kton: 0,
// 				deposit_items: vec![],
// 				ring_staking_lock: StakingLock {
// 					staking_amount: 1000,
// 					unbondings: vec![]
// 				},
// 				kton_staking_lock: StakingLock {
// 					staking_amount: 0,
// 					unbondings: vec![]
// 				}
// 			})
// 		);
// 		// Account 20 controls the stash from account 21, which is 200 * balance_factor units
// 		assert_eq!(
// 			Staking::ledger(&20),
// 			Some(StakingLedger {
// 				stash: 21,
// 				active_ring: 1000,
// 				active_deposit_ring: 0,
// 				active_kton: 0,
// 				deposit_items: vec![],
// 				ring_staking_lock: StakingLock {
// 					staking_amount: 1000,
// 					unbondings: vec![]
// 				},
// 				kton_staking_lock: StakingLock {
// 					staking_amount: 0,
// 					unbondings: vec![]
// 				}
// 			})
// 		);
// 		// Account 1 does not control any stash
// 		assert_eq!(Staking::ledger(&1), None);
//
// 		// ValidatorPrefs are default
// 		assert_eq!(
// 			<Validators<Test>>::enumerate().collect::<Vec<_>>(),
// 			vec![
// 				(31, ValidatorPrefs::default()),
// 				(21, ValidatorPrefs::default()),
// 				(11, ValidatorPrefs::default())
// 			]
// 		);
//
// 		// @review(power)
// 		assert_eq!(
// 			Staking::ledger(100),
// 			Some(StakingLedger {
// 				stash: 101,
// 				active_ring: 500,
// 				active_deposit_ring: 0,
// 				active_kton: 0,
// 				deposit_items: vec![],
// 				ring_staking_lock: StakingLock {
// 					staking_amount: 500,
// 					unbondings: vec![]
// 				},
// 				kton_staking_lock: StakingLock {
// 					staking_amount: 0,
// 					unbondings: vec![]
// 				}
// 			})
// 		);
// 		assert_eq!(Staking::nominators(101).unwrap().targets, vec![11, 21]);
//
// 		// @review(power)
// 		// assert_eq!(
// 		// 	Staking::stakers(21),
// 		// 	Exposure {
// 		// 		own_ring_balance: 1000,
// 		// 		own_kton_balance: 0,
// 		// 		own_power: 142816338,
// 		// 		total_power: 196372465,
// 		// 		others: vec![IndividualExposure {
// 		// 			who: 101,
// 		// 			ring_balance: 375,
// 		// 			kton_balance: 0,
// 		// 			power: 53556127,
// 		// 		}]
// 		// 	}
// 		// );
// 		//
// 		// assert_eq!(
// 		// 	Staking::stakers(21),
// 		// 	Exposure {
// 		// 		own_ring_balance: 1000,
// 		// 		own_kton_balance: 0,
// 		// 		own_power: 142816338,
// 		// 		total_power: 196372465,
// 		// 		others: vec![IndividualExposure {
// 		// 			who: 101,
// 		// 			ring_balance: 375,
// 		// 			kton_balance: 0,
// 		// 			power: 53556127,
// 		// 		}]
// 		// 	}
// 		// );
// 		//
// 		// @review(power): check the value below
// 		// // initial slot_stake
// 		// assert_eq!(Staking::slot_stake(), 160668380);
//
// 		// The number of validators required.
// 		assert_eq!(Staking::validator_count(), 2);
//
// 		// Initial Era and session
// 		assert_eq!(Staking::current_era(), 0);
//
// 		// Account 10 has `balance_factor` free balance
// 		assert_eq!(Ring::free_balance(10), 1);
// 		assert_eq!(Ring::free_balance(10), 1);
//
// 		// New era is not being forced
// 		assert_eq!(Staking::force_era(), Forcing::NotForcing);
//
// 		// All exposures must be correct.
// 		check_exposure_all();
// 		check_nominator_all();
// 	});
// }
//
// #[test]
// fn change_controller_works() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		assert_eq!(Staking::bonded(&11), Some(10));
//
// 		assert!(<Validators<Test>>::enumerate()
// 			.map(|(c, _)| c)
// 			.collect::<Vec<u64>>()
// 			.contains(&11));
// 		// 10 can control 11 who is initially a validator.
// 		assert_ok!(Staking::chill(Origin::signed(10)));
// 		assert!(!<Validators<Test>>::enumerate()
// 			.map(|(c, _)| c)
// 			.collect::<Vec<u64>>()
// 			.contains(&11));
//
// 		assert_ok!(Staking::set_controller(Origin::signed(11), 5));
//
// 		start_era(1);
//
// 		assert_noop!(
// 			Staking::validate(Origin::signed(10), ValidatorPrefs::default()),
// 			Error::<Test>::NotController,
// 		);
// 		assert_ok!(Staking::validate(Origin::signed(5), ValidatorPrefs::default()));
// 	})
// }
//
// #[test]
// fn rewards_should_work() {
// 	// should check that:
// 	// * rewards get recorded per session
// 	// * rewards get paid per Era
// 	// * Check that nominators are also rewarded
// 	ExtBuilder::default().nominate(false).build().execute_with(|| {
// 		// Init some balances
// 		let _ = Ring::make_free_balance_be(&2, 500);
//
// 		let delay = 1000;
// 		let init_balance_2 = Ring::total_balance(&2);
// 		let init_balance_10 = Ring::total_balance(&10);
// 		let init_balance_11 = Ring::total_balance(&11);
//
// 		// Set payee to controller
// 		assert_ok!(Staking::set_payee(Origin::signed(10), RewardDestination::Controller));
//
// 		// Initial config should be correct
// 		assert_eq!(Staking::current_era(), 0);
// 		assert_eq!(Session::current_index(), 0);
//
// 		// Add a dummy nominator.
// 		//
// 		// Equal division indicates that the reward will be equally divided among validator and
// 		// nominator.
// 		<Stakers<Test>>::insert(
// 			&11,
// 			Exposure {
// 				own_ring_balance: 1000,
// 				own_kton_balance: 0,
// 				own_power: 0,
// 				total_power: 0,
// 				others: vec![IndividualExposure {
// 					who: 2,
// 					ring_balance: 500,
// 					kton_balance: 0,
// 					power: 0,
// 				}],
// 			},
// 		);
//
// 		<Payee<Test>>::insert(&2, RewardDestination::Stash);
// 		assert_eq!(Staking::payee(2), RewardDestination::Stash);
// 		assert_eq!(Staking::payee(11), RewardDestination::Controller);
//
// 		let mut block = 3; // Block 3 => Session 1 => Era 0
// 		System::set_block_number(block);
// 		Timestamp::set_timestamp(block * 5000); // on time.
// 		Session::on_initialize(System::block_number());
// 		assert_eq!(Staking::current_era(), 0);
// 		assert_eq!(Session::current_index(), 1);
// 		Staking::reward_by_ids(vec![(11, 50)]);
// 		Staking::reward_by_ids(vec![(11, 50)]);
// 		// This is the second validator of the current elected set.
// 		Staking::reward_by_ids(vec![(21, 50)]);
// 		// This must be no-op as it is not an elected validator.
// 		Staking::reward_by_ids(vec![(1001, 10_000)]);
//
// 		// Compute total payout now for whole duration as other parameter won't change
// 		let total_payout = current_total_payout_for_duration(9 * 5 * 1000);
// 		assert!(total_payout > 10); // Test is meaningful if reward something
//
// 		// No reward yet
// 		assert_eq!(Ring::total_balance(&2), init_balance_2);
// 		assert_eq!(Ring::total_balance(&10), init_balance_10);
// 		assert_eq!(Ring::total_balance(&11), init_balance_11);
//
// 		block = 6; // Block 6 => Session 2 => Era 0
// 		System::set_block_number(block);
// 		Timestamp::set_timestamp(block * 5000 + delay); // a little late.
// 		Session::on_initialize(System::block_number());
// 		assert_eq!(Staking::current_era(), 0);
// 		assert_eq!(Session::current_index(), 2);
//
// 		block = 9; // Block 9 => Session 3 => Era 1
// 		System::set_block_number(block);
// 		Timestamp::set_timestamp(block * 5000); // back to being on time. no delays
// 		Session::on_initialize(System::block_number());
// 		assert_eq!(Staking::current_era(), 1);
// 		assert_eq!(Session::current_index(), 3);
//
// 		// @review(reward): check the reward
// 		// 11 validator has 2/3 of the total rewards and half half for it and its nominator
// 		// assert_eq_error_rate!(Ring::total_balance(&2), init_balance_2 + total_payout / 3, 1);
// 		// assert_eq_error_rate!(Ring::total_balance(&10), init_balance_10 + total_payout / 3, 1);
// 		// --------
//
// 		assert_eq!(Ring::total_balance(&11), init_balance_11);
// 	});
// }
//
// #[test]
// fn multi_era_reward_should_work() {
// 	// Should check that:
// 	// The value of current_session_reward is set at the end of each era, based on
// 	// slot_stake and session_reward.
// 	ExtBuilder::default().nominate(false).build().execute_with(|| {
// 		let init_balance_10 = Ring::total_balance(&10);
//
// 		// Set payee to controller
// 		assert_ok!(Staking::set_payee(Origin::signed(10), RewardDestination::Controller));
//
// 		// Compute now as other parameter won't change
// 		let total_payout_0 = current_total_payout_for_duration(3000);
// 		assert!(total_payout_0 > 10); // Test is meaningful if reward something
// 		Staking::reward_by_ids(vec![(11, 1)]);
//
// 		start_session(0);
// 		start_session(1);
// 		start_session(2);
// 		start_session(3);
//
// 		assert_eq!(Staking::current_era(), 1);
// 		assert_eq!(Ring::total_balance(&10), init_balance_10 + total_payout_0);
//
// 		start_session(4);
//
// 		let total_payout_1 = current_total_payout_for_duration(3000);
// 		assert!(total_payout_1 > 10); // Test is meaningful if reward something
// 		Staking::reward_by_ids(vec![(11, 101)]);
//
// 		// new era is triggered here.
// 		start_session(5);
//
// 		// pay time
// 		assert_eq!(
// 			Ring::total_balance(&10),
// 			init_balance_10 + total_payout_0 + total_payout_1
// 		);
// 	});
// }
//
// #[test]
// fn staking_should_work() {
// 	// should test:
// 	// * new validators can be added to the default set
// 	// * new ones will be chosen per era
// 	// * either one can unlock the stash and back-down from being a validator via `chill`ing.
// 	ExtBuilder::default()
// 		.nominate(false)
// 		.fair(false) // to give 20 more staked value
// 		.build()
// 		.execute_with(|| {
// 			Timestamp::set_timestamp(1); // Initialize time.
//
// 			// remember + compare this along with the test.
// 			assert_eq_uvec!(validator_controllers(), vec![20, 10]);
//
// 			// put some money in account that we'll use.
// 			for i in 1..5 {
// 				let _ = Ring::make_free_balance_be(&i, 2000);
// 			}
//
// 			// --- Block 1:
// 			start_session(1);
// 			// add a new candidate for being a validator. account 3 controlled by 4.
// 			assert_ok!(Staking::bond(
// 				Origin::signed(3),
// 				4,
// 				StakingBalance::RingBalance(1500),
// 				RewardDestination::Controller,
// 				0
// 			));
// 			assert_ok!(Staking::validate(Origin::signed(4), ValidatorPrefs::default()));
//
// 			// No effects will be seen so far.
// 			assert_eq_uvec!(validator_controllers(), vec![20, 10]);
//
// 			// --- Block 2:
// 			start_session(2);
//
// 			// No effects will be seen so far. Era has not been yet triggered.
// 			assert_eq_uvec!(validator_controllers(), vec![20, 10]);
//
// 			// --- Block 3: the validators will now be queued.
// 			start_session(3);
// 			assert_eq!(Staking::current_era(), 1);
//
// 			// --- Block 4: the validators will now be changed.
// 			start_session(4);
//
// 			assert_eq_uvec!(validator_controllers(), vec![20, 4]);
// 			// --- Block 4: Unstake 4 as a validator, freeing up the balance stashed in 3
// 			// 4 will chill
// 			Staking::chill(Origin::signed(4)).unwrap();
//
// 			// --- Block 5: nothing. 4 is still there.
// 			start_session(5);
// 			assert_eq_uvec!(validator_controllers(), vec![20, 4]);
//
// 			// --- Block 6: 4 will not be a validator.
// 			start_session(7);
// 			assert_eq_uvec!(validator_controllers(), vec![20, 10]);
//
// 			// Note: the stashed value of 4 is still lock
// 			assert_eq!(
// 				Staking::ledger(&4),
// 				Some(StakingLedger {
// 					stash: 3,
// 					active_ring: 1500,
// 					active_deposit_ring: 0,
// 					active_kton: 0,
// 					deposit_items: vec![],
// 					ring_staking_lock: StakingLock {
// 						staking_amount: 1500,
// 						unbondings: vec![]
// 					},
// 					kton_staking_lock: StakingLock {
// 						staking_amount: 0,
// 						unbondings: vec![]
// 					}
// 				})
// 			);
//
// 			// e.g. it cannot spend more than 500 that it has free from the total 2000
// 			assert_noop!(Ring::reserve(&3, 501), RingError::<Test, _>::LiquidityRestrictions,);
// 			assert_ok!(Ring::reserve(&3, 409));
// 		});
// }
//
// #[test]
// fn less_than_needed_candidates_works() {
// 	ExtBuilder::default()
// 		.minimum_validator_count(1)
// 		.validator_count(4)
// 		.nominate(false)
// 		.num_validators(3)
// 		.build()
// 		.execute_with(|| {
// 			assert_eq!(Staking::validator_count(), 4);
// 			assert_eq!(Staking::minimum_validator_count(), 1);
// 			assert_eq_uvec!(validator_controllers(), vec![30, 20, 10]);
//
// 			start_era(1);
//
// 			// Previous set is selected. NO election algorithm is even executed.
// 			assert_eq_uvec!(validator_controllers(), vec![30, 20, 10]);
//
// 			// But the exposure is updated in a simple way. No external votes exists.
// 			// This is purely self-vote.
// 			assert_eq!(Staking::stakers(10).others.len(), 0);
// 			assert_eq!(Staking::stakers(20).others.len(), 0);
// 			assert_eq!(Staking::stakers(30).others.len(), 0);
// 			check_exposure_all();
// 			check_nominator_all();
// 		});
// }
//
// #[test]
// fn no_candidate_emergency_condition() {
// 	ExtBuilder::default()
// 		.minimum_validator_count(1)
// 		.validator_count(15)
// 		.num_validators(4)
// 		.validator_pool(true)
// 		.nominate(false)
// 		.build()
// 		.execute_with(|| {
// 			// initial validators
// 			assert_eq_uvec!(validator_controllers(), vec![10, 20, 30, 40]);
// 			let prefs = ValidatorPrefs {
// 				commission: Perbill::one(),
// 			};
// 			<Staking as crate::Store>::Validators::insert(11, prefs.clone());
//
// 			// set the minimum validator count.
// 			<Staking as crate::Store>::MinimumValidatorCount::put(10);
//
// 			let _ = Staking::chill(Origin::signed(10));
//
// 			// trigger era
// 			start_era(1);
//
// 			// Previous ones are elected. chill is invalidates. TODO: #2494
// 			assert_eq_uvec!(validator_controllers(), vec![10, 20, 30, 40]);
// 			// Though the validator preferences has been removed.
// 			assert!(Staking::validators(11) != prefs);
// 		});
// }
//
// #[test]
// fn nominating_and_rewards_should_work() {
// 	// PHRAGMEN OUTPUT: running this test with the reference impl gives:
// 	//
// 	// Sequential Phragmén gives
// 	// 10  is elected with stake  2200.0 and score  0.0003333333333333333
// 	// 20  is elected with stake  1800.0 and score  0.0005555555555555556
//
// 	// 10  has load  0.0003333333333333333 and supported
// 	// 10  with stake  1000.0
// 	// 20  has load  0.0005555555555555556 and supported
// 	// 20  with stake  1000.0
// 	// 30  has load  0 and supported
// 	// 30  with stake  0
// 	// 40  has load  0 and supported
// 	// 40  with stake  0
// 	// 2  has load  0.0005555555555555556 and supported
// 	// 10  with stake  600.0 20  with stake  400.0 30  with stake  0.0
// 	// 4  has load  0.0005555555555555556 and supported
// 	// 10  with stake  600.0 20  with stake  400.0 40  with stake  0.0
//
// 	// Sequential Phragmén with post processing gives
// 	// 10  is elected with stake  2000.0 and score  0.0003333333333333333
// 	// 20  is elected with stake  2000.0 and score  0.0005555555555555556
//
// 	// 10  has load  0.0003333333333333333 and supported
// 	// 10  with stake  1000.0
// 	// 20  has load  0.0005555555555555556 and supported
// 	// 20  with stake  1000.0
// 	// 30  has load  0 and supported
// 	// 30  with stake  0
// 	// 40  has load  0 and supported
// 	// 40  with stake  0
// 	// 2  has load  0.0005555555555555556 and supported
// 	// 10  with stake  400.0 20  with stake  600.0 30  with stake  0
// 	// 4  has load  0.0005555555555555556 and supported
// 	// 10  with stake  600.0 20  with stake  400.0 40  with stake  0.0
// 	ExtBuilder::default()
// 		.nominate(false)
// 		.validator_pool(true)
// 		.build()
// 		.execute_with(|| {
// 			// initial validators -- everyone is actually even.
// 			assert_eq_uvec!(validator_controllers(), vec![40, 30]);
//
// 			// Set payee to controller
// 			assert_ok!(Staking::set_payee(Origin::signed(10), RewardDestination::Controller));
// 			assert_ok!(Staking::set_payee(Origin::signed(20), RewardDestination::Controller));
// 			assert_ok!(Staking::set_payee(Origin::signed(30), RewardDestination::Controller));
// 			assert_ok!(Staking::set_payee(Origin::signed(40), RewardDestination::Controller));
//
// 			// give the man some money
// 			let initial_balance = 1000;
// 			for i in [1, 2, 3, 4, 5, 10, 11, 20, 21].iter() {
// 				let _ = Ring::make_free_balance_be(i, initial_balance);
// 			}
//
// 			// bond two account pairs and state interest in nomination.
// 			// 2 will nominate for 10, 20, 30
// 			assert_ok!(Staking::bond(
// 				Origin::signed(1),
// 				2,
// 				StakingBalance::RingBalance(1000),
// 				RewardDestination::Controller,
// 				0
// 			));
// 			assert_ok!(Staking::nominate(Origin::signed(2), vec![11, 21, 31]));
// 			// 4 will nominate for 10, 20, 40
// 			assert_ok!(Staking::bond(
// 				Origin::signed(3),
// 				4,
// 				StakingBalance::RingBalance(1000),
// 				RewardDestination::Controller,
// 				0
// 			));
// 			assert_ok!(Staking::nominate(Origin::signed(4), vec![11, 21, 41]));
//
// 			// the total reward for era 0
// 			let total_payout_0 = current_total_payout_for_duration(3000);
// 			assert!(total_payout_0 > 100); // Test is meaningful if reward something
// 			Staking::reward_by_ids(vec![(41, 1)]);
// 			Staking::reward_by_ids(vec![(31, 1)]);
// 			Staking::reward_by_ids(vec![(21, 10)]); // must be no-op
// 			Staking::reward_by_ids(vec![(11, 10)]); // must be no-op
//
// 			start_era(1);
//
// 			// 10 and 20 have more votes, they will be chosen by phragmen.
// 			assert_eq_uvec!(validator_controllers(), vec![20, 10]);
//
// 			// OLD validators must have already received some rewards.
// 			assert_eq!(Ring::total_balance(&40), 1 + total_payout_0 / 2);
// 			assert_eq!(Ring::total_balance(&30), 1 + total_payout_0 / 2);
//
// 			// ------ check the staked value of all parties.
//
// 			// total expo of 10, with 1200 coming from nominators (externals), according to phragmen.
// 			assert_eq!(Staking::stakers(11).own_ring_balance, 1000);
// 			// @review(power): check the power below
// 			// assert_eq!(Staking::stakers(11).total_power, 138461539);
// 			// @review(phragmen): check if below is necessary.
// 			// 2 and 4 supported 10, each with stake 600, according to phragmen.
// 			// assert_eq!(
// 			// 	Staking::stakers(11)
// 			// 		.others
// 			// 		.iter()
// 			// 		.map(|e| e.value)
// 			// 		.collect::<Vec<BalanceOf<Test>>>(),
// 			// 	vec![400, 400]
// 			// );
//
// 			assert_eq!(
// 				Staking::stakers(11).others.iter().map(|e| e.who).collect::<Vec<u64>>(),
// 				vec![3, 1]
// 			);
// 			// total expo of 20, with 500 coming from nominators (externals), according to phragmen.
// 			assert_eq!(Staking::stakers(21).own_ring_balance, 1000);
//
// 			// @review(power): check if we need to assert power here
// 			// assert_eq_error_rate!(Staking::stakers(21).total, 1000 + 1200, 2);
// 			// 2 and 4 supported 20, each with stake 250, according to phragmen.
// 			// @review(phragmen): check below
// 			// assert_eq!(
// 			// 	Staking::stakers(21)
// 			// 		.others
// 			// 		.iter()
// 			// 		.map(|e| e.value)
// 			// 		.collect::<Vec<BalanceOf<Test>>>(),
// 			// 	vec![600, 600]
// 			// );
// 			// --------
// 			assert_eq!(
// 				Staking::stakers(21).others.iter().map(|e| e.who).collect::<Vec<u64>>(),
// 				vec![3, 1]
// 			);
//
// 			// They are not chosen anymore
// 			assert_eq!(Staking::stakers(31).total_power, 0);
// 			assert_eq!(Staking::stakers(41).total_power, 0);
//
// 			// the total reward for era 1
// 			let total_payout_1 = current_total_payout_for_duration(3000);
// 			assert!(total_payout_1 > 100); // Test is meaningful if reward something
// 			Staking::reward_by_ids(vec![(41, 10)]); // must be no-op
// 			Staking::reward_by_ids(vec![(31, 10)]); // must be no-op
// 			Staking::reward_by_ids(vec![(21, 2)]);
// 			Staking::reward_by_ids(vec![(11, 1)]);
//
// 			start_era(2);
//
// 			// nothing else will happen, era ends and rewards are paid again,
// 			// it is expected that nominators will also be paid. See below
//
// 			let payout_for_10 = total_payout_1 / 3;
// 			let payout_for_20 = 2 * total_payout_1 / 3;
// 			// Nominator 2: has [400/1800 ~ 2/9 from 10] + [600/2200 ~ 3/11 from 20]'s reward. ==> 2/9 + 3/11
// 			assert_eq_error_rate!(
// 				Ring::total_balance(&2),
// 				initial_balance + (2 * payout_for_10 / 9 + 3 * payout_for_20 / 11),
// 				1,
// 			);
// 			// Nominator 4: has [400/1800 ~ 2/9 from 10] + [600/2200 ~ 3/11 from 20]'s reward. ==> 2/9 + 3/11
// 			assert_eq_error_rate!(
// 				Ring::total_balance(&4),
// 				initial_balance + (2 * payout_for_10 / 9 + 3 * payout_for_20 / 11),
// 				1,
// 			);
//
// 			// @review(reward): check if the reward below is correct
// 			// // Validator 10: got 800 / 1800 external stake => 8/18 =? 4/9 => Validator's share = 5/9
// 			// assert_eq_error_rate!(Ring::total_balance(&10), initial_balance + 5 * payout_for_10 / 9, 1,);
// 			// // Validator 20: got 1200 / 2200 external stake => 12/22 =? 6/11 => Validator's share = 5/11
// 			// assert_eq_error_rate!(Ring::total_balance(&20), initial_balance + 5 * payout_for_20 / 11, 1,);
//
// 			check_exposure_all();
// 			check_nominator_all();
// 		});
// }
//
// #[test]
// fn nominators_also_get_slashed() {
// 	// A nominator should be slashed if the validator they nominated is slashed
// 	// Here is the breakdown of roles:
// 	// 10 - is the controller of 11
// 	// 11 - is the stash.
// 	// 2 - is the nominator of 20, 10
// 	ExtBuilder::default().nominate(false).build().execute_with(|| {
// 		assert_eq!(Staking::validator_count(), 2);
//
// 		// Set payee to controller
// 		assert_ok!(Staking::set_payee(Origin::signed(10), RewardDestination::Controller));
//
// 		// give the man some money.
// 		let initial_balance = 1000;
// 		for i in [1, 2, 3, 10].iter() {
// 			let _ = Ring::make_free_balance_be(i, initial_balance);
// 		}
//
// 		// 2 will nominate for 10, 20
// 		let nominator_stake = StakingBalance::RingBalance(500);
// 		assert_ok!(Staking::bond(
// 			Origin::signed(1),
// 			2,
// 			nominator_stake.clone(),
// 			RewardDestination::default(),
// 			0
// 		));
// 		assert_ok!(Staking::nominate(Origin::signed(2), vec![20, 10]));
//
// 		let total_payout = current_total_payout_for_duration(3000);
// 		assert!(total_payout > 100); // Test is meaningful if reward something
// 		Staking::reward_by_ids(vec![(11, 1)]);
//
// 		// new era, pay rewards,
// 		start_era(1);
//
// 		// Nominator stash didn't collect any.
// 		assert_eq!(Ring::total_balance(&2), initial_balance);
//
// 		// 10 goes offline
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(5)],
// 		);
// 		let expo = Staking::stakers(11);
// 		let slash_value = 50;
// 		let total_slash = expo.total_power.min(slash_value) as u128;
// 		let validator_slash = expo.own_ring_balance.min(total_slash);
// 		let nominator_slash = match nominator_stake {
// 			StakingBalance::RingBalance(v) => v.min(total_slash - validator_slash),
// 			_ => panic!("nominator slash should be Ring Balance here"),
// 		};
//
// 		// initial + first era reward + slash
// 		assert_eq!(Ring::total_balance(&11), initial_balance - validator_slash);
// 		assert_eq!(Ring::total_balance(&2), initial_balance - nominator_slash);
// 		check_exposure_all();
// 		check_nominator_all();
// 		// Because slashing happened.
// 		assert!(is_disabled(10));
// 	});
// }
//
// #[test]
// fn double_staking_should_fail() {
// 	// should test (in the same order):
// 	// * an account already bonded as stash cannot be be stashed again.
// 	// * an account already bonded as stash cannot nominate.
// 	// * an account already bonded as controller can nominate.
// 	ExtBuilder::default().build().execute_with(|| {
// 		let arbitrary_value = 5;
// 		// 2 = controller, 1 stashed => ok
// 		assert_ok!(Staking::bond(
// 			Origin::signed(1),
// 			2,
// 			StakingBalance::RingBalance(arbitrary_value),
// 			RewardDestination::default(),
// 			0
// 		));
// 		// 4 = not used so far, 1 stashed => not allowed.
// 		assert_noop!(
// 			Staking::bond(
// 				Origin::signed(1),
// 				4,
// 				StakingBalance::RingBalance(arbitrary_value),
// 				RewardDestination::default(),
// 				0
// 			),
// 			Error::<Test>::AlreadyBonded,
// 		);
// 		// 1 = stashed => attempting to nominate should fail.
// 		assert_noop!(
// 			Staking::nominate(Origin::signed(1), vec![1]),
// 			Error::<Test>::NotController
// 		);
// 		// 2 = controller  => nominating should work.
// 		assert_ok!(Staking::nominate(Origin::signed(2), vec![1]));
// 	});
// }
//
// #[test]
// fn double_controlling_should_fail() {
// 	// should test (in the same order):
// 	// * an account already bonded as controller CANNOT be reused as the controller of another account.
// 	ExtBuilder::default().build().execute_with(|| {
// 		let arbitrary_value = 5;
// 		// 2 = controller, 1 stashed => ok
// 		assert_ok!(Staking::bond(
// 			Origin::signed(1),
// 			2,
// 			StakingBalance::RingBalance(arbitrary_value),
// 			RewardDestination::default(),
// 			0
// 		));
// 		// 2 = controller, 3 stashed (Note that 2 is reused.) => no-op
// 		assert_noop!(
// 			Staking::bond(
// 				Origin::signed(3),
// 				2,
// 				StakingBalance::RingBalance(arbitrary_value),
// 				RewardDestination::default(),
// 				0
// 			),
// 			Error::<Test>::AlreadyPaired,
// 		);
// 	});
// }
//
// #[test]
// fn session_and_eras_work() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		assert_eq!(Staking::current_era(), 0);
//
// 		// Block 1: No change.
// 		start_session(0);
// 		assert_eq!(Session::current_index(), 1);
// 		assert_eq!(Staking::current_era(), 0);
//
// 		// Block 2: Simple era change.
// 		start_session(2);
// 		assert_eq!(Session::current_index(), 3);
// 		assert_eq!(Staking::current_era(), 1);
//
// 		// Block 3: Schedule an era length change; no visible changes.
// 		start_session(3);
// 		assert_eq!(Session::current_index(), 4);
// 		assert_eq!(Staking::current_era(), 1);
//
// 		// Block 4: Era change kicks in.
// 		start_session(5);
// 		assert_eq!(Session::current_index(), 6);
// 		assert_eq!(Staking::current_era(), 2);
//
// 		// Block 5: No change.
// 		start_session(6);
// 		assert_eq!(Session::current_index(), 7);
// 		assert_eq!(Staking::current_era(), 2);
//
// 		// Block 6: No change.
// 		start_session(7);
// 		assert_eq!(Session::current_index(), 8);
// 		assert_eq!(Staking::current_era(), 2);
//
// 		// Block 7: Era increment.
// 		start_session(8);
// 		assert_eq!(Session::current_index(), 9);
// 		assert_eq!(Staking::current_era(), 3);
// 	});
// }
//
// #[test]
// fn forcing_new_era_works() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		// normal flow of session.
// 		assert_eq!(Staking::current_era(), 0);
// 		start_session(0);
// 		assert_eq!(Staking::current_era(), 0);
// 		start_session(1);
// 		assert_eq!(Staking::current_era(), 0);
// 		start_session(2);
// 		assert_eq!(Staking::current_era(), 1);
//
// 		// no era change.
// 		ForceEra::put(Forcing::ForceNone);
// 		start_session(3);
// 		assert_eq!(Staking::current_era(), 1);
// 		start_session(4);
// 		assert_eq!(Staking::current_era(), 1);
// 		start_session(5);
// 		assert_eq!(Staking::current_era(), 1);
// 		start_session(6);
// 		assert_eq!(Staking::current_era(), 1);
//
// 		// back to normal.
// 		// this immediately starts a new session.
// 		ForceEra::put(Forcing::NotForcing);
// 		start_session(7);
// 		assert_eq!(Staking::current_era(), 2);
// 		start_session(8);
// 		assert_eq!(Staking::current_era(), 2);
//
// 		// forceful change
// 		ForceEra::put(Forcing::ForceAlways);
// 		start_session(9);
// 		assert_eq!(Staking::current_era(), 3);
// 		start_session(10);
// 		assert_eq!(Staking::current_era(), 4);
// 		start_session(11);
// 		assert_eq!(Staking::current_era(), 5);
//
// 		// just one forceful change
// 		ForceEra::put(Forcing::ForceNew);
// 		start_session(12);
// 		assert_eq!(Staking::current_era(), 6);
//
// 		assert_eq!(ForceEra::get(), Forcing::NotForcing);
// 		start_session(13);
// 		assert_eq!(Staking::current_era(), 6);
// 	});
// }
//
// #[test]
// fn cannot_transfer_staked_balance() {
// 	// Tests that a stash account cannot transfer funds
// 	ExtBuilder::default().nominate(false).build().execute_with(|| {
// 		// Confirm account 11 is stashed
// 		assert_eq!(Staking::bonded(&11), Some(10));
// 		// Confirm account 11 has some free balance
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		// Confirm account 11 (via controller 10) is totally staked
// 		assert_eq!(Staking::stakers(&11).own_ring_balance, 1000);
// 		// Confirm account 11 cannot transfer as a result
// 		assert_noop!(
// 			Ring::transfer(Origin::signed(11), 20, 1),
// 			RingError::<Test, _>::LiquidityRestrictions
// 		);
//
// 		// Give account 11 extra free balance
// 		let _ = Ring::make_free_balance_be(&11, 10000);
// 		// Confirm that account 11 can now transfer some balance
// 		assert_ok!(Ring::transfer(Origin::signed(11), 20, 1));
// 	});
// }
//
// #[test]
// fn cannot_transfer_staked_balance_2() {
// 	// Tests that a stash account cannot transfer funds
// 	// Same test as above but with 20, and more accurate.
// 	// 21 has 2000 free balance but 1000 at stake
// 	ExtBuilder::default()
// 		.nominate(false)
// 		.fair(true)
// 		.build()
// 		.execute_with(|| {
// 			// Confirm account 21 is stashed
// 			assert_eq!(Staking::bonded(&21), Some(20));
// 			// Confirm account 21 has some free balance
// 			assert_eq!(Ring::free_balance(21), 2000);
// 			// Confirm account 21 (via controller 20) is totally staked
// 			assert_eq!(Staking::stakers(&21).own_ring_balance, 1000);
// 			// Confirm account 21 can transfer at most 1000
// 			assert_noop!(
// 				Ring::transfer(Origin::signed(21), 20, 1001),
// 				RingError::<Test, _>::LiquidityRestrictions,
// 			);
// 			assert_ok!(Ring::transfer(Origin::signed(21), 20, 1000));
// 		});
// }
//
// #[test]
// fn cannot_reserve_staked_balance() {
// 	// Checks that a bonded account cannot reserve balance from free balance
// 	ExtBuilder::default().build().execute_with(|| {
// 		// Confirm account 11 is stashed
// 		assert_eq!(Staking::bonded(&11), Some(10));
// 		// Confirm account 11 has some free balance
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		// Confirm account 11 (via controller 10) is totally staked
// 		assert_eq!(Staking::stakers(&11).own_ring_balance, 1000);
// 		// Confirm account 11 cannot transfer as a result
// 		assert_noop!(Ring::reserve(&11, 1), RingError::<Test, _>::LiquidityRestrictions);
//
// 		// Give account 11 extra free balance
// 		let _ = Ring::make_free_balance_be(&11, 10000);
// 		// Confirm account 11 can now reserve balance
// 		assert_ok!(Ring::reserve(&11, 1));
// 	});
// }
//
// #[test]
// fn reward_destination_works() {
// 	// Rewards go to the correct destination as determined in Payee
// 	ExtBuilder::default().nominate(false).build().execute_with(|| {
// 		// Check that account 11 is a validator
// 		assert!(Staking::current_elected().contains(&11));
// 		// Check the balance of the validator account
// 		assert_eq!(Ring::free_balance(10), 1);
// 		// Check the balance of the stash account
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		// Check how much is at stake
// 		assert_eq!(
// 			Staking::ledger(&10),
// 			Some(StakingLedger {
// 				stash: 11,
// 				active_ring: 1000,
// 				active_deposit_ring: 0,
// 				active_kton: 0,
// 				deposit_items: vec![],
// 				ring_staking_lock: StakingLock {
// 					staking_amount: 1000,
// 					unbondings: vec![]
// 				},
// 				kton_staking_lock: StakingLock {
// 					staking_amount: 0,
// 					unbondings: vec![]
// 				}
// 			})
// 		);
//
// 		// Compute total payout now for whole duration as other parameter won't change
// 		let total_payout_0 = current_total_payout_for_duration(3000);
// 		assert!(total_payout_0 > 100); // Test is meaningful if reward something
// 		Staking::reward_by_ids(vec![(11, 1)]);
//
// 		start_era(1);
//
// 		// Check that RewardDestination is Staked (default)
// 		assert_eq!(Staking::payee(&11), RewardDestination::Staked { promise_month: 0 });
// 		// Check that reward went to the stash account of validator
// 		assert_eq!(Ring::free_balance(11), 1000 + total_payout_0);
// 		// Check that amount at stake increased accordingly
// 		assert_eq!(
// 			Staking::ledger(&10),
// 			Some(StakingLedger {
// 				stash: 11,
// 				active_ring: 1000 + total_payout_0,
// 				active_deposit_ring: 0,
// 				active_kton: 0,
// 				deposit_items: vec![],
// 				ring_staking_lock: StakingLock {
// 					staking_amount: 1000 + total_payout_0,
// 					unbondings: vec![]
// 				},
// 				kton_staking_lock: StakingLock {
// 					staking_amount: 0,
// 					unbondings: vec![]
// 				}
// 			})
// 		);
//
// 		//Change RewardDestination to Stash
// 		<Payee<Test>>::insert(&11, RewardDestination::Stash);
//
// 		// Compute total payout now for whole duration as other parameter won't change
// 		let total_payout_1 = current_total_payout_for_duration(3000);
// 		assert!(total_payout_1 > 100); // Test is meaningful if reward something
// 		Staking::reward_by_ids(vec![(11, 1)]);
//
// 		start_era(2);
//
// 		// Check that RewardDestination is Stash
// 		assert_eq!(Staking::payee(&11), RewardDestination::Stash);
// 		// Check that reward went to the stash account
// 		assert_eq!(Ring::free_balance(11), 1000 + total_payout_0 + total_payout_1);
// 		// Record this value
// 		let recorded_stash_balance = 1000 + total_payout_0 + total_payout_1;
// 		// Check that amount at stake is NOT increased
// 		assert_eq!(
// 			Staking::ledger(&10),
// 			Some(StakingLedger {
// 				stash: 11,
// 				active_ring: 1000 + total_payout_0,
// 				active_deposit_ring: 0,
// 				active_kton: 0,
// 				deposit_items: vec![],
// 				ring_staking_lock: StakingLock {
// 					staking_amount: 1000 + total_payout_0,
// 					unbondings: vec![]
// 				},
// 				kton_staking_lock: StakingLock {
// 					staking_amount: 0,
// 					unbondings: vec![]
// 				}
// 			})
// 		);
//
// 		// Change RewardDestination to Controller
// 		<Payee<Test>>::insert(&11, RewardDestination::Controller);
//
// 		// Check controller balance
// 		assert_eq!(Ring::free_balance(10), 1);
//
// 		// Compute total payout now for whole duration as other parameter won't change
// 		let total_payout_2 = current_total_payout_for_duration(3000);
// 		assert!(total_payout_2 > 100); // Test is meaningful if reward something
// 		Staking::reward_by_ids(vec![(11, 1)]);
//
// 		start_era(3);
//
// 		// Check that RewardDestination is Controller
// 		assert_eq!(Staking::payee(&11), RewardDestination::Controller);
// 		// Check that reward went to the controller account
// 		assert_eq!(Ring::free_balance(10), 1 + total_payout_2);
// 		// Check that amount at stake is NOT increased
// 		assert_eq!(
// 			Staking::ledger(&10),
// 			Some(StakingLedger {
// 				stash: 11,
// 				active_ring: 1000 + total_payout_0,
// 				active_deposit_ring: 0,
// 				active_kton: 0,
// 				deposit_items: vec![],
// 				ring_staking_lock: StakingLock {
// 					staking_amount: 1000 + total_payout_0,
// 					unbondings: vec![]
// 				},
// 				kton_staking_lock: StakingLock {
// 					staking_amount: 0,
// 					unbondings: vec![]
// 				}
// 			})
// 		);
// 		// Check that amount in staked account is NOT increased.
// 		assert_eq!(Ring::free_balance(11), recorded_stash_balance);
// 	});
// }
//
// #[test]
// fn validator_payment_prefs_work() {
// 	// Test that validator preferences are correctly honored
// 	// Note: unstake threshold is being directly tested in slashing tests.
// 	// This test will focus on validator payment.
// 	ExtBuilder::default().build().execute_with(|| {
// 		// Initial config
// 		let stash_initial_balance = Ring::total_balance(&11);
//
// 		// check the balance of a validator accounts.
// 		assert_eq!(Ring::total_balance(&10), 1);
// 		// check the balance of a validator's stash accounts.
// 		assert_eq!(Ring::total_balance(&11), stash_initial_balance);
// 		// and the nominator (to-be)
// 		let _ = Ring::make_free_balance_be(&2, 500);
//
// 		// add a dummy nominator.
// 		<Stakers<Test>>::insert(
// 			&11,
// 			Exposure {
// 				own_ring_balance: 500,
// 				total_power: 1000,
// 				own_kton_balance: 0,
// 				own_power: 0,
// 				others: vec![IndividualExposure {
// 					who: 2,
// 					ring_balance: 500,
// 					kton_balance: 0,
// 					power: 0,
// 				}],
// 			},
// 		);
// 		<Payee<Test>>::insert(&2, RewardDestination::Stash);
// 		<Validators<Test>>::insert(
// 			&11,
// 			ValidatorPrefs {
// 				commission: Perbill::from_percent(50),
// 			},
// 		);
//
// 		// Compute total payout now for whole duration as other parameter won't change
// 		let total_payout_0 = current_total_payout_for_duration(3000);
// 		assert!(total_payout_0 > 100); // Test is meaningful if reward something
// 		Staking::reward_by_ids(vec![(11, 1)]);
//
// 		start_era(1);
// 		// @review(reward): caculate the reward
// 		// // whats left to be shared is the sum of 3 rounds minus the validator's cut.
// 		// let shared_cut = total_payout_0 / 2;
// 		// // Validator's payee is Staked account, 11, reward will be paid here.
// 		// assert_eq!(
// 		// 	Ring::total_balance(&11),
// 		// 	stash_initial_balance + shared_cut / 2 + shared_cut
// 		// );
// 		// // Controller account will not get any reward.
// 		// assert_eq!(Ring::total_balance(&10), 1);
// 		// // Rest of the reward will be shared and paid to the nominator in stake.
// 		// assert_eq!(Ring::total_balance(&2), 500 + shared_cut / 2);
// 		// -------
// 		check_exposure_all();
// 		check_nominator_all();
// 	});
// }
//
// #[test]
// fn bond_extra_works() {
// 	// Tests that extra `free_balance` in the stash can be added to stake
// 	// NOTE: this tests only verifies `StakingLedger` for correct updates
// 	// See `bond_extra_and_withdraw_unbonded_works` for more details and updates on `Exposure`.
// 	ExtBuilder::default().build().execute_with(|| {
// 		// Check that account 10 is a validator
// 		assert!(<Validators<Test>>::contains_key(11));
// 		// Check that account 10 is bonded to account 11
// 		assert_eq!(Staking::bonded(&11), Some(10));
// 		// Check how much is at stake
// 		assert_eq!(
// 			Staking::ledger(&10),
// 			Some(StakingLedger {
// 				stash: 11,
// 				active_ring: 1000,
// 				active_deposit_ring: 0,
// 				active_kton: 0,
// 				deposit_items: vec![],
// 				ring_staking_lock: StakingLock {
// 					staking_amount: 1000,
// 					unbondings: vec![]
// 				},
// 				kton_staking_lock: StakingLock {
// 					staking_amount: 0,
// 					unbondings: vec![]
// 				}
// 			})
// 		);
//
// 		// Give account 11 some large free balance greater than total
// 		let _ = Ring::make_free_balance_be(&11, 1000000);
//
// 		// Call the bond_extra function from controller, add only 100
// 		assert_ok!(Staking::bond_extra(
// 			Origin::signed(11),
// 			StakingBalance::RingBalance(100),
// 			12
// 		));
// 		// There should be 100 more `total` and `active` in the ledger
// 		assert_eq!(
// 			Staking::ledger(&10),
// 			Some(StakingLedger {
// 				stash: 11,
// 				active_ring: 1000 + 100,
// 				active_deposit_ring: 100,
// 				active_kton: 0,
// 				deposit_items: vec![TimeDepositItem {
// 					value: 100,
// 					start_time: 0,
// 					expire_time: 31104000000
// 				}],
// 				ring_staking_lock: StakingLock {
// 					staking_amount: 1000 + 100,
// 					unbondings: vec![]
// 				},
// 				kton_staking_lock: StakingLock {
// 					staking_amount: 0,
// 					unbondings: vec![]
// 				}
// 			})
// 		);
//
// 		// Call the bond_extra function with a large number, should handle it
// 		assert_ok!(Staking::bond_extra(
// 			Origin::signed(11),
// 			StakingBalance::RingBalance(u64::max_value() as u128),
// 			0
// 		));
//
// 		// The full amount of the funds should now be in the total and active
// 		assert_eq!(
// 			Staking::ledger(&10),
// 			Some(StakingLedger {
// 				stash: 11,
// 				// @review(bond): check if below is correct
// 				// if it is correct, please delete this comment.
// 				active_ring: 998900,
// 				active_deposit_ring: 100,
// 				active_kton: 0,
// 				deposit_items: vec![TimeDepositItem {
// 					value: 100,
// 					start_time: 0,
// 					expire_time: 31104000000
// 				}],
// 				ring_staking_lock: StakingLock {
// 					// @review(bond): check if below is correct,
// 					// if it is correct, please delete this comment.
// 					staking_amount: 998900,
// 					unbondings: vec![]
// 				},
// 				kton_staking_lock: StakingLock {
// 					staking_amount: 0,
// 					unbondings: vec![]
// 				}
// 			})
// 		);
// 	});
// }
//
// // @rm: `bond_extra_and_withdraw_unbonded_works` testcase
// // because we are not using this function
//
// // @review(unbond)
// #[test]
// fn too_many_unbond_calls_should_not_work() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		// locked at era 0 until 3
// 		for _ in 0..MAX_UNLOCKING_CHUNKS - 1 {
// 			assert_ok!(Staking::unbond(Origin::signed(10), StakingBalance::RingBalance(1)));
// 		}
//
// 		start_era(1);
//
// 		// locked at era 1 until 4
// 		assert_ok!(Staking::unbond(Origin::signed(10), StakingBalance::RingBalance(1)));
//
// 		// can't do more.
// 		assert_noop!(
// 			Staking::unbond(Origin::signed(10), StakingBalance::RingBalance(1)),
// 			Error::<Test>::NoMoreChunks
// 		);
//
// 		start_era(3);
//
// 		assert_noop!(
// 			Staking::unbond(Origin::signed(10), StakingBalance::RingBalance(1)),
// 			Error::<Test>::NoMoreChunks
// 		);
//
// 		// @review(unbond)
// 		// free up.
// 		// assert_ok!(Staking::withdraw_unbonded(Origin::signed(10)));
// 		//
// 		// Can add again.
// 		// assert_ok!(Staking::unbond(Origin::signed(10), StakingBalance::RingBalance(1)));
// 		// assert_eq!(Ring::locks(&10).len(), 2);
// 	})
// }
//
// // @rm `rebond_works` testcase,
// // because darwinia doesn't has the `rebond` method currently.
//
// // @rm `rebond_is_fifo` testcase,
// // because darwinia doesn't has the `rebond` method currently.
//
// #[test]
// fn slot_stake_is_least_staked_validator_and_exposure_defines_maximum_punishment() {
// 	// Test that slot_stake is determined by the least staked validator
// 	// Test that slot_stake is the maximum punishment that can happen to a validator
// 	ExtBuilder::default()
// 		.nominate(false)
// 		.fair(false)
// 		.build()
// 		.execute_with(|| {
// 			// Confirm validator count is 2
// 			assert_eq!(Staking::validator_count(), 2);
// 			// Confirm account 10 and 20 are validators
// 			assert!(<Validators<Test>>::contains_key(&11) && <Validators<Test>>::contains_key(&21));
//
// 			assert_eq!(Staking::stakers(&11).own_ring_balance, 1000);
// 			assert_eq!(Staking::stakers(&21).own_ring_balance, 2000);
//
// 			// Give the man some money.
// 			let _ = Ring::make_free_balance_be(&10, 1000);
// 			let _ = Ring::make_free_balance_be(&20, 1000);
//
// 			// We confirm initialized slot_stake is this value
// 			assert_eq!(Staking::slot_stake(), Staking::stakers(&11).total_power);
//
// 			// Now lets lower account 20 stake
// 			<Stakers<Test>>::insert(
// 				&21,
// 				Exposure {
// 					own_ring_balance: 0,
// 					own_kton_balance: 0,
// 					total_power: 69,
// 					own_power: 69,
// 					others: vec![],
// 				},
// 			);
// 			assert_eq!(Staking::stakers(&21).total_power, 69);
// 			<Ledger<Test>>::insert(
// 				&20,
// 				StakingLedger {
// 					stash: 22,
// 					active_ring: 69,
// 					active_deposit_ring: 69,
// 					active_kton: 0,
// 					deposit_items: vec![],
// 					ring_staking_lock: StakingLock {
// 						staking_amount: 0,
// 						unbondings: vec![],
// 					},
// 					kton_staking_lock: StakingLock {
// 						staking_amount: 0,
// 						unbondings: vec![],
// 					},
// 				},
// 			);
//
// 			// Compute total payout now for whole duration as other parameter won't change
// 			let total_payout_0 = current_total_payout_for_duration(3000);
// 			assert!(total_payout_0 > 100); // Test is meaningful if reward something
// 			Staking::reward_by_ids(vec![(11, 1)]);
// 			Staking::reward_by_ids(vec![(21, 1)]);
//
// 			// New era --> rewards are paid --> stakes are changed
// 			start_era(1);
//
// 			// @review(power): caculate the power below
// 			// // -- new balances + reward
// 			// assert_eq!(Staking::stakers(&11).total_power as u128, 1000 + total_payout_0 / 2);
// 			// assert_eq!(Staking::stakers(&21).total_power as u128, 69 + total_payout_0 / 2);
// 			//
// 			// let _11_balance = Ring::free_balance(&11);
// 			// assert_eq!(_11_balance as u128, 1000 + total_payout_0 / 2);
// 			//
// 			// // -- slot stake should also be updated.
// 			// assert_eq!(Staking::slot_stake() as u128, 69 + total_payout_0 / 2);
//
// 			check_exposure_all();
// 			check_nominator_all();
// 		});
// }
// // --------
//
// #[test]
// fn on_free_balance_zero_stash_removes_validator() {
// 	// Tests that validator storage items are cleaned up when stash is empty
// 	// Tests that storage items are untouched when controller is empty
// 	ExtBuilder::default().existential_deposit(10).build().execute_with(|| {
// 		// Check the balance of the validator account
// 		assert_eq!(Ring::free_balance(10), 256);
// 		// Check the balance of the stash account
// 		assert_eq!(Ring::free_balance(11), 256000);
// 		// Check these two accounts are bonded
// 		assert_eq!(Staking::bonded(&11), Some(10));
//
// 		// Set some storage items which we expect to be cleaned up
// 		// Set payee information
// 		assert_ok!(Staking::set_payee(Origin::signed(10), RewardDestination::Stash));
//
// 		// Check storage items that should be cleaned up
// 		assert!(<Ledger<Test>>::contains_key(&10));
// 		assert!(<Bonded<Test>>::contains_key(&11));
// 		assert!(<Validators<Test>>::contains_key(&11));
// 		assert!(<Payee<Test>>::contains_key(&11));
//
// 		// Reduce free_balance of controller to 0
// 		let _ = Ring::slash(&10, u64::max_value() as u128);
//
// 		// Check the balance of the stash account has not been touched
// 		assert_eq!(Ring::free_balance(11), 256000);
// 		// Check these two accounts are still bonded
// 		assert_eq!(Staking::bonded(&11), Some(10));
//
// 		// Check storage items have not changed
// 		assert!(<Ledger<Test>>::contains_key(&10));
// 		assert!(<Bonded<Test>>::contains_key(&11));
// 		assert!(<Validators<Test>>::contains_key(&11));
// 		assert!(<Payee<Test>>::contains_key(&11));
//
// 		// Reduce free_balance of stash to 0
// 		let _ = Ring::slash(&11, u64::max_value() as u128);
// 		// Check total balance of stash
// 		assert_eq!(Ring::total_balance(&11), 0);
//
// 		// Reap the stash
// 		assert_ok!(Staking::reap_stash(Origin::NONE, 11));
//
// 		// Check storage items do not exist
// 		assert!(!<Ledger<Test>>::contains_key(&10));
// 		assert!(!<Bonded<Test>>::contains_key(&11));
// 		assert!(!<Validators<Test>>::contains_key(&11));
// 		assert!(!<Nominators<Test>>::contains_key(&11));
// 		assert!(!<Payee<Test>>::contains_key(&11));
// 	});
// }
//
// #[test]
// fn on_free_balance_zero_stash_removes_nominator() {
// 	// Tests that nominator storage items are cleaned up when stash is empty
// 	// Tests that storage items are untouched when controller is empty
// 	ExtBuilder::default().existential_deposit(10).build().execute_with(|| {
// 		// Make 10 a nominator
// 		assert_ok!(Staking::nominate(Origin::signed(10), vec![20]));
// 		// Check that account 10 is a nominator
// 		assert!(<Nominators<Test>>::contains_key(11));
// 		// Check the balance of the nominator account
// 		assert_eq!(Ring::free_balance(10), 256);
// 		// Check the balance of the stash account
// 		assert_eq!(Ring::free_balance(11), 256000);
//
// 		// Set payee information
// 		assert_ok!(Staking::set_payee(Origin::signed(10), RewardDestination::Stash));
//
// 		// Check storage items that should be cleaned up
// 		assert!(<Ledger<Test>>::contains_key(&10));
// 		assert!(<Bonded<Test>>::contains_key(&11));
// 		assert!(<Nominators<Test>>::contains_key(&11));
// 		assert!(<Payee<Test>>::contains_key(&11));
//
// 		// Reduce free_balance of controller to 0
// 		let _ = Ring::slash(&10, u64::max_value() as u128);
// 		// Check total balance of account 10
// 		assert_eq!(Ring::total_balance(&10), 0);
//
// 		// Check the balance of the stash account has not been touched
// 		assert_eq!(Ring::free_balance(11), 256000);
// 		// Check these two accounts are still bonded
// 		assert_eq!(Staking::bonded(&11), Some(10));
//
// 		// Check storage items have not changed
// 		assert!(<Ledger<Test>>::contains_key(&10));
// 		assert!(<Bonded<Test>>::contains_key(&11));
// 		assert!(<Nominators<Test>>::contains_key(&11));
// 		assert!(<Payee<Test>>::contains_key(&11));
//
// 		// Reduce free_balance of stash to 0
// 		let _ = Ring::slash(&11, u64::max_value() as u128);
// 		// Check total balance of stash
// 		assert_eq!(Ring::total_balance(&11), 0);
//
// 		// Reap the stash
// 		assert_ok!(Staking::reap_stash(Origin::NONE, 11));
//
// 		// Check storage items do not exist
// 		assert!(!<Ledger<Test>>::contains_key(&10));
// 		assert!(!<Bonded<Test>>::contains_key(&11));
// 		assert!(!<Validators<Test>>::contains_key(&11));
// 		assert!(!<Nominators<Test>>::contains_key(&11));
// 		assert!(!<Payee<Test>>::contains_key(&11));
// 	});
// }
//
// #[test]
// fn switching_roles() {
// 	// Test that it should be possible to switch between roles (nominator, validator, idle) with minimal overhead.
// 	ExtBuilder::default().nominate(false).build().execute_with(|| {
// 		Timestamp::set_timestamp(1); // Initialize time.
//
// 		// Reset reward destination
// 		for i in &[10, 20] {
// 			assert_ok!(Staking::set_payee(Origin::signed(*i), RewardDestination::Controller));
// 		}
//
// 		assert_eq_uvec!(validator_controllers(), vec![20, 10]);
//
// 		// put some money in account that we'll use.
// 		for i in 1..7 {
// 			let _ = Ring::deposit_creating(&i, 5000);
// 		}
//
// 		// add 2 nominators
// 		assert_ok!(Staking::bond(
// 			Origin::signed(1),
// 			2,
// 			StakingBalance::RingBalance(2000),
// 			RewardDestination::Controller,
// 			0
// 		));
// 		assert_ok!(Staking::nominate(Origin::signed(2), vec![11, 5]));
//
// 		assert_ok!(Staking::bond(
// 			Origin::signed(3),
// 			4,
// 			StakingBalance::RingBalance(500),
// 			RewardDestination::Controller,
// 			0
// 		));
// 		assert_ok!(Staking::nominate(Origin::signed(4), vec![21, 1]));
//
// 		// add a new validator candidate
// 		assert_ok!(Staking::bond(
// 			Origin::signed(5),
// 			6,
// 			StakingBalance::RingBalance(1000),
// 			RewardDestination::Controller,
// 			0
// 		));
// 		assert_ok!(Staking::validate(Origin::signed(6), ValidatorPrefs::default()));
//
// 		// new block
// 		start_session(1);
//
// 		// no change
// 		assert_eq_uvec!(validator_controllers(), vec![20, 10]);
//
// 		// new block
// 		start_session(2);
//
// 		// no change
// 		assert_eq_uvec!(validator_controllers(), vec![20, 10]);
//
// 		// new block --> ne era --> new validators
// 		start_session(3);
//
// 		// with current nominators 10 and 5 have the most stake
// 		assert_eq_uvec!(validator_controllers(), vec![6, 10]);
//
// 		// 2 decides to be a validator. Consequences:
// 		assert_ok!(Staking::validate(Origin::signed(2), ValidatorPrefs::default()));
// 		// new stakes:
// 		// 10: 1000 self vote
// 		// 20: 1000 self vote + 250 vote
// 		// 6 : 1000 self vote
// 		// 2 : 2000 self vote + 250 vote.
// 		// Winners: 20 and 2
//
// 		start_session(4);
// 		assert_eq_uvec!(validator_controllers(), vec![6, 10]);
//
// 		start_session(5);
// 		assert_eq_uvec!(validator_controllers(), vec![6, 10]);
//
// 		// ne era
// 		start_session(6);
// 		assert_eq_uvec!(validator_controllers(), vec![2, 20]);
//
// 		check_exposure_all();
// 		check_nominator_all();
// 	});
// }
//
// #[test]
// fn wrong_vote_is_null() {
// 	ExtBuilder::default()
// 		.nominate(false)
// 		.validator_pool(true)
// 		.build()
// 		.execute_with(|| {
// 			assert_eq_uvec!(validator_controllers(), vec![40, 30]);
//
// 			// put some money in account that we'll use.
// 			for i in 1..3 {
// 				let _ = Ring::deposit_creating(&i, 5000);
// 			}
//
// 			// add 1 nominators
// 			assert_ok!(Staking::bond(
// 				Origin::signed(1),
// 				2,
// 				StakingBalance::RingBalance(2000),
// 				RewardDestination::default(),
// 				0
// 			));
// 			assert_ok!(Staking::nominate(
// 				Origin::signed(2),
// 				vec![
// 					11, 21, // good votes
// 					1, 2, 15, 1000, 25 // crap votes. No effect.
// 				]
// 			));
//
// 			// new block
// 			start_era(1);
//
// 			assert_eq_uvec!(validator_controllers(), vec![20, 10]);
// 		});
// }
//
// #[test]
// fn bond_with_no_staked_value() {
// 	// Behavior when someone bonds with no staked value.
// 	// Particularly when she votes and the candidate is elected.
// 	ExtBuilder::default()
// 		.validator_count(3)
// 		.existential_deposit(5)
// 		.nominate(false)
// 		.minimum_validator_count(1)
// 		.build()
// 		.execute_with(|| {
// 			// Can't bond with 1
// 			assert_noop!(
// 				Staking::bond(
// 					Origin::signed(1),
// 					2,
// 					StakingBalance::RingBalance(1),
// 					RewardDestination::Controller,
// 					0
// 				),
// 				Error::<Test>::InsufficientValue,
// 			);
// 			// bonded with absolute minimum value possible.
// 			assert_ok!(Staking::bond(
// 				Origin::signed(1),
// 				2,
// 				StakingBalance::RingBalance(5),
// 				RewardDestination::Controller,
// 				0
// 			));
//
// 			match &Ring::locks(&1)[0].lock_for {
// 				LockFor::Common { amount: _ } => {
// 					panic!("Locked balance should convert to StakingLock.");
// 				}
// 				LockFor::Staking(staking_lock) => {
// 					assert_eq!(staking_lock.staking_amount, (5 as u128));
// 				}
// 			}
//
// 			// @darwinia(unbond)
// 			// ~~unbonding even 1 will cause all to be unbonded.~~
// 			// Only active normal ring can be unbond
// 			//
// 			// normal_ring: active_ring - active_deposit_ring;
// 			assert_ok!(Staking::unbond(Origin::signed(2), StakingBalance::RingBalance(1)));
// 			assert_eq!(
// 				Staking::ledger(2),
// 				Some(StakingLedger {
// 					stash: 1,
// 					active_ring: 4,
// 					active_deposit_ring: 0,
// 					active_kton: 0,
// 					deposit_items: vec![],
// 					ring_staking_lock: StakingLock {
// 						staking_amount: 4,
// 						unbondings: vec![Unbonding { amount: 1, until: 541 }]
// 					},
// 					kton_staking_lock: StakingLock {
// 						staking_amount: 0,
// 						unbondings: vec![]
// 					}
// 				})
// 			);
//
// 			start_era(1);
// 			start_era(2);
//
// 			assert!(Staking::ledger(2).is_some());
// 			match &Ring::locks(&1)[0].lock_for {
// 				LockFor::Common { amount: _ } => {
// 					panic!("Locked balance should convert to StakingLock.");
// 				}
// 				LockFor::Staking(staking_lock) => {
// 					assert_eq!(staking_lock.staking_amount, (4 as u128));
// 				}
// 			}
//
// 			// @review(withdraw): origin from substrate //
// 			//
// 			// Substrate write like below because they want to unbond
// 			// all balances at last process, so, if we want to test the
// 			// `remove_account` method here, just unbond all.
// 			//
// 			// // not yet removed.
// 			// assert_ok!(Staking::withdraw_unbonded(Origin::signed(2)));
// 			// assert!(Staking::ledger(2).is_some());
// 			// assert_eq!(Ring::locks(&1)[0].amount, 5);
// 			//
// 			// start_era(3);
// 			//
// 			// // poof. Account 1 is removed from the staking system.
// 			// assert_ok!(Staking::withdraw_unbonded(Origin::signed(2)));
// 			// assert!(Staking::ledger(2).is_none());
// 			// assert_eq!(Ring::locks(&1).len(), 0);
// 			// -------- //
//
// 			// @review(unbond)
// 			assert_ok!(Staking::unbond(Origin::signed(2), StakingBalance::RingBalance(4)));
// 			assert!(Staking::ledger(2).is_none());
//
// 			start_era(3);
//
// 			// @review(unbond): here is the point, lock should be drained,
// 			// but we haven't made the choice now.
// 			match &Ring::locks(&1)[0].lock_for {
// 				LockFor::Common { amount: _ } => {
// 					panic!("Locked balance should convert to StakingLock.");
// 				}
// 				LockFor::Staking(staking_lock) => {
// 					assert_eq!(staking_lock.staking_amount, (0 as u128));
// 				}
// 			}
// 		});
// }
//
// #[test]
// fn bond_with_little_staked_value_bounded_by_slot_stake() {
// 	// Behavior when someone bonds with little staked value.
// 	// Particularly when she votes and the candidate is elected.
// 	ExtBuilder::default()
// 		.validator_count(3)
// 		.nominate(false)
// 		.minimum_validator_count(1)
// 		.build()
// 		.execute_with(|| {
// 			// setup
// 			assert_ok!(Staking::chill(Origin::signed(30)));
// 			assert_ok!(Staking::set_payee(Origin::signed(10), RewardDestination::Controller));
// 			let init_balance_2 = Ring::free_balance(&2);
// 			let init_balance_10 = Ring::free_balance(&10);
//
// 			// Stingy validator.
// 			assert_ok!(Staking::bond(
// 				Origin::signed(1),
// 				2,
// 				StakingBalance::RingBalance(1),
// 				RewardDestination::Controller,
// 				0
// 			));
// 			assert_ok!(Staking::validate(Origin::signed(2), ValidatorPrefs::default()));
//
// 			let total_payout_0 = current_total_payout_for_duration(3000);
// 			assert!(total_payout_0 > 100); // Test is meaningful if reward something
// 			reward_all_elected();
// 			start_era(1);
//
// 			// @review(power): caculate the correct power
// 			// 2 is elected.
// 			// // and fucks up the slot stake.
// 			// assert_eq_uvec!(validator_controllers(), vec![20, 10, 2]);
// 			// assert_eq!(Staking::slot_stake(), 1);
// 			//
// 			// // Old ones are rewarded.
// 			// assert_eq!(Ring::free_balance(10), init_balance_10 + total_payout_0 / 3);
// 			//
// 			// // no rewards paid to 2. This was initial election.
// 			// assert_eq!(Ring::free_balance(2), init_balance_2);
// 			//
// 			// let total_payout_1 = current_total_payout_for_duration(3000);
// 			// assert!(total_payout_1 > 100); // Test is meaningful if reward something
// 			// reward_all_elected();
// 			// start_era(2);
// 			//
// 			// assert_eq_uvec!(validator_controllers(), vec![20, 10, 2]);
// 			// assert_eq!(Staking::slot_stake(), 1);
// 			// assert_eq!(Ring::free_balance(2), init_balance_2 + total_payout_1 / 3);
// 			// assert_eq!(
// 			// 	Ring::free_balance(&10),
// 			// 	init_balance_10 + total_payout_0 / 3 + total_payout_1 / 3,
// 			// );
//
// 			check_exposure_all();
// 			check_nominator_all();
// 		});
// }
//
// #[test]
// fn new_era_elects_correct_number_of_validators() {
// 	ExtBuilder::default()
// 		.nominate(true)
// 		.validator_pool(true)
// 		.fair(true)
// 		.validator_count(1)
// 		.build()
// 		.execute_with(|| {
// 			assert_eq!(Staking::validator_count(), 1);
// 			assert_eq!(validator_controllers().len(), 1);
//
// 			System::set_block_number(1);
// 			Session::on_initialize(System::block_number());
//
// 			assert_eq!(validator_controllers().len(), 1);
// 			check_exposure_all();
// 			check_nominator_all();
// 		})
// }
//
// // @review(power): check if the power result is right.
// #[test]
// fn phragmen_should_not_overflow_validators() {
// 	ExtBuilder::default().nominate(false).build().execute_with(|| {
// 		let _ = Staking::chill(Origin::signed(10));
// 		let _ = Staking::chill(Origin::signed(20));
//
// 		bond_validator(2, StakingBalance::RingBalance(CAP));
// 		bond_validator(4, StakingBalance::KtonBalance(CAP));
//
// 		bond_nominator(6, StakingBalance::RingBalance(1), vec![3, 5]);
// 		bond_nominator(8, StakingBalance::KtonBalance(1), vec![3, 5]);
//
// 		start_era(1);
//
// 		assert_eq_uvec!(validator_controllers(), vec![4, 2]);
//
// 		// Saturate.
// 		assert_eq!(Staking::stakers(3).total_power, TOTAL_POWER / 2);
// 		assert_eq!(Staking::stakers(5).total_power, TOTAL_POWER / 2);
// 	})
// }
//
// // @review(power): check if the power result is right.
// #[test]
// fn phragmen_should_not_overflow_nominators() {
// 	ExtBuilder::default().nominate(false).build().execute_with(|| {
// 		let _ = Staking::chill(Origin::signed(10));
// 		let _ = Staking::chill(Origin::signed(20));
//
// 		bond_validator(2, StakingBalance::RingBalance(CAP));
// 		bond_validator(4, StakingBalance::KtonBalance(CAP));
//
// 		bond_nominator(6, StakingBalance::RingBalance(1), vec![3, 5]);
// 		bond_nominator(8, StakingBalance::KtonBalance(1), vec![3, 5]);
//
// 		start_era(1);
//
// 		assert_eq_uvec!(validator_controllers(), vec![4, 2]);
//
// 		// Saturate.
// 		assert_eq!(Staking::stakers(3).total_power, TOTAL_POWER / 2);
// 		assert_eq!(Staking::stakers(5).total_power, TOTAL_POWER / 2);
// 	})
// }
//
// // @review(power): check if the power result is right.
// #[test]
// fn phragmen_should_not_overflow_ultimate() {
// 	ExtBuilder::default().nominate(false).build().execute_with(|| {
// 		bond_validator(2, StakingBalance::RingBalance(CAP));
// 		bond_validator(4, StakingBalance::KtonBalance(CAP));
//
// 		bond_nominator(6, StakingBalance::RingBalance(CAP), vec![3, 5]);
// 		bond_nominator(8, StakingBalance::KtonBalance(CAP), vec![3, 5]);
//
// 		start_era(1);
//
// 		assert_eq_uvec!(validator_controllers(), vec![4, 2]);
//
// 		// Saturate.
// 		assert_eq!(Staking::stakers(3).total_power, TOTAL_POWER / 2 - TOTAL_POWER / 20);
// 		assert_eq!(Staking::stakers(5).total_power, TOTAL_POWER / 2 + TOTAL_POWER / 20);
// 	})
// }
//
// // @darwinia(reward)
// #[test]
// fn reward_validator_slashing_validator_doesnt_overflow() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		let stake = CAP * 2;
// 		let reward_slash = CAP * 2;
//
// 		// Assert multiplication overflows in balance arithmetic.
// 		assert!(stake.checked_mul(reward_slash).is_none());
//
// 		// Set staker
// 		let _ = Ring::make_free_balance_be(&11, stake);
// 		<Stakers<Test>>::insert(
// 			&11,
// 			Exposure {
// 				own_ring_balance: stake,
// 				total_power: stake as u32,
// 				own_kton_balance: 0,
// 				own_power: 0,
// 				others: vec![],
// 			},
// 		);
//
// 		// Check reward
// 		let _ = Staking::reward_validator(&11, reward_slash);
// 		assert_eq!(Ring::total_balance(&11), stake);
//
// 		// Set staker
// 		let _ = Ring::make_free_balance_be(&11, stake);
// 		let _ = Ring::make_free_balance_be(&2, stake);
//
// 		// only slashes out of bonded stake are applied. without this line,
// 		// it is 0.
// 		Staking::bond(
// 			Origin::signed(2),
// 			20000,
// 			StakingBalance::RingBalance(stake - 1),
// 			RewardDestination::default(),
// 			0,
// 		)
// 		.unwrap();
//
// 		<Stakers<Test>>::insert(
// 			&11,
// 			Exposure {
// 				own_ring_balance: stake,
// 				total_power: stake as u32,
// 				own_kton_balance: 0,
// 				own_power: 0,
// 				others: vec![IndividualExposure {
// 					who: 2,
// 					ring_balance: stake - 1,
// 					kton_balance: 0,
// 					power: 0,
// 				}],
// 			},
// 		);
//
// 		// Check slashing
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(100)],
// 		);
//
// 		assert_eq!(Ring::total_balance(&11), stake - 1000);
// 		assert_eq!(Ring::total_balance(&2), 1);
// 	})
// }
//
// #[test]
// fn reward_from_authorship_event_handler_works() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		use pallet_authorship::EventHandler;
//
// 		assert_eq!(<pallet_authorship::Module<Test>>::author(), 11);
//
// 		Staking::note_author(11);
// 		Staking::note_uncle(21, 1);
// 		// An uncle author that is not currently elected doesn't get rewards,
// 		// but the block producer does get reward for referencing it.
// 		Staking::note_uncle(31, 1);
// 		// Rewarding the same two times works.
// 		Staking::note_uncle(11, 1);
//
// 		// Not mandatory but must be coherent with rewards
// 		assert_eq!(<CurrentElected<Test>>::get(), vec![21, 11]);
//
// 		// 21 is rewarded as an uncle producer
// 		// 11 is rewarded as a block producer and uncle referencer and uncle producer
// 		assert_eq!(CurrentEraPointsEarned::get().individual, vec![1, 20 + 2 * 3 + 1]);
// 		assert_eq!(CurrentEraPointsEarned::get().total, 28);
// 	})
// }
//
// #[test]
// fn add_reward_points_fns_works() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		let validators = Staking::current_elected();
// 		// Not mandatory but must be coherent with rewards
// 		assert_eq!(validators, vec![21, 11]);
//
// 		Staking::reward_by_indices(vec![(0, 1), (1, 1), (2, 1), (1, 1)]);
//
// 		Staking::reward_by_ids(vec![(21, 1), (11, 1), (31, 1), (11, 1)]);
//
// 		assert_eq!(CurrentEraPointsEarned::get().individual, vec![2, 4]);
// 		assert_eq!(CurrentEraPointsEarned::get().total, 6);
// 	})
// }
//
// // @rm: `unbonded_balance_is_not_slashable` testcase
// // because `slashable_balance_of` is not used
//
// #[test]
// fn era_is_always_same_length() {
// 	// This ensures that the sessions is always of the same length if there is no forcing no
// 	// session changes.
// 	ExtBuilder::default().build().execute_with(|| {
// 		start_era(1);
// 		assert_eq!(Staking::current_era_start_session_index(), SessionsPerEra::get());
//
// 		start_era(2);
// 		assert_eq!(Staking::current_era_start_session_index(), SessionsPerEra::get() * 2);
//
// 		let session = Session::current_index();
// 		ForceEra::put(Forcing::ForceNew);
// 		advance_session();
// 		assert_eq!(Staking::current_era(), 3);
// 		assert_eq!(Staking::current_era_start_session_index(), session + 1);
//
// 		start_era(4);
// 		assert_eq!(
// 			Staking::current_era_start_session_index(),
// 			session + SessionsPerEra::get() + 1
// 		);
// 	});
// }
//
// #[test]
// fn offence_forces_new_era() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(5)],
// 		);
//
// 		assert_eq!(Staking::force_era(), Forcing::ForceNew);
// 	});
// }
//
// #[test]
// fn offence_ensures_new_era_without_clobbering() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		assert_ok!(Staking::force_new_era_always(Origin::ROOT));
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(5)],
// 		);
//
// 		assert_eq!(Staking::force_era(), Forcing::ForceAlways);
// 	});
// }
//
// #[test]
// fn offence_deselects_validator_when_slash_is_zero() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		assert!(<Validators<Test>>::contains_key(11));
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(0)],
// 		);
// 		assert_eq!(Staking::force_era(), Forcing::ForceNew);
// 		assert!(!<Validators<Test>>::contains_key(11));
// 	});
// }
//
// #[test]
// fn slashing_performed_according_exposure() {
// 	// This test checks that slashing is performed according the exposure (or more precisely,
// 	// historical exposure), not the current balance.
// 	ExtBuilder::default().build().execute_with(|| {
// 		assert_eq!(Staking::stakers(&11).own_ring_balance, 1000);
//
// 		// Handle an offence with a historical exposure.
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (
// 					11,
// 					Exposure {
// 						total_power: 500,
// 						own_ring_balance: 500,
// 						own_kton_balance: 0,
// 						own_power: 0,
// 						others: vec![],
// 					},
// 				),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(50)],
// 		);
//
// 		// The stash account should be slashed for 250 (50% of 500).
// 		assert_eq!(Ring::free_balance(11), 1000 - 250);
// 	});
// }
//
// #[test]
// fn slash_in_old_span_does_not_deselect() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		start_era(1);
//
// 		assert!(<Validators<Test>>::contains_key(11));
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(0)],
// 		);
// 		assert_eq!(Staking::force_era(), Forcing::ForceNew);
// 		assert!(!<Validators<Test>>::contains_key(11));
//
// 		start_era(2);
//
// 		Staking::validate(Origin::signed(10), Default::default()).unwrap();
// 		assert_eq!(Staking::force_era(), Forcing::NotForcing);
// 		assert!(<Validators<Test>>::contains_key(11));
//
// 		start_era(3);
//
// 		// this staker is in a new slashing span now, having re-registered after
// 		// their prior slash.
//
// 		on_offence_in_era(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(0)],
// 			1,
// 		);
//
// 		// not for zero-slash.
// 		assert_eq!(Staking::force_era(), Forcing::NotForcing);
// 		assert!(<Validators<Test>>::contains_key(11));
//
// 		on_offence_in_era(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			// NOTE: A 100% slash here would clean up the account, causing de-registration.
// 			&[Perbill::from_percent(95)],
// 			1,
// 		);
//
// 		// or non-zero.
// 		assert_eq!(Staking::force_era(), Forcing::NotForcing);
// 		assert!(<Validators<Test>>::contains_key(11));
// 		assert_ledger_consistent(11);
// 	});
// }
//
// // @review(reward): caculate the correct reward
// // #[test]
// // fn reporters_receive_their_slice() {
// // 	// This test verifies that the reporters of the offence receive their slice from the slashed
// // 	// amount.
// // 	ExtBuilder::default().build().execute_with(|| {
// // 		// The reporters' reward is calculated from the total exposure.
// // 		let initial_balance = 1125;
// //
// // 		assert_eq!(Staking::stakers(&11).own_ring_balance, initial_balance);
// //
// // 		on_offence_now(
// // 			&[OffenceDetails {
// // 				offender: (11, Staking::stakers(&11)),
// // 				reporters: vec![1, 2],
// // 			}],
// // 			&[Perbill::from_percent(50)],
// // 		);
// //
// // 		// F1 * (reward_proportion * slash - 0)
// // 		// 50% * (10% * initial_balance / 2)
// // 		let reward = (initial_balance / 20) / 2;
// // 		let reward_each = reward / 2; // split into two pieces.
// // 		assert_eq!(Ring::free_balance(1), 10 + reward_each);
// // 		assert_eq!(Ring::free_balance(2), 20 + reward_each);
// // 		assert_ledger_consistent(11);
// // 	});
// // }
//
// // @review(reward): caculate the correct reward
// // #[test]
// // fn subsequent_reports_in_same_span_pay_out_less() {
// // 	// This test verifies that the reporters of the offence receive their slice from the slashed
// // 	// amount.
// // 	ExtBuilder::default().build().execute_with(|| {
// // 		// The reporters' reward is calculated from the total exposure.
// // 		let initial_balance = 1125p;
// //
// // 		assert_eq!(Staking::stakers(&11).own_ring_balance, initial_balance);
// //
// // 		on_offence_now(
// // 			&[OffenceDetails {
// // 				offender: (11, Staking::stakers(&11)),
// // 				reporters: vec![1],
// // 			}],
// // 			&[Perbill::from_percent(20)],
// // 		);
// //
// // 		// F1 * (reward_proportion * slash - 0)
// // 		// 50% * (10% * initial_balance * 20%)
// // 		let reward = (initial_balance / 5) / 20;
// // 		assert_eq!(Ring::free_balance(1), 10 + reward);
// //
// // 		on_offence_now(
// // 			&[OffenceDetails {
// // 				offender: (11, Staking::stakers(&11)),
// // 				reporters: vec![1],
// // 			}],
// // 			&[Perbill::from_percent(50)],
// // 		);
// //
// // 		let prior_payout = reward;
// //
// // 		// F1 * (reward_proportion * slash - prior_payout)
// // 		// 50% * (10% * (initial_balance / 2) - prior_payout)
// // 		let reward = ((initial_balance / 20) - prior_payout) / 2;
// // 		assert_eq!(Ring::free_balance(1), 10 + prior_payout + reward);
// // 		assert_ledger_consistent(11);
// // 	});
// // }
//
// // @rm: `invulnerables_are_not_slashed` testcase
// // because `slashable_balance_of` is not used
//
// #[test]
// fn dont_slash_if_fraction_is_zero() {
// 	// Don't slash if the fraction is zero.
// 	ExtBuilder::default().build().execute_with(|| {
// 		assert_eq!(Ring::free_balance(11), 1000);
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(0)],
// 		);
//
// 		// The validator hasn't been slashed. The new era is not forced.
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		assert_ledger_consistent(11);
// 	});
// }
//
// #[test]
// fn only_slash_for_max_in_era() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		assert_eq!(Ring::free_balance(11), 1000);
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(50)],
// 		);
//
// 		// The validator has been slashed and has been force-chilled.
// 		assert_eq!(Ring::free_balance(11), 500);
// 		assert_eq!(Staking::force_era(), Forcing::ForceNew);
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(25)],
// 		);
//
// 		// The validator has not been slashed additionally.
// 		assert_eq!(Ring::free_balance(11), 500);
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(60)],
// 		);
//
// 		// The validator got slashed 10% more.
// 		assert_eq!(Ring::free_balance(11), 400);
// 		assert_ledger_consistent(11);
// 	})
// }
//
// #[test]
// fn garbage_collection_after_slashing() {
// 	ExtBuilder::default().existential_deposit(2).build().execute_with(|| {
// 		assert_eq!(Ring::free_balance(11), 256_000);
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(10)],
// 		);
//
// 		assert_eq!(Ring::free_balance(11), 256_000 - 25_600);
// 		assert!(<Staking as crate::Store>::SlashingSpans::get(&11).is_some());
// 		assert_eq!(
// 			<Staking as crate::Store>::SpanSlash::get(&(11, 0)).amount_slashed().r,
// 			25_600
// 		);
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(100)],
// 		);
//
// 		// validator and nominator slash in era are garbage-collected by era change,
// 		// so we don't test those here.
//
// 		assert_eq!(Ring::free_balance(11), 0);
// 		assert_eq!(Ring::total_balance(&11), 0);
//
// 		assert_ok!(Staking::reap_stash(Origin::NONE, 11));
//
// 		assert!(<Staking as crate::Store>::SlashingSpans::get(&11).is_none());
// 		assert_eq!(
// 			<Staking as crate::Store>::SpanSlash::get(&(11, 0)).amount_slashed().r,
// 			0
// 		);
// 	})
// }
//
// #[test]
// fn garbage_collection_on_window_pruning() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		start_era(1);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
//
// 		let exposure = Staking::stakers(&11);
// 		assert_eq!(Ring::free_balance(101), 2000);
// 		let nominated_value = exposure.others.iter().find(|o| o.who == 101).unwrap().ring_balance;
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(10)],
// 		);
//
// 		let now = Staking::current_era();
//
// 		assert_eq!(Ring::free_balance(11), 900);
// 		assert_eq!(Ring::free_balance(101), 2000 - (nominated_value / 10));
//
// 		assert!(<Staking as crate::Store>::ValidatorSlashInEra::get(&now, &11).is_some());
// 		assert!(<Staking as crate::Store>::NominatorSlashInEra::get(&now, &101).is_some());
//
// 		// + 1 because we have to exit the bonding window.
// 		for era in (0..(BondingDurationInEra::get() + 1)).map(|offset| offset + now + 1) {
// 			assert!(<Staking as crate::Store>::ValidatorSlashInEra::get(&now, &11).is_some());
// 			assert!(<Staking as crate::Store>::NominatorSlashInEra::get(&now, &101).is_some());
//
// 			start_era(era);
// 		}
//
// 		assert!(<Staking as crate::Store>::ValidatorSlashInEra::get(&now, &11).is_none());
// 		assert!(<Staking as crate::Store>::NominatorSlashInEra::get(&now, &101).is_none());
// 	})
// }
//
// // @rm: `slashing_nominators_by_span_max` testcase
// // because `slashable_balance_of` is not used
//
// // @rm: `slashes_are_summed_across_spans` testcase
// // because `slashable_balance_of` is not used
//
// #[test]
// fn deferred_slashes_are_deferred() {
// 	ExtBuilder::default().slash_defer_duration(2).build().execute_with(|| {
// 		start_era(1);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
//
// 		let exposure = Staking::stakers(&11);
// 		assert_eq!(Ring::free_balance(101), 2000);
// 		let nominated_value = exposure.others.iter().find(|o| o.who == 101).unwrap().ring_balance;
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, Staking::stakers(&11)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(10)],
// 		);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		assert_eq!(Ring::free_balance(101), 2000);
//
// 		start_era(2);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		assert_eq!(Ring::free_balance(101), 2000);
//
// 		start_era(3);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		assert_eq!(Ring::free_balance(101), 2000);
//
// 		// at the start of era 4, slashes from era 1 are processed,
// 		// after being deferred for at least 2 full eras.
// 		start_era(4);
//
// 		assert_eq!(Ring::free_balance(11), 900);
// 		assert_eq!(Ring::free_balance(101), 2000 - (nominated_value / 10));
// 	})
// }
//
// #[test]
// fn remove_deferred() {
// 	ExtBuilder::default().slash_defer_duration(2).build().execute_with(|| {
// 		start_era(1);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
//
// 		let exposure = Staking::stakers(&11);
// 		assert_eq!(Ring::free_balance(101), 2000);
// 		let nominated_value = exposure.others.iter().find(|o| o.who == 101).unwrap().ring_balance;
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, exposure.clone()),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(10)],
// 		);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		assert_eq!(Ring::free_balance(101), 2000);
//
// 		start_era(2);
//
// 		on_offence_in_era(
// 			&[OffenceDetails {
// 				offender: (11, exposure.clone()),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(15)],
// 			1,
// 		);
//
// 		Staking::cancel_deferred_slash(Origin::ROOT, 1, vec![0]).unwrap();
//
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		assert_eq!(Ring::free_balance(101), 2000);
//
// 		start_era(3);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		assert_eq!(Ring::free_balance(101), 2000);
//
// 		// at the start of era 4, slashes from era 1 are processed,
// 		// after being deferred for at least 2 full eras.
// 		start_era(4);
//
// 		// the first slash for 10% was cancelled, so no effect.
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		assert_eq!(Ring::free_balance(101), 2000);
//
// 		start_era(5);
//
// 		let slash_10 = Perbill::from_percent(10);
// 		let slash_15 = Perbill::from_percent(15);
// 		let initial_slash = slash_10 * nominated_value;
//
// 		let total_slash = slash_15 * nominated_value;
// 		let actual_slash = total_slash - initial_slash;
//
// 		// 5% slash (15 - 10) processed now.
// 		assert_eq!(Ring::free_balance(11), 950);
// 		assert_eq!(Ring::free_balance(101), 2000 - actual_slash);
// 	})
// }
//
// #[test]
// fn remove_multi_deferred() {
// 	ExtBuilder::default().slash_defer_duration(2).build().execute_with(|| {
// 		start_era(1);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
//
// 		let exposure = Staking::stakers(&11);
// 		assert_eq!(Ring::free_balance(101), 2000);
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, exposure.clone()),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(10)],
// 		);
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (21, Staking::stakers(&21)),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(10)],
// 		);
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, exposure.clone()),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(25)],
// 		);
//
// 		assert_eq!(<Staking as Store>::UnappliedSlashes::get(&1).len(), 3);
// 		Staking::cancel_deferred_slash(Origin::ROOT, 1, vec![0, 2]).unwrap();
//
// 		let slashes = <Staking as Store>::UnappliedSlashes::get(&1);
// 		assert_eq!(slashes.len(), 1);
// 		assert_eq!(slashes[0].validator, 21);
// 	})
// }
//
// #[test]
// fn slash_kicks_validators_not_nominators() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		start_era(1);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
//
// 		let exposure = Staking::stakers(&11);
// 		assert_eq!(Ring::free_balance(101), 2000);
// 		let nominated_value = exposure.others.iter().find(|o| o.who == 101).unwrap().ring_balance;
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, exposure.clone()),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(10)],
// 		);
//
// 		assert_eq!(Ring::free_balance(11), 900);
// 		assert_eq!(Ring::free_balance(101), 2000 - (nominated_value / 10));
//
// 		// This is the best way to check that the validator was chilled; `get` will
// 		// return default value.
// 		for (stash, _) in <Staking as Store>::Validators::enumerate() {
// 			assert!(stash != 11);
// 		}
//
// 		let nominations = <Staking as Store>::Nominators::get(&101).unwrap();
//
// 		// and make sure that the vote will be ignored even if the validator
// 		// re-registers.
// 		let last_slash = <Staking as Store>::SlashingSpans::get(&11)
// 			.unwrap()
// 			.last_nonzero_slash();
// 		assert!(nominations.submitted_in < last_slash);
// 	});
// }
//
// #[test]
// fn zero_slash_keeps_nominators() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		start_era(1);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
//
// 		let exposure = Staking::stakers(&11);
// 		assert_eq!(Ring::free_balance(101), 2000);
//
// 		on_offence_now(
// 			&[OffenceDetails {
// 				offender: (11, exposure.clone()),
// 				reporters: vec![],
// 			}],
// 			&[Perbill::from_percent(0)],
// 		);
//
// 		assert_eq!(Ring::free_balance(11), 1000);
// 		assert_eq!(Ring::free_balance(101), 2000);
//
// 		// This is the best way to check that the validator was chilled; `get` will
// 		// return default value.
// 		for (stash, _) in <Staking as Store>::Validators::enumerate() {
// 			assert!(stash != 11);
// 		}
//
// 		let nominations = <Staking as Store>::Nominators::get(&101).unwrap();
//
// 		// and make sure that the vote will not be ignored, because the slash was
// 		// zero.
// 		let last_slash = <Staking as Store>::SlashingSpans::get(&11)
// 			.unwrap()
// 			.last_nonzero_slash();
// 		assert!(nominations.submitted_in >= last_slash);
// 	});
// }
