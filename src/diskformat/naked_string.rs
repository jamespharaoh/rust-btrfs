use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

pub struct NakedString <'a> (Cow <'a, str>);

impl <'a> From <& 'a str> for NakedString <'a> {

	fn from (
		value: & str,
	) -> NakedString {

		NakedString (Cow::Borrowed (value))

	}

}

impl <'a> From <String> for NakedString <'a> {

	fn from (
		value: String,
	) -> NakedString <'a> {

		NakedString (Cow::Owned (value))

	}

}

impl <'a> Debug for NakedString <'a> {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let NakedString (ref value) =
			* self;

		formatter.write_str (
			value.as_ref ())

	}

}

impl <'a> Display for NakedString <'a> {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let NakedString (ref value) =
			* self;

		formatter.write_str (
			value.as_ref ())

	}

}

// ex: noet ts=4 filetype=rust
