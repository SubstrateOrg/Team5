use support::{decl_module, decl_storage, StorageValue, StorageMap};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;

pub trait Trait: system::Trait {}

#[derive(Encode, Decode, Default,PartialEq)]
pub struct Kitty(pub [u8; 16]);
//pub struct Kitty2(pub [u128]); 没有字符串的定义

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

//		/// 繁殖⼩小猫
//		/// 选择两只现有的猫作为⽗母
//		/// ⼩小猫必须继承⽗父⺟母的基因
//		/// 同样的⽗父⺟母⽣生出来的⼩小猫不不能相同
		pub fn generate_child_kitty(origin, m_kitty_index: u32 ,f_kitty_index: u32 ) {
			let sender = ensure_signed(origin)?;

			//TODO create 这⾥里里⾯面的kitties_count有溢出的可能性，修复 这个问题
			let count = Self::kitties_count();
			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}

			let Kitty(_m_kitty): Kitty = <Kitties>::get(m_kitty_index);
			let Kitty(_f_kitty): Kitty = <Kitties>::get(f_kitty_index);

			let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number(),_m_kitty,_f_kitty);
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
		}


	}
}
