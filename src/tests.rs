use super::*;
use crate::{
    mock::{RuntimeOrigin, *},
    Event::*,
};
use allfeat_support::types::actors::artist::{ArtistData, CandidateData};
use rand::{thread_rng, Rng};

use frame_support::{assert_noop, assert_ok, ensure};
use frame_system::pallet_prelude::BlockNumberFor;
use frame_system::{ensure_signed, pallet_prelude::OriginFor};
use sp_runtime::traits::BadOrigin;

type AccountId = <Test as frame_system::Config>::AccountId;

// Test accounts used
pub const ALICE: AccountId = 0; // Root, Artist
pub const BOB: AccountId = 1; // Candidate
pub const JOHN: AccountId = 2; // Nothing

/// Helper function that generates a random string from a given length
/// Should only be used for testing purpose
fn generate_random_string(length: usize) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = String::with_capacity(length);
    let mut rng = thread_rng();
    for _ in 0..length {
        let x: usize = rng.gen();
        result.push(chars[x % chars.len()])
    }
    result
}

/// Panic is the given event is different that the last emitted event
fn assert_last_event(event: pallet::Event<Test>) {
    System::assert_last_event(mock::RuntimeEvent::ArtistsPallet(event))
}

impl<T: Config> Pallet<T> {
    /// Simple extrinsic that success if the caller is a certified artist
    pub fn test_caller_is_artist(origin: OriginFor<T>) -> DispatchResult {
        let caller = ensure_signed(origin)?;
        ensure!(Artists::<T>::contains_key(&caller), Error::<T>::NotAnArtist);
        Ok(())
    }

    /// Simple extrinsic that success if the caller is a certified artist
    pub fn test_caller_is_candidate(origin: OriginFor<T>) -> DispatchResult {
        let caller = ensure_signed(origin)?;
        ensure!(
            Candidates::<T>::contains_key(&caller),
            Error::<T>::NotACandidate
        );
        Ok(())
    }
}

/// Genesis tests
#[test]
fn test_genesis_config() {
    new_test_ext(true).execute_with(|| {
        // Test genesis from artists:
        // ==========================
        let artist = ArtistsPallet::get_artist(ALICE).unwrap();
        let expected_artist: ArtistData<
            BoundedVec<u8, <Test as Config>::NameMaxLength>,
            BlockNumberFor<Test>,
        > = ArtistData {
            name: b"Genesis Alice".to_vec().try_into().unwrap(),
            created_at: 0,
        };

        assert_eq!(artist.name, expected_artist.name);
        assert_eq!(artist.created_at, expected_artist.created_at);

        // Ensure that the deposit is also effected in the genesis build
        let deposit = CreationDepositAmount::get();
        let alice_balance = Balances::free_balance(ALICE);
        let alice_reserve = Balances::reserved_balance(ALICE);

        assert_eq!(alice_reserve, deposit);
        assert_eq!(alice_balance, 100 - deposit);

        // Test genesis from artists:
        // ==========================
        let candidate = ArtistsPallet::get_candidate(BOB).unwrap();
        let expected_candidate: CandidateData<
            BoundedVec<u8, <Test as Config>::NameMaxLength>,
            BlockNumberFor<Test>,
        > = CandidateData {
            name: b"Genesis Bob".to_vec().try_into().unwrap(),
            created_at: 0,
        };

        assert_eq!(candidate.name, expected_candidate.name);
        assert_eq!(candidate.created_at, expected_candidate.created_at);

        // Ensure that the deposit is also effected in the genesis build
        let deposit = CreationDepositAmount::get();
        let bob_balance = Balances::free_balance(BOB);
        let bob_reserve = Balances::reserved_balance(BOB);

        assert_eq!(bob_reserve, deposit);
        assert_eq!(bob_balance, 100 - deposit);
    });
}

#[test]
fn test_submit_candidacy_with_too_long_name() {
    new_test_ext(true).execute_with(|| {
        let name = generate_random_string(60);

        assert_noop!(
            ArtistsPallet::submit_candidacy(
                mock::RuntimeOrigin::signed(JOHN),
                name.as_bytes().to_vec().try_into().unwrap()
            ),
            Error::<Test>::NameTooLong
        );
    });
}

#[test]
fn test_submit_candidacy_should_fail_for_existing_artist() {
    new_test_ext(true).execute_with(|| {
        assert_noop!(
            ArtistsPallet::submit_candidacy(
                RuntimeOrigin::signed(ALICE),
                b"Alice".to_vec().try_into().unwrap()
            ),
            Error::<Test>::AlreadyAnArtist
        );
    });
}

#[test]
fn test_submit_candidacy_should_fail_for_existing_candidate() {
    new_test_ext(true).execute_with(|| {
        assert_noop!(
            ArtistsPallet::submit_candidacy(
                RuntimeOrigin::signed(BOB),
                b"Bob".to_vec().try_into().unwrap()
            ),
            Error::<Test>::AlreadyACandidate
        );
    });
}

