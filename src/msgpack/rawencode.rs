
use magic::*;

use std::cast;
use std::vec;

// TODO: Rewrite most of this crap to use std::io::Writer.

static ErrTypeReserved : &'static str = "Negative types are reserved for msgpack.";
static ErrWontFit : &'static str = "Provided type will not fit in requested data type.";

/// Contains encoding functions for byte vectors which may be used to produce
/// potentially invalid MsgPack streams. There are no safety checks at this
/// level, so the direct use of this trait is highly discouraged.
pub trait EncodingLowLayer {
	/// Pushes a MsgPack type tag to the stream.
	fn push_msgpack_tag(&mut self, tag: EncodedType);

	/// Pushes an 8-bit byte. Not actually different from the regular push()
	/// method, this is just here for consistency.
	fn push_msgpack_raw8(&mut self, t: u8);

	/// Pushes a 16-bit value, encoded in big endian.
	fn push_msgpack_raw16(&mut self, t: u16);

	/// Pushes a 32-bit value, encoded in big endian.
	fn push_msgpack_raw32(&mut self, t: u32);

	/// Pushes a 64-bit value, encoded in big endian.
	fn push_msgpack_raw64(&mut self, t: u64);

	/// Pushes an array heading with a 5-bit element count (up to 15 elements.)
	fn push_msgpack_array_header_fix(&mut self, element_count: u8);

	/// Pushes an array heading with a 16-bit element count.
	fn push_msgpack_array_header16(&mut self, element_count: u16);

	/// Pushes an array heading with a 32-bit element count.
	fn push_msgpack_array_header32(&mut self, element_count: u32);

	/// Pushes an array heading with a 5-bit element count (up to 15 elements.)
	fn push_msgpack_map_header_fix(&mut self, element_count: u8);

	/// Pushes a map heading with a 16-bit element count.
	fn push_msgpack_map_header16(&mut self, element_count: u16);

	/// Pushes a map heading with a 32-bit element count.
	fn push_msgpack_map_header32(&mut self, element_count: u32);

	/// Pushes a 7-bit positive integer.
	fn push_msgpack_fix_num(&mut self, t: u8);

	/// Pushes a 5-bit signed integer.
	fn push_msgpack_fix_num_neg(&mut self, t: i8);

	/// Pushes a string with a length of up to 31 bytes.
	fn push_msgpack_str_fix(&mut self, data: &str);

	/// Pushes a string with an 8-bit length field.
	fn push_msgpack_str8(&mut self, data: &str);

	/// Pushes a string with a 16-bit length field.
	fn push_msgpack_str16(&mut self, data: &str);

	/// Pushes a string with a 32-bit length field.
	fn push_msgpack_str32(&mut self, data: &str);

	/// Pushes an extension block with an 8-bit length field.
	fn push_msgpack_ext8(&mut self, user_type: i8, data: &[u8]);

	/// Pushes an extension block with a 16-bit length field.
	fn push_msgpack_ext16(&mut self, user_type: i8, data: &[u8]);

	/// Pushes an extension block with a 32-bit length field.
	fn push_msgpack_ext32(&mut self, user_type: i8, data: &[u8]);

	// TODO: See if we need to change how the fixext methods work.

	/// Pushes a fixed extension block with a single byte of user data.
	fn push_msgpack_fixext1(&mut self, user_type: i8, data: u8);

	/// Pushes a fixed extension block with two bytes of user data.
	fn push_msgpack_fixext2(&mut self, user_type: i8, data: &[u8, ..2]);

	/// Pushes a fixed extension block with four bytes of user data.
	fn push_msgpack_fixext4(&mut self, user_type: i8, data: &[u8, ..4]);

	/// Pushes a fixed extension block with eight bytes of user data.
	fn push_msgpack_fixext8(&mut self, user_type: i8, data: &[u8, ..8]);

	/// Pushes a fixed extension block with sixteen bytes of user data.
	fn push_msgpack_fixext16(&mut self, user_type: i8, data: &[u8, ..16]);

