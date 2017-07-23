macro_rules! leaf_item_composite_type_entries {

	(
		$ container : ident ,
		$ entries : ident ,
	) => {

		pub struct $ entries <'a> {
			container: & 'a $ container <'a>,
		}

		impl <'a> $ entries <'a> {

			pub fn new (
				container: & 'a $ container <'a>,
			) -> $ entries <'a> {

				$ entries {
					container: container,
				}

			}

		}

		impl <'a> Debug for $ entries <'a> {

			fn fmt (
				& self,
				formatter: & mut Formatter,
			) -> Result <(), FmtError> {

				let mut debug_list =
					formatter.debug_list ();

				debug_list.entries (
					self.container.entries ());

				debug_list.finish ()

			}

		}

	}

}

// ex: noet ts=4 filetype=rust
