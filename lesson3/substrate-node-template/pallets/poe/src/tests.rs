// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

// test case for create_claim
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone(), 100));
        assert_eq!(Proofs::<Test>::get(&claim), (1,system::Module::<Test>::block_number(), 100));
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), 100);

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone(), 100),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn create_claim_failed_when_claim_is_too_long() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1 ,2, 3, 4, 5, 6];

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone(), 100),
            Error::<Test>::ProofTooLong
        );
    })
}

// test cases for revoke_claim
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1 ];
        let _= PoeModule::create_claim(Origin::signed(1), claim.clone(), 100);

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone(), 100));
    })
}

#[test]
fn revoke_claim_failed_when_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1 ];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone(), 100),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1 ];
        let _= PoeModule::create_claim(Origin::signed(1), claim.clone(), 100);

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone(), 100),
            Error::<Test>::NotClaimOwner
        );
    })
}

// test case for transfer_claim
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone(), 100);

        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2, 100));
    })
}

#[test]
fn transfer_claim_when_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2, 100),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _= PoeModule::create_claim(Origin::signed(1), claim.clone(), 100);

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3, 100),
            Error::<Test>::NotClaimOwner
        );
    })
}

#[test]
fn buy_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone(), 100);

        assert_ok!(PoeModule::buy_claim(Origin::signed(2), claim.clone(), 200));
    })
}

#[test]
fn buy_claim_failed_with_low_price() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone(), 100);

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 500),
            Error::<Test>::TransferPriceToLow
        );
    })
}

#[test]
fn buy_claim_failed_when_is_no_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 200),
            Error::<Test>::ClaimNotExist
        );
    })
}