use linux::imports::*;

#[ derive (Debug, Eq, PartialEq) ]
pub enum GroupType {
	Data,
	System,
	MetaData,
	DataAndMetaData,
	GlobalReserve,
	Unknown,
}

impl From <u64> for GroupType {

	fn from (
		flags: u64,
	) -> GroupType {

		match flags & BLOCK_GROUP_TYPE_AND_RESERVED_MASK {

			BLOCK_GROUP_DATA =>
				GroupType::Data,

			BLOCK_GROUP_SYSTEM =>
				GroupType::System,

			BLOCK_GROUP_METADATA =>
				GroupType::MetaData,

			BLOCK_GROUP_DATA_AND_METADATA =>
				GroupType::DataAndMetaData,

			BLOCK_GROUP_RESERVED =>
				GroupType::GlobalReserve,

			_ =>
				GroupType::Unknown,

		}

	}

}

// ex: noet ts=4 filetype=rust
