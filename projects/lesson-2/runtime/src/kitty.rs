/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result, ensure, decl_event};
use system::ensure_signed;
use sr_primitives::traits::Hash;
use codec::{Encode, Decode};
// use std::convert::TryInto;
use byteorder::{ByteOrder, LittleEndian};



#[derive(Debug, Encode, Decode, Default, Clone, PartialEq)]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    // dna: Hash,
    dna: u128,
    price: Balance,
    gen: u64,
}

/// The module's configuration trait.
pub trait Trait: balances::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
        KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;
		// OwnedKitty get(kitty_of_owner): map T::AccountId => T::Hash;
		// index => kitty hash
		AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
		// count of kitties
        AllKittiesCount get(all_kitties_count): u64;
		// kitty hash => index
        AllKittiesIndex: map T::Hash => u64;
		// (owner account, index) => kitty hash
        OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
		// owner account => kitty count
        OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
		// kitty hash => index under owner
        OwnedKittiesIndex: map T::Hash => u64;
        Nonce: u128;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

		pub fn create_kitty(origin) -> Result {
			let sender = ensure_signed(origin)?;

            let owned_kitty_count = Self::owned_kitty_count(&sender);

            let new_owned_kitty_count = owned_kitty_count.checked_add(1)
                .ok_or("Overflow adding a new kitty to account balance")?;

            let all_kitties_count = Self::all_kitties_count();

            let new_all_kitties_count = all_kitties_count.checked_add(1)
                .ok_or("Overflow adding a new kitty to total supply")?;

            let nonce = Nonce::get();
            // let dna_hash:u128 = (<system::Module<T>>::random_seed(), &sender, nonce).using_encoded(|encoded| LittleEndian::read_u128(&encoded[0..16]));
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);
			let _dna_hash_array = random_hash.as_ref();
			let dna_hash = LittleEndian::read_u128(&_dna_hash_array[0..16]);

            ensure!(!<KittyOwner<T>>::exists(random_hash), "Kitty already exists");

            let new_kitty = Kitty {
                id: random_hash,
                dna: dna_hash,
                price: 0.into(),
                gen: 0,
            };

            <Kitties<T>>::insert(random_hash, new_kitty);
            <KittyOwner<T>>::insert(random_hash, &sender);

            <AllKittiesArray<T>>::insert(all_kitties_count, random_hash);
            AllKittiesCount::put(new_all_kitties_count);
            <AllKittiesIndex<T>>::insert(random_hash, all_kitties_count);

            <OwnedKittiesArray<T>>::insert((sender.clone(), owned_kitty_count), random_hash);
            <OwnedKittiesCount<T>>::insert(&sender, new_owned_kitty_count);
            <OwnedKittiesIndex<T>>::insert(random_hash, owned_kitty_count);

            Nonce::mutate(|n| *n += 1);


            Self::deposit_event(RawEvent::Created(sender, random_hash));

            Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where 
	 <T as system::Trait>::AccountId,
	 <T as system::Trait>::Hash
	{
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		Created(AccountId, Hash),
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
			
		});
	}
}
