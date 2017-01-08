use libc::c_int;
use libc::c_void;

use std::mem;
use std::ptr;

#[ repr (C) ]
struct ZlibStream {

	next_in: * const u8,
	avail_in: u32,
	total_in: u64,

	next_out: * mut u8,
	avail_out: u32,
	total_out: u64,

	msg: * const u8,
	internal_state: * const c_void,

	alloc_func: * const c_void,
	free_func: * const c_void,
	opaque: * const c_void,

	data_type: i32,
	adler: u64,
	reserved: u64,

}

#[ link (name = "z") ]
extern {

	fn deflateInit2_ (
		strm: * mut ZlibStream,
		level: c_int,
		method: c_int,
		windowBits: c_int,
		memLevel: c_int,
		strategy: c_int,
		version: * const u8,
		stream_size: c_int,
	) -> c_int;

	fn deflate (
		strm: * mut ZlibStream,
		flush: c_int,
	) -> c_int;

	fn inflateInit2_ (
		strm: * mut ZlibStream,
		windowBits: c_int,
		version: * const u8,
		stream_size: c_int,
	) -> c_int;

	fn inflate (
		strm: * mut ZlibStream,
		flush: c_int,
	) -> c_int;

}

// flush values

const Z_NO_FLUSH: c_int = 0;
//const Z_PARTIAL_FLUSH: c_int = 1;
//const Z_SYNC_FLUSH: c_int = 2;
//const Z_FULL_FLUSH: c_int = 3;
const Z_FINISH: c_int = 4;
//const Z_BLOCK: c_int = 5;
//const Z_TREES: c_int = 6;

// return codes

const Z_OK: c_int = 0;
const Z_STREAM_END: c_int = 1;
//const Z_NEED_DICT: c_int = 2;
//const Z_MEM_ERROR: c_int = -4;
//const Z_BUF_ERROR: c_int = -5;
//const Z_VERSION_ERROR: c_int = -6;

// compression level

const Z_DEFAULT_COMPRESSION: c_int = -1;

// compression method

const Z_DEFLATED: c_int = 8;

// compression strategy

//const Z_FILTERED: c_int = 1;
//const Z_HUFFMAN_ONLY: c_int = 2;
//const Z_RLE: c_int = 3;
//const Z_FIXED: c_int = 4;
const Z_DEFAULT_STRATEGY: c_int = 0;

// data type

//const Z_BINARY: c_int = 0;
//const Z_TEXT: c_int = 1;
//const Z_UNKNOWN: c_int = 2;

// compression

pub fn compress (
	input: & [u8],
) -> Result <Vec <u8>, String> {

	// create structure

	let mut zlib_stream = ZlibStream {

		next_in: & input [0],
		avail_in: input.len () as u32,
		total_in: 0,

		next_out: ptr::null_mut (),
		avail_out: 0,
		total_out: 0,

		msg: ptr::null (),
		internal_state: ptr::null (),

		alloc_func: ptr::null (),
		free_func: ptr::null (),
		opaque: ptr::null (),

		data_type: 0,
		adler: 0,
		reserved: 0,

	};

	// init

	let init_result = unsafe {

		deflateInit2_ (
			& mut zlib_stream,
			Z_DEFAULT_COMPRESSION,
			Z_DEFLATED,
			-15,
			8,
			Z_DEFAULT_STRATEGY,
			& b"1.2.8\0" [0],
			mem::size_of::<ZlibStream> () as i32)

	};

	if init_result != Z_OK {

		return Err (
			format! (
				"Deflate init returned {}",
					init_result));

	}

	// main loop

	let mut result: Vec <u8> =
		vec! ();

	loop {

		result.reserve (
			1024 * 1024);

		let temp_len =
			result.len ();

		zlib_stream.next_out = unsafe {

			result.get_unchecked_mut (
				temp_len)

		};

		zlib_stream.avail_out =
			1024 * 1024;

		let deflate_mode =
			if zlib_stream.total_in == input.len () as u64 {
				Z_FINISH
			} else {
				Z_NO_FLUSH
			};

		let loop_result = unsafe {

			deflate (
				& mut zlib_stream,
				deflate_mode)

		};

		unsafe {

			result.set_len (
				zlib_stream.total_out as usize);

		}

		if loop_result == Z_STREAM_END {
			break;
		}

		if loop_result != Z_OK {

			// TODO clean up!

			return Err (
				format! (
					"Deflate returned {}",
					loop_result));

		}

	}

	Ok (result)

}

pub fn decompress (
	input: & [u8],
) -> Result <Vec <u8>, String> {

	// create structure

	let mut zlib_stream = ZlibStream {

		next_in: & input [0],
		avail_in: input.len () as u32,
		total_in: 0,

		next_out: ptr::null_mut (),
		avail_out: 0,
		total_out: 0,

		msg: ptr::null (),
		internal_state: ptr::null (),

		alloc_func: ptr::null (),
		free_func: ptr::null (),
		opaque: ptr::null (),

		data_type: 0,
		adler: 0,
		reserved: 0,

	};

	// init

	let init_result = unsafe {

		inflateInit2_ (
			& mut zlib_stream,
			-15,
			& b"1.2.8\0" [0],
			mem::size_of::<ZlibStream> () as i32)

	};

	if init_result != Z_OK {

		return Err (
			format! (
				"Inflate init returned {}",
				init_result));

	}

	// read data

	let mut result: Vec <u8> =
		vec! ();

	loop {

		result.reserve (
			1024 * 1024);

		let temp_len =
			result.len ();

		zlib_stream.next_out = unsafe {

			result.get_unchecked_mut (
				temp_len)

		};

		zlib_stream.avail_out =
			1024 * 1024;

		let loop_result = unsafe {

			inflate (
				& mut zlib_stream,
				Z_NO_FLUSH)

		};

		unsafe {

			result.set_len (
				zlib_stream.total_out as usize);

		}

		if loop_result == Z_STREAM_END {
			break;
		}

		if loop_result != Z_OK {

			// TODO clean up!

			return Err (
				format! (
					"Deflate returned {}",
					loop_result));

		}

	}

	result.shrink_to_fit ();

	Ok (result)

}
