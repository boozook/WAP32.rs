use zero::Pod;


#[repr(C, packed)]
pub struct Header {
	pub head_size: u32,

	__gap_0: [u8; 8],

	pub frames_num: u32,
	pub offset: u32,

	__gap_1: [u8; 12],
}
unsafe impl Pod for Header {}


#[cfg(test)]
mod tests {
	use super::*;
	use wap_utils::size_of;

	#[test]
	fn struct_sizes() {
		assert_eq!(size_of::<Header>(), 32);
	}
}
