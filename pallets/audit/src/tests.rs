use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};


//type Audit = Module<Test>;

#[test]
fn it_works_for_default_value() {
	// new_test_ext().execute_with(|| {
	// 	// Dispatch a signed extrinsic.
	// 	// assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
	// 	// // Read pallet storage and assert an expected result.
	// 	// assert_eq!(TemplateModule::something(), Some(42));
	// });
}


#[test]
fn it_works_for_file_create() {
	// new_test_ext().execute_with(|| {
	// 	// Dispatch a signed extrinsic.
	// 	// assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
	// 	// // Read pallet storage and assert an expected result.
	// 	// assert_eq!(TemplateModule::something(), Some(42));
	// });

	new_test_ext().execute_with(|| {
		//Dispatch a signed extrinsic.
		//let tag = Vec::<u8>::new();
		let tag = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = 666_666_u64;

		assert_ok!(Audit::create_new_file(Origin::signed(1) ,tag, filehash), ());

		// Read pallet storage and assert an expected result.
		//assert_eq!(TemplateModule::something(), Some(42));
	});
}