	/// Pushes a binary block with an 8-bit length field.
	fn push_msgpack_bin8(&mut self, data: &[u8]);

	/// Pushes a binary block with a 16-bit length field.
	fn push_msgpack_bin16(&mut self, data: &[u8]);

	/// Pushes a binary block with a 32-bit length field.
	fn push_msgpack_bin32(&mut self, data: &[u8]);
}

impl EncodingLowLayer for ~[u8] {
	#[inline]
	fn push_msgpack_tag(&mut self, tag: EncodedType) {
		self.push(tag as u8)
	}

	#[inline]
	fn push_msgpack_raw8(&mut self, t: u8) {
		self.push(t)
	}

	#[inline]
	fn push_msgpack_raw16(&mut self, t: u16) {
		self.push(((t & 0xFF00) >> 8) as u8);
		self.push((t & 0x00FF) as u8);
	}

	#[inline]
	fn push_msgpack_raw32(&mut self, t: u32) {
		self.push(((t & 0xFF000000) >> 24) as u8);
		self.push(((t & 0x00FF0000) >> 16) as u8);
		self.push(((t & 0x0000FF00) >> 8) as u8);
		self.push( (t & 0x000000FF) as u8);
	}

	#[inline]
	fn push_msgpack_raw64(&mut self, t: u64) {
		self.push(((t & 0xFF00000000000000) >> 56) as u8);
		self.push(((t & 0x00FF000000000000) >> 48) as u8);
		self.push(((t & 0x0000FF0000000000) >> 40) as u8);
		self.push(((t & 0x000000FF00000000) >> 32) as u8);
		self.push(((t & 0x00000000FF000000) >> 24) as u8);
		self.push(((t & 0x0000000000FF0000) >> 16) as u8);
		self.push(((t & 0x000000000000FF00) >> 8) as u8);
		self.push( (t & 0x00000000000000FF) as u8);
	}

	#[inline]
	fn push_msgpack_array_header_fix(&mut self, element_count: u8) {
		self.push(0x90 | (element_count & 0x0F));
	}

	#[inline]
	fn push_msgpack_array_header16(&mut self, element_count: u16) {
		self.push_msgpack_tag(Array16);
		self.push_msgpack_raw16(element_count)
	}

	#[inline]
	fn push_msgpack_array_header32(&mut self, element_count: u32) {
		self.push_msgpack_tag(Array32);
		self.push_msgpack_raw32(element_count);
	}

	#[inline]
	fn push_msgpack_map_header_fix(&mut self, element_count: u8) {
		self.push(0x80 | (element_count & 0x0F));
	}

	#[inline]
	fn push_msgpack_map_header16(&mut self, element_count: u16) {
		self.push_msgpack_tag(Map16);
		self.push_msgpack_raw16(element_count)
	}

	#[inline]
	fn push_msgpack_map_header32(&mut self, element_count: u32) {
		self.push_msgpack_tag(Map32);
		self.push_msgpack_raw32(element_count);
	}

	#[inline]
	fn push_msgpack_fix_num(&mut self, t: u8) {
		self.push(t & 0x7F)
	}

	fn push_msgpack_fix_num_neg(&mut self, t: i8) {
		let x : u8 = unsafe { cast::transmute(t & 0x1F) };
		self.push(0xE0 | x)
	}

	fn push_msgpack_str_fix(&mut self, data: &str) {
		let x :u8 = 0xA0 | ((data.len() as u8) & 0x1F);
		self.push(x);
		vec::bytes::push_bytes(self, data.as_bytes())
	}

	fn push_msgpack_str8(&mut self, data: &str) {
		if data.len() > 0xFF { fail!(ErrWontFit); }
		self.push_msgpack_tag(Str8);
		self.push_msgpack_raw8(data.len() as u8);
		vec::bytes::push_bytes(self, data.as_bytes())
	}

	fn push_msgpack_str16(&mut self, data: &str) {
		if data.len() > 0xFFFF { fail!(ErrWontFit); }
		self.push_msgpack_tag(Str16);
		self.push_msgpack_raw16(data.len() as u16);
		vec::bytes::push_bytes(self, data.as_bytes())
	}

