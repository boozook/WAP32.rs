//! test WWD loaded from standalone file.wwd

use std::fs::File;

extern crate wap_res;
extern crate wap_wwd;
extern crate wap_utils;

use wap_res::node::*;
use wap_wwd::read_as_wwd;


static WWDS: [&str; 2] = ["tests/data/BombzAway!.wwd", "tests/data/GRUNTZREZ_AREA8_WORLDZ_LEVEL29.WWD"];


#[inline]
fn read_wwd_file(path: &str) -> Result<(), std::io::Error> {
	let mut file = File::open(path)?;
	let node = Node { offset: 0,
	                  size: std::fs::metadata(&path)?.len() as u32,
	                  t: NodeType::Wwd, };
	let wwd = read_as_wwd(&node, &mut file)?;
	assert!(wwd.0.len() > 0);

	// let name = unsafe { std::str::from_utf8_unchecked(&wwd.1.name) };
	// println!("{}\t : {}, tile-attrs: {}", path, name, wwd.0.len());

	Ok(())
}

#[test]
fn read_wwd_files() {
	for path in WWDS.iter() {
		read_wwd_file(path).unwrap();
	}
}
