// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;
//创建存证的测试用例
#[test]
fn create_claim_works(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
    })
}
#[test]
fn create_claim_failed_when_claim_already_exist(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1),claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
        // assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
    })
}
#[test]
fn create_claim_failed_when_claim_too_long(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1,2,3,4,5,6];
        let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1),claim.clone()),
            Error::<Test>::ProofTooLong
        );
        // assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
    })
}
//撤销存证的测试用例
#[test]
fn revoke_claim_works(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()));
        // assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
    })
}
#[test]
fn revoke_claim_failed_when_claim_not_exist(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1),claim.clone()),
            Error::<Test>::ClaimNotExist
        );
        // assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
    })
}
#[test]
fn revoke_claim_failed_with_wrong_owner(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2),claim.clone()),
            Error::<Test>::NotClaimOwner
        );
        // assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
    })
}
//转移存证的测试用例
#[test]
fn transfer_claim_works(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1),claim.clone(),3));
    })
}
#[test]
fn transfer_claim_failed_when_claim_not_exist(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1),claim.clone(),3),
            Error::<Test>::ClaimNotExist
        );
        // assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
    })
}
#[test]
fn transfer_claim_failed_with_wrong_owner(){
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2),claim.clone(),3),
            Error::<Test>::NotClaimOwner
        );
        // assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
    })
}
