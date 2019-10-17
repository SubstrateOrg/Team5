use support::{StorageMap, Parameter};
use sr_primitives::traits::Member;
use codec::{Encode, Decode};

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
pub struct LinkedItem<Value> {
	pub prev: Option<Value>,
	pub next: Option<Value>,
}

pub struct LinkedList<Storage, Key, Value>(rstd::marker::PhantomData<(Storage, Key, Value)>);

impl<Storage, Key, Value> LinkedList<Storage, Key, Value> where
    Value: Parameter + Member + Copy,
    Key: Parameter,
    Storage: StorageMap<(Key, Option<Value>), LinkedItem<Value>, Query = Option<LinkedItem<Value>>>,
{
    fn read_head(key: &Key) -> LinkedItem<Value> {
 		Self::read(key, None)
 	}

  	fn write_head(account: &Key, item: LinkedItem<Value>) {
 		Self::write(account, None, item);
 	}

  	fn read(key: &Key, value: Option<Value>) -> LinkedItem<Value> {
 		Storage::get(&(key.clone(), value)).unwrap_or_else(|| LinkedItem {
 			prev: None,
 			next: None,
 		})
 	}

  	fn write(key: &Key, value: Option<Value>, item: LinkedItem<Value>) {
 		Storage::insert(&(key.clone(), value), item);
 	}

    pub fn append(key: &Key, value: Value) {
        // 作业：实现 append
		let head = Self::read_head(key);
		let new_head = LinkedItem {
			prev: Some(value),
			next: head.next,
		};

		Self::write_head(key, new_head);

		let prev = Self::read(key, head.prev);
		let new_prev = LinkedItem {
			prev: prev.prev,
			next: Some(value),
		};
		Self::write(key, head.prev, new_prev);

		let item = LinkedItem {
			prev: head.prev,
			next: None,
		};
		Self::write(key, Some(value), item);
    }

    pub fn remove(key: &Key, value: Value) {
        // 作业：实现 remove
		if let Some(item) = Storage::take(&(key.clone(), Some(value))) {
			let prev = Self::read(key, item.prev);
			let new_prev = LinkedItem {
				prev: prev.prev,
				next: item.next,
			};

			Self::write(key, item.prev, new_prev);

			let next = Self::read(key, item.next);
			let new_next = LinkedItem {
				prev: item.prev,
				next: next.next,
			};

			Self::write(key, item.next, new_next);
		}
    }
}

//
///// tests for this module
//#[cfg(test)]
//mod tests {
//	use super::*;
//
//	use runtime_io::with_externalities;
//	use primitives::{H256, Blake2Hasher};
//	use support::{impl_outer_origin, parameter_types};
//	use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
//	use sr_primitives::weights::Weight;
//	use sr_primitives::Perbill;
//
//	impl_outer_origin! {
//		pub enum Origin for Test {}
//	}
//
//	// For testing the module, we construct most of a mock runtime. This means
//	// first constructing a configuration type (`Test`) which `impl`s each of the
//	// configuration traits of modules we want to use.
//	#[derive(Clone, Eq, PartialEq, Debug)]
//	pub struct Test;
//	parameter_types! {
//		pub const BlockHashCount: u64 = 250;
//		pub const MaximumBlockWeight: Weight = 1024;
//		pub const MaximumBlockLength: u32 = 2 * 1024;
//		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
//	}
//	impl system::Trait for Test {
//		type Origin = Origin;
//		type Call = ();
//		type Index = u64;
//		type BlockNumber = u64;
//		type Hash = H256;
//		type Hashing = BlakeTwo256;
//		type AccountId = u64;
//		type Lookup = IdentityLookup<Self::AccountId>;
//		type Header = Header;
//		type WeightMultiplierUpdate = ();
//		type Event = ();
//		type BlockHashCount = BlockHashCount;
//		type MaximumBlockWeight = MaximumBlockWeight;
//		type MaximumBlockLength = MaximumBlockLength;
//		type AvailableBlockRatio = AvailableBlockRatio;
//		type Version = ();
//	}
//
//	// This function basically just builds a genesis storage key/value store according to
//	// our desired mockup.
//	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
//		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
//	}
//
//	#[test]
//	fn owned_kitties_can_append_values() {
//		with_externalities(&mut new_test_ext(), || {
//			LinkedList::append(&0, 1);
//
////			assert_eq!(LinkedList::read(&0, None), LinkedItem {
////				prev: Some(1),
////				next: Some(1),
////			});
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
//// 				prev: None,
//// 				next: None,
//// 			}));
////
////			OwnedKittiesTest::append(&0, 2);
////
////			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
//// 				prev: Some(2),
//// 				next: Some(1),
//// 			}));
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
//// 				prev: None,
//// 				next: Some(2),
//// 			}));
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), Some(KittyLinkedItem {
//// 				prev: Some(1),
//// 				next: None,
//// 			}));
////
////			OwnedKittiesTest::append(&0, 3);
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
//// 				prev: Some(3),
//// 				next: Some(1),
//// 			}));
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
//// 				prev: None,
//// 				next: Some(2),
//// 			}));
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), Some(KittyLinkedItem {
//// 				prev: Some(1),
//// 				next: Some(3),
//// 			}));
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), Some(KittyLinkedItem {
//// 				prev: Some(2),
//// 				next: None,
//// 			}));
//		});
//	}
//
////	#[test]
//// 	fn owned_kitties_can_remove_values() {
//// 		with_externalities(&mut new_test_ext(), || {
////			OwnedKittiesTest::append(&0, 1);
//// 			OwnedKittiesTest::append(&0, 2);
//// 			OwnedKittiesTest::append(&0, 3);
////
////			OwnedKittiesTest::remove(&0, 2);
////
////			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
//// 				prev: Some(3),
//// 				next: Some(1),
//// 			}));
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
//// 				prev: None,
//// 				next: Some(3),
//// 			}));
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), Some(KittyLinkedItem {
//// 				prev: Some(1),
//// 				next: None,
//// 			}));
////
////			OwnedKittiesTest::remove(&0, 1);
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
//// 				prev: Some(3),
//// 				next: Some(3),
//// 			}));
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), None);
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), Some(KittyLinkedItem {
//// 				prev: None,
//// 				next: None,
//// 			}));
////
////			OwnedKittiesTest::remove(&0, 3);
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
//// 				prev: None,
//// 				next: None,
//// 			}));
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), None);
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);
////
////  			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);
////		});
////	}
//}
