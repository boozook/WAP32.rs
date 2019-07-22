extern crate wap_img as img;
extern crate wap_pid as pid;
extern crate wap_res as res;
#[cfg(feature = "image")]
extern crate image;

use img::point::*;
use img::pixels::*;
use img::pixels::{format, Pxls};
use pid::pid::{Pid, read_as_pid};
use res::node::Node;

mod paths {
	pub const PID_PAL: &str = "tests/data/PERFECT.PID";
	pub const PID_PAL_RLE: &str = "tests/data/FRAME043.PID";
	pub const PID_LIGHTS: &str = "tests/data/FRAME003.PID";
	pub static PID_ALL: [&str; 3] = [PID_PAL, PID_PAL_RLE, PID_LIGHTS];
}


#[inline]
fn read_pid_file(path: &str) -> Result<Pid<format::RGB>, std::io::Error> {
	use std::fs::File;
	let mut file = File::open(path)?;
	let node = Node { offset: 0,
	                  size: std::fs::metadata(&path)?.len() as u32 /* t: Format::Pid */ };
	read_as_pid(&node, &mut file)
}


#[test]
fn pid_no_rle() {
	let pid = read_pid_file(paths::PID_PAL).unwrap();
	assert_eq!([252, 54], pid.info.size);
	assert_eq!([2, 3], pid.info.offset);
	assert!(pid.info.transparent);
	assert!(!pid.info.lights);

	assert!(pid.palette.is_some());
	assert_eq!(256 * 3, pid.palette.unwrap().pixels().len())
}

#[test]
fn pid_rle() {
	let pid = read_pid_file(paths::PID_PAL_RLE).unwrap();
	assert_eq!([64, 65], pid.info.size);
	assert_eq!([-6, -21], pid.info.offset);
	assert!(pid.info.transparent);
	assert!(!pid.info.lights);

	assert!(pid.palette.is_some());
	assert_eq!(256 * 3, pid.palette.unwrap().pixels().len())
}

#[test]
fn pid_lights() {
	let pid = read_pid_file(paths::PID_LIGHTS).unwrap();
	assert_eq!([100, 100], pid.info.size);
	assert_eq!([0, 0], pid.info.offset);
	assert!(pid.info.transparent);
	assert!(pid.info.lights);

	assert!(pid.palette.is_none());
}


#[test]
fn pid_to_image() -> Result<(), std::io::Error> {
	for path in paths::PID_ALL.iter() {
		let pid = read_pid_file(path)?;

		assert_eq!(pid.lightmap(), pid.palette.is_none());

		let size = *pid.size();

		let buffer = {
			let mut buf = Vec::with_capacity((*size.width() * *size.height()) as usize);
			let indices = pid.indices.into_inner();
			assert_eq!(size.width() * size.height(), indices.len() as u32);

			if pid.palette.is_none() {
				for i in indices.iter().cloned() {
					buf.extend_from_slice(&[i, i, i]);
				}
			} else {
				let palette = pid.palette.unwrap().into_inner();
				for i in indices.iter() {
					let pos = *i as usize * 3;
					buf.extend_from_slice(&palette[pos..pos + 3]);
				}
			}
			buf
		};

		assert!(buffer.iter().max().unwrap() > &0);
	}

	assert!(true);
	Ok(())
}


#[test]
#[cfg(feature = "image")]
fn pid_build_image() -> Result<(), std::io::Error> {
	use image::{ImageBuffer, Rgb, Luma};
	use wap_pid::pid::BuildImage;

	for path in paths::PID_ALL.iter() {
		let pid = read_pid_file(path)?;
		println!("loaded: {}", path);

		let img = if pid.palette.is_some() {
			let imgbuf: ImageBuffer<Rgb<u8>, _> = pid.build_image();
			image::ImageRgb8(imgbuf)
		} else {
			let size = *pid.size();
			let imgbuf =
				ImageBuffer::<Luma<u8>, _>::from_raw(*size.width(), *size.height(), pid.indices.into_inner()).unwrap();
			image::ImageLuma8(imgbuf)
		};
		save_image(path, img);
	}

	assert!(true);
	Ok(())
}


#[cfg(feature = "image")]
fn save_image(path: &str, img: image::DynamicImage) {
	use std::fs::File;
	use std::path::Path;
	let fp = Path::new(path).with_extension("png");
	let filename = fp.file_name().unwrap().to_str().unwrap();
	let export_path = filename;
	println!("export: {:?}", export_path);
	let ref mut out = File::create(export_path).unwrap();
	match img.write_to(out, image::PNG) {
		Err(err) => println!("ERROR!: {:?}", err),
		_ => {},
	}
}
