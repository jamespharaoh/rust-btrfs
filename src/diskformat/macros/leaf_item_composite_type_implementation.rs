macro_rules! leaf_item_composite_type_implementation {

	(
		$ container : ident,
		$ data : ident ,
		$ entries : ident ,
		$ iterator : ident ,
	) => {

		impl <'a> $ container <'a> {

			pub fn from_bytes (
				header: & 'a BtrfsLeafItemHeader,
				data_bytes: & 'a [u8],
			) -> Result <$ container <'a>, String> {

				// sanity check

				if data_bytes.len () < mem::size_of::<$ data> () {

					return Err (
						format! (
							"Must be at least 0x{:x} bytes",
							mem::size_of::<$ data> ()));

				}

				// return

				Ok ($ container {
					header,
					data_bytes,
				})

			}

			pub fn entries (& self) -> $ iterator <'a> {

				$ iterator ::new (
					self.header,
					self.data_bytes)

			}

		}

		impl <'a> BtrfsLeafItemContents <'a> for $ container <'a> {

			fn header (& self) -> & BtrfsLeafItemHeader {
				self.header
			}

		}

		impl <'a> Debug for $ container <'a> {

			fn fmt (
				& self,
				formatter: & mut Formatter,
			) -> Result <(), FmtError> {

				let mut debug_struct =
					formatter.debug_struct (
						stringify! ($ container));

				debug_struct.field (
					"key",
					& NakedString::from (
						self.key ().to_string_no_type_decimal ()));

				debug_struct.field (
					"entries",
					& $ entries ::new (
						self));

				debug_struct.finish ()

			}

		}

	}

}

// ex: noet ts=4 filetype=rust
