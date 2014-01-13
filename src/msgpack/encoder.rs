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

/* XXX: Remove these when the module is done */
#[allow(unused_imports,dead_code)];

use magic::*;

use std::cast;
use std::vec;
use std::io;

static ErrTypeReserved : &'static str = "Negative types are reserved for msgpack.";
static ErrWontFit : &'static str = "Provided type is too large to be encoded in msgpack.";

/// A utility which writes MsgPack-encoded data to an underlying `io::Writer`.
pub struct Encoder<'a> {
	priv writer: &'a mut io::Writer
}

/* Put the constructor up here */
impl<'a> Encoder<'a> {
	pub fn new(dst: &'a mut io::Writer) -> Encoder {
		Encoder { writer: dst }
	}
}

/* This is all low-level stuff; don't touch it */
impl<'a> Encoder<'a> {
	#[inline]
	fn write_tag(&mut self, tag: u8) {
		self.writer.write_u8(tag)
	}

	#[inline]
	fn write_array8(&mut self, element_count: u8) {
		self.writer.write_u8(0x90 | (element_count & 0x0F));
	}

	#[inline]
	fn write_array16(&mut self, element_count: u16) {
		self.write_tag(encoded_type::Array16);
		self.writer.write_be_u16(element_count)
	}

	#[inline]
	fn write_array32(&mut self, element_count: u32) {
		self.write_tag(encoded_type::Array32);
		self.writer.write_be_u32(element_count);
	}

	#[inline]
	fn write_map_fix(&mut self, element_count: u8) {
		self.writer.write_u8(0x80 | (element_count & 0x0F));
	}

	#[inline]
	fn write_map16(&mut self, element_count: u16) {
		self.write_tag(encoded_type::Map16);
		self.writer.write_be_u16(element_count)
	}

	#[inline]
	fn write_map32(&mut self, element_count: u32) {
		self.write_tag(encoded_type::Map32);
		self.writer.write_be_u32(element_count);
	}

	#[inline]
	fn write_fix_num(&mut self, t: u8) {
		self.writer.write_u8(t & 0x7F)
	}

	fn write_fix_neg(&mut self, t: i8) {
		let x : u8 = unsafe { cast::transmute(t) };
		self.writer.write_u8(x)
	}

	fn write_str_fix(&mut self, data: &str) {
		let x : u8 = 0xA0 | ((data.len() as u8) & 0x1F);
		self.writer.write_u8(x);
		self.writer.write(data.as_bytes())
	}

	fn write_str8(&mut self, data: &str) {
		self.write_tag(encoded_type::Str8);
		self.writer.write_u8(data.len() as u8);
		self.writer.write(data.as_bytes())
	}

	fn write_str16(&mut self, data: &str) {
		self.write_tag(encoded_type::Str16);
		self.writer.write_be_u16(data.len() as u16);
		self.writer.write(data.as_bytes())
	}

	fn write_str32(&mut self, data: &str) {
		self.write_tag(encoded_type::Str32);
		self.writer.write_be_u32(data.len() as u32);
		self.writer.write(data.as_bytes())
	}

	fn write_ext8(&mut self, user_type: i8, data: &[u8]) {
		self.write_tag(encoded_type::Ext8);
		self.writer.write_i8(user_type);
		self.writer.write_u8(data.len() as u8);
	}

	fn write_ext16(&mut self, user_type: i8, data: &[u8]) {
		self.write_tag(encoded_type::Ext16);
		self.writer.write_i8(user_type);
		self.writer.write_be_u16(data.len() as u16);
	}

	fn write_ext32(&mut self, user_type: i8, data: &[u8]) {
		self.write_tag(encoded_type::Ext32);
		self.writer.write_i8(user_type);
		self.writer.write_be_u32(data.len() as u32);
	}

	fn write_fixext1(&mut self, user_type: i8, data: u8) {
		self.write_tag(encoded_type::Fixext1);
		self.writer.write_i8(user_type);
		self.writer.write_u8(data);
	}

	fn write_fixext2(&mut self, user_type: i8, data: &[u8]) {
		self.write_tag(encoded_type::Fixext2);
		self.writer.write_i8(user_type);
		self.writer.write(data.slice_to(2))
	}

	fn write_fixext4(&mut self, user_type: i8, data: &[u8]) {
		self.write_tag(encoded_type::Fixext4);
		self.writer.write_i8(user_type);
		self.writer.write(data.slice_to(4))
	}

