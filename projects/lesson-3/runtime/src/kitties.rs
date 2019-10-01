use support::{decl_module, decl_storage, StorageValue, StorageMap};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;
use rstd::result;

pub trait Trait: system::Trait {
}

#[derive(Encode, Decode, Default)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map u32 => Option<Kitty>;
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
		pub fn breed(origin, kitty_id_1: u32, kitty_id_2: u32) {
			let sender = ensure_signed(origin)?;

			Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;

		}
	}

	
}

impl<T: Trait> Module<T> {
	fn do_breed(sender: &T::AccountId, kitty_id_1: u32, kitty_id_2: u32) -> result::Result<u32, &'static str> {
		let kitty1 = Self::kitty(kitty_id_1);
		let kitty2 = Self::kitty(kitty_id_2);


		
		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.unwrap().0;
		let kitty2_dna = kitty2.unwrap().0;

		// Generate a random 128bit value
		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		// Combine parents and selector to create new kitty
		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty( kitty_id, Kitty(new_dna));

		Ok(kitty_id)
	}

	fn insert_kitty(kitty_id: u32, kitty: Kitty) {
		// Create and store kitty
		<Kitties>::insert(kitty_id, kitty);
		<KittiesCount>::put(kitty_id + 1);

	}

	fn next_kitty_id() -> result::Result<u32, &'static str> {
		let kitty_id = Self::kitties_count();
		if kitty_id == u32::max_value() {
			return Err("Kitties count overflow");
		}
		Ok(kitty_id)
	}

	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
		payload.using_encoded(blake2_128)
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	((selector & dna1) | (!selector & dna2))
}