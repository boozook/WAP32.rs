use zero::Pod;

// XXX: TODO: REMOVE COPY,CLONE IMPLS!

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Head {
	pub size: u32,
	__pad: u32, // null
}
unsafe impl Pod for Head {}


#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct RawWwdHeader {
	pub flags: u32,
	__pad_0: u32, // null

	pub name: [u8; 64],
	pub author: [u8; 64],
	pub birth: [u8; 64],
	pub rez: [u8; 256],

	// path:
	pub images: [u8; 128],
	pub palrez: [u8; 128],

	// camera:
	pub cam_x: u32,
	pub cam_y: u32,

	__pad_unknown: u32, // null / unknown ?

	pub planes_num: u32,
	pub planes_offset: u32, // main block offset (planesOffset)
	pub tile_descriptions_offset: u32,

	// ONLY if deflate compression used:
	pub all_planes_length: u32, //empty [4] (or decompressed main block size)
	pub checksum: u32,

	__pad_2: u32, // null
	exe: [u8; 128],

	// img-sets:
	pub img_sets: [[u8; 128]; 4],
	pub img_sets_prefixes: [[u8; 32]; 4],
}
unsafe impl Pod for RawWwdHeader {}


// #[repr(C, packed)]
// pub struct Body {}
// unsafe impl Pod for Body {}


#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PlaneHead {
	pub size: u32, // always 0x A0 00 00 00 = 160 = length of header
	__pad_0: u32,  // empty[4]
	pub flags: u32,
	__pad_1: u32, // empty[4]
	pub name: [u8; 64],
	// pub name: [[u8; 32]; 2],
	pub plane_width: u32,  // pos: 80
	pub plane_height: u32, // pos: 84
	pub tile_width: u32,   // pos: 88
	pub tile_height: u32,  // pos: 92
	pub tile_wide: u32,    // pos: 96
	pub tile_high: u32,    // pos: 100

	__pad_3: u32, // empty[4]
	__pad_4: u32, // empty[4]

	pub movement_x: i32, // pos: 112
	pub movement_y: i32, // pos: 116

	pub fill_color: u32,   // pos: 120
	pub tilesets_num: u32, // pos: 124

	// objects:
	pub objects_num: u32,             // pos: 128
	pub offset: u32,                  // pos: 132 (body offset)
	pub image_sets_names_offset: u32, // pos: 136
	pub objects_offset: u32,          // pos: 140

	// z
	pub plane_z: i32, // pos: 144

	// delimiter:
	// empty[4] x 3
	__pad_5: [u32; 3], // null
}
unsafe impl Pod for PlaneHead {}


#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct PlaneObjectProperties {
	pub id: u32,             // (0)
	pub name_length: u32,    // (4)
	pub logic_length: u32,   // (8)
	pub graphic_length: u32, // (12)
	pub ani_length: u32,     // (16)

	// location:
	pub x: u32, // (20)
	pub y: u32, // (24)
	pub z: u32, // (28)
	pub i: u32, // (32)

	// flags:
	pub add_flags: u32,     // (36)
	pub dynamic_flags: u32, // (40)
	pub draw_flags: u32,    // (44)
	pub user_flags: u32,    // (48)

	// attributes:
	pub score: u32,   // (52)
	pub points: u32,  // (56)
	pub powerup: u32, // (60)
	pub damage: u32,  // (64)
	pub smarts: u32,  // (68)
	pub health: u32,  // (72)

	// RECTS HERE: 76...172
	pub rects: PlaneObjectPropertiesRects,

	// user values:
	pub user_values: [u32; 8],

	pub x_min: u32, // (204)
	pub y_min: u32, // (208)
	pub x_max: u32, // (212)
	pub y_max: u32, // (216)

	pub speed_x: u32, // (220)
	pub speed_y: u32, // (224)

	pub x_tweak: u32, // (228)
	pub y_tweak: u32, // (232)
	pub counter: u32, // (236)

	pub speed: u32, // (240)

	pub width: u32,  // (244)
	pub height: u32, // (248)

	pub direction: u32, // (252)

	pub face_dir: u32,    // (256)
	pub time_delay: u32,  // (260)
	pub frame_delay: u32, // (264)

	// uint32_t object_type /* WAP_OBJECT_TYPE_ single value */
	pub object_type: u32, // (268)
	// uint32_t hit_type_flags /* WAP_OBJECT_TYPE_ flags */
	pub hit_type_flags: u32, // ObjectTypeFlag>(bytes.readInteger(272))
	pub x_move_res: u32,     // (276)
	pub y_move_res: u32,     // (280)
}
unsafe impl Pod for PlaneObjectProperties {}


#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct PlaneObjectPropertiesRects {
	/// (sibling) groups:
	pub movement: WapRect,
	pub hit: WapRect,
	pub attack: WapRect,
	pub clip: WapRect,
	pub user1: WapRect,
	pub user2: WapRect,
}
unsafe impl Pod for PlaneObjectPropertiesRects {}


/// left, top, right, bottom
pub type WapRect = [u32; 4];


#[repr(C, packed)]
pub struct TilePropertiesHead {
	// header: 32, 0, `num tile descriptions`, 0, 0, 0, 0, 0
	pub length: u32,
	__pad: u32,
	pub tile_descriptions_num: u32,
}
unsafe impl Pod for TilePropertiesHead {}


#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct TilePropertiesSingle {
	/// tile type (flags: 1/2)
	pub dtype: TilePropertiesType,
	__pad: u32, // unknown
	/// width of tile-cell in pixels
	pub width: u32,
	/// height of tile-cell in pixels
	pub height: u32,

	// inside / outside:
	/// inside is inside if dtype == SINGLE only.
	pub inside: u32,
}
unsafe impl Pod for TilePropertiesSingle {}


#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct TilePropertiesDouble {
	/// tile type (flags: 1/2)
	pub dtype: TilePropertiesType,
	__pad: u32, // unknown
	/// width of tile-cell in pixels
	pub width: u32,
	/// height of tile-cell in pixels
	pub height: u32,

	pub outside: u32,
	pub inside: u32,
	pub rect: WapRect,
}
unsafe impl Pod for TilePropertiesDouble {}


#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum TilePropertiesType {
	Unknown,
	Single,
	Double,
}
unsafe impl Pod for TilePropertiesType {}


pub mod header_flags {
	pub const USE_Z_COORDS: u32 = 0b0001;
	pub const COMPRESS: u32 = 0b0010;
}


#[cfg(test)]
mod tests {
	use super::*;
	use common::utils::size_of;

	#[test]
	fn struct_sizes() {
		assert_eq!(8, size_of::<Head>());
		assert_eq!(1524, size_of::<Head>() + size_of::<RawWwdHeader>());
		assert_eq!(160, size_of::<PlaneHead>());
		assert_eq!(284, size_of::<PlaneObjectProperties>());
		assert_eq!(12, size_of::<TilePropertiesHead>());
		assert_eq!(20, size_of::<TilePropertiesSingle>());
		assert_eq!(40, size_of::<TilePropertiesDouble>());
	}
}
