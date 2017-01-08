use linux::imports::*;

#[ repr (C) ]
#[ derive (Copy) ]
pub struct IoctlDevicePath {
	pub path: [u8; DEVICE_PATH_NAME_MAX],
}

impl IoctlDevicePath {

	pub fn as_vec (
		& self,
	) -> Vec <u8> {

		self.path.iter ().cloned ().take_while (
			|item|
			* item != 0
		).collect ()

	}

	pub fn as_os_string (
		& self,
	) -> OsString {

		OsString::from_vec (
			self.as_vec ())

	}

}

impl Clone for IoctlDevicePath {

	fn clone (
		& self,
	) -> Self {

		IoctlDevicePath {
			path: self.path,
		}

	}

	fn clone_from (
		& mut self,
		source: & Self,
	) {

		self.path.clone_from_slice (
			& source.path)

	}

}

impl fmt::Debug for IoctlDevicePath {

	fn fmt (
		& self,
		formatter: & mut fmt::Formatter,
	) -> fmt::Result {

		let string_bytes =
			self.as_vec ();

		let rust_string =
			String::from_utf8_lossy (
				& string_bytes);

		rust_string.fmt (
			formatter)

	}

}

// ex: noet ts=4 filetype=rust
