use super::*;
use crate::mock::Balances;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, bounded_vec, ensure};
use frame_system::ensure_signed;
use frame_system::pallet_prelude::OriginFor;
use sp_runtime::DispatchError::BadOrigin;

impl<T: Config> Pallet<T> {
    /// Simple extrinsic that success if the caller is a certified artist
    pub fn test_artist_caller(origin: OriginFor<T>) -> DispatchResult {
        let caller = ensure_signed(origin)?;
        ensure!(
            CertifiedMembers::<T>::get().contains(&caller),
            Error::<T>::NotAnArtist
        );

        Ok(())
    }
}

// Test accounts used
pub const ALICE: <Test as frame_system::Config>::AccountId = 0;
pub const BOB: <Test as frame_system::Config>::AccountId = 1;

/// Genesis tests
#[test]
fn genesis_config() {
    new_test_ext(false).execute_with(|| {
        assert!(
            Artists::get_artist(ALICE)
                == Some(ArtistInfos {
                    account: ALICE,
                    is_certified: false,
                    name: b"Genesis Artist".to_vec().try_into().unwrap(),
                    styles: bounded_vec![Styles::Rock, Styles::Electronic, Styles::Pop],
                    age: 0, // Genesis block is 0
                })
        );
        // Ensure that the deposit is also effected in the genesis build
        let alice_free = Balances::free_balance(ALICE);
        let alice_reserve = Balances::reserved_balance(ALICE);
        assert_eq!(alice_reserve, CreationDepositAmount::get());
        assert_eq!(
            alice_free,
            (10_000_000_000_000 as u64) - CreationDepositAmount::get()
        )

        // TODO assert!(ArtistCommittee::is_member(&ALICE))
    });
}

#[test]
fn create_artist() {
    new_test_ext(true).execute_with(|| {
        let old_deposit = Balances::reserved_balance(ALICE);
        // Verify that the call fail if the name is longer that the defined String limit.
        assert_noop!(
            Artists::create(
                Origin::signed(ALICE),
                b"Is more than 20 chars".to_vec(),
                bounded_vec![Styles::Electronic, Styles::Pop],
            ),
            Error::<Test>::StringTooLong
        );
        // should create the artist profile of `ALICE`
        assert_ok!(Artists::create(
            Origin::signed(ALICE),
            b"Test Artist".to_vec(),
            bounded_vec![Styles::Electronic, Styles::Pop],
        ));
        // We also verify the stored datas
        assert_eq!(
            Artists::get_artist(ALICE),
            Some(ArtistInfos {
                account: ALICE,
                is_certified: false,
                name: b"Test Artist".to_vec().try_into().unwrap(),
                styles: bounded_vec![Styles::Electronic, Styles::Pop],
                age: 1, // Emitted on the first mock block
            })
        );
        let new_deposit = Balances::reserved_balance(ALICE);
        // Verify that the caller have deposited the expected amount in reserve to create his artist profile
        assert_eq!(new_deposit, old_deposit + CreationDepositAmount::get());

        let old_deposit = Balances::reserved_balance(ALICE);
        // Shouldn't be able to create his artist profile again from the same account
        assert_noop!(
            Artists::create(
                Origin::signed(ALICE),
                b"Test Artist again".to_vec(),
                bounded_vec![Styles::Rock, Styles::Pop],
            ),
            Error::<Test>::AlreadyCreated
        );
        let new_deposit = Balances::reserved_balance(ALICE);
        // Verify that any funds wasn't taken to reserve because of the error
        assert_eq!(new_deposit, old_deposit);

        // Verify that someone can't create an artist profile if he don't have required funds
        let bob_balance = Balances::free_balance(BOB);
        assert!(bob_balance < CreationDepositAmount::get());
        assert_noop!(
            Artists::create(
                Origin::signed(BOB),
                b"Test Artist 2".to_vec(),
                bounded_vec![Styles::Blues, Styles::Pop],
            ),
            Error::<Test>::NotEnoughFunds
        );
    });
}

#[ignore]
#[test]
fn caller_is_artist() {
    new_test_ext(false).execute_with(|| {
        // Should execute the extrinsic as `ALICE` is in the artists group
        assert_ok!(Artists::test_artist_caller(Origin::signed(ALICE)));
        // Should refuse the root origin then
        assert_noop!(Artists::test_artist_caller(Origin::root()), BadOrigin);
        // Should refuse the root origin then
        assert_noop!(
            Artists::test_artist_caller(Origin::signed(BOB)),
            Error::<Test>::NotAnArtist
        );
    })
}
