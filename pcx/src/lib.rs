// PCX ver. 3.0
// https://github.com/kryptan/pcx/blob/master/src/reader.rs
// https://docs.rs/pcx/0.2.0/pcx/
// https://crates.io/crates/pcx

extern crate pcx;

use std::io::{prelude::*, SeekFrom, Take};

extern crate wap_res;
use wap_res::node::Node;

extern crate wap_img;
use wap_img::pixels::*;

// use pm::NodePointer;


type Result<T> = std::result::Result<T, std::io::Error>;

type Pcx<T> = PxBufSized<format::RGB, T>;


// TODO: КОГДА NodePointer ДОСТАВЯТ
// impl<'a, 'b, R: Seek + Read> ReadNode<Pcx<u8>, &'b mut R> for NodePointer<'a, 'b, R> {
// 	fn read(&mut self) -> Result<Pcx<u8>> { read_as_pcx(&self.0, &mut self.1) }
// }

// TODO: КОГДА NodePointer ДОСТАВЯТ
// impl<'a, 'b, R: Seek + Read> ReadNode<Pcx<u32>, &'b mut R> for NodePointer<'a, 'b, R> {
// 	fn read(&mut self) -> Result<Pcx<u32>> {
// 		let rgb = read_as_pcx(&self.0, &mut self.1)?;
// 		Ok(rgb.into())
// 	}
// }


#[inline]
pub fn reader_for_pcx<'a, 'b, R: Seek + Read>(node: &'a Node, r: &'b mut R) -> Result<pcx::Reader<Take<&'b mut R>>> {
	r.seek(SeekFrom::Start(node.offset as u64))?;
	pcx::Reader::new(r.take(node.size as u64))
}

/// Returns __RGB__-buffer with size.
pub fn read_as_pcx<'a, 'b, R: Seek + Read>(node: &'a Node, r: &'b mut R) -> Result<Pcx<u8>> {
	r.seek(SeekFrom::Start(node.offset as u64))?;
	// Take needed for paletted because the end needed there
	let mut pcx = pcx::Reader::new(r.take(node.size as u64))?;
	// Infinite stream is ok for non-paletted pcx:
	// let mut pcx = pcx::Reader::new(from)?;

	let (width, height) = (pcx.width() as u32, pcx.height() as u32);

	let buffer = if pcx.is_paletted() {
		// Create a new img-buf with width and height
		let mut indexes = vec![0u8; (width * height) as usize];

		// read row-by-row:
		for y in 0..height {
			pcx.next_row_paletted(&mut indexes[(y * width) as usize..((y * width) + width) as usize])?;
		}

		// palette:
		let mut palette = [0; 256 * 3];
		pcx.read_palette(&mut palette)?;

		// get colours:
		let mut img_raw = vec![0u8; (width * height * 3) as usize];
		for v in indexes.iter() {
			let start = (*v as usize * 3) - 1;
			let end = start + 3;
			img_raw.extend(&palette[start..end]);
		}
		img_raw
	} else {
		// Create a new img-buf with width and height
		let mut img_raw = vec![0u8; (width * height * 3) as usize];

		// read row-by-row:
		for y in 0..height {
			let start = y * (width * 3);
			let end = start + width * 3;
			pcx.next_row_rgb(&mut img_raw[start as usize..end as usize])?;
		}
		img_raw
	};
	Ok(PxBufSized::new(buffer, width, height))
}
