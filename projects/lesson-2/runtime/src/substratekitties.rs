use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result};

//TODO-002
pub trait Trait: balances::Trait {}

//TODO-001
use system::ensure_signed;

//TODO-003
use codec::{Encode, Decode};

//TODO-004
use sr_primitives::traits::{Hash};

//#[derive(Encode, Decode, Default, Clone, PartialEq)]
//#[cfg_attr(feature = "std", derive(Debug))]
//pub struct MyStruct<A, B> {
//    some_number: u32,
//    some_generic: A,
//    some_other_generic: B,
//}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    dna: Hash,
    price: Balance,
}

decl_storage! {
    trait Store for Module<T: Trait> as SubstratekittiesModule {
        // Declare storage and getter functions here
//        Value: u64;
//        SomeValue get(some_value_getter): map u32 => u32;
//
//        MyValue: map T::AccountId => u32;
//
//        MyItem: map T::AccountId => MyStruct<T::Balance, T::Hash>;

          Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;

          Nonce: u64;

    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here

//        fn set_value(origin, value: u32) -> Result {
//
//            let sender = ensure_signed(origin)?;
//
//            <SomeValue>::insert(value, value);
//
//            let my_value = <SomeValue>::get(value);
//            let also_my_value = Self::some_value_getter(value);
//
////            <Value>::put(value);
//            <MyValue<T>>::insert(sender, value);
//
//            Ok(())
//        }

//         fn create_struct(origin, value: u32, balance: T::Balance, hash: T::Hash) -> Result {
//            let sender = ensure_signed(origin)?;
//
//            let new_struct = MyStruct {
//                some_number: value,
//                some_generic: balance,
//                some_other_generic: hash,
//            };
//
//            <MyItem<T>>::insert(sender, new_struct);
//            Ok(())
//        }
       fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;

            let nonce = <Nonce>::get();
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);


            let new_kitty = Kitty {
                id: random_hash,
                dna: random_hash,
                price: 0.into(),
            };

            <Kitties<T>>::insert(random_hash, new_kitty);

            Ok(())
        }
    }
}
