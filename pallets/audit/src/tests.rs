use crate::{Error, mock::*};
use frame_support::{assert_err, assert_noop, assert_ok, dispatch::{
		DispatchResult, 
		DispatchError, 
		Vec,
}};


#[test]
fn it_works_for_create_new_file() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = 666_666_u64;

		assert_ok!(Audit::create_new_file(Origin::signed(1), tag.clone(), filehash), ());
		assert_ok!(Audit::create_new_file(Origin::signed(1), tag.clone(), 888), ());
		assert_ok!(Audit::create_new_file(Origin::signed(1), tag.clone(), 999), ());
		
		let file_result = Audit::get_file_by_id(Origin::signed(1), 0);
		let file_result = Audit::get_file_by_id(Origin::signed(1), 1);
		let file_result = Audit::get_file_by_id(Origin::signed(1), 2);
		let file_result = Audit::get_file_by_id(Origin::signed(1), 3);
		let file_result = Audit::get_file_by_id(Origin::signed(1), 5);
		assert_ok!(file_result, ());
		//todo!("add storage tests");
	});
}

#[test]
fn it_fails_for_create_new_file_incorrect_file_input() {
	new_test_ext().execute_with(|| {
		let tag = Vec::new();
		let filehash = 666_666_u64;

		assert_eq!(Audit::create_new_file(Origin::signed(1), tag, filehash), DispatchResult::Err(DispatchError::Other("empty file error")));
		//todo!("add storage tests");
	});
}