use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;
use sr_primitives::traits::{Hash};
use codec::{Encode, Decode};

#[derive(Encode, Decode, Default, Clone)]
pub struct Kitty<Hash, U64> {
    id: Hash,
    dna: Hash,
    price: U64,
    gen: U64,
}

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // 加密猫ID获取加密猫
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, u64>;
        // 加密猫ID获取主人
        KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

        // 以下三项用于遍历所有加密猫
        AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
        AllKittiesCount get(all_kitties_count): u64;
        AllKittiesIndex: map T::Hash => u64;

        // 以下三项用于遍历用户拥有的所有加密猫
        OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
        OwnedKittiesIndex: map T::Hash => u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_kitty(origin, salt: u64) -> Result {
            let _sender = ensure_signed(origin)?;
            let random_hash = (<system::Module<T>>::random_seed(), &_sender, salt)
                .using_encoded(<T as system::Trait>::Hashing::hash);

            // 验证加密猫主人是否已存在
            ensure!(!<KittyOwner<T>>::exists(random_hash), "Kitty already exists");

            // 加密猫总数+1，并验证是否会溢出
            let _all_kitties_count = Self::all_kitties_count();
            let _new_all_kitties_count = _all_kitties_count.checked_add(1)
                .ok_or("Overflow adding a new kitty to total supply")?;

            // 主人拥有加密猫总数+1，并验证是否会溢出
            let _owned_kitty_count = Self::owned_kitty_count(&_sender);
            let _new_owned_kitty_count = _owned_kitty_count.checked_add(1)
                .ok_or("Overflow adding a new kitty to account")?;

            let new_kitty = Kitty {
                id: random_hash,
                dna: random_hash,
                price: 0,
                gen: 0,
            };

            // 添加新的加密猫
            <Kitties<T>>::insert(random_hash, new_kitty);
            // 设置加密猫主人
            <KittyOwner<T>>::insert(random_hash, &_sender);

            // 加密猫列表添加新的加密猫
            <AllKittiesArray<T>>::insert(_all_kitties_count, random_hash);
            <AllKittiesCount>::put(_new_all_kitties_count);
            <AllKittiesIndex<T>>::insert(random_hash, _all_kitties_count);

            // 主人的加密猫列表添加新的加密猫
            <OwnedKittiesArray<T>>::insert((_sender.clone(), _owned_kitty_count), random_hash);
            <OwnedKittiesCount<T>>::insert(&_sender, _new_owned_kitty_count);
            <OwnedKittiesIndex<T>>::insert(random_hash, _owned_kitty_count);

            Ok(())
        }
    }
}