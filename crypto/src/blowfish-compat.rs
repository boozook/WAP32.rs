//! Wrapper under the blowfish algorithm
//! Implements compatibility with
//! https://stackoverflow.com/a/11423057/829264
//! https://github.com/winlibs/libmcrypt/blob/master/modules/algorithms/blowfish-compat.c


use std::io::Read;
use std::io::Write;

// extern crate bswap;
extern crate blowfish;
use blowfish::Blowfish;
use blowfish::BlockCipher;
use blowfish::block_cipher_trait::InvalidKeyLength;
use blowfish::block_cipher_trait::generic_array::GenericArray;
use blowfish::block_cipher_trait::generic_array::typenum::{U8, U56};


/// Size (length) of word
pub(crate) const WORD: usize = 4;
/// Size (length) of chunk/block
pub(crate) const BLOCK: usize = 8;


/**
	Takes data and reverses byte order inplace to fit
	blowfish-compat format.
	```
	use wap_crypto::reverse_words;
	let mut s = "12345678".to_owned();
	reverse_words(unsafe { s.as_bytes_mut() });
	assert_eq!(&s, "43218765");
	```
*/
#[inline]
pub fn reverse_words(buf: &mut [u8]) {
	#[cfg(target_endian = "little")]
	// TODO: chunk by chunk where size is power of WORD but not huge or "bus-err/out of mem".
	// unsafe { bswap::u32::swap_memory_inplace(&mut buf[0] as *mut u8, buf.len()); }
	for chunk in buf.chunks_mut(WORD) {
		chunk.reverse();
	}
}

pub fn decrypt(buf: &mut [u8], key: &[u8]) -> Result<(), InvalidKeyLength> {
	// check size & padding:
	assert!(buf.len() % BLOCK == 0);

	let cypher = BlowfishCompat::new_varkey(key)?;
	for chunk in buf.chunks_mut(BLOCK) {
		cypher.decrypt_block(GenericArray::from_mut_slice(chunk));
	}
	Ok(())
}

pub fn decrypt_stream<R: Read, W: Write>(reader: &mut R, key: &[u8], out: &mut W) -> Result<(), std::io::Error> {
	let cypher = BlowfishCompat::new_varkey(key).unwrap();
	let mut buf = [0; BLOCK];
	loop {
		match reader.read_exact(&mut buf) {
			Ok(_) => {
				cypher.decrypt_block(GenericArray::from_mut_slice(&mut buf));
				// out.write(&buf)?;
				out.write_all(&buf)?;
			},
			Err(_err) => break,
		}
	}
	out.flush()
}

// copy of the private type-alias `blowfish::Block`.
type Block = GenericArray<u8, U8>;

#[derive(Clone, Copy)]
pub struct BlowfishCompat {
	inner: Blowfish,
}

impl BlockCipher for BlowfishCompat {
	type KeySize = <Blowfish as BlockCipher>::KeySize;
	type BlockSize = <Blowfish as BlockCipher>::BlockSize;
	type ParBlocks = <Blowfish as BlockCipher>::ParBlocks;

	fn new(key: &GenericArray<u8, U56>) -> Self { Self { inner: <Blowfish as BlockCipher>::new(key) } }

	fn new_varkey(key: &[u8]) -> Result<Self, InvalidKeyLength> {
		<Blowfish as BlockCipher>::new_varkey(key).map(|bf| Self { inner: bf })
	}

	#[inline]
	fn encrypt_block(&self, block: &mut Block) {
		reverse_words(block);
		self.inner.encrypt_block(block);
		reverse_words(block);
	}

	#[inline]
	fn decrypt_block(&self, block: &mut Block) {
		reverse_words(block);
		self.inner.decrypt_block(block);
		reverse_words(block);
	}
}
