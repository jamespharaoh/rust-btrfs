macro_rules! leaf_item_composite_type_iterator {

	(
		$ container : ident ,
		$ data : ident ,
		$ entry : ident ,
		$ iterator : ident ,
		$ ( $ size_field : ident , ) *
	) => {

		pub struct $ iterator <'a> {
			header: & 'a BtrfsLeafItemHeader,
			data: & 'a [u8],
		}

		impl <'a> $ iterator <'a> {

			pub fn new (
				header: & 'a BtrfsLeafItemHeader,
				data: & 'a [u8],
			) -> $ iterator <'a> {

				$ iterator {
					header: header,
					data: data,
				}

			}

		}

		impl <'a> Iterator for $ iterator <'a> {

			type Item = $ entry <'a>;

			fn next (
				& mut self,
			) -> Option <$ entry <'a>> {

				use std::mem;

				if ! self.data.is_empty () {

					let data: & 'a $ data =
						unsafe {

						mem::transmute (
							& self.data [0])

					};

					let entry_size = vec! [
						mem::size_of::<$ data> (),
						$ (data . $ size_field as usize , ) *
					].iter ().sum ();

					let entry =
						$ entry ::from_bytes (
							self.header,
							& self.data [0 .. entry_size],
						).unwrap ();

					self.data =
						& self.data [entry_size .. ];

					Some (entry)

				} else {

					None

				}

			}

		}

	}

}

// ex: noet ts=4 filetype=rust
