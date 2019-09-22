use support::{decl_module, decl_storage, StorageValue, StorageMap};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;

pub trait Trait: system::Trait {}

#[derive(Encode, Decode, Default, Clone)]
pub struct Kitty(pub [u8; 16]);

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
			let new_count = count.checked_add(1).ok_or("Kitties count overflow")?;

			let payload = (<system::Module<T>>::random_seed(), sender,
			    <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
			KittiesCount::put(new_count);
		}

        /// Breed a kitty
        pub fn breed(origin, dna1: u32, dna2: u32) {
   			let sender = ensure_signed(origin)?;
   			let count = Self::kitties_count();
			let new_count = count.checked_add(1).ok_or("Kitties count overflow")?;

            let kitty_1 = Self::kitty(dna1);
            let kitty_2 = Self::kitty(dna2);

            let payload = (<system::Module<T>>::random_seed(), sender,
			    <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());

            let mut final_dna = kitty_1.0;
            for (i, (dna_2_element, r)) in kitty_2.0.using_encoded(blake2_128).iter().zip(payload.using_encoded(blake2_128).iter()).enumerate() {
                if r % 2 == 0 {
                    final_dna.as_mut()[i] = *dna_2_element;
                }
            }

            Kitties::insert(count, Kitty(final_dna));
			KittiesCount::put(new_count);
        }
	}
}
