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

		let a = Audit::get_file_by_id(Origin::signed(1), 1);
		todo!("add storage check tests");
	});
}

#[test]
fn it_fails_for_create_new_file_incorrect_file_input() {
	new_test_ext().execute_with(|| {
		let tag = Vec::new();
		let filehash = 666_666_u64;

		assert_eq!(Audit::create_new_file(Origin::signed(1), tag, filehash), DispatchResult::Err(DispatchError::Other("empty file error")));
		todo!("add storage check tests");
	});
}

#[test]
fn it_works_sign_latest_version() {
	new_test_ext().execute_with(|| {
		let tag = Vec::new();
		let filehash = 666_666_u64;

		assert_eq!(Audit::create_new_file(Origin::signed(1), tag, filehash), DispatchResult::Err(DispatchError::Other("empty file error")));
		let account_id = 1;
		let sign_latest_version_result = Audit::sign_latest_version(Origin::signed(1), 1);
		todo!("continue test");

	});
}

#[test]
fn it_works_assign_auditor() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = 666_666_u64;
		let account_id = 1;

		let create_file_result = Audit::create_new_file(Origin::signed(1), tag.clone(), filehash);
		let assign_auditor_result = Audit::assign_auditor(Origin::signed(1), 1, account_id);

		assert_ok!(create_file_result, ());
		assert_ok!(assign_auditor_result, ());
	});
}

#[test]
fn it_works_delete_auditor() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = 666_666_u64;
		let account_id = 1;

		let create_file_result = Audit::create_new_file(Origin::signed(1), tag.clone(), filehash);
		let assign_auditor_result = Audit::assign_auditor(Origin::signed(1), 1, account_id);
		let delete_auditor_result = Audit::delete_auditor(Origin::signed(1), 1, account_id);

		assert_ok!(create_file_result, ());
		assert_ok!(assign_auditor_result, ());
		assert_ok!(delete_auditor_result, ());
	});
}

#[test]
fn it_fails_delete_auditor_no_auditors() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = 666_666_u64;

		let account_id = 1;
		let _ = Audit::create_new_file(Origin::signed(1), tag.clone(), filehash);

		// First - try to delete unexisting auditor 
		let delete_auditor_result_no_auditors = Audit::delete_auditor(Origin::signed(1), 1, account_id);

		// Second - try to delete unexisting auditor after delete:
		let _ = Audit::assign_auditor(Origin::signed(1), 1, account_id);
		let _ = Audit::delete_auditor(Origin::signed(1), 1, account_id);
		let delete_auditor_result_after_delete = Audit::delete_auditor(Origin::signed(1), 1, account_id);

		assert_ne!(delete_auditor_result_no_auditors, DispatchResult::Ok(()));
		assert_ne!(delete_auditor_result_after_delete, DispatchResult::Ok(()));
	});
}
