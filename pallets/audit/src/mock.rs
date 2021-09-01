use crate as pallet_audit;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use frame_system as system;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage},
		Audit: pallet_audit::{Pallet, Call, Storage},
	}
);

impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

impl pallet_audit::Config for Test {
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

// static ROLES: [(u64, u8); 8] = [
//     (1_u64, MASTER_ROLE_MASK),
//     (2_u64, CUSTODIAN_ROLE_MASK),
//     (3_u64, ISSUER_ROLE_MASK),
//     (4_u64, INVESTOR_ROLE_MASK),
//     (5_u64, AUDITOR_ROLE_MASK),
//     (6_u64, INVESTOR_ROLE_MASK),
//     (7_u64, ISSUER_ROLE_MASK | INVESTOR_ROLE_MASK),
//     (8_u64, MANAGER_ROLE_MASK),
// ];