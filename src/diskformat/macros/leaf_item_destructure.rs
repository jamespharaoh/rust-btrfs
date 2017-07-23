macro_rules! leaf_item_destructure {

	(
		$ leaf_item : expr ,
		$ enum_type : ident ,
	) => {

		match $ leaf_item {

			& BtrfsLeafItem::$ enum_type (ref item) =>
				Some (item),

			_ =>
				None,

		}

	}

}

// ex: noet ts=4 filetype=rust
