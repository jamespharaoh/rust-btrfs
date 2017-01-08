use linux::imports::*;

#[ derive (Debug, Eq, PartialEq) ]
pub enum CompressionType {
	None,
	Zlib,
	Lzo,
}

impl Into <u32> for CompressionType {

	fn into (self) -> u32 {

		use CompressionType::*;

		match self {
			None => COMPRESS_NONE,
			Zlib => COMPRESS_ZLIB,
			Lzo => COMPRESS_LZO,
		}

	}

}

// ex: noet ts=4 filetype=rust
