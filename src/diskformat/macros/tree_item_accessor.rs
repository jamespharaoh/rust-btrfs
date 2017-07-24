macro_rules! tree_item_accessor {

	(
		$ accessor_name : ident ,
		$ item_type : ident ,
		$ item_type_id : expr ,
		$ leaf_item_type : ident ,
	) => {

		pub fn $ accessor_name (
			& 'a self,
			object_id: u64,
			offset: u64,
		) -> Option <$ item_type <'a>> {

			self.tree_items ().get (
				& BtrfsKey::new (
					object_id,
					$ item_type_id,
					offset)
			).map (
				|item|

				leaf_item_destructure! (
					item,
					$ leaf_item_type,
				).unwrap ().clone ()

			)

		}

	};

	(
		$ accessor_name : ident ,
		$ item_type : ident ,
		$ item_type_id : expr ,
		$ leaf_item_type : ident ,
		$ offset : expr ,
	) => {

		pub fn $ accessor_name (
			& 'a self,
			object_id: u64,
		) -> Option <$ item_type <'a>> {

			self.tree_items ().get (
				& BtrfsKey::new (
					object_id,
					$ item_type_id,
					$ offset)
			).map (
				|item|

				leaf_item_destructure! (
					item,
					$ leaf_item_type,
				).unwrap ().clone ()

			)

		}

	};

}

// ex: noet ts=4 filetype=rust
