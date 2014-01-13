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

#[doc(hidden)];

/*
    Magic numbers and statics go here.
 */

/// Defines msgpack types which have a concrete type code. This does not
/// include types which are partially embedded with the data.
pub mod encoded_type {
	pub static Array16  : u8 = 0xDC;
	pub static Array32  : u8 = 0xDD;
	pub static Bin16    : u8 = 0xC5;
	pub static Bin32    : u8 = 0xC6;
	pub static Bin8     : u8 = 0xC4;
	pub static Ext16    : u8 = 0xC8;
	pub static Ext32    : u8 = 0xC9;
	pub static Ext8     : u8 = 0xC7;
	pub static False    : u8 = 0xC2;
	pub static Fixext1  : u8 = 0xD4;
	pub static Fixext16 : u8 = 0xD8;
	pub static Fixext2  : u8 = 0xD5;
	pub static Fixext4  : u8 = 0xD6;
	pub static Fixext8  : u8 = 0xD7;
	pub static Float32  : u8 = 0xCA;
	pub static Float64  : u8 = 0xCB;
	pub static Int16    : u8 = 0xD1;
	pub static Int32    : u8 = 0xD2;
	pub static Int64    : u8 = 0xD3;
	pub static Int8     : u8 = 0xD0;
	pub static Map16    : u8 = 0xDE;
	pub static Map32    : u8 = 0xDF;
	pub static Nil      : u8 = 0xC0;
	pub static Str16    : u8 = 0xDA;
	pub static Str32    : u8 = 0xDB;
	pub static Str8     : u8 = 0xD9;
	pub static True     : u8 = 0xC3;
	pub static Uint16   : u8 = 0xCD;
	pub static Uint32   : u8 = 0xCE;
	pub static Uint64   : u8 = 0xCF;
	pub static Uint8    : u8 = 0xCC;
}
