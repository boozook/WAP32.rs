extern crate wap_crypto;

use wap_crypto::*;
use std::io::Read;
use std::fs::File;

include!("paths-inc.rs");


#[test]
fn decrypt_streammed() {
	test_decrypt_stream(ATTRS, ATTRS_KEY, ATTRS_DE).unwrap();
	test_decrypt_stream(CHEATZ, CHEATZ_KEY, CHEATZ_DE).unwrap();
}

#[test]
fn decrypt_inplace() {
	test_decrypt_inplace(ATTRS, ATTRS_KEY, ATTRS_DE).unwrap();
	test_decrypt_inplace(CHEATZ, CHEATZ_KEY, CHEATZ_DE).unwrap();
}


fn test_decrypt_inplace(file_path: &str, key: &[u8], expected_file_path: &str) -> Result<(), std::io::Error> {
	let mut buf = Vec::new();
	let mut f = File::open(file_path)?;

	f.read_to_end(&mut buf)?;
	add_padding(&mut buf);

	decrypt(&mut buf, key).unwrap();

	drain_tail(&mut buf);

	cmp_slice_file(&buf, expected_file_path)
}


fn test_decrypt_stream(file_path: &str, key: &[u8], expected_file_path: &str) -> Result<(), std::io::Error> {
	use std::io::Cursor;
	let mut inp = File::open(file_path)?;
	let mut out = Cursor::new(Vec::new());

	decrypt_stream(&mut inp, key, &mut out)?;

	let result = out.into_inner();
	cmp_slice_file(&result, expected_file_path)
}


#[inline]
fn cmp_slice_file(buf: &[u8], file_path: &str) -> Result<(), std::io::Error> {
	let mut expected = Vec::new();
	{
		let mut f = File::open(file_path)?;
		f.read_to_end(&mut expected)?;
	}
	assert_eq!(expected.len(), buf.len());
	assert_eq!(expected, buf);
	Ok(())
}
