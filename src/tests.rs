use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, ensure};
use frame_system::ensure_signed;
use frame_system::pallet_prelude::OriginFor;
use sp_runtime::DispatchError::BadOrigin;

impl<T: Config<I>, I: 'static> Pallet<T, I> {
    /// Simple extrinsic that success if the caller is a verified artist
    pub fn test_artist_caller(origin: OriginFor<T>) -> DispatchResult {
        let caller = ensure_signed(origin)?;
        ensure!(Members::<T, I>::get().contains(&caller), Error::<T, I>::NotAnArtist);

        Ok(())
    }
}

// Test accounts used
pub const ALICE: <Test as frame_system::Config>::AccountId = 0;
pub const BOB: <Test as frame_system::Config>::AccountId = 1;

/// Genesis tests
#[test]
fn genesis_config() {
    new_test_ext(false).execute_with(||{
        assert!(Artists::get_artist(0) == 
            Some(ArtistInfos {
                id: 0,
                account: ALICE,
                name: b"Genesis Artist".to_vec().try_into().unwrap(),
                age: 0, // Genesis block is 0
            }
        ));
        assert!(ArtistCommittee::is_member(&ALICE))
    });
}

#[test]
fn create_artist_root() {
    new_test_ext(true).execute_with(||{
        // expect that nobody is in the artists group
        assert!(!ArtistCommittee::is_member(&ALICE));
        // should create an artist, creating his assets
        // and registering the artist account in the artists group
        assert_ok!(Artists::force_create(
            Origin::root(),
            1,
            ALICE,
            b"Test Artist".to_vec(),
            b"Test Artist Asset".to_vec(),
            b"TAA".to_vec(),
        ));
        // expect ALICE to be in the artists group now
        assert!(ArtistCommittee::is_member(&ALICE));

        // Shouldn't be able to create an artist with the same artist ID
        assert_noop!(Artists::force_create(
            Origin::root(),
            1,
            ALICE,
            b"Test Artist 2".to_vec(),
            b"Test Artist Asset 2".to_vec(),
            b"TAA2".to_vec(),
        ), Error::<Test>::AlreadyExist);
    });
}

#[test]
fn caller_is_artist() {
    new_test_ext(false).execute_with(||{
        // Should execute the extrinsic as `ALICE` is in the artists group
        assert_ok!(Artists::test_artist_caller(Origin::signed(ALICE)));
        // Should refuse the root origin then
        assert_noop!(
            Artists::test_artist_caller(Origin::root()),
            BadOrigin
        );
        // Should refuse the root origin then
        assert_noop!(
            Artists::test_artist_caller(Origin::signed(BOB)),
            Error::<Test>::NotAnArtist
        );
    })
}