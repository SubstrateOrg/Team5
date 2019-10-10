use support::{StorageMap, Parameter};
use sr_primitives::traits::Member;
use codec::{Encode, Decode};

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
// double direction linked list
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

		// update head to reservely traversing the linked list
		let current_head = Self::read_head(key);
		let new_head = LinkedItem{
			prev: Some(value),
			next: current_head.next,
		};

		// write new head
		Self::write(key, new_head);

		// get the rear item
		let rear_item = Self::read(key,head.prev);

		let new_rear_item = LinkedItem{
			prev: rear_item.prev,
			next: Some(value),
		};
		
		// set a new rear
		Self::write(key,head.prev,new_rear_item);

		let new_item = LinkedItem {
			prev: head.prev,
			next: None,
		}

		Self::write(key, Some(value), new_item);
    }

    pub fn remove(key: &Key, value: Value) {
        // 作业：实现 remove
		if let Some(item) = Storage::take(&(key.clone(),Some(value))))){
			
			let prev = Self::read(key, item.prev);

			let new_prev = LinkedItem{
				prev: prev.prev,
				next: item.next,
			}

			Self::write(key, item.prev, new_prev);

			let next = Self::read(key, item.next);
			let new_next = LinkedItem {
				prev: item.prev,
				next: next.next,
			}

			Self::write(key, item.next, new_next);
		}
    }
}