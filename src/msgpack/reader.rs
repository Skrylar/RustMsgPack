/*
	Copyright (c) 2014 Joshua A. Cearley

	This software is provided as-is, without any express or implied
	warranty. In no event will the authors be held liable for any damages
	arising from the use of this software.

	Permission is granted to anyone to use this software for any purpose,
	including commercial applications, and to alter it and redistribute
	it freely, subject to the following restrictions:

	1. The origin of this software must not be misrepresented; you must not
	claim that you wrote the original software. If you use this software
	in a product, an acknowledgment in the product documentation would be
	appreciated but is not required.

	2. Altered source versions must be plainly marked as such, and must not
	be misrepresented as being the original software.

	3. This notice may not be removed or altered from any source
	distribution.
 */

use magic::*;

use std::cast;
use std::io;
use std::str;
use std::vec;

static ErrNoData : &'static str = "No valid MsgPack type to read.";
static ErrInsufficientData : &'static str = "Not enough bytes available to read value.";

pub enum DecodedValue {
	Signed(i64),
	Unsigned(u64),
	String(~str),
	Binary(~[u8]),
	Array(~[DecodedValue]),
	Float32(f32),
	Float64(f64),
	Extension(i8, ~[u8]),
	// TODO: Map.
	Boolean(bool),
	Nil
}

pub struct MsgPackReader<'a> {
	priv reader: &'a mut io::Reader
}

impl<'a> MsgPackReader<'a> {
	pub fn new(reader: &'a mut io::Reader) -> MsgPackReader {
		MsgPackReader { reader: reader }
	}
}

impl<'a> MsgPackReader<'a> {
	pub fn read(&mut self) -> DecodedValue {
		let x = self.try_read();
		match x {
			Ok(y) => y,
			Err(e) => fail!(e)
		}
	}

