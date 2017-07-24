use super::prelude::*;

pub fn btrfs_decompress_pages (
	compression_type: u8,
	raw_data: & [u8],
	logical_size: u64,
) -> Result <Cow <[u8]>, String> {

	match compression_type {

		BTRFS_EXTENT_DATA_NO_COMPRESSION =>
			Ok (Cow::Borrowed (
				raw_data,
			)),

		BTRFS_EXTENT_DATA_LZO_COMPRESSION => {

			let mut buffer: Vec <u8> =
				Vec::new ();

			let mut position: usize = 0;

			if raw_data.len () < 4 {

				return Err (
					format! (
						"LZO paging corruption (0)"));

			}

			let total_compressed_size =
				LittleEndian::read_u32 (
					& raw_data [position .. position + 4],
				) as usize;

			if raw_data.len () < total_compressed_size {

				return Err (
					format! (
						"LZO paging corruption (1) @ {} / {}",
						position,
						total_compressed_size));

			}

			position += 4;

			while position < total_compressed_size {

				// skip to next memory page if page compressed size won't fit

				let bytes_left =
					0x1000 - (position % 0x1000);

				if bytes_left < 4 {

					if position + bytes_left > total_compressed_size {

						return Err (
							format! (
								"LZO paging corruption (2) @ {} / {}",
								position,
								total_compressed_size));

					}

					position += bytes_left;

				}

				if position == total_compressed_size {
					break;
				}

				// read page compressed size

				if position + 4 > total_compressed_size {

					return Err (
						format! (
							"LZO paging corruption (3) @ {} / {}",
							position,
							total_compressed_size));

				}

				let page_compressed_size =
					LittleEndian::read_u32 (
						& raw_data [position .. position + 4],
					) as usize;

				position += 4;

				// read page

				if position + page_compressed_size > total_compressed_size {

					return Err (
						format! (
							"LZO paging corruption (4) @ {} / {}",
							position,
							total_compressed_size));

				}

				buffer.extend_from_slice (
					& minilzo::decompress (
						& raw_data [
							position
						..
							position + page_compressed_size
						],
						logical_size as usize,
					).map_err (
						|error|

						format! (
							"LZO decompression failed: {:?}",
							error)

					) ?,
				);

				position += page_compressed_size;

			}

			Ok (Cow::Owned (buffer))

		},

		BTRFS_EXTENT_DATA_ZLIB_COMPRESSION =>
			btrfs_decompress (
				compression_type,
				& raw_data [8 .. ],
				logical_size),

		_ =>
			panic! (),

	}

}

pub fn btrfs_decompress (
	compression_type: u8,
	raw_data: & [u8],
	logical_size: u64,
) -> Result <Cow <[u8]>, String> {

	match compression_type {

		BTRFS_EXTENT_DATA_NO_COMPRESSION =>
			Ok (Cow::Borrowed (
				raw_data,
			)),

		BTRFS_EXTENT_DATA_LZO_COMPRESSION => try! (
			minilzo::decompress (
				raw_data,
				logical_size as usize,
			).map (
				|uncompressed_data|

				Ok (Cow::Owned (
					uncompressed_data,
				))

			).or_else (
				|error|

				Err (
					format! (
						"LZO decompression failed: {:?}",
						error)
				)

			)
		),

		BTRFS_EXTENT_DATA_ZLIB_COMPRESSION => {

			let mut uncompressed_data =
				Vec::with_capacity (
					logical_size as usize);

			uncompressed_data.resize (
				logical_size as usize,
				0u8);

			let mut decompress =
				flate2::Decompress::new (
					false);

			match (
				decompress.decompress (
					raw_data,
					& mut uncompressed_data,
					flate2::Flush::Finish,
				).unwrap_or_else (
					|error|

					panic! (
						"ZLIB decompression failed: {:?}",
						error)

				)
			) {

				flate2::Status::Ok =>
					(),

				_ =>
					panic! (
						"ZLIB decompression failed"),

			}

			if decompress.total_out () as usize
				!= uncompressed_data.len () {

				panic! (
					"ZLIB decompressed size {} does not match {}",
					decompress.total_out (),
					uncompressed_data.len ());

			}

			Ok (Cow::Owned (
					uncompressed_data
			))

		},

		_ =>
			panic! (
				format! (
					"Unrecognised inline extent data compression {}",
					compression_type)),

	}

}

// ex: noet ts=4 filetype=rust
