use linux::imports::*;

// ---------- file descriptor with destructor

pub struct FileDescriptor {
	value: libc::c_int,
}

impl FileDescriptor {

	pub fn open <AsPath: AsRef <Path>> (
		path: AsPath,
		flags: libc::c_int,
	) -> Result <FileDescriptor, String> {

		let path =
			path.as_ref ();

		// TODO should be able to do this cleanly on linux...

		let path_string =
			try! (

			path.to_str ().ok_or (

				format! (
					"Invalid characters in path: {}",
					path.to_string_lossy ())

			)

		).to_owned ();

		let path_c =
			try! (

			CString::new (
				path_string.into_bytes (),
			).map_err (
				|_|

				format! (
					"Invalid characters in path: {}",
					path.to_string_lossy ())

			)

		);

		let fd = unsafe {

			libc::open (
				path_c.as_ptr (),
				flags)

		};

		if fd >= 0 {

			Ok (
				FileDescriptor {
					value: fd,
				}
			)

		} else {

			Err (
				format! (
					"error opening {:?}",
					path)
			)

		}

	}

	pub fn get_value (
		& self,
	) -> libc::c_int {
		self.value
	}

}

impl Drop for FileDescriptor {

	fn drop (
		& mut self,
	) {

		unsafe {

			libc::close (
				self.value);

		}

	}

}

impl <'a> From <& 'a FileDescriptor> for libc::c_int {

	fn from (
		file_descriptor: & 'a FileDescriptor,
	) -> libc::c_int {

		file_descriptor.value

	}

}

// ex: noet ts=4 filetype=rust
