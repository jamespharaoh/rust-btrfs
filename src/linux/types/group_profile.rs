use linux::imports::*;

#[ derive (Debug, Eq, PartialEq) ]
pub enum GroupProfile {
	Single,
	Raid0,
	Raid1,
	Raid5,
	Raid6,
	Dup,
	Raid10,
	Unknown,
}

impl GroupProfile {

	pub fn from_string (
		string_value: & str,
	) -> Option <GroupProfile> {

		match string_value {

			"single" => Some (GroupProfile::Single),
			"raid0" => Some (GroupProfile::Raid0),
			"raid1" => Some (GroupProfile::Raid1),
			"raid5" => Some (GroupProfile::Raid5),
			"raid6" => Some (GroupProfile::Raid6),
			"dup" => Some (GroupProfile::Dup),
			"raid10" => Some (GroupProfile::Raid10),
			"unknown" => Some (GroupProfile::Unknown),

			_ => None,

		}

	}

}

impl From <u64> for GroupProfile {

	fn from (
		flags: u64,
	) -> GroupProfile {

		match flags & BLOCK_GROUP_PROFILE_MASK {

			0 =>
				GroupProfile::Single,

			BLOCK_GROUP_RAID0 =>
				GroupProfile::Raid0,

			BLOCK_GROUP_RAID1 =>
				GroupProfile::Raid1,

			BLOCK_GROUP_RAID5 =>
				GroupProfile::Raid5,

			BLOCK_GROUP_RAID6 =>
				GroupProfile::Raid6,

			BLOCK_GROUP_DUP =>
				GroupProfile::Dup,

			BLOCK_GROUP_RAID10 =>
				GroupProfile::Raid10,

			_ =>
				GroupProfile::Unknown,

		}

	}

}

// ex: noet ts=4 filetype=rust
