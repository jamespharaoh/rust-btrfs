use std::io;
use std::io::Write;

use libc;

use std::mem;
use std::ptr;

pub const LZO_E_OK: libc::c_int = 0;
pub const LZO_E_ERROR: libc::c_int = -1;

#[ link (name = "lzo2") ]
extern {

	fn __lzo_init_v2 (
		version: libc::c_uint,
		short_size: libc::c_int,
		int_size: libc::c_int,
		long_size: libc::c_int,
		uint32_size: libc::c_int,
		uint_size: libc::c_int,
		dict_size: libc::c_int,
		charp_size: libc::c_int,
		voidp_size: libc::c_int,
		callback_size: libc::c_int,
	) -> libc::c_int;

	fn lzo2a_decompress (
		src: * const u8,
		src_len: libc::c_uint,
		dst: * const u8,
		dst_len: * mut libc::c_uint,
		wrkmem: * const u8,
	) -> libc::c_int;

	fn lzo2a_decompress_safe (
		src: * const u8,
		src_len: libc::c_uint,
		dst: * const u8,
		dst_len: * mut libc::c_uint,
		wrkmem: * const u8,
	) -> libc::c_int;

}

// initialisation

static mut INITIALISED: bool = false;

pub fn initialise (
) -> Result <(), String> {

	if unsafe { INITIALISED } {
		return Ok (());
	}

	let result = unsafe {

		__lzo_init_v2 (
			0x2080,
			mem::size_of::<libc::c_short> () as libc::c_int,
			mem::size_of::<libc::c_int> () as libc::c_int,
			mem::size_of::<libc::c_long> () as libc::c_int,
			mem::size_of::<u32> () as libc::c_int,
			mem::size_of::<u64> () as libc::c_int,
			mem::size_of::<* const u8> () as libc::c_int,
			mem::size_of::<* const u8> () as libc::c_int,
			mem::size_of::<* const u8> () as libc::c_int,
			(
				mem::size_of::<* const u8> () * 4
				+ mem::size_of::<u64> () * 2
			) as libc::c_int)

	};

	match result {

		LZO_E_OK => {

			unsafe {
				INITIALISED = true;
			}

			Ok (())

		},

		LZO_E_ERROR =>
			Err (
				"LZO initialisation failed".to_owned ()),

		_ =>
			Err (
				format! (
					"LZO initialisation error {}",
					result)),

	}

}

// decompression

pub fn decompress (
	input: & [u8],
	output_size: usize,
) -> Result <Vec <u8>, String> {

	io::stderr ().write_all (
		b"LZO\r\n");

	initialise ().unwrap_or_else (
		|error|

		panic! (
			error)

	);

	io::stderr ().write_all (
		b"LZO2\r\n");

	let mut output =
		Vec::new ();

	output.resize (
		output_size,
		0u8);

	let mut output_len =
		output.len () as u32;

	let result = unsafe {

		lzo2a_decompress_safe (
			input.as_ptr (),
			input.len () as u32,
			output.as_ptr (),
			& mut output_len as * mut u32,
			ptr::null ())

	};

	output.resize (
		output_len as usize,
		0u8);

	io::stderr ().write_all (
		b"LZO3\r\n");

	match result {

		LZO_E_OK =>
			Ok (output),

		_ =>
			Err (
				format! (
					"LZO error code {}",
					result)),

	}

}

// ex: noet ts=4 filetype=rust
