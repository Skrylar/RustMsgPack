
/*
    Magic numbers and statics go here.
 */

/// Defines msgpack types which have a concrete type code. This does not
/// include types which are partially embedded with the data.
#[repr(C)]
pub enum EncodedType {
	Array16  = 0xDC,
	Array32  = 0xDD,
	Bin16    = 0xC5,
	Bin32    = 0xC6,
	Bin8     = 0xC4,
	Ext16    = 0xC8,
	Ext32    = 0xC9,
	Ext8     = 0xC7,
	False    = 0xC2,
	Fixext1  = 0xD4,
	Fixext16 = 0xD8,
	Fixext2  = 0xD5,
	Fixext4  = 0xD6,
	Fixext8  = 0xD7,
	Float32  = 0xCA,
	Float64  = 0xCB,
	Int16    = 0xD1,
	Int32    = 0xD2,
	Int64    = 0xD3,
	Int8     = 0xD0,
	Map16    = 0xDE,
	Map32    = 0xDF,
	Nil      = 0xC0,
	Str16    = 0xDA,
	Str32    = 0xDB,
	Str8     = 0xD9,
	True     = 0xC3,
	Uint16   = 0xCD,
	Uint32   = 0xCE,
	Uint64   = 0xCF,
	Uint8    = 0xCC
}
