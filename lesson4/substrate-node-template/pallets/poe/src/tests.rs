// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number()));
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
            TemplateModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn create_claim_failed_when_claim_length_too_long() {
    new_test_ext().execute_with(|| {
        let claim = vec![0; 7];
        assert_noop!(
            TemplateModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::LengthTooLong
        );
    })
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0; 6];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(TemplateModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}

#[test]
fn revoke_claim_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TemplateModule::revoke_claim(Origin::signed(1), vec![9;2]),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0;6];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));

        assert_noop!(
            TemplateModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotOwner
        );
    })
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0; 6];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(TemplateModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
        assert!(Proofs::<Test>::contains_key(&claim));
        assert_eq!(Proofs::<Test>::get(claim).0, 2);
    })
}

#[test]
fn transfer_claim_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TemplateModule::transfer_claim(Origin::signed(1), vec![0, 5], 2),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn transfer_claim_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0; 6];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
            TemplateModule::transfer_claim(Origin::signed(3), claim.clone(), 2),
            Error::<Test>::NotOwner
        );
    })
}