#[test]
fn test_only_an_existing_candidacy_could_be_removed() {
    new_test_ext(true).execute_with(|| {
        assert_noop!(
            ArtistsPallet::withdraw_candidacy(RuntimeOrigin::signed(JOHN)),
            Error::<Test>::NotACandidate
        );
    });
}

#[test]
fn test_submit_candidacy_twice_should_fail() {
    new_test_ext(true).execute_with(|| {
        assert_noop!(
            ArtistsPallet::submit_candidacy(
                RuntimeOrigin::signed(BOB),
                b"Bobby".to_vec().try_into().unwrap(),
            ),
            Error::<Test>::AlreadyACandidate
        );
    });
}

#[test]
fn test_submit_candidacy() {
    new_test_ext(true).execute_with(|| {
        // John should be able to candidate
        assert_ok!(ArtistsPallet::submit_candidacy(
            RuntimeOrigin::signed(JOHN),
            b"Johnny".to_vec().try_into().unwrap()
        ));

        // Ensure that the deposit is also effected in the genesis build
        let deposit = CreationDepositAmount::get();
        let balance = Balances::free_balance(JOHN);
        let reserve = Balances::reserved_balance(JOHN);

        assert_eq!(reserve, deposit);
        assert_eq!(balance, 100 - deposit);

        // John should now be in the candidate list
        assert_ok!(ArtistsPallet::test_caller_is_candidate(
            RuntimeOrigin::signed(JOHN)
        ));

        assert_last_event(CandidateAdded(JOHN));
    });
}

#[test]
fn test_withdraw_candidacy() {
    new_test_ext(true).execute_with(|| {
        let deposit = CreationDepositAmount::get();
        let initial_balance = Balances::free_balance(BOB);

        assert_ok!(ArtistsPallet::withdraw_candidacy(RuntimeOrigin::signed(
            BOB
        )));

        // Ensure that the deposit is also effected in the genesis build
        let current_balance = Balances::free_balance(BOB);
        let current_reserve = Balances::reserved_balance(BOB);

        assert_eq!(current_reserve, 0);
        assert_eq!(current_balance, initial_balance + deposit);

        // Bob should have been removed from the candidate list
        assert_noop!(
            ArtistsPallet::test_caller_is_candidate(RuntimeOrigin::signed(BOB)),
            Error::<Test>::NotACandidate
        );

        assert_last_event(CandidateWithdrew(BOB));
    });
}

#[test]
fn test_approve_candidacy_to_artist() {
    new_test_ext(true).execute_with(|| {
        // An candidate cannot approve itself
        assert_noop!(
            ArtistsPallet::approve_candidacy(RuntimeOrigin::signed(BOB), BOB),
            BadOrigin
        );

        // Could not approve an artist without a valid candidacy
        assert_noop!(
            ArtistsPallet::approve_candidacy(RuntimeOrigin::root(), JOHN),
            Error::<Test>::CandidateNotFound
        );

        // Root could approve an artist
        assert_ok!(ArtistsPallet::approve_candidacy(RuntimeOrigin::root(), BOB));

        assert_last_event(CandidateApproved(BOB));

        // Could not approve an artist twice
        assert_noop!(
            ArtistsPallet::approve_candidacy(RuntimeOrigin::root(), BOB),
            Error::<Test>::AlreadyAnArtist
        );

        // The artist is well added to the artist group
        assert_ok!(ArtistsPallet::test_caller_is_artist(RuntimeOrigin::signed(
            BOB
        )));

        // The candidacy was well removed from the candidacies pool
        assert_noop!(
            ArtistsPallet::test_caller_is_candidate(RuntimeOrigin::signed(BOB)),
            Error::<Test>::NotACandidate
        );
    });
}

#[test]
fn test_caller_is_artist() {
    new_test_ext(true).execute_with(|| {
        // Should execute the extrinsic as `ALICE` is in the artists group
        assert_ok!(ArtistsPallet::test_caller_is_artist(RuntimeOrigin::signed(
            ALICE
        )));

        // Should refuse BOB who isn't an artist
        assert_noop!(
            ArtistsPallet::test_caller_is_artist(RuntimeOrigin::signed(BOB)),
            Error::<Test>::NotAnArtist
        );
    })
}

#[test]
fn test_caller_is_candidate() {
    new_test_ext(true).execute_with(|| {
        // Should execute the extrinsic as `BOB` is in the candidate list
        assert_ok!(ArtistsPallet::test_caller_is_candidate(
            RuntimeOrigin::signed(BOB)
        ));

        // Should refuse the ALICE who isn't a candidate
        assert_noop!(
            ArtistsPallet::test_caller_is_candidate(RuntimeOrigin::signed(ALICE)),
            Error::<Test>::NotACandidate
        );
    })
}
