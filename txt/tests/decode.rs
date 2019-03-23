extern crate wap_crypto;
extern crate gruntz_txt;

use wap_crypto::*;
use gruntz_txt::*;
use std::io::Read;
use std::fs::File;
use std::io::Cursor;

include!("paths-inc.rs");


#[test]
fn decrypt_decode_gruntz_txt() {
	test_decrypt_gruntz(ATTRS, ATTRS_KEY, ATTRS_DE).unwrap();
	test_decrypt_gruntz(CHEATZ, CHEATZ_KEY, CHEATZ_DE).unwrap();
}


fn test_decrypt_gruntz(file_path: &str, key: &[u8], expected_file_path: &str) -> Result<(), std::io::Error> {
	let mut inp = File::open(file_path)?;
	let mut out = Cursor::new(Vec::new());

	decrypt_stream(&mut inp, key, &mut out)?;

	let mut result = out.into_inner();

	// extra fix for Gruntz:
	decode_cheatz(&mut result);

	cmp_slice_file(&result, expected_file_path)?;
	test_parse_ini(&result);

	Ok(())
}


#[inline]
fn test_parse_ini(buf: &[u8]) {
	assert!(parse_ini(buf).is_ok());
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
