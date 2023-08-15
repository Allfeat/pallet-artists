use crate::{
    self as pallet_artists,
    tests::{ALICE, BOB},
};
use codec::{Decode, Encode, MaxEncodedLen};

use frame_support::traits::AsEnsureOriginWithArg;
use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU32, ConstU64},
};
use frame_system::EnsureRoot;
use scale_info::TypeInfo;
use sp_core::{RuntimeDebug, H256};
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

#[derive(
    Encode,
    Decode,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    MaxEncodedLen,
    TypeInfo,
    RuntimeDebug,
)]
pub enum TestId {
    Foo,
    Bar,
    Baz,
}

pub const DAYS: u32 = 24 * 60 * 60 * 1000;

type Block = frame_system::mocking::MockBlock<Test>;

type AccountId = u64;

impl frame_system::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type BlockHashCount = ConstU64<250>;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u64;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type MaxHolds = ();
}

impl pallet_assets::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u64;
    type RemoveItemsLimit = ConstU32<5>;
    type AssetId = u32;
    type AssetIdParameter = u32;
    type Currency = Balances;
    type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
    type ForceOrigin = EnsureRoot<u64>;
    type AssetDeposit = ConstU64<1>;
    type AssetAccountDeposit = ConstU64<10>;
    type MetadataDepositBase = ConstU64<1>;
    type MetadataDepositPerByte = ConstU64<1>;
    type ApprovalDeposit = ConstU64<1>;
    type StringLimit = ConstU32<50>;
    type Freezer = ();
    type Extra = ();
    type CallbackHandle = ();
    type WeightInfo = ();
}

parameter_types! {
    // We use small max values for testing purpose
    pub const CreationDepositAmount: u64 = 10;
    pub const MaxArtists: u32 = 5;
    pub const MaxCandidates: u32 = 10;
    pub const NameMaxLength: u32 = 20;
}

impl pallet_artists::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Origin = RuntimeOrigin;
    type AdminOrigin = EnsureRoot<AccountId>;
    type Call = RuntimeCall;
    type CreationDepositAmount = CreationDepositAmount;
    type NameMaxLength = NameMaxLength;
    type WeightInfo = ();
}

construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        Assets: pallet_assets,
        ArtistsPallet: pallet_artists,
    }
);

pub(crate) fn new_test_ext(include_genesis: bool) -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    // Give 100 tokens to the 100 first accounts
    let config: pallet_balances::GenesisConfig<Test> = pallet_balances::GenesisConfig {
        balances: (0..100)
            .collect::<Vec<u64>>()
            .iter()
            .map(|&account_id| (account_id, 100))
            .collect(),
    };

    let artists_config: pallet_artists::GenesisConfig<Test> = match include_genesis {
        true => pallet_artists::GenesisConfig {
            artists: vec![(ALICE, "Genesis Alice".into())],
            candidates: vec![(BOB, "Genesis Bob".into())],
        },
        false => pallet_artists::GenesisConfig::default(),
    };

    config.assimilate_storage(&mut storage).unwrap();
    artists_config.assimilate_storage(&mut storage).unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}
