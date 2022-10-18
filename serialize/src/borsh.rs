use borsh::{
	de::BorshDeserialize,
	ser::BorshSerialize
};
use crate::{CanonicalDeserialize, CanonicalSerialize};
use std::io::{Write, Cursor};

impl<T: CanonicalDeserialize + Sized> BorshDeserialize for T {
	fn deserialize_compressed(buf: &mut &[u8]) -> Result<Self> {
		let mut reader = Cursor::new(*buf);
		let value = Self::deserialize(&mut reader)?;
		let position = reader.position() as usize;
		*buf = &buf[position..];
		Ok(value)
	}
}

impl<T: CanonicalSerialize> BorshSerialize for T {
	fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
		self.serialize_compressed(writer)
	}
}
