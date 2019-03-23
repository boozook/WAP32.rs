//! WAP32 encrypted text files.

// TODO: remove usings of `Vec`
// #![no_std]
/* TODO: remove padding -
	The Blowfish algorithm is a block cipher which works on 8-byte blocks.
	However, if we take a closer look at the ATTRIBUTEZ.TXT file, its size turns out not to be a multiple of 8 bytes.
	At the end of the file there is one additional byte with the value 0x05.
	It'a the last byte in the encrypted file is the amount of bytes from the last decrypted block which should be actually read.
	See explanation: http://datashenanigans.pl/2017/12/gruntz-deciphering-the-attributez-txt-file/
*/


#[path = "blowfish-compat.rs"]
mod blowfish_compat;
pub use self::blowfish_compat::*;


#[inline]
pub fn add_padding(buf: &mut Vec<u8>) {
	static ZERO_BLOCK: [u8; BLOCK] = [0, 0, 0, 0, 0, 0, 0, 0];
	let diff = buf.len() % BLOCK;
	if diff > 0 {
		buf.extend(&ZERO_BLOCK[..(BLOCK - diff)]);
	}
}

/// Drains zero-padding and the last byte in the encrypted file is the amount of bytes
/// from the last decrypted block which should be actually read.
#[inline]
pub fn drain_tail(buf: &mut Vec<u8>) {
	if &buf[(buf.len() - 11)..(buf.len() - 8)] == &[0, 0, 0] {
		buf.drain((buf.len() - 8)..);
	}
}
