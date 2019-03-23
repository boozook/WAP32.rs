#![cfg(test)]
#![feature(test)]

use std::fs::File;
use std::path::PathBuf;
// use std::process::Termination;

extern crate test;
use self::test::{Bencher, black_box};

extern crate wap_res;
use wap_res::{Package, PackageReader};

#[path = "../../tests/test_data_paths.rs"]
mod test_data_paths;
use self::test_data_paths::*;


#[inline]
fn create_package_reader(filename: &str) -> PackageReader<File> {
	let path = [get_dir(), PathBuf::from(filename)].iter().collect::<PathBuf>();
	PackageReader::new_with_path(&path).expect(&format!("Can not create the package {:?}.", filename))
}


#[bench]
fn read_package_index(b: &mut Bencher) {
	let mut reader = create_package_reader(GRUNTZ_REZ);
	b.iter(|| {
		 black_box(reader.read().unwrap());
		});
	black_box(reader.pack);
}
