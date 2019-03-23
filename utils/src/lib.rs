#![no_std]


#[inline]
// TODO: use core::intrinsics::size_of
pub fn size_of<T: Sized>() -> usize { core::mem::size_of::<T>() }


pub mod bytes {

	/// Read until zero and return [0...zero] slice.
	pub fn read_to_zero<'a>(input: &'a [u8]) -> &'a [u8] {
		for (i, byte) in input.iter().enumerate() {
			if *byte == 0 {
				return &input[..i];
			}
		}
		// else &input
		panic!("No null byte in input");
	}
}


pub mod flags {
	#[macro_export]
	macro_rules! flags_has {
		($v:expr, $f:expr) => {
			($v & $f) == $f
		};
	}

	#[macro_export]
	macro_rules! flags_set {
		($v:expr, $f:expr) => {
			$v |= 1 << $f
		};
	}

	#[macro_export]
	macro_rules! flags_unset {
		($v:expr, $f:expr) => {
			$v &= 0xFFFFFFF - (1 << $f)
		};
	}

}
