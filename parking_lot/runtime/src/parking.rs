#![allow(unused_imports)]
#![allow(unused_variables)]

use support::{decl_event, decl_module, decl_storage, dispatch::Result, traits::Currency, StorageValue};
use system::ensure_signed;
use codec::{Encode, Decode};
use rstd::{convert::TryInto, prelude::*, result};
use log;


/// The module's configuration trait.
pub trait Trait: timestamp::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// The Currency
    type Currency: Currency<Self::AccountId>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

/// ParkingLot store parking lot's info
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub struct ParkingLot<T: Trait> {
    pub owner: T::AccountId,
    pub remain: u32,
    pub capacity: u32,
    pub current_price: BalanceOf<T>,
    pub min_price: BalanceOf<T>,
    pub max_price: BalanceOf<T>,
    pub latitude: i32,
    pub longitude: i32,
}

impl<T: Trait> ParkingLot<T> {
    pub fn new(
        owner: T::AccountId,
        latitude: i32,
        longitude: i32,
        capacity: u32,
        min_price: BalanceOf<T>,
        max_price: BalanceOf<T>,
    ) -> Self {
        Self {
            owner: owner,
            capacity,
            current_price: min_price.clone(),
            min_price,
            max_price,
            remain: capacity,
            latitude,
            longitude,
        }
    }

}

/// ParkingInfo stores parking info of user
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub struct ParkingInfo<T: Trait> {
    pub user_id: T::AccountId,
    pub parking_lot_hash: T::Hash,
    pub info_hash: T::Hash,
    pub enter_time: T::Moment,
    pub current_time: T::Moment,
    pub current_fee: BalanceOf<T>,
}


impl<T: Trait> ParkingInfo<T> {
    pub fn new(user_id: T::AccountId, parking_lot_hash: T::Hash, info_hash: T::Hash, enter_time: T::Moment) -> Self {
        Self {
            user_id,
            parking_lot_hash,
            info_hash,
            enter_time,
            current_time: enter_time.clone(),
            current_fee: 0.into(),
        }
    }
}


decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Moment = <T as timestamp::Trait>::Moment,
        ParkingLotInfo = ParkingLot<T>,
        EnteringInfo = ParkingInfo<T>,
        LeavingInfo = ParkingInfo<T>,
    {
        /// Deposit a new parking lot
        NewParkingLot(Moment, ParkingLotInfo),
        /// Deposit a event that current user enter the parking lot
        Entering(Moment, EnteringInfo),
        /// Deposit a event that current user leave the parkint lot
        Leaving(Moment, AccountId, AccountId, LeavingInfo),
    }
);


decl_storage! {
    trait Store for Module<T: Trait> as Parking {
        /// Current parking lot count of a owner
        OwnerParkingLotsCount get(owner_parking_lots_count): map T::AccountId => u64;
        /// Access the all parking lot infos
        OwnerParkingLotsArray get(owner_parking_lots_array): map (T::AccountId, u64) => T::Hash;
        /// index to one parking lot hash
        ParkingLotsByIndex get(parking_lots_by_index): map u64 => T::Hash;
        /// Hash map to one parking lot
        ParkingLots get(parking_lots): map T::Hash => Option<ParkingLot<T>>;
        /// Last time for the parking lot fresh fees
        ParkingLotLastTime get(parking_lot_last_time): map T::Hash => Option<T::Moment>;
        /// All user id of current parking lot
        CurrentParkingAccounts get(current_parking_accounts): map T::Hash => Vec<T::AccountId>;
        /// Total number of parking lots
        AllParkingLotsCount get(all_parking_lots_count): u64;
        /// Parking info of current user
        UserParkingInfo get(user_parking_info): map T::AccountId => Option<ParkingInfo<T>>;
    }
    
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event() = default;

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


impl <T: Trait> Module<T> {

}

/// tests for this module
#[cfg(test)]
mod tests {
    use super::*;

    use primitives::{Blake2Hasher, H256};
    use runtime_io::{with_externalities, TestExternalities};
    use sr_primitives::weights::Weight;
    use sr_primitives::Perbill;
    use sr_primitives::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    };
    use support::{assert_err, assert_ok, impl_outer_origin, parameter_types};

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: Weight = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    }
    
    impl system::Trait for Test {
        type Origin = Origin;
        type Call = ();
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type WeightMultiplierUpdate = ();
        type Event = ();
        type BlockHashCount = BlockHashCount;
        type MaximumBlockWeight = MaximumBlockWeight;
        type MaximumBlockLength = MaximumBlockLength;
        type AvailableBlockRatio = AvailableBlockRatio;
        type Version = ();
    }

    parameter_types! {
        pub const ExistentialDeposit: u64 = 0;
        pub const TransferFee: u64 = 0;
        pub const CreationFee: u64 = 0;
        pub const TransactionBaseFee: u64 = 0;
        pub const TransactionByteFee: u64 = 0;
    }

    impl balances::Trait for Test {
        type Balance = u64;
        type OnFreeBalanceZero = ();
        type OnNewAccount = ();
        type Event = ();
        type TransactionPayment = ();
        type TransferPayment = ();
        type DustRemoval = ();
        type ExistentialDeposit = ExistentialDeposit;
        type TransferFee = TransferFee;
        type CreationFee = CreationFee;
        type TransactionBaseFee = TransactionBaseFee;
        type TransactionByteFee = TransactionByteFee;
        type WeightToFee = ();
    }

    parameter_types! {
        pub const MinimumPeriod: u64 = 1000;
    }

    impl timestamp::Trait for Test {
        type Moment = u64;
        type OnTimestampSet = ();
        type MinimumPeriod = MinimumPeriod;
    }

    impl Trait for Test {
        type Event = ();
        type Currency = balances::Module<Test>;
    }


    type Parking = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .into()
    }

    #[test]
    fn test_new_parking_lot() {
        with_externalities(&mut new_test_ext(), || {
        });
    }
    
    #[test]
    fn test_entering() {
        with_externalities(&mut new_test_ext(), || {
        });
    }

    #[test]
    fn test_leaving() {
        with_externalities(&mut new_test_ext(), || {
        });
    }

}
