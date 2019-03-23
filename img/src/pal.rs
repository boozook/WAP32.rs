use std::io::{Read, Seek, SeekFrom};
use crate::res::node::Node;
use crate::pixels::format::*;
use crate::pixels::*;


type Result<T> = std::result::Result<T, std::io::Error>;


/// Read 256-colors palette from the end of buffer and wrap into the `Palette<RGB>`.
#[inline]
pub fn read_palette_256<'a, 'b, R: Seek + Read>(node: &'a Node, from: &'b mut R) -> Result<Palette<RGB>> {
	let mut palette = vec![0u8; 256 * 3];
	read_palette_256_to(&node, from, &mut palette)?;
	Ok(Palette::new(palette))
}

/// Read 256-colors palette from the end of buffer.
#[inline]
pub fn read_palette_256_to<'a, 'b, R: Seek + Read>(node: &'a Node, from: &'b mut R, to: &'b mut [u8]) -> Result<()> {
	if !(node.offset == 0 && node.size == 256 * 3) {
		// ommit seek for exactly pal-files
		from.seek(SeekFrom::Start((node.offset + (node.size - (256 * 3))) as u64))?;
	}
	from.read_exact(&mut to[..])
}
