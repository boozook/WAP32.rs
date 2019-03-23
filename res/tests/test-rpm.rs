#[cfg(all(test, any(ios, target_os = "ios")))]
extern crate dinghy_test;
extern crate wap_res;

#[path = "../../tests/test_data_paths.rs"]
mod test_data_paths;
use self::test_data_paths::*;

use std::fs::File;
use std::path::{Path, PathBuf};

use wap_res::*;
use wap_res::node::NodeType;
use wap_res::pack::*;
use wap_res::rpm::*;


#[inline]
fn create_package(filename: &str, kind: PackageType) -> Package {
	let path = [get_dir(), PathBuf::from(filename)].iter().collect::<PathBuf>();
	let path = path.as_path();
	let pack = Package::new(
	                        path.to_str()
	                            .expect(&format!("Can't open the package {:?}.", filename))
	                            .to_string(),
	                        kind,
	);
	pack
}

fn open_package(pack: Package) -> Result<OpenPackage<File>, std::io::Error> {
	let path = PathBuf::from(pack.uri());
	let path = path.as_path();
	println!("read package from path: {:?}", path);

	let file = File::open(&path)?;
	Ok(pack.open_with(file))
}

fn create_rpm_builder() -> RpmBuilder {
	let main = create_package(GRUNTZ_REZ, PackageType::Main);
	let patch = create_package(GRUNTZ_ZZZ, PackageType::Patch);
	let voice = create_package(GRUNTZ_VRZ, PackageType::Voice);
	let mut builder = RpmBuilder::new(main);
	builder.add_pack(patch);
	builder.add_pack(voice);
	builder
}


#[test]
fn rpm_builder() {
	//
	let mut rpm = create_rpm_builder().build();

}
