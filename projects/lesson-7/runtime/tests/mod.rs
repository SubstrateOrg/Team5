
pub struct Kitty(pub [u8; 16]);

impl Encode for Kitty {
	fn encode_to<T: Output>(&self, dest: &mut T)
	//TODO 
	}
}

impl Decode for Kitty {
	fn decode<I: Input>(input: &mut I) -> core::result::Result<Self, codec::Error>
		//TODO 
		Err("Decode always fails".into())
	}
}

pub struct LinkedItem<u32> {
	pub prev: Option<u32>,
	pub next: Option<u32>,
}

impl Encode for LinkedItem {
	fn encode_to<T: Output>(&self, dest: &mut T)
		//TODO 
	}
}

impl Decode for LinkedItem {
	fn decode<I: Input>(input: &mut I) -> core::result::Result<Self, codec::Error>
		//TODO		
		Err("DecodeFails always fails".into())
	}
}