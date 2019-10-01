use support::{decl_module, decl_storage, StorageValue, StorageMap};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;

pub trait Trait: system::Trait {
}

// #[derive(Encode, Decode, Default)]
// pub struct Kitty(pub [u8; 16]);

//using my kitty struct instead
#[derive(Debug, Encode, Decode, Default, Clone, PartialEq)]
pub struct Kitty<Hash,Balance>{
    id:Hash,
    dna:u128,
    price:Balance,
    gen:u64,
}

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map u32 => Kitty;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): u32;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();
			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
		}

		//breed a kitty from parent
		pub fn breed_kitty(origin,father_kitty_hash: T::Hash, mother_kitty_hash: T::Hash){
			
			// checking msg sender
			let sender = ensure_signed(origin)?;

			// parents hash must exist
			ensure!(<Kittyies<T>>::exists(father_kitty_hash));
			ensure!(<Kittyies<T>>::exists(mother_kitty_hash));

			// get parents struct
			let father_kitty = Self::kitty(father_kitty_hash);
			let mother_kitty_hash = Self::kitty(father_kitty_hash);

			// todo: mix parents DNA to generate child new DNA
			let child_dna = 0;
			
			// construct the child kitty
			let child_kitty = Kitty {
                id: random_hash,
                dna: child_dna,
                price: <T::Balance as As<u64>>::sa(0),
                gen: cmp::max(kitty_1.gen, kitty_2.gen) + 1,
            };

			// checking before inserting 
			ensure!(!<Kitties<T>>::exists(child_kitty.id), "Kitty already exists");

			// add the new child kitty to the data storage
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);

			ok(())
		}
	}
}
