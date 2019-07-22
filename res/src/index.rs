use common::ds::*;
use crate::node::*;
use crate::uri::*;


#[derive(Debug)]
pub struct Index {
	// pub name: String,
	// title: String,
	// version: u32,
	pub(crate) nodes: HashMap<Uri, Node>,

	/// Directories, contains the `Node`s, represented in the `nodes`.
	pub(crate) directories: HashMap<Uri, Vec<Uri>>,
	// Location of this package.
	// pub uri: &str,
}

impl Index {
	pub fn new() -> Self {
		Index { nodes: HashMap::default(),
		        directories: HashMap::default() }
	}

	pub fn nodes(&self) -> &HashMap<Uri, Node> { &self.nodes }
	pub fn dirs(&self) -> &HashMap<Uri, Vec<Uri>> { &self.directories }

	pub fn is_empty(&self) -> bool { self.nodes.is_empty() }
}
