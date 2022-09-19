use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use crate::NextKittyId;

// #[test]
// fn it_works_for_create(){
//     new_test_ext().execute_with(||{
//         //Dispatch a signed extrinsic. 
//         System::set_block_number(1);

//         assert_ok!(KittiesModule::create(Origin::signed(1)));
//         assert_eq!(KittiesModule::next_kitty_id(), 1);
        
//         assert_noop!(KittiesModule::transfer(Origin::signed(2), 0, 1), Error::<Test>::NotOwner);
//     })
// }

#[test]
fn create_success(){
    new_test_ext().execute_with(||{
        System::set_block_number(1);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_eq!(KittiesModule::next_kitty_id(), 1);
    })
}

#[test]
fn create_failed_not_enough_fee(){
    new_test_ext().execute_with(||{
        System::set_block_number(1);
        assert_noop!(KittiesModule::create(Origin::signed(5)), Error::<Test>::NotEnoughFee);
    })
}

#[test]
fn create_failed_invalid_kitty_id(){
    new_test_ext().execute_with(||{
        System::set_block_number(1);
        NextKittyId::<Test>::put(u32::max_value());
        assert_noop!(KittiesModule::create(Origin::signed(1)), Error::<Test>::NotEnoughStorage);
    })
}

#[test]
fn breed_success(){
    new_test_ext().execute_with(||{
        System::set_block_number(1);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
        assert_eq!(KittiesModule::next_kitty_id(), 3);
    })
}

#[test]
fn breed_fail_same_kitty_id(){
    new_test_ext().execute_with(||{
        System::set_block_number(1);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 0), Error::<Test>::SameKittyId);
    })
}

#[test]
fn breed_fail_invalid_kitty_id(){
    new_test_ext().execute_with(||{
        System::set_block_number(1);
        assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 1), Error::<Test>::InvalidKittyId);
    })
}

#[test]
fn breed_fail_not_enough_storage(){
    new_test_ext().execute_with(||{
        System::set_block_number(1);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        NextKittyId::<Test>::put(u32::max_value());
        assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 1), Error::<Test>::NotEnoughStorage);
    })
}

#[test]
fn transfer_success() {
    new_test_ext().execute_with(||{
        System::set_block_number(1);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::transfer(Origin::signed(1), 0, 2));
    })
}

#[test]
fn transfer_fail_not_owner(){
    new_test_ext().execute_with(||{
        System::set_block_number(1);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(KittiesModule::transfer(Origin::signed(2), 0, 1), Error::<Test>::NotOwner);
    })
}
