use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};


#[test]
fn it_works_for_create_new_file() {
	new_test_ext().execute_with(|| {
		//Dispatch a signed extrinsic.
		let tag = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = 666_666_u64;

		assert_ok!(Audit::create_new_file(Origin::signed(1) ,tag, filehash), ());
		todo!("add storage tests");
	});
}