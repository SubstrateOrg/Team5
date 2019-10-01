/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageValue,ensure,StorageMap,dispatch::Result};
use system::ensure_signed;
use sr_primitives::traits::Hash;
use codec::{Encode, Decode};
//use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug, Encode, Decode, Default, Clone, PartialEq)]
pub struct Kitty<Hash,Balance>{
    id:Hash,
    dna:u128,
    price:Balance,
    gen:u64,
}

/// The module's configuration trait.
pub trait Trait: balances::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as KittyModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(something): Option<u32>;

        //using map to store kitties
        OwnedKitties get(kitty_owner): map T::AccountId => Kitty<T::Hash, T::Balance>;
        KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;

        Nonce:u64;

        //using map to simulating the array storage
        AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
        // counter of all kitties
        AllKittiesCounter get(all_kitties_count): u64;
        AllKittiesIndex: map T::Hash => u64;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		pub fn do_something(origin, something: u32) -> Result {
			// TODO: You only need this if you want to check it was signed.
			let who = ensure_signed(origin)?;

			// TODO: Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

        pub fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;

            // get lastest kitties count
            let all_kitties_count = Self::all_kitties_count();

            // using safe math alike method to add count safely
            let new_all_kitties_count = all_kitties_count.checked_add(1)
            .ok_or("Overflow adding a new kitty to total supply")?;

            // get current nonce as seed
            let nonce = <Nonce<T>>::get();
            
            // generate random hash
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);

            //checking
            ensure!(!<KittyOwner<T>>::exists(random_hash), "Kitty already exists");

            let new_kitty = Kitty {
                id: <T as system::Trait>::Hashing::hash_of(&0),
                dna: 0,
                price: 0.into(),
                gen: 0,
            };


            <AllKittiesArray<T>>::insert(all_kitties_count,random_hash);
            // generating the new kitty
            <Kitty<T>>::insert(random_hash,new_kitty);
            // binding sender to the kitty hash
            <KittyOwner<T>>::insert(random_hash,&sender);

            // kitties count + 1
            <AllKittiesArray<T>>::insert(all_kitties_count,random_hash);
            <AllKittiesCounter<T>>::put(new_all_kitties_count);
            <AllKittiesIndex<T>>::insert(random_hash,all_kitties_count);

            <OwnedKitties<T>>::insert(&sender, new_kitty);

            <Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u32, AccountId),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, parameter_types};
	use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
	use sr_primitives::weights::Weight;
	use sr_primitives::Perbill;

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
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}
