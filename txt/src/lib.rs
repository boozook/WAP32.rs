//! Gruntz encrypted text files.
// #![no_std]

/* TODO: remove padding -
	The Blowfish algorithm is a block cipher which works on 8-byte blocks.
	However, if we take a closer look at the ATTRIBUTEZ.TXT file, its size turns out not to be a multiple of 8 bytes.
	At the end of the file there is one additional byte with the value 0x05.
	It'a the last byte in the encrypted file is the amount of bytes from the last decrypted block which should be actually read.
	See explanation: http://datashenanigans.pl/2017/12/gruntz-deciphering-the-attributez-txt-file/
*/

/// Extra decoding for Gruntz:
pub fn decode_cheatz(buf: &mut [u8]) {
	const S: u8 = b'"';
	const OFFSET: u8 = 0x1D; // or 0x3D for UPPERCASE
	let mut is_open = false;
	let mut is_cheat = false;
	for b in buf.iter_mut() {
		if *b == S {
			is_open = !is_open;
			if !is_open {
				is_cheat = false;
			}
			continue;
		}

		if is_open && !b.is_ascii() && (*b - OFFSET).is_ascii() {
			is_cheat = true;
		}

		if is_cheat {
			*b = *b - OFFSET;
		}
	}
}


extern crate ini;
pub use ini::Ini;
pub use ini::ini::ParseError;

pub fn parse_ini(buf: &[u8]) -> Result<Ini, ParseError> {
	let s = unsafe { std::str::from_utf8_unchecked(buf) };

	// TODO: remove /* // commets */

	Ini::load_from_str(s)
}
