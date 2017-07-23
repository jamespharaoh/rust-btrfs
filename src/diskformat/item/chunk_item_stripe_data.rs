use super::super::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug) ]
pub struct BtrfsChunkItemStripeData {
	pub device_id: u64,
	pub offset: u64,
	pub device_uuid: BtrfsUuid,
}

impl BtrfsChunkItemStripeData {

	pub fn device_id (& self) -> u64 {
		self.device_id
	}

	pub fn offset (& self) -> u64 {
		self.offset
	}

	pub fn device_uuid (& self) -> BtrfsUuid {
		self.device_uuid
	}

}

// ex: noet ts=4 filetype=rust
