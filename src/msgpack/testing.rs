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

#[cfg(test)];
#[doc(hidden)];

/// Tests that what is written by the encoder has the correctly minimal
/// packing size, as well as our decoder being able to interpret the same data
/// from the encoded stream as we originally inserted.
mod round_trip {
	use std::io;
	use std::io::mem;

	use reader::*;
	use writer::MsgPackWriter;

	fn test_harness(len: u64, encode: |&mut MsgPackWriter|, decode: |DecodedValue|) {
		let buffer : &mut [u8] = ~[0, .. 1024];
		/* encoding cycle */ {
			let mut writer = mem::BufWriter::new(buffer);
			let mut packer = MsgPackWriter::new(&mut writer as &mut io::Writer);
			encode(&mut packer);
			assert!(writer.tell() == len,
				format!("Expected encoding of {} byte(s), got {} byte(s)", len, writer.tell()));
		}
		/* decoding cycle */ {
			let mut reader = mem::BufReader::new(buffer);
			let mut unpacker = MsgPackReader::new(&mut reader as &mut io::Reader);
			decode(unpacker.read());
		}
	}

	#[test]
	fn boolean_true() {
		test_harness(1, |packer| {
			packer.write_bool(true)
		},
		|x| match x {
			Boolean(x) => match x {
				true => {},
				_ => fail!("Did not unpack a true value.")
			},
			_ => fail!("Did not unpack a boolean.")
		})
	}

	#[test]
	fn boolean_false() {
		test_harness(1, |packer| {
			packer.write_bool(false)
		},
		|x| match x {
			Boolean(x) => match x {
				false => {},
				_ => fail!("Did not unpack a true value.")
			},
			_ => fail!("Did not unpack a boolean.")
		})
	}

	#[test]
	fn unsigned_fixed() {
		test_harness(1, |packer| {
			packer.write_uint(64)
		},
		|x| match x {
			Unsigned(x) => match x {
				64 => {},
				_ => fail!("Did not unpack 64.")
			},
			_ => fail!("Did not unpack an unsigned.")
		})
	}

	#[test]
	fn unsigned_8bit() {
		test_harness(2, |packer| {
			packer.write_uint(128)
		},
		|x| match x {
			Unsigned(x) => match x {
				128 => {},
				_ => fail!("Did not unpack 128.")
			},
			_ => fail!("Did not unpack an unsigned.")
		})
	}

	#[test]
	fn unsigned_16bit() {
		test_harness(3, |packer| {
			packer.write_uint(256)
		},
		|x| match x {
			Unsigned(x) => match x {
				256 => {},
				_ => fail!("Did not unpack 256.")
			},
			_ => fail!("Did not unpack an unsigned.")
		})
	}

	#[test]
	fn unsigned_32bit() {
		test_harness(5, |packer| {
			packer.write_uint(65536)
		},
		|x| match x {
			Unsigned(x) => match x {
				65536 => {},
				_ => fail!("Did not unpack 65536.")
			},
			_ => fail!("Did not unpack an unsigned.")
		})
	}

	#[test]
	fn unsigned_64bit() {
		test_harness(9, |packer| {
			packer.write_uint(4294967296)
		},
		|x| match x {
			Unsigned(x) => match x {
				4294967296 => {},
				_ => fail!("Did not unpack 4294967296.")
			},
			_ => fail!("Did not unpack an unsigned.")
		})
	}

	#[test]
	fn string_fixed() {
		let fish = ~"fish";
		test_harness(5, |packer| {
			packer.write_str(fish)
		},
		|x| match x {
			String(x) => if !x.eq(&fish) {
				fail!("Did not unpack fixed string correctly.")
			},
			_ => fail!("Did not unpack an unsigned.")
		})
	}

	#[test]
	fn string_8bit() {
		let fish = ~"tomato soup is a long string that needs eight bytes";
		test_harness(53, |packer| {
			packer.write_str(fish)
		},
		|x| match x {
			String(x) => if !x.eq(&fish) {
				fail!("Did not unpack 8-bit string correctly.")
			},
			_ => fail!("Did not unpack an unsigned.")
		})
	}

    /* 16 and 32-bit strings are presently absent as they would require a lot
     * of memory to test */

	#[test]
	fn bin_8bit() {
		let data : ~[u8] = ~[0, .. 64];
		/* TODO: Put sentry information in data, test if it codes/decodes */
		test_harness(66, |packer| {
			packer.write_bin(data)
		},
		|x| match x {
			Binary(x) => if x == data {
				/* TODO: Check sentry information */
			},
			_ => fail!("Did not unpack a binary.")
		})
	}

	/* 16 and 32-bit binaries are presently absent as they would require a lot
	 * of memory to test */

	#[test]
	fn signed_fixed() {
		test_harness(1, |packer| {
			packer.write_int(-7)
		},
		|x| match x {
			Signed(x) => match x {
				-7 => {},
				_ => fail!("Expected -7, got {}", x)
			},
			_ => fail!("Did not unpack a signed.")
		})
	}

	#[test]
	fn signed_8bit() {
		test_harness(2, |packer| {
			packer.write_int(-69)
		},
		|x| match x {
			Signed(x) => match x {
				-69 => {},
				_ => fail!("Did not unpack -69.")
			},
			_ => fail!("Did not unpack a signed.")
		})
	}
}
