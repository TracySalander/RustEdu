use crate as pallet_kitties;
use frame_support::traits::{ConstU16, ConstU64, ConstU32, ConstU128};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use pallet_randomness_collective_flip;
use frame_support::parameter_types;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;

// Configure a mock runtime to test the pallet
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        KittiesModule: pallet_kitties::{Pallet, Call, Storage, Event<T>},
        Randomness: pallet_randomness_collective_flip::{Pallet, Storage},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>, Config<T>},
    }
);

impl system::Config for Test{
    type BaseCallFilter = frame_support::traits::Everything;
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
    type Event = Event;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const KittyReserve: u64 = 1000;
    pub const EssentialDeposit: u128 = 500;
    pub const MaxLock: u32 = 50;
    pub const MaxLength: u32 = 4;
}

impl pallet_kitties::Config for Test{
    type Event = Event;
    type Randomness = Randomness;
	type KittyIndex = u32;
	type MaxLength = ConstU32<64>;
	type KittyReserve = KittyReserve;
	type Currency = Balances;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<500>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}

impl pallet_randomness_collective_flip::Config for Test{
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities{
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10_000_000_000), (2, 10_000_000_000), (3, 10_000_000_000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
    sp_io::TestExternalities::new(t)
}