extern crate zero;

use std::io::Error as IoError;
use std::io::{prelude::*, SeekFrom};

use common::utils::bytes::*;
use common::utils::size_of;
use common::reader::*;

use crate::uri::{self, Uri, UriRef};
use crate::index::*;
use crate::node::*;
use self::ds::*;


/* NOTE:
	Max length of any URI < 70 bytes,
	so there we can store URIs in `[u8; 70]` for opt.
*/


pub struct PackReader {}

impl<R: Read + Seek> Reader<R, Index> for PackReader {
	type Error = IoError;

	#[inline]
	fn read(&mut self, from: &mut R) -> Result<Index, Self::Error> {
		let mut index = Index::new();
		self.read_to(from, &mut index).map(|_| index)
	}
}

impl<R: Read + Seek> ReaderTo<R, Index> for PackReader {
	type Error = IoError;
	#[inline]
	fn read_to(&mut self, from: &mut R, to: &mut Index) -> Result<(), Self::Error> { read_to(from, to) }
}


pub fn read_to<R: Read + Seek>(from: &mut R, to: &mut Index) -> Result<(), IoError> {
	let reader = from;
	let index = to;

	// package header:
	let mut temp = [0u8; 256];
	reader.read_exact(&mut temp)?;
	// TODO: share up full head or head.text() only:
	let head = zero::read::<Header>(&temp);

	unsafe { debug_assert_eq!(1, head.version) };

	// pos of root:
	let root = zero::read::<NodePosition>(&temp[size_of::<Header>()..]);
	// misc/meta info:
	// TODO: know what is in the `unknown`
	let _unknown = zero::read::<HeaderUnknown>(&temp[size_of::<Header>() + size_of::<NodePosition>()..]);

	// children storage:
	// type DirNodes<'a> = Vec<(&'a [u8], u32, u32)>;
	type DirNodes = Vec<(Vec<u8>, u32, u32)>;
	let mut dirs: DirNodes = vec![(vec![], root.offset, root.size)];
	let delimiter = &[uri::DELIMITER][..];

	while let Some(dir) = dirs.pop() {
		let mut files: Vec<Vec<u8>> = Vec::new();

		// seek & read:
		reader.seek(SeekFrom::Start(dir.1 as u64))?;
		let mut buf = vec![0u8; dir.2 as usize];
		reader.read_exact(&mut buf)?;


		// parse nodes:
		let mut offset = 0;
		while offset < buf.len() {
			let head = zero::read::<NodeHead>(&buf[offset..]);
			offset += 16; // size_of::<NodeHead>()

			let is_empty = head.position.size == 0;
			let is_dir = head.directory;

			// TODO: research part:
			// File ID or Unix timestamp?
			// let mut asset_id = [0u8; 4];
			let extension/* : [u8; 4] */ = if !is_dir {
				// let id = &buf[offset..offset + 4];
				// asset_id = [id[0], id[1], id[2], id[3]];

				let extension = &buf[offset + 4..offset + 4 + 4];

				// asset_id + ext0 + ditch null:
				offset += 4 + 4 + 4;
				read_to_zero(extension)
			} else {
				&[]
			};

			let name = read_to_zero(&buf[offset..]);

			// length of name + zero null byte:
			offset += name.len() + 1;

			// save:
			if !is_empty {
				if is_dir {
					let dir_name = [&dir.0, name, delimiter].concat();
					dirs.push((dir_name, head.position.offset, head.position.size));
				} else {
					let node_file_name = [name, b".", extension].concat();
					let uri = [dir.0.as_slice(), &node_file_name].concat();
					files.push(node_file_name);
					index.nodes.insert(
					                   uri,
					                   Node { // id: asset_id,
					                          //   t: type_from_ext(&extension),
					                          offset: head.position.offset,
					                          size: head.position.size, },
					);
				}
			}

			if !is_dir {
				// ditch trailing null byte:
				offset += 1;
			}
		}

		// save:
		if files.len() > 0 {
			let key = dir.0;
			index.directories.insert(key, files);
		}
	}

	Ok(())
}

