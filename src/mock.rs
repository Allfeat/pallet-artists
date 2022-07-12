use super::*;
use crate::{
    self as pallet_artists,
    mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GenesisBuild,
    tests::{ALICE, BOB, JOHN},
};

use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU32, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

pub const DAYS: u32 = 24 * 60 * 60 * 1000;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

type AccountId = u64;

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
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
    type MaxConsumers = ConstU32<2>;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}

impl pallet_assets::Config for Test {
    type Event = Event;
    type Balance = u64;
    type AssetId = u32;
    type Currency = Balances;
    type ForceOrigin = frame_system::EnsureRoot<u64>;
    type AssetDeposit = ConstU64<1>;
    type AssetAccountDeposit = ConstU64<10>;
    type MetadataDepositBase = ConstU64<1>;
    type MetadataDepositPerByte = ConstU64<1>;
    type ApprovalDeposit = ConstU64<1>;
    type StringLimit = ConstU32<50>;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = ();
}

parameter_types! {
    pub const TechnicalMotionDuration: u32 = 5 * DAYS;
    pub const TechnicalMaxProposals: u32 = 30;
    pub const TechnicalMaxMembers: u32 = 10;
}

type TechnicalCollective = pallet_collective::Instance1;
impl pallet_collective::Config<TechnicalCollective> for Test {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = ();
    type MaxProposals = TechnicalMaxProposals;
    type MaxMembers = TechnicalMaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
    type WeightInfo = ();
}

type HalfOfCouncilOrigin =
    pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 2>;

// TODO: Why the following does not work with ArtistMamagerOrigin?
//   type EnsureRootOrHalfCouncil = EnsureOneOf<
//     EnsureRoot<AccountId>,
//     pallet_collective::EnsureProportionMoreThan<AccountId, TechnicalCommittee, 1, 2>,
// >;

parameter_types! {
    // We use small max values for testing purpose
    pub const CreationDepositAmount: u64 = 10;
    pub const MaxArtists: u32 = 5;
    pub const MaxCandidates: u32 = 10;
    pub const NameMaxLength: u32 = 20;
}

impl pallet_artists::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type ArtistsManagerOrigin = HalfOfCouncilOrigin;
    type MembershipInitialized = TechnicalCommittee;
    type MembershipChanged = TechnicalCommittee;
    type CreationDepositAmount = CreationDepositAmount;
    type MaxArtists = MaxArtists;
    type MaxCandidates = MaxCandidates;
    type NameMaxLength = NameMaxLength;
}

construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Balances: pallet_balances,
        TechnicalCommittee: pallet_collective::<Instance1>,
        Assets: pallet_assets,
        ArtistsPallet: pallet_artists,
    }
);

use pallet_collective::Instance1;

pub(crate) fn new_test_ext(include_genesis: bool) -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    let config: pallet_balances::GenesisConfig<Test> = pallet_balances::GenesisConfig {
        balances: vec![(ALICE.into(), 100), (JOHN.into(), 100), (BOB.into(), 100)],
    };

    let artists_config: pallet_artists::GenesisConfig<Test> = match include_genesis {
        true => pallet_artists::GenesisConfig {
            artists: vec![(ALICE, "Genesis Alice".into())],
            candidates: vec![(BOB, "Genesis Bob".into())],
        },
        false => pallet_artists::GenesisConfig::default(),
    };

    let technical_collective_config: pallet_collective::GenesisConfig<Test, Instance1> =
        pallet_collective::GenesisConfig {
            members: vec![ALICE],
            phantom: Default::default(),
        };

    config.assimilate_storage(&mut storage).unwrap();
    artists_config.assimilate_storage(&mut storage).unwrap();
    technical_collective_config
        .assimilate_storage(&mut storage)
        .unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| {
        // assert_ok!(TechnicalCommittee::init_members(Origin::root(), ALICE));
        System::set_block_number(1);
    });
    ext
}