	fn push_msgpack_str32(&mut self, data: &str) {
		if data.len() > 0xFFFFFFFF { fail!(ErrWontFit); }
		self.push_msgpack_tag(Str32);
		self.push_msgpack_raw32(data.len() as u32);
		vec::bytes::push_bytes(self, data.as_bytes())
	}

	fn push_msgpack_ext8(&mut self, user_type: i8, data: &[u8]) {
		if user_type < 0 { fail!(ErrTypeReserved); }
		if data.len() > 0xFF { fail!(ErrWontFit); }
		self.push_msgpack_tag(Ext8);
		self.push_msgpack_raw8(data.len() as u8);
	}

	fn push_msgpack_ext16(&mut self, user_type: i8, data: &[u8]) {
		if user_type < 0 { fail!(ErrTypeReserved); }
		if data.len() > 0xFFFF { fail!(ErrWontFit); }
		self.push_msgpack_tag(Ext16);
		self.push_msgpack_raw16(data.len() as u16);
	}
	
	fn push_msgpack_ext32(&mut self, user_type: i8, data: &[u8]) {
		if user_type < 0 { fail!(ErrTypeReserved); }
		if data.len() > 0xFFFFFFFF { fail!(ErrWontFit); }
		self.push_msgpack_tag(Ext32);
		self.push_msgpack_raw32(data.len() as u32);
	}

	fn push_msgpack_fixext1(&mut self, user_type: i8, data: u8) {
		if user_type < 0 { fail!(ErrTypeReserved); }
		self.push_msgpack_tag(Fixext1);
		unsafe { self.push_msgpack_raw8(cast::transmute(user_type)); }
		self.push_msgpack_raw8(data);
	}

	fn push_msgpack_fixext2(&mut self, user_type: i8, data: &[u8, ..2]) {
		if user_type < 0 { fail!(ErrTypeReserved); }
		self.push_msgpack_tag(Fixext2);
		unsafe { self.push_msgpack_raw8(cast::transmute(user_type)); }
		vec::bytes::push_bytes(self, data.slice_to(2))
	}

	fn push_msgpack_fixext4(&mut self, user_type: i8, data: &[u8, ..4]) {
		if user_type < 0 { fail!(ErrTypeReserved); }
		self.push_msgpack_tag(Fixext4);
		unsafe { self.push_msgpack_raw8(cast::transmute(user_type)); }
		vec::bytes::push_bytes(self, data.slice_to(4))
	}

	fn push_msgpack_fixext8(&mut self, user_type: i8, data: &[u8, ..8]) {
		if user_type < 0 { fail!(ErrTypeReserved); }
		self.push_msgpack_tag(Fixext8);
		unsafe { self.push_msgpack_raw8(cast::transmute(user_type)); }
		vec::bytes::push_bytes(self, data.slice_to(8))
	}

	fn push_msgpack_fixext16(&mut self, user_type: i8, data: &[u8, ..16]) {
		if user_type < 0 { fail!(ErrTypeReserved); }
		self.push_msgpack_tag(Fixext16);
		unsafe { self.push_msgpack_raw8(cast::transmute(user_type)); }
		vec::bytes::push_bytes(self, data.slice_to(16))
	}

	fn push_msgpack_bin8(&mut self, data: &[u8]) {
		if data.len() > 0xFF { fail!(ErrWontFit); }
		self.push_msgpack_tag(Bin8);
		self.push_msgpack_raw8(data.len() as u8);
		vec::bytes::push_bytes(self, data)
	}

	fn push_msgpack_bin16(&mut self, data: &[u8]) {
		if data.len() > 0xFFFF { fail!(ErrWontFit); }
		self.push_msgpack_tag(Bin16);
		self.push_msgpack_raw16(data.len() as u16);
		vec::bytes::push_bytes(self, data)
	}

