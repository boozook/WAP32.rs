use std::io::prelude::*;
use std::io::Error;
use std::fs::File;

use crate::reader::read_to;
use crate::index::Index;


#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PackageKind {
	Main,
	Patch,
	Voice,
	// VoicePatch
	Extra,
}

#[derive(Debug)]
pub struct Package<P> {
	index: Index,
	kind: PackageKind,
	// uri: std::path::Path,
	uri: P,
}

impl<P: AsRef<str>> Package<P> {
	pub fn uri_str(&self) -> &str { self.uri.as_ref() }
}

impl<P> Package<P> {
	pub fn new(uri: P, kind: PackageKind) -> Self {
		Self { uri,
		       kind,
		       index: Index::new() }
	}

	pub fn uri(&self) -> &P { &self.uri }

	pub fn kind(&self) -> &PackageKind { &self.kind }
	pub fn index(&self) -> &Index { &self.index }
	pub fn index_mut(&mut self) -> &mut Index { &mut self.index }

	#[inline]
	pub fn is_empty(&self) -> bool { self.index.is_empty() }

	pub fn open_with<R: Read + Seek>(self, reader: R) -> OpenPackage<R, P> {
		OpenPackage { inner: self, reader }
	}
}


pub struct OpenPackage<R: Read + Seek, P> {
	inner: Package<P>,
	reader: R,
}

impl<R: Read + Seek, P: AsRef<str>> OpenPackage<R, P> {
	pub fn uri_str(&self) -> &str { self.inner.uri_str() }
}

impl<R: Read + Seek, P> OpenPackage<R, P> {
	pub fn new(pack: Package<P>, reader: R) -> Self {
		Self { inner: pack,
		       reader: reader }
	}

	pub fn uri(&self) -> &P { self.inner.uri() }

	pub fn index(&self) -> &Index { self.inner.index() }
	pub fn index_mut(&mut self) -> &mut Index { self.inner.index_mut() }

	pub fn is_empty(&self) -> bool { self.inner.is_empty() }

	pub fn reader(&mut self) -> &mut R { &mut self.reader }

	pub fn split_mut(&mut self) -> (&Index, &mut R) {
		let index = &self.inner.index;
		(index, &mut self.reader)
	}

	pub fn split_mut_ext(&mut self) -> (&mut Index, &mut R) {
		let index = &mut self.inner.index;
		(index, &mut self.reader)
	}

	pub fn split(self) -> (Package<P>, R) { (self.inner, self.reader) }

	// pub fn read_with(&mut self, READER: FnMut(...

	pub fn close(self) -> Package<P> { self.inner }
}

impl<P: AsRef<str>> OpenPackage<File, P> {
	pub fn file(&mut self) -> &mut File { self.reader() }
}

impl<R: Read + Seek, P> AsRef<Package<P>> for OpenPackage<R, P> {
	fn as_ref(&self) -> &Package<P> { &self.inner }
}

impl<P> AsRef<Index> for Package<P> {
	fn as_ref(&self) -> &Index { &self.index }
}


#[inline]
pub fn read_package<R: Read + Seek, P: AsRef<str>>(pack: &mut OpenPackage<R, P>) -> Result<(), Error> {
	let (index, stream) = pack.split_mut_ext();
	read_to(stream, index)
}
