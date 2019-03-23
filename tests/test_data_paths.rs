#![allow(dead_code)]

#[cfg(all(test, any(ios, target_os = "ios")))]
extern crate dinghy_test;

#[cfg(any(ios, target_os = "ios"))]
mod data_paths {
	use std::path::PathBuf;
	extern crate dinghy_test;
	use dinghy_test as dinghy;

	pub static GRUNTZ_REZ: &'static str = "GRUNTZ_REZ";
	pub static GRUNTZ_ZZZ: &'static str = "GRUNTZ_ZZZ";
	pub static GRUNTZ_VRZ: &'static str = "GRUNTZ_VRZ";

	pub fn get_dir() -> PathBuf { dinghy::test_project_path().join("test_data") }
	pub fn get_all() -> [PathBuf; 3] { [get_rez(), get_zzz(), get_vrz()] }
	pub fn get_rez() -> PathBuf { dinghy::test_file_path(GRUNTZ_REZ) }
	pub fn get_zzz() -> PathBuf { dinghy::test_file_path(GRUNTZ_ZZZ) }
	pub fn get_vrz() -> PathBuf { dinghy::test_file_path(GRUNTZ_VRZ) }
}

#[cfg(not(target_os = "ios"))]
mod data_paths {
	use std::path::PathBuf;

	static RES_DIR_PATH: &'static str = include!("data/GRUNTZ_REZ_DIR_PATH");
	pub static GRUNTZ_REZ: &'static str = "GRUNTZ.REZ";
	pub static GRUNTZ_ZZZ: &'static str = "GRUNTZ.ZZZ";
	pub static GRUNTZ_VRZ: &'static str = "GRUNTZ.VRZ";

	pub fn get_dir() -> PathBuf { PathBuf::from(RES_DIR_PATH) }
	pub fn get_all() -> [PathBuf; 3] { [get_rez(), get_zzz(), get_vrz()] }
	// pub fn get_all() -> [PathBuf; 2] { [get_rez(), get_zzz()] }
	pub fn get_rez() -> PathBuf { [RES_DIR_PATH, GRUNTZ_REZ].iter().collect() }
	pub fn get_zzz() -> PathBuf { [RES_DIR_PATH, GRUNTZ_ZZZ].iter().collect() }
	pub fn get_vrz() -> PathBuf { [RES_DIR_PATH, GRUNTZ_VRZ].iter().collect() }
}

pub use self::data_paths::*;
