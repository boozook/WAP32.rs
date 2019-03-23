//! Resource-Package Manager

use std::fs::File;
use std::io::{Read, Seek};

use crate::node::NodeType;
use crate::pack::Index;
use crate::pack::{Package, PackageType};


pub enum ResourceType {
	Package(PackageType),
	File(NodeType),
}


/// File Manager
pub struct Fm {}


/// Prioritized storage for `Package`s.
#[derive(Debug)]
pub struct Rpm {
	main: Package,
	patch: Option<Package>,
	voice: Option<Package>,
	extra: Vec<Package>,
}


pub struct RpmBuilder {
	inner: Rpm,
}

impl RpmBuilder {
	pub fn new(main: Package) -> Self {
		Self { inner: Rpm { main,
		                    patch: None,
		                    voice: None,
		                    extra: Default::default(), }, }
	}


	pub fn add_pack(&mut self, pack: Package) {
		match pack.kind() {
			PackageType::Patch => {
				if self.inner.patch.is_none() {
					self.inner.patch.replace(pack);
				}
			},
			PackageType::Voice => {
				if self.inner.voice.is_none() {
					self.inner.voice.replace(pack);
				}
			},
			PackageType::Extra => self.inner.extra.push(pack),
			PackageType::Main => panic!("Main pack is already set: '{}', new: '{}'", self.inner.main.uri(), pack.uri()),
		}
	}

	fn read_all(&mut self) {

	}

	/// Read index for all packages,
	/// consolidate indexes into one.
	pub fn consolidate(&mut self) {}

	pub fn build(mut self) -> Rpm {
		self.read_all();
		self.consolidate();
		self.inner
	}
}
