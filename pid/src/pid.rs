//! Looks like PCX ver. 2.5 image data and have same as in the DIB.
//! DIB spec: https://www.aelius.com/njh/wavemetatools/doc/riffmci.pdf (search there: Device-Independent Bitmap (DIB))

use std::io::{prelude::*, Seek, SeekFrom};

extern crate zero;

use crate::res::node::Node;
use crate::utils::size_of;
use img::pixels::*;
use img::pixels::format::*;
use img::point::*;
use img::pal::*;


type Result<T> = std::result::Result<T, std::io::Error>;


#[derive(Debug)]
pub struct PidInfo {
	/// Raw point - `[x, y]`.
	pub offset: RawPoint<i32>,
	/// Raw point - `[width, height]`.
	pub size: RawPoint<u32>,

	// WAP32 standard flags:
	pub video_memory: bool,
	pub system_memory: bool,
	pub mirror: bool,
	pub invert: bool,

	/// meta: lightmap
	pub lights: bool,
	/// meta: blitting by mask
	pub transparent: bool,
}

#[derive(Debug)]
struct PidInfoInternal {
	info: PidInfo,
	embedded_palette: bool,

	/// Bynary RLE comression.
	rle: bool,
}

#[derive(Debug)]
pub struct Pid<PalFmt: PxFmt> {
	pub info: PidInfo,
	pub indices: Indices,
	pub palette: Option<Palette<PalFmt>>,
}

impl<F: PxFmt> Pxls<u8> for Pid<F> {
	fn pixels(&self) -> &[u8] { self.indices.pixels() }
}

impl<F: PxFmt> PxlsSized<u8> for Pid<F> {
	fn size(&self) -> &RawPoint<u32> { &self.info.size }
	fn offset(&self) -> &RawPoint<i32> { &self.info.offset }
}

impl<F: PxFmt> PxMeta for Pid<F> {
	fn mirror(&self) -> bool { self.info.mirror }
	fn invert(&self) -> bool { self.info.invert }
	fn lightmap(&self) -> bool { self.info.lights }
	fn transparent(&self) -> bool { self.info.transparent }
}


/// Read image in PID format.
/// Indices: are one byte per index.
/// Palette: RGB in tree bytes per color.
pub fn read_as_pid<'a, 'b, T: Seek + Read>(node: &'a Node, from: &'b mut T) -> Result<Pid<RGB>> {
	// head/meta:
	let info = read_pid_header(node, from)?;
	let (width, height) = (info.info.size[0], info.info.size[1]);

	// Index pointing to "EMPTY" color:
	// TODO: API for setting empty color index (`EMPTY_INDEX`).
	const EMPTY_INDEX: u8 = 0;


	// indices:
	let indices = {
		// TODO: opt: use unsafe alloc instead `push(0)` repeated `w * h` times!
		let mut indices = vec![EMPTY_INDEX; (width * height) as usize];
		read_indices_to(node, from, &mut indices, &info)?;
		Indices::new(indices)
	};

	// palette:
	let palette: Option<_> = if info.embedded_palette {
		Some(read_palette_256(node, from)?)
	} else {
		None
	};

	Ok(Pid { info: info.info,
	         palette,
	         indices, })
}


// ----------- Read Header ----------- //

#[inline]
/// Seek and read header.
fn read_pid_header<'a, 'b, T: Seek + Read>(node: &'a Node, from: &'b mut T) -> Result<PidInfoInternal> {
	from.seek(SeekFrom::Start(node.offset as u64))?;
	read_pid_header_noseek(node, from)
}

#[inline]
/// Read header without seek.
fn read_pid_header_noseek<'a, 'b, T: Seek + Read>(_node: &'a Node, from: &'b mut T) -> Result<PidInfoInternal> {
	use self::ds::*;

	let head = {
		let mut buf = vec![0u8; size_of::<ds::Head>()];
		from.read_exact(&mut buf)?;
		zero::read::<ds::Head>(&buf).to_owned()
	};

	// WAP32 standard flags:
	let flags = head.flags;
	let transparent = flags_has!(flags, WAP_PID_FLAG::TRANSPARENCY);
	let video_memory = flags_has!(flags, WAP_PID_FLAG::VIDEO_MEMORY);
	let system_memory = flags_has!(flags, WAP_PID_FLAG::SYSTEM_MEMORY);
	let mirror = flags_has!(flags, WAP_PID_FLAG::MIRROR);
	let invert = flags_has!(flags, WAP_PID_FLAG::INVERT);
	let rle = flags_has!(flags, WAP_PID_FLAG::COMPRESSION);
	let lights = flags_has!(flags, WAP_PID_FLAG::LIGHTS);
	let embedded_palette = flags_has!(flags, WAP_PID_FLAG::EMBEDDED_PALETTE);

	let (size, offset) = (head.size, head.offset);
	let info = PidInfo { size,
	                     offset,
	                     mirror,
	                     invert,
	                     lights,
	                     transparent,
	                     video_memory,
	                     system_memory, };
	let result = PidInfoInternal { rle,
	                               info,
	                               embedded_palette, };
	Ok(result)
}


// ----------- Read Indices ----------- //

