#[cfg(not(feature = "fxhash"))]
pub use std::collections::{HashMap, HashSet};

#[cfg(feature = "fxhash")]
pub use self::fxhash_sup::{HashMap, HashSet};
#[cfg(feature = "fxhash")]
mod fxhash_sup {
	extern crate fxhash;
	use std::collections::HashMap as StdHashMap;
	use std::collections::HashSet as StdHashSet;
	use std::hash::BuildHasherDefault;
	use fxhash::FxHasher;
	pub type HashMap<K, V> = StdHashMap<K, V, BuildHasherDefault<FxHasher>>;
	pub type HashSet<K> = StdHashSet<K, BuildHasherDefault<FxHasher>>;
}