pub fn read_ref_to<R: Read + Seek>(from: &mut R, push_node: fn(d: UriRef, f: UriRef, Node) -> (),
                                   push_dir: fn(uri: UriRef, files: &[Uri]) -> ())
                                   -> Result<(), IoError>
{
	let reader = from;

	// package header:
	let mut temp = [0u8; 256];
	reader.read_exact(&mut temp)?;
	// TODO: share up full head or head.text() only:
	let head = zero::read::<Header>(&temp);

	unsafe { debug_assert_eq!(1, head.version) };

	// pos of root:
	let root = zero::read::<NodePosition>(&temp[size_of::<Header>()..]);
	// misc/meta info:
	// TODO: know what is in the `unknown`
	let _unknown = zero::read::<HeaderUnknown>(&temp[size_of::<Header>() + size_of::<NodePosition>()..]);

	// children storage:
	// type DirNodes<'a> = Vec<(&'a [u8], u32, u32)>;
	type DirNodes = Vec<(Vec<u8>, u32, u32)>;
	let mut dirs: DirNodes = vec![(vec![], root.offset, root.size)];
	let delimiter = &[uri::DELIMITER][..];

	while let Some(dir) = dirs.pop() {
		let mut files: Vec<Vec<u8>> = Vec::new();

		// seek & read:
		reader.seek(SeekFrom::Start(dir.1 as u64))?;
		let mut buf = vec![0u8; dir.2 as usize];
		reader.read_exact(&mut buf)?;


		// parse nodes:
		let mut offset = 0;
		while offset < buf.len() {
			let head = zero::read::<NodeHead>(&buf[offset..]);
			offset += 16; // size_of::<NodeHead>()

			let is_empty = head.position.size == 0;
			let is_dir = head.directory;

			// TODO: research part:
			// File ID or Unix timestamp?
			// let mut asset_id = [0u8; 4];
			let extension/* : [u8; 4] */ = if !is_dir {
				// let id = &buf[offset..offset + 4];
				// asset_id = [id[0], id[1], id[2], id[3]];

				let extension = &buf[offset + 4..offset + 4 + 4];

				// asset_id + ext0 + ditch null:
				offset += 4 + 4 + 4;
				read_to_zero(extension)
			} else {
				&[]
			};

			let name = read_to_zero(&buf[offset..]);

			// length of name + zero null byte:
			offset += name.len() + 1;

			// save:
			if !is_empty {
				if is_dir {
					let dir_name = [&dir.0, name, delimiter].concat();
					dirs.push((dir_name, head.position.offset, head.position.size));
				} else {
					let node_file_name = [name, b".", extension].concat();
					let node = Node { // id: asset_id,
					                  //   t: type_from_ext(&extension),
					                  offset: head.position.offset,
					                  size: head.position.size };
					push_node(&dir.0, &node_file_name, node);
					files.push(node_file_name);
				}
			}

			if !is_dir {
				// ditch trailing null byte:
				offset += 1;
			}
		}

		// save:
		if files.len() > 0 {
			let key = dir.0;
			push_dir(&key, &files);
		}
	}

	Ok(())
}


pub(crate) mod ds {
	use zero::Pod;
	use std::fmt;
	use std::str::Utf8Error;

	#[repr(C, packed)]
	pub struct Header {
		pub head: [u8; 126],
		__gap: u8,
		pub version: u32,
	}
	unsafe impl Pod for Header {}
	impl Header {
		pub fn text(&self) -> Result<&str, Utf8Error> { Ok(std::str::from_utf8(&self.head)?.trim()) }
	}
	impl fmt::Debug for Header {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "Header[{}, {:?}]", unsafe { &self.version }, &self.text())
		}
	}

	#[repr(C, packed)]
	/// https://github.com/cubuspl42/Gruntz-REZ-Patcher/blob/master/CREZ.cpp#L423-L435
	pub struct HeaderUnknown {
		//uint32 {4}(b139)   - Unknown (1C FD 12 00(hex) - 1244444(dec))
		__unknown_0: u32,
		//uint32 {4}(b143)   - Offset to Last Dir (?? What last dir??)
		// [REZ, VRZ, ZZZ] = [75552553, 90691075, 95494]
		__unknown_1: u32,
		//uint32 {4}(b147)   - Unknown (GRUNTZ - 3E 3B 91 36/915487550; CLAW - 51 68 CA 33/868902993)
		__unknown_2: u32,
		//uint32 {4}   - null
		__null_0: u32,
		//uint32 {4}(b155)   - Unknown (GR-15 00 00 00/21; CL-13 00 00 00/19)
		__unknown_3: u32,
		//uint32 {4}(b159)   - Unknown (GR-19 00 00 00/25; CL-18 00 00 00/24)
		__unknown_4: u32,
		//uint32 {4}   - null
		__null_1: u32,
		//uint32 {4}(b167)   - Unknown (01 20 00 00/8193)
		__unknown_5: u32,
		//byte {13}    - null
		__null_2: [u8; 14],
	}
	unsafe impl Pod for HeaderUnknown {}

	#[repr(C, packed)]
	pub struct NodePosition {
		// node offset and size:
		pub offset: u32,
		pub size: u32,
	}
	unsafe impl Pod for NodePosition {}

	#[repr(C, packed)]
	pub struct NodeHead {
		pub directory: bool,   // one byte
		__gap_directory: bool, // null
		__gap_null: u16,       // null

		pub position: NodePosition,

		// unix date-time:
		__gap: i32,
	}
	unsafe impl Pod for NodeHead {}


	#[cfg(test)]
	mod tests {
		use super::*;
		use common::utils::size_of;

		#[test]
		fn struct_sizes() {
			assert_eq!(size_of::<Header>(), 131);
			assert_eq!(size_of::<NodeHead>(), 16);
			assert_eq!(size_of::<NodePosition>(), 8);
		}
	}

}