/// Reads indixes. Indices points to colors in a palette.
/// Empty colors (transparent or COLOR_MASK) is skipped.
/// So if we want use specified "empty color" we should prefill `indices`.
fn read_indices_to<'a, 'b, 'c, 'd, T: Seek + Read>(node: &'a Node, from: &'b mut T, to: &'d mut [u8],
                                                   info: &'c PidInfoInternal)
                                                   -> Result<()>
{
	let (width, height) = (info.info.size[0], info.info.size[1]);
	debug_assert_eq!((width * height) as usize, to.len());

	let mut n = 0;
	let mut x = 0;
	let mut y = 0;
	let head_size = size_of::<ds::Head>();
	let buf_size = usize::min((width * height) as usize, node.size as usize - head_size);
	let mut buf = vec![0; buf_size];
	from.seek(SeekFrom::Start((node.offset + head_size as u32) as u64))?;
	from.read_exact(&mut buf)?;

	if info.rle {
		while y < height {
			debug_assert!(n < buf.len());

			let b = buf[n];
			n += 1;

			if b > 128 {
				let mut i = b as i16 - 128;
				while i > 0 && y < height {
					i -= 1;

					// NOTE: skip the call beacause the empty
					// let px_i = (x + (y * width)) as usize;
					// indices[px_i] = set_empty(px_i, x, y);

					x += 1;
					if x == width {
						x = 0;
						y += 1;
					}
				}
			} else {
				let mut i = b as i32;
				while i > 0 && y < height {
					i -= 1;

					let b = buf[n];
					n += 1;

					let px_i = (x + (y * width)) as usize;
					to[px_i] = b;

					x += 1;
					if x == width {
						x = 0;
						y += 1;
					}
				}
			}
		}
	} else {
		while y < height {
			debug_assert!(n < buf.len());

			let mut b = buf[n];
			n += 1;
			let mut i: i32;
			if b > 192 {
				i = b as i32 - 192;
				b = buf[n];
				n += 1;
			} else {
				i = 1;
			}

			while i > 0 && y < height {
				i -= 1;

				let px_i = (x + (y * width)) as usize;
				to[px_i] = b;

				x += 1;
				if x == width {
					x = 0;
					y += 1;
				}
			}
		}
	}
	Ok(())
}


/// internal ll data-structs
pub(crate) mod ds {
	use super::*;
	use super::zero::Pod;


	#[allow(dead_code)]
	#[allow(non_snake_case)]
	#[allow(non_camel_case_types)]
	pub mod WAP_PID_FLAG {
		pub static TRANSPARENCY: u32 = 1 << 0;
		pub static VIDEO_MEMORY: u32 = 1 << 1;
		pub static SYSTEM_MEMORY: u32 = 1 << 2;
		pub static MIRROR: u32 = 1 << 3;
		pub static INVERT: u32 = 1 << 4;
		pub static COMPRESSION: u32 = 1 << 5;
		pub static LIGHTS: u32 = 1 << 6;
		pub static EMBEDDED_PALETTE: u32 = 1 << 7;
	}


	#[derive(Debug, Copy, Clone)]
	#[repr(C, packed)]
	pub struct Head {
		/// MAgic byte like in PCX.
		/// But it's not a PCX!
		magic_byte: u32,

		/// WAP_PID_FLAG
		pub flags: u32,

		/// width, height
		pub size: RawPoint<u32>,
		/// x, y
		pub offset: RawPoint<i32>,

		__pad: [u32; 2], // two nulls
	}
	unsafe impl Pod for Head {}


	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn struct_sizes() {
			assert_eq!(32, size_of::<Head>());
		}
	}
}


pub trait BuildImage<Target> {
	// fn to_image(self) -> Target;
	fn build_image(&self) -> Target;
	// fn build_image_with_palette(&self, pal: Palette) -> Target;
}


// optional support //


impl BuildImage<PxBufSized<RGB, u8>> for Pid<RGB> {
	fn build_image(&self) -> PxBufSized<RGB, u8> {
		let size = *self.size();
		let mut buf = Vec::with_capacity((*size.width() * *size.height()) as usize);
		let indices = self.indices.as_inner();
		if let Some(ref palette) = self.palette {
			let palette = palette.as_inner();
			debug_assert_eq!(size.width() * size.height(), indices.len() as u32);
			for i in indices.iter() {
				let pos = *i as usize * 3;
				buf.extend_from_slice(&palette[pos..pos + 3]);
			}
			PxBufSized::new(buf, *size.width(), *size.height())
		} else {
			panic!("Pid without palette -> Image<GRB> does not implemented yet.");
		}
	}
}


#[cfg(feature = "image")]
extern crate image;
#[cfg(feature = "image")]
pub use self::image_sup::*;
#[cfg(feature = "image")]
mod image_sup {
	use super::*;
	use image::{ImageBuffer, Rgb};

	impl BuildImage<ImageBuffer<Rgb<u8>, Vec<u8>>> for Pid<RGB> {
		fn build_image(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
			let size = *self.size();
			// COLORIZE:
			// TODO: move out colorization to standalone method
			let mut buf = Vec::with_capacity((*size.width() * *size.height()) as usize);
			let indices = self.indices.as_inner();
			if let Some(ref palette) = self.palette {
				let palette = palette.as_inner();
				debug_assert_eq!(size.width() * size.height(), indices.len() as u32);
				for i in indices.iter() {
					let pos = *i as usize * 3;
					buf.extend_from_slice(&palette[pos..pos + 3]);
				}
				ImageBuffer::<Rgb<u8>, _>::from_raw(*size.width(), *size.height(), buf).unwrap()
			} else {
				panic!("Pid without palette -> Image<GRB> does not implemented yet.");
			}
		}
	}
}