	fn write_fixext8(&mut self, user_type: i8, data: &[u8]) {
		self.write_tag(encoded_type::Fixext8);
		self.writer.write_i8(user_type);
		self.writer.write(data.slice_to(8))
	}

	fn write_fixext16(&mut self, user_type: i8, data: &[u8]) {
		self.write_tag(encoded_type::Fixext16);
		self.writer.write_i8(user_type);
		self.writer.write(data.slice_to(16))
	}

	fn write_bin8(&mut self, data: &[u8]) {
		self.write_tag(encoded_type::Bin8);
		self.writer.write_u8(data.len() as u8);
		self.writer.write(data)
	}

	fn write_bin16(&mut self, data: &[u8]) {
		self.write_tag(encoded_type::Bin16);
		self.writer.write_be_u16(data.len() as u16);
		self.writer.write(data)
	}

	fn write_bin32(&mut self, data: &[u8]) {
		self.write_tag(encoded_type::Bin32);
		self.writer.write_be_u32(data.len() as u32);
		self.writer.write(data)
	}
}

/* This is all the middle-level stuff; still don't touch. */

impl<'a> Encoder<'a> {
 	#[inline]
	fn write_nil(&mut self) {
		self.write_tag(encoded_type::Nil)
	}

	#[inline]
	fn write_false(&mut self) {
		self.write_tag(encoded_type::False)
	}

	#[inline]
	fn write_true(&mut self) {
		self.write_tag(encoded_type::True)
	}

	#[inline]
	fn write_i8(&mut self, t: i8) {
		self.write_tag(encoded_type::Int8);
		self.writer.write_i8(t)
	}

	#[inline]
	fn write_u8(&mut self, t: u8) {
		self.write_tag(encoded_type::Uint8);
		self.writer.write_u8(t)
	}

	#[inline]
	fn write_i16(&mut self, t: i16) {
		self.write_tag(encoded_type::Int16);
		self.writer.write_be_i16(t)
	}

	#[inline]
	fn write_u16(&mut self, t: u16) {
		self.write_tag(encoded_type::Uint16);
		self.writer.write_be_u16(t)
	}

	#[inline]
	fn write_i32(&mut self, t: i32) {
		self.write_tag(encoded_type::Int32);
		self.writer.write_be_i32(t)
	}

	#[inline]
	fn write_u32(&mut self, t: u32) {
		self.write_tag(encoded_type::Uint32);
		self.writer.write_be_u32(t)
	}

	#[inline]
	fn write_i64(&mut self, t: i64) {
		self.write_tag(encoded_type::Int64);
		self.writer.write_be_i64(t)
	}

	#[inline]
	fn write_u64(&mut self, t: u64) {
		self.write_tag(encoded_type::Uint64);
		self.writer.write_be_u64(t)
	}
}

/// A trait which indicates a type knows how to instruct a high layer encoder
/// how to bundle itself. System types have this provided (and call the
/// appropriate pack functions in the layer), however your own POD structures
/// will need to define this for themselves.
pub trait Encodable {
	/// Performs the necessary encoding on a given high layer encoding target.
	fn encode (&self, destination: &mut Encoder);
}

/// Methods which write encoded data in to the stream. Writing functions will
/// select the appropriate header tags to accomidate the amount of data being
/// written, and will ensure both a minimal and correct amount of data is
/// output to the stream.
impl<'a> Encoder<'a> {
	/// Writes an `Encodable` type to the underlying stream.
	#[inline]
	pub fn write<T: Encodable>(&mut self, t: &T) {
		t.encode(self)
	}

	/// Writes a signed integer to the stream, using the least number of bytes
	/// possible. Note that if the provided value is zero or greater, it will
	/// be encoded on the wire as an _unsigned_ number to save space.
	pub fn write_int(&mut self, t: i64) {
		// If this value is positive, safely short-circuit and treat as an
		// unsigned value.
		if (t >= 0) {
			self.write_uint(t as u64)
		} else {
			// Negative value, so now we need to find which type we can cram
			// this in.
			match t {
				x if x >= -15 => self.write_fix_neg(t as i8),
				x if x >= -127 => self.write_i8(t as i8),
				x if x >= -32767 => self.write_i16(t as i16),
				x if x >= -2147483647 => self.write_i32(t as i32),
				x if x >= -9223372036854775808 => self.write_i64(t as i64),
				_ => { fail!(ErrWontFit) }
			}
		}
	}

