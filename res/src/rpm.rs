//! Resource-Package Manager

use std::fs::File;
use std::io::{Read, Seek};

use crate::node::NodeType;
use crate::pack::Index;
use crate::pack::{Package, PackageType};
use crate::pack::read_package;


pub enum ResourceType {
	Package(PackageType),
	File(NodeType),
}


/// File Manager
pub struct Fm {}


/// Prioritized storage for `Package`s.
#[derive(Debug)]
pub struct Rpm {
	// TODO: XXX: pubs is temprly!
	pub main: Option<Package>,
	patch: Option<Package>,
	voice: Option<Package>,
	extra: Vec<Package>,
}


pub struct RpmBuilder {
	inner: Rpm,
}

impl RpmBuilder {
	pub fn new(main: Package) -> Self {
		Self { inner: Rpm { main: Some(main),
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
			PackageType::Main => {
				if self.inner.main.is_none() {
					self.inner.main.replace(pack);
				} else {
					panic!("Main pack is already set: '{:?}', new: '{}'", &self.inner.main, pack.uri());
				}
			}
		}
	}

	fn read_all(&mut self) -> Result<(), std::io::Error> {
		// TODO: XXX: FIXME: There is TEMP impl!

		let pack = if let Some(pack) = self.inner.main.take() {
			let uri = pack.uri();
			let file = File::open(uri)?;
			let mut opened = pack.open_with(file);
			read_package(&mut opened)?;
			opened.close()
		} else { unimplemented!() };
		self.inner.main.replace(pack);
		// let mut pack = &mut self.inner.main;

		Ok(())
	}

	/// Read index for all packages,
	/// consolidate indexes into one.
	pub fn consolidate(&mut self) {}

	pub fn build(mut self) -> Result<Rpm, std::io::Error> {
		self.read_all()?;
		self.consolidate();
		Ok(self.inner)
	}
}
