#![allow(unused_imports)]

use log;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, traits::Currency, StorageValue};
use system::ensure_signed;

/// The module's configuration trait.
pub trait Trait: timestamp::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// The Currency
    type Currency: Currency<Self::AccountId>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;


decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        SomethingStored(u32, AccountId),
        NewParkingLot(AccountId),
        Entering(AccountId),
        Leaving(AccountId),
    }
);


decl_storage! {
    trait Store for Module<T: Trait> as Parking {
        Something get(something): Option<u32>;
    }
    
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event() = default;

        pub fn do_something(origin, something: u32) -> Result {
            let who = ensure_signed(origin)?;

            // For example: the following line stores the passed in u32 in the storage
            Something::put(something);

            // here we are raising the Something event
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }

        pub fn new_parking_lot(origin, latitude: i32, longitude: i32, capacity: u32, min_price: BalanceOf<T>, max_price: BalanceOf<T>) -> Result {
            unimplemented!()
        }
        
        pub fn entering(origin, parking_lot_hash: T::Hash) -> Result {
            unimplemented!()
        }

        pub fn leaving(origin) -> Result {
            unimplemented!()
        }

    }
}


/// tests for this module
#[cfg(test)]
mod tests {
    use super::*;

    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use runtime_primitives::{
        testing::{Digest, DigestItem, Header},
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };
    use support::{assert_ok, impl_outer_origin};

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    impl system::Trait for Test {
        type Origin = Origin;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type Digest = Digest;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type Log = DigestItem;
    }
    impl Trait for Test {
        type Event = ();
    }
    type Parking = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    #[test]
    fn test_new_parking_lot() {
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Parking::do_something(Origin::signed(1), 42));
            assert_eq!(Parking::something(), Some(42));
        });
    }
    
    #[test]
    fn test_entering() {
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Parking::do_something(Origin::signed(1), 42));
            assert_eq!(Parking::something(), Some(42));
        });
    }

    #[test]
    fn test_leaving() {
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Parking::do_something(Origin::signed(1), 42));
            assert_eq!(Parking::something(), Some(42));
        });
    }

}