	/// Writes an unsigned integer to the stream, using the least number of
	/// bytes possible.
	pub fn write_uint(&mut self, t: u64) {
		match t {
			x if x < 128 => self.write_fix_num(t as u8),
			x if x < 256 => self.write_u8(t as u8),
			x if x < 65536 => self.write_u16(t as u16),
			x if x < 4294967296 => self.write_u32(t as u32),
			_ => self.write_u64(t as u64)
		}
	}

	/// Writes a 32-bit floating point value to the stream.
	pub fn write_f32(&mut self, t: f32) {
		self.write_tag(encoded_type::Float32);
		self.writer.write_be_f32(t);
	}

	/// Writes a 64-bit floating point value to the stream.
	pub fn write_f64(&mut self, t: f64) {
		self.write_tag(encoded_type::Float64);
		self.writer.write_be_f64(t)
	}

	/// Writes a boolean to the stream. It will be encoded on the wire as a
	/// MsgPack `true` or a MsgPack `false` value.
	#[inline]
	pub fn write_bool(&mut self, t: bool) {
		match t {
			true => self.write_true(),
			false => self.write_false()
		}
	}

	/// Writes a UTF-8 encoded string to the stream. The smallest possible
	/// string header will be used to convey the length of the string on the
	/// wire.
	pub fn write_str(&mut self, t: &str) {
		let x = t.len();
		match x {
			x if x <= 31 => self.write_str_fix(t),
			x if x <= 255 => self.write_str8(t),
			x if x <= 65535 => self.write_str16(t),
			x if x <= 4294967295 => self.write_str32(t),
			_ => { fail!(ErrWontFit) }
		}
	}

	/// Writes a block of binary data to the stream. The smallest possible
	/// string header will be used to convey the length of the string on the
	/// wire.
	pub fn write_bin(&mut self, t: &[u8]) {
		let x = t.len();
		match x {
			x if x <= 255 => self.write_bin8(t),
			x if x <= 65535 => self.write_bin16(t),
			x if x <= 4294967295 => self.write_bin32(t),
			_ => { fail!(ErrWontFit) }
		}
	}

	/// Writes an extension type to the stream. Sizes of 1, 2, 4, 8 and 16
	/// bytes have special headers which reduces the size on the wire to
	/// encode binary blobs of those exact lengths. You must specify a
	/// `type_code` to identify your application-specific extension type, and
	/// this value must be a positive number. Negative values are reserved for
	/// future MsgPack use, and the function will `fail` if their use is
	/// attempted.
	pub fn write_ext(&mut self, type_code: i8, t: &[u8]) {
		if (type_code < 0) { fail!(ErrTypeReserved); }
		let x = t.len();
		match x {
			x if x == 1 => self.write_fixext1(type_code, t[0]),
 			x if x == 2 => self.write_fixext2(type_code, t),
 			x if x == 4 => self.write_fixext4(type_code, t),
 			x if x == 8 => self.write_fixext8(type_code, t),
 			x if x == 16 => self.write_fixext16(type_code, t),
			x if x <= 255 => self.write_ext8(type_code, t),
			x if x <= 65535 => self.write_ext16(type_code, t),
			x if x <= 4294967295 => self.write_ext32(type_code, t),
			_ => { fail!(ErrWontFit) }
		}
	}

	/// Writes a vector of encodable Rust data as a MsgPack-encoded array.
	pub fn write_array<T: Encodable>(&mut self, t: &[T]) {
		let x = t.len();
		match x {
			y if y <= 16 => self.write_array8(x as u8),
			y if y <= 65535 => self.write_array16(x as u16),
			y if y <= 4294967295 => self.write_array32(x as u32),
			_ => { fail!(ErrWontFit) }
		}
		for z in t.iter() {
			z.encode(self)
		}
	}

	// TODO map

	/// Writes an `Option` type to the stream; if `None`, then a MsgPack NIL
	/// is written to the stream, otherwise the encoded value of the option is
	/// written.
	#[experimental]
	pub fn write_option<T: Encodable>(&mut self, t: &Option<T>) {
		match t.as_ref() {
			Some(x) => x.encode(self),
			None => self.write_nil()
		}
	}
}