	#[allow(experimental)]
	fn read_array(&mut self, elements: uint) -> Result<DecodedValue, &'static str> {
		/* reserve space for each value */
		let mut accum : ~[DecodedValue] = vec::with_capacity(elements);
		/* for each element we are to read */
		let mut current = 0;
		while (current < elements) {
			let next_candidate = self.try_read();
			match next_candidate {
				Ok(x) => accum.push(x),
				Err(_) => return next_candidate
			}
			current += 1;
		}
		/* return decoded array */
		Ok(Array(accum))
	}

	fn read_str(&mut self, len: uint) -> Result<DecodedValue, &'static str> {
		let data = self.reader.read_bytes(len);
		if data.len() == len {
			Ok(String(str::from_utf8_owned(data)))
		} else {
			Err(ErrInsufficientData)
		}
	}

	/// Note: If reading fails, the stream is not reset to its original
	/// position. It is up to the user to store the current position and
	/// perform any seeking if they desire. (Seeking may not always be
	/// desirable, and the reader may not support arbitrary seeking; so this
	/// is left to the user who will know more about what they want.)
	#[experimental] // experimental until the TODOs are fixed.
	pub fn try_read(&mut self) -> Result<DecodedValue, &'static str> {
		let tag = self.reader.read_u8();
		match tag {
			// fixed numbers (7-bit)
			x if ((x & 0x80) == 0) => { Ok(Unsigned((x & 0x7F) as u64)) }
			// fixed negative values (5-bit)
			x if ((x & 0xE0) == 0xE0) => {
				let x : i8 = unsafe { cast::transmute( x ) };
				Ok(Signed(x as i64) )
			}
			// fixed string (5-bit, up to 31 elements)
			x if ((x & 0xE0) == 0xA0) => {
				self.read_str((x & 0x1F) as uint)
			}
			// fixed array (4-bit, up to 15 elements)
			x if ((x & 0xF0) == 0x90) => {
				let len : u8 = x & 0xF;
				self.read_array(len as uint)
			}
			// fixed map (4-bit, up to 15 elements)
			x if ((x & 0xF0) == 0x80) => { fail!("TODO.") }

			encoded_type::True => { Ok(Boolean(true)) }
			encoded_type::False  => { Ok(Boolean(false)) }
			encoded_type::Nil => { Ok(Nil) }
			encoded_type::Array16  => {
				let len = self.reader.read_be_u16();
				self.read_array(len as uint)
			}
			encoded_type::Array32  => {
				let len = self.reader.read_be_u32();
				self.read_array(len as uint)
			}
			encoded_type::Bin8  => {
				let len = self.reader.read_u8() as uint;
				let data : ~[u8] = self.reader.read_bytes(len);
				if data.len() != len {
					Err(ErrInsufficientData)
				} else {
					Ok(Binary(data))
				}
			}
			encoded_type::Bin16  => {
				let len = self.reader.read_u8() as uint;
				let data : ~[u8] = self.reader.read_bytes(len);
				if data.len() != len {
					Err(ErrInsufficientData)
				} else {
					Ok(Binary(data))
				}
			}
			encoded_type::Bin32  => {
				let len = self.reader.read_u8() as uint;
				let data : ~[u8] = self.reader.read_bytes(len);
				if data.len() != len {
					Err(ErrInsufficientData)
				} else {
					Ok(Binary(data))
				}
			}
			encoded_type::Ext8  => {
				let len = self.reader.read_u8() as uint;
				let type_code = self.reader.read_i8();
				let data = self.reader.read_bytes(len);
				if data.len() == len {
					Ok(Extension(type_code, data))
				} else {
					Err(ErrInsufficientData)
				}
			}
			encoded_type::Ext16  => {
				let len = self.reader.read_be_u16() as uint;
				let type_code = self.reader.read_i8();
				let data = self.reader.read_bytes(len);
				if data.len() == len {
					Ok(Extension(type_code, data))
				} else {
					Err(ErrInsufficientData)
				}
			}
			encoded_type::Ext32  => {
				let len = self.reader.read_be_u32() as uint;
				let type_code = self.reader.read_i8();
				let data = self.reader.read_bytes(len);
				if data.len() == len {
					Ok(Extension(type_code, data))
				} else {
					Err(ErrInsufficientData)
				}
			}
			encoded_type::Fixext1  => {
				let type_code = self.reader.read_i8();
				let data = self.reader.read_bytes(1);
				if data.len() == 1 {
					Ok(Extension(type_code, data))
				} else {
					Err(ErrInsufficientData)
				}
			}
			encoded_type::Fixext2  => {
				let type_code = self.reader.read_i8();
				let data = self.reader.read_bytes(2);
				if data.len() == 2 {
					Ok(Extension(type_code, data))
				} else {
					Err(ErrInsufficientData)
				}
			}
			encoded_type::Fixext4  => {
				let type_code = self.reader.read_i8();
				let data = self.reader.read_bytes(4);
				if data.len() == 4 {
					Ok(Extension(type_code, data))
				} else {
					Err(ErrInsufficientData)
				}
			}
			encoded_type::Fixext8  => {
				let type_code = self.reader.read_i8();
				let data = self.reader.read_bytes(8);
				if data.len() == 8 {
					Ok(Extension(type_code, data))
				} else {
					Err(ErrInsufficientData)
				}
			}
			encoded_type::Fixext16  => {
				let type_code = self.reader.read_i8();
				let data = self.reader.read_bytes(16);
				if data.len() == 16 {
					Ok(Extension(type_code, data))
				} else {
					Err(ErrInsufficientData)
				}
			}
			encoded_type::Float32  => { Ok(Float32(self.reader.read_be_f32())) }
			encoded_type::Float64  => { Ok(Float64(self.reader.read_be_f64())) }
			encoded_type::Int8  => { Ok(Signed(self.reader.read_i8() as i64)) }
			encoded_type::Int16  => { Ok(Signed(self.reader.read_be_i16() as i64)) }
			encoded_type::Int32  => { Ok(Signed(self.reader.read_be_i32() as i64)) }
			encoded_type::Int64  => { Ok(Signed(self.reader.read_be_i64())) }
			encoded_type::Map16  => { fail!("TODO.") }
			encoded_type::Map32  => { fail!("TODO.") }
			encoded_type::Str8  => {
				let len = self.reader.read_u8() as uint;
				self.read_str(len)
			}
			encoded_type::Str16  => {
				let len = self.reader.read_be_u16() as uint;
				self.read_str(len)
			}
			encoded_type::Str32  => {
				let len = self.reader.read_be_u32() as uint;
				self.read_str(len)
			}
			encoded_type::Uint8  => { Ok(Unsigned(self.reader.read_u8() as u64)) }
			encoded_type::Uint16  => { Ok(Unsigned(self.reader.read_be_u16() as u64)) }
			encoded_type::Uint32  => { Ok(Unsigned(self.reader.read_be_u32() as u64)) }
			encoded_type::Uint64  => { Ok(Unsigned(self.reader.read_be_u64())) }

			_ => Err(ErrNoData)
		}
	}
}
