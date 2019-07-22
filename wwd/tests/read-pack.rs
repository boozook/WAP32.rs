//! test WWDs loaded from Pack
extern crate wap_res as res;
extern crate wap_wwd as wwd;
extern crate wap_common as common;

#[path = "../../tests/test_data_paths.rs"]
mod test_data_paths;
use self::test_data_paths::*;

use std::fs::File;
use std::path::PathBuf;

use wwd::read_as_wwd;

use res::*;
use res::pack::*;
use res::pack::*;


#[inline]
fn open_package(filename: &str, kind: PackageType) -> Result<OpenPackage<File>, std::io::Error> {
	let path = [get_dir(), PathBuf::from(filename)].iter().collect::<PathBuf>();
	let path = path.as_path();
	println!("read package from path: {:?}", path);

	let file = File::open(&path)?;
	let pack = Package::new(
	                        path.to_str()
	                            .expect(&format!("Can't open the package {:?}.", filename))
	                            .to_string(),
	                        kind,
	);
	Ok(pack.open_with(file))
}


#[test]
fn read_wwd_package_zzz() {
	let mut pack = open_package(GRUNTZ_ZZZ, PackageType::Main).unwrap();
	read_package(&mut pack).expect(&format!("Can't read the package {:?}.", pack.uri()));

	let (pack, mut stream) = pack.split();
	let index = pack.index();

	for (_path, node) in index.nodes() {
		let wwd = read_as_wwd(&node, &mut stream).unwrap();
		assert!(wwd.0.len() > 0);

		// let path = unsafe { std::str::from_utf8_unchecked(&_path) };
		// let name = unsafe { std::str::from_utf8_unchecked(&wwd.1.name) };
		// println!("{}\t : {}, tile-attrs: {}", path, name, wwd.0.len());
	}
}
