#[cfg(all(test, any(ios, target_os = "ios")))]
extern crate dinghy_test;
extern crate wap_res;

#[path = "../../tests/test_data_paths.rs"]
mod test_data_paths;
use self::test_data_paths::*;

use std::fs::File;
use std::path::PathBuf;

use wap_res::pack::*;
use wap_res::node::NodeType;


#[inline]
fn open_pack(filename: &str) -> Result<OpenPackage<File>, std::io::Error> {
	let path = [get_dir(), PathBuf::from(filename)].iter().collect::<PathBuf>();
	let path = path.as_path();
	println!("read package from path: {:?}", path);

	let file = File::open(&path)?;
	let pack = Package::new(
	                        path.to_str()
	                            .expect(&format!("Can't open the package {:?}.", filename))
	                            .to_string(),
	                        PackageType::Extra,
	);
	Ok(pack.open_with(file))
}

#[inline]
fn read_pack(filename: &str) -> OpenPackage<File> {
	let mut pack = open_pack(filename).unwrap();
	read_package(&mut pack).expect(&format!("Can't read the package {:?}.", pack.uri()));
	pack
}

#[test]
fn package_empty() {
	let pack = open_pack(GRUNTZ_ZZZ).unwrap();
	let index = pack.index();
	assert!(index.nodes.is_empty());
	assert!(index.directories.is_empty());
}

#[test]
fn read_package_index_zzz() {
	let pack = read_pack(GRUNTZ_ZZZ);
	let index = pack.index();

	check_nodes_num(&index, 7, 5);
	check_uri_max_length(&index);
}

#[test]
fn read_package_index_rez() {
	let pack = read_pack(GRUNTZ_REZ);
	let index = pack.index();

	check_nodes_type(&index);
	check_nodes_num(&index, 21303, 1784);
	check_uri_max_length(&index);
}

#[test]
fn read_package_index_vrz() {
	let pack = read_pack(GRUNTZ_VRZ);
	let index = pack.index();

	check_nodes_num(&index, 1517, 58);
	check_uri_max_length(&index);
}

#[inline]
fn check_nodes_num(index: &Index, nodes: usize, dirs: usize) {
	assert_eq!(nodes, index.nodes.len());
	assert_eq!(dirs, index.directories.len());

	for (_path, filenames) in &index.directories {
		assert!(filenames.len() > 0);
	}
}

#[inline]
fn check_nodes_type(index: &Index) {
	use self::NodeType::*;
	for (path, node) in &index.nodes {
		let ext = &path[path.len() - 3..];
		let t = match ext {
			b"INA" => Ani,
			b"LAP" => Pal,
			b"DIP" => Pid,
			b"XCP" => Pcx,
			b"VAW" => Wav,
			b"DWW" => Wwd,
			b"IMX" => Xmi,
			b"TXT" => Txt,
			_ => Unknown,
		};
		assert_eq!(t, node.t);
	}
}


#[inline]
fn check_uri_max_length(index: &Index) {
	const MAX_LENGTH: usize = 70;

	for (path, _node) in &index.nodes {
		assert!(path.len() < MAX_LENGTH);
	}

	for (path, filenames) in &index.directories {
		assert!(path.len() < MAX_LENGTH);
		for filename in filenames {
			assert!(filename.len() < MAX_LENGTH);
		}
	}
}
