macro_rules! tree_item_range_accessor {

	(
		$ accessor_name : ident ,
		$ item_type : ident ,
		$ item_type_id : expr ,
		$ leaf_item_type : ident ,
	) => {

		pub fn $ accessor_name (
			& 'a self,
			object_id: u64,
		) -> Vec <$ item_type <'a>> {

			assert_ne! (
				$ item_type_id,
				0xff);

			self.tree_items ().range (
				BtrfsKey::new (
					object_id,
					$ item_type_id,
					0)
			..
				BtrfsKey::new (
					object_id,
					$ item_type_id + 1,
					0)
			).map (
				|(_key, item)|

				leaf_item_destructure! (
					item,
					$ leaf_item_type,
				).unwrap ().clone ()

			).collect ()

		}

	};

}

// ex: noet ts=4 filetype=rust
