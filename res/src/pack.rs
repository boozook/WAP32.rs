use std::io::{prelude::*, SeekFrom};
use std::io::Error;
use std::fs::File;
use std::path::Path;

extern crate wap_utils;
use wap_utils::bytes::*;
use wap_utils::size_of;

use crate::node::*;
use crate::uri;
use crate::ds::*;
use self::ds::*;


/* NOTE:
	Max length of any URI < 70 bytes,
	so there we can store URIs in `[u8; 70]` for opt.
*/


type Uri = Vec<u8>;


#[derive(Debug)]
pub struct Index {
	// pub name: String,
	// title: String,
	// version: u32,
	pub nodes: HashMap<Uri, Node>,

	/// Directories, contains the `Node`s, represented in the `nodes`.
	pub directories: HashMap<Uri, Vec<Uri>>,
	// Location of this package.
	// pub uri: &str,
}

impl Index {
	pub fn new() -> Self {
		Index { nodes: HashMap::default(),
		        directories: HashMap::default(), }
	}

	pub fn is_empty(&self) -> bool { self.nodes.is_empty() }
}

#[derive(Debug, Eq, PartialEq)]
pub enum PackageType {
	Main,
	Patch,
	Voice,
	// VoicePatch
	Extra,
}

#[derive(Debug)]
pub struct Package {
	index: Index,
	kind: PackageType,
	// uri: std::path::Path,
	uri: String,
}

impl Package {
	pub fn new(uri: String, kind: PackageType) -> Self {
		Self { uri,
		       kind,
		       index: Index::new(), }
	}

	pub fn uri(&self) -> &str { &self.uri }
	pub fn kind(&self) -> &PackageType { &self.kind }
	pub fn index(&self) -> &Index { &self.index }
	pub fn index_mut(&mut self) -> &mut Index { &mut self.index }

	#[inline]
	pub fn is_empty(&self) -> bool { self.index.is_empty() }

	pub fn open_with<R: Read + Seek>(self, stream: R) -> OpenPackage<R> {
		//
		OpenPackage { inner: self, stream }
	}
}


pub struct OpenPackage<R: Read + Seek> {
	inner: Package,
	stream: R,
}

impl<R: Read + Seek> OpenPackage<R> {
	pub fn uri(&self) -> &str { self.inner.uri() }
	pub fn index(&self) -> &Index { self.inner.index() }
	pub fn index_mut(&mut self) -> &mut Index { self.inner.index_mut() }

	pub fn is_empty(&self) -> bool { self.inner.is_empty() }

	pub fn stream(&mut self) -> &mut R { &mut self.stream }

	pub fn split_mut(&mut self) -> (&Index, &mut R) {
		let index = &self.inner.index;
		(index, &mut self.stream)
	}

	fn split_mut_ext(&mut self) -> (&mut Index, &mut R) {
		let index = &mut self.inner.index;
		(index, &mut self.stream)
	}

	pub fn split(self) -> (Package, R) { (self.inner, self.stream) }

	// pub fn read_with(&mut self, READER: FnMut(...

	pub fn close(self) -> Package { self.inner }
}

impl OpenPackage<File> {
	pub fn new(pack: Package, file: File) -> Self {
		Self { inner: pack,
		       stream: file, }
	}

	pub fn file(&mut self) -> &mut File { self.stream() }
}

impl<R: Read + Seek> AsRef<Package> for OpenPackage<R> {
	fn as_ref(&self) -> &Package { &self.inner }
}

impl AsRef<Index> for Package {
	fn as_ref(&self) -> &Index { &self.index }
}


//

#[inline]
pub fn read_package<R: Read + Seek>(pack: &mut OpenPackage<R>) -> Result<(), Error> {
	let (index, stream) = pack.split_mut_ext();
	read_pack_to_index(stream, index)
}


pub fn read_pack_to_index<R: Read + Seek>(from: &mut R, to: &mut Index) -> Result<(), Error> {
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
				extension
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
					let name_len = name.len() + extension.len();
					let uri = [&dir.0, name, b".", read_to_zero(extension)].concat();
					// XXX: FUCK! // TODO: Don't do it!
					files.push(uri[&uri.len() - name_len..].to_vec());
					index.nodes.insert(
					                   uri,
					                   Node { // id: asset_id,
					                          t: type_from_ext(&extension),
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
		use wap_utils::size_of;

		#[test]
		fn struct_sizes() {
			assert_eq!(size_of::<Header>(), 131);
			assert_eq!(size_of::<NodeHead>(), 16);
			assert_eq!(size_of::<NodePosition>(), 8);
		}
	}

}