	fn push_msgpack_bin32(&mut self, data: &[u8]) {
		if data.len() > 0xFFFFFFFF { fail!(ErrWontFit); }
		self.push_msgpack_tag(Bin32);
		self.push_msgpack_raw32(data.len() as u32);
		vec::bytes::push_bytes(self, data)
	}
}

/// Contains middle level encoding functions for MsgPack. These methods also
/// do not perform safety checks, however they cannot be misused (they accept
/// the exact types they encode.)
pub trait EncodingMidLayer : EncodingLowLayer {
	fn push_msgpack_false(&mut self);
	fn push_msgpack_i16(&mut self, t: i16);
	fn push_msgpack_i32(&mut self, t: i32);
	fn push_msgpack_i64(&mut self, t: i64);
	fn push_msgpack_i8(&mut self, t: i8);
	fn push_msgpack_nil(&mut self);
	fn push_msgpack_true(&mut self);
	fn push_msgpack_u16(&mut self, t: u16);
	fn push_msgpack_u32(&mut self, t: u32);
	fn push_msgpack_u64(&mut self, t: u64);
	fn push_msgpack_u8(&mut self, t: u8);
}

impl EncodingMidLayer for ~[u8] {
	#[inline]
	fn push_msgpack_nil(&mut self) {
		self.push_msgpack_tag(Nil)
	}

	#[inline]
	fn push_msgpack_false(&mut self) {
		self.push_msgpack_tag(False)
	}

	#[inline]
	fn push_msgpack_true(&mut self) {
		self.push_msgpack_tag(True)
	}

	#[inline]
	fn push_msgpack_i8(&mut self, t: i8) {
		self.push_msgpack_tag(Int8);
		unsafe { self.push_msgpack_raw8(cast::transmute(t)) }
	}

	#[inline]
	fn push_msgpack_u8(&mut self, t: u8) {
		self.push_msgpack_tag(Uint8);
		self.push_msgpack_raw8(t)
	}


	#[inline]
	fn push_msgpack_i16(&mut self, t: i16) {
		self.push_msgpack_tag(Int16);
		unsafe { self.push_msgpack_raw16(cast::transmute(t)) }
	}

	#[inline]
	fn push_msgpack_u16(&mut self, t: u16) {
		self.push_msgpack_tag(Uint16);
		self.push_msgpack_raw16(t)
	}

	#[inline]
	fn push_msgpack_i32(&mut self, t: i32) {
		self.push_msgpack_tag(Int32);
		unsafe { self.push_msgpack_raw32(cast::transmute(t)) }
	}
	
	#[inline]
	fn push_msgpack_u32(&mut self, t: u32) {
		self.push_msgpack_tag(Uint32);
		self.push_msgpack_raw32(t);
	}

	#[inline]
	fn push_msgpack_i64(&mut self, t: i64) {
		self.push_msgpack_tag(Int64);
		unsafe { self.push_msgpack_raw64(cast::transmute(t)) }
	}

	#[inline]
	fn push_msgpack_u64(&mut self, t: u64) {
		self.push_msgpack_tag(Uint64);
		self.push_msgpack_raw64(t)
	}
}

/// A trait which indicates a type knows how to instruct a high layer encoder
/// how to bundle itself. System types have this provided (and call the
/// appropriate pack functions in the layer), however your own POD structures
/// will need to define this for themselves.
pub trait Encodable {
	/// Performs the necessary encoding on a given high layer encoding target.
	fn encode<T: EncodingHighLayer> (&self, destination: &mut T);
}

/// The "high layer" is intended for use by normal Rust code, as it takes care
/// of analyzing the data provided and selecting the most compact and
/// appropriate lower level encoders.
pub trait EncodingHighLayer : EncodingMidLayer {
	/// Allows the type to choose which packing function is most suitable;
	/// such as structs which need to choose how their fields will be
	/// serialized.
	fn push_msgpack<T: Encodable>(&mut self, t: &T) {
		t.encode(self)
	}

