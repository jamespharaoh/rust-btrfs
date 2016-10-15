use libc;

use std::error::Error;
use std::fs;
use std::iter;
use std::iter::FromIterator;
use std::mem;
use std::path::Path;
use std::slice;

use ctypes::*;
use filedescriptor::*;
use ioctlwrapper;
use types::*;

pub fn deduplicate_range (
	file_descriptor: libc::c_int,
	dedupe_range: & mut DedupeRange,
) -> Result <(), String> {

	// allocate c structs

	let mut c_dedupe_range_and_range_infos_buffer: Vec <u8> =
		Vec::from_iter (
			iter::repeat (0u8).take (
				mem::size_of::<IoctlFileDedupeRange> ()
				+ (
					dedupe_range.dest_infos.len ()
					* mem::size_of::<IoctlFileDedupeRangeInfo> ()
				)));

	let (c_dedupe_range_buffer, c_dedupe_range_infos_buffer) =
		c_dedupe_range_and_range_infos_buffer.split_at_mut (
			mem::size_of::<IoctlFileDedupeRange> ());

	let c_dedupe_range_slice: & mut [IoctlFileDedupeRange] =
		unsafe {
			slice::from_raw_parts_mut (
				mem::transmute (
					c_dedupe_range_buffer.as_mut_ptr ()),
				1)
		};

	let c_dedupe_range =
		& mut c_dedupe_range_slice [0];

	let c_dedupe_range_infos: & mut [IoctlFileDedupeRangeInfo] =
		unsafe {
			slice::from_raw_parts_mut (
				mem::transmute (
					c_dedupe_range_infos_buffer.as_mut_ptr ()),
				dedupe_range.dest_infos.len ())
		};

	// set input values

	* c_dedupe_range =
		IoctlFileDedupeRange {

		src_offset:
			dedupe_range.src_offset,

		src_length:
			dedupe_range.src_length,

		dest_count:
			dedupe_range.dest_infos.len () as u16,

		reserved1: 0,
		reserved2: 0,

	};

	for index in 0 .. dedupe_range.dest_infos.len () {

		let dest_info =
			& mut dedupe_range.dest_infos [index];

		c_dedupe_range_infos [index] =
			IoctlFileDedupeRangeInfo {

			dest_fd: 
				dest_info.dest_fd,

			dest_offset:
				dest_info.dest_offset,

			bytes_deduped: 0,
			status: 0,
			reserved: 0,

		};

	};

	// perform ioctl

	let file_dedupe_range_result = unsafe {

		ioctlwrapper::file_dedupe_range (
			file_descriptor,
			c_dedupe_range)

	};

	if file_dedupe_range_result != 0 {

		return Err (
			format! (
				"Dedupe ioctl returned {}",
				file_dedupe_range_result)
		);

	}

	// decode c result

	for index in 0 .. dedupe_range.dest_infos.len () {

		let dest_info =
			& mut dedupe_range.dest_infos [index];

		let c_dedupe_range_info =
			& c_dedupe_range_infos [index];

		dest_info.bytes_deduped =
			c_dedupe_range_info.bytes_deduped;

		dest_info.status =
			match c_dedupe_range_info.status {

			FILE_DEDUPE_RANGE_SAME =>
				DedupeRangeStatus::Same,

			FILE_DEDUPE_RANGE_DIFFERS =>
				DedupeRangeStatus::Differs,

			unrecognised_status =>
				return Err (
					format! (
						"Unrecognised status: {}",
						unrecognised_status)
				),

		};

	}

	// return

	Ok (())

}

pub fn deduplicate_files <AsPath: AsRef <Path>> (
	filenames: & [AsPath],
) -> Result <(), String> {

	// nothing to do unless there is more than one filename

	if filenames.len () <= 1 {
		return Ok (());
	}

	// create data structures

	let (source_filename, dest_filenames) =
		filenames.split_at (1);

	let source_filename =
		source_filename [0].as_ref ();

	let source_file_metadata =
		try! (

		fs::metadata (
			source_filename,
		).map_err (
			|io_error|

			format! (
				"Error getting metadata for {:?}: {}",
				source_filename,
				io_error.description ())

		)

	);

	let source_file_descriptor =
		try! (

		FileDescriptor::open (
			source_filename,
			libc::O_RDWR,
		).map_err (
			|error|

			format! (
				"Error opening file: {:?}: {}",
				source_filename,
				error)

		)

	);

	let mut target_file_descriptors: Vec <FileDescriptor> =
		Vec::new ();

	for dest_filename in dest_filenames {

		let dest_filename =
			dest_filename.as_ref ();

		let target_file_descriptor =
			try! (

			FileDescriptor::open (
				dest_filename,
				libc::O_RDWR,
			).map_err (
				|error|

				format! (
					"Error opening file: {:?}: {}",
					dest_filename,
					error)

			)

		);

		target_file_descriptors.push (
			target_file_descriptor);

	}

	let mut dedupe_range =
		DedupeRange {

		src_offset: 0,

		src_length:
			source_file_metadata.len (),

		dest_infos:
			target_file_descriptors.iter ().map (
				|target_file_descriptor|

			DedupeRangeDestInfo {
				dest_fd: target_file_descriptor.get_value () as i64,
				dest_offset: 0,
				bytes_deduped: 0,
				status: DedupeRangeStatus::Same,
			}

		).collect (),

	};

	// perform dedupe

	try! (
		deduplicate_range (
			source_file_descriptor.get_value (),
			& mut dedupe_range));

	// process result

	// TODO

	Ok (())

}

// ex: noet ts=4 filetype=rust
