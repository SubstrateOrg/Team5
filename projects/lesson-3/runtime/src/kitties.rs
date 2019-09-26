use support::{decl_module, decl_storage, StorageValue, StorageMap, ensure, dispatch::Result};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;
use sr_primitives::traits::Hash;
use rstd::cmp;

pub trait Trait: system::Trait + balances::Trait {
}

// #[derive(Encode, Decode, Default)]
// pub struct Kitty(pub [u8; 16]);

#[derive(Debug, Encode, Decode, Default, Clone, PartialEq)]
pub struct Kitty<Hash, Balance> {
    id : Hash,
	dna : [u8; 16],
	price : Balance,
	gen : u64,
}

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		//pub Kitties get(kitty): map u32 => Kitty;
		/// Stores the total number of kitties. i.e. the next kitty index
		//pub KittiesCount get(kitties_count): u32;

		Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
		KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

		AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
		AllKittiesCount get(all_kitties_count): u64;
		AllKittiesIndex: map T::Hash => u64;

		OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
		OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
		OwnedKittiesIndex: map T::Hash => u64;

		Nonce: u128;

		
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		// pub fn create(origin) {
		// 	let sender = ensure_signed(origin)?;
		// 	let count = Self::kitties_count();
		// 	if count == u32::max_value() {
		// 		return Err("Kitties count overflow");
		// 	}
		// 	let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
		// 	let dna = payload.using_encoded(blake2_128);
		// 	let kitty = Kitty(dna);
		// 	Kitties::insert(count, kitty);
		// 	KittiesCount::put(count + 1);
		// }

		// Initializing events
		// this is needed only if you are using events in your module
	    //fn deposit_event() = default;

		fn create_kitty(origin) -> Result {
		    let sender = ensure_signed(origin)?;

			let count = Self::all_kitties_count();
			if count == u64::max_value() {
				return Err("Kitties count overflow");
			}

		    // ACTION: Move this `owned_kitty_count` and `new_owned_kitty_count` logic into the `mint()` function
		    let owned_kitty_count = Self::owned_kitty_count(&sender);

		    let new_owned_kitty_count = owned_kitty_count.checked_add(1)
		        .ok_or("Overflow adding a new kitty to account balance")?;

		    // ACTION: Move this `all_kitties_count` and `new_all_kitties_count` logic into the `mint()` function
		    let all_kitties_count = Self::all_kitties_count();

		    let new_all_kitties_count = all_kitties_count.checked_add(1)
		        .ok_or("Overflow adding a new kitty to total supply")?;

		    // `nonce` and `random_hash` generation can stay here
		    let nonce = Nonce::get();
		    let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
		        .using_encoded(<T as system::Trait>::Hashing::hash);	    
			let payload = (<system::Module<T>>::random_seed(), &sender, 
				<system::Module<T>>::extrinsic_index(), 
				<system::Module<T>>::block_number());
			let dna_hash = payload.using_encoded(blake2_128);


		    // ACTION: Move this collision check to the `mint()` function
		    ensure!(!<KittyOwner<T>>::exists(random_hash), "Kitty already exists");

		    // Creating the `Kitty` object can stay here
		    let new_kitty = Kitty {
		        id: random_hash,
		        dna: dna_hash,
		        price: 0.into(),
		        gen: 0,
		    };


		    // ACTION: Move all of the kitty related storage updates to the `mint()` function
		    <Kitties<T>>::insert(random_hash, new_kitty);
		    <KittyOwner<T>>::insert(random_hash, &sender);

		    <AllKittiesArray<T>>::insert(all_kitties_count, random_hash);
		    <AllKittiesCount>::put(new_all_kitties_count);
		    <AllKittiesIndex<T>>::insert(random_hash, all_kitties_count);

		    <OwnedKittiesArray<T>>::insert((sender.clone(), owned_kitty_count), random_hash);
		    <OwnedKittiesCount<T>>::insert(&sender, new_owned_kitty_count);
		    <OwnedKittiesIndex<T>>::insert(random_hash, owned_kitty_count);

		    // Nonce update can stay here
		    Nonce::mutate(|n| *n += 1);

		    // ACTION: Move this event to the `mint()` function
		    //Self::deposit_event(RawEvent::Created(sender, random_hash));

		    Ok(())
		}
		
		fn breed_kitty(origin, kitty_id_1: T::Hash, kitty_id_2: T::Hash) -> Result{
            let sender = ensure_signed(origin)?;

            ensure!(<Kitties<T>>::exists(kitty_id_1), "This cat 1 does not exist");
            ensure!(<Kitties<T>>::exists(kitty_id_2), "This cat 2 does not exist");

            let nonce = Nonce::get();
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);

			let payload = (<system::Module<T>>::random_seed(), &sender, 
				<system::Module<T>>::extrinsic_index(), 
				<system::Module<T>>::block_number());
			let dna_hash = payload.using_encoded(blake2_128);

            let kitty_1 = Self::kitty(kitty_id_1);
            let kitty_2 = Self::kitty(kitty_id_2);

            let mut final_dna = kitty_1.dna;
            //for (i, (dna_2_element, r)) in kitty_2.dna.as_ref().iter().zip(random_hash.as_ref().iter()).enumerate() {
            //     if r % 2 == 0 {
            //         final_dna.as_mut()[i] = *dna_2_element;
            //     }
            // }			
			for (i, (dna_2_element, r)) in kitty_2.dna.as_ref().iter().zip(dna_hash.as_ref().iter()).enumerate()  {
				if r % 2 == 0 {
					final_dna.as_mut()[i] = *dna_2_element;
				}
			}

            let new_kitty = Kitty {
                id: random_hash,
                dna: final_dna,
                price: 0.into(),
                gen: cmp::max(kitty_1.gen, kitty_2.gen) + 1,
            };

            //Self::mint(sender, random_hash, new_kitty)?;
			// ACTION: Move this `owned_kitty_count` and `new_owned_kitty_count` logic into the `mint()` function
		    let owned_kitty_count = Self::owned_kitty_count(&sender);

		    let new_owned_kitty_count = owned_kitty_count.checked_add(1)
		        .ok_or("Overflow adding a new kitty to account balance")?;

		    // ACTION: Move this `all_kitties_count` and `new_all_kitties_count` logic into the `mint()` function
		    let all_kitties_count = Self::all_kitties_count();

		    let new_all_kitties_count = all_kitties_count.checked_add(1)
		        .ok_or("Overflow adding a new kitty to total supply")?;

			// ACTION: Move all of the kitty related storage updates to the `mint()` function
		    <Kitties<T>>::insert(random_hash, new_kitty);
		    <KittyOwner<T>>::insert(random_hash, &sender);

		    <AllKittiesArray<T>>::insert(all_kitties_count, random_hash);
		    <AllKittiesCount>::put(new_all_kitties_count);
		    <AllKittiesIndex<T>>::insert(random_hash, all_kitties_count);

		    <OwnedKittiesArray<T>>::insert((sender.clone(), owned_kitty_count), random_hash);
		    <OwnedKittiesCount<T>>::insert(&sender, new_owned_kitty_count);
		    <OwnedKittiesIndex<T>>::insert(random_hash, owned_kitty_count);

            // Nonce update can stay here
		    Nonce::mutate(|n| *n += 1);

            Ok(())
        }
	}
}