	fn push_msgpack_int(&mut self, t: i64) {
		// If this value is positive, safely short-circuit and treat as an
		// unsigned value.
		if (t > 0) {
			self.push_msgpack_uint(t as u64)
		} else {
			match t {
				x if x >= -15 => self.push_msgpack_fix_num_neg(t as i8),
				x if x >= -127 => self.push_msgpack_i8(t as i8),
				x if x >= -32767 => self.push_msgpack_i16(t as i16),
				x if x >= -2147483647 => self.push_msgpack_i32(t as i32),
				x if x >= -9223372034707292159 => self.push_msgpack_i64(t as i64),
				_ => { /* TODO: fail */ }
			}
		}
	}

	fn push_msgpack_uint(&mut self, t: u64) {
		match t {
			x if x <= 2^7 => self.push_msgpack_fix_num(t as u8),
			x if x <= 2^8 => self.push_msgpack_u8(t as u8),
			x if x <= 2^16 => self.push_msgpack_u16(t as u16),
			x if x <= 2^32 => self.push_msgpack_u32(t as u32),
			x if x <= 2^64 => self.push_msgpack_u64(t as u64),
			_ => { /* TODO: fail */ }
		}
	}

	fn push_msgpack_f32(&mut self, t: f32) {
		self.push_msgpack_tag(Float32);
		unsafe { self.push_msgpack_raw32(cast::transmute(t)) }
	}

	fn push_msgpack_f64(&mut self, t: f64) {
		self.push_msgpack_tag(Float64);
		unsafe { self.push_msgpack_raw64(cast::transmute(t)) }
	}

	fn push_msgpack_bool(&mut self, t: bool) {
		match t {
			true => self.push_msgpack_true(),
			false => self.push_msgpack_false()
		}
	}

	fn push_msgpack_str(&mut self, t: &str) {
		let x = t.len();
		match x {
			x if x <= 31 => self.push_msgpack_str_fix(t),
			x if x <= (2^8)-1 => self.push_msgpack_str8(t),
			x if x <= (2^16)-1 => self.push_msgpack_str16(t),
			x if x <= (2^32)-1 => self.push_msgpack_str32(t),
			_ => { /* TODO: error */ }
		}
	}

	fn push_msgpack_bytes(&mut self, t: &[u8]) {
		let x = t.len();
		match x {
			x if x <= (2^8)-1 => self.push_msgpack_bin8(t),
			x if x <= (2^16)-1 => self.push_msgpack_bin16(t),
			x if x <= (2^32)-1 => self.push_msgpack_bin32(t),
			_ => { /* TODO: error */ }
		}
	}

	fn push_msgpack_extension(&mut self, type_code: i8, t: &[u8]) {
		let x = t.len();
		match x {
			x if x == 1 => self.push_msgpack_fixext1(type_code, t[0]),
 			x if x == 2 => self.push_msgpack_fixext2(type_code, t),
 			x if x == 4 => self.push_msgpack_fixext4(type_code, t),
 			x if x == 8 => self.push_msgpack_fixext8(type_code, t),
 			x if x == 16 => self.push_msgpack_fixext16(type_code, t),
			x if x <= (2^8)-1 => self.push_msgpack_ext8(type_code, t),
			x if x <= (2^16)-1 => self.push_msgpack_ext16(type_code, t),
			x if x <= (2^32)-1 => self.push_msgpack_ext32(type_code, t),
			_ => { /* TODO: error */ }
		}
	}

	/// Encodes a Rust vector of a variable length, by placing the
	/// appropriately sized array header in the stream and recursively
	/// encoding each member of the vector in sequence.
	fn push_msgpack_array<T: Encodable>(&mut self, t: &[T]) {
		let x = t.len();
		match x {
			y if y <= 16 => self.push_msgpack_array_header_fix(x as u8),
			y if y <= 2^16 => self.push_msgpack_array_header16(x as u16),
			y if y <= 2^32 => self.push_msgpack_array_header32(x as u32),
			_ => { /* TODO: Fail */ }
		}
		for z in t.iter() {
			z.encode(self)
		}
	}

	// TODO map
	// TODO option types and nulls
}
