use std::io::{prelude::*, SeekFrom};

extern crate zero;

extern crate wap_res;
use wap_res::node::Node;

extern crate wap_utils;
use wap_utils::size_of;

pub(crate) mod ds;
use self::ds::*;


type Result = core::result::Result<AnimationDurations, std::io::Error>;


// FIXME: TODO: Get default duration from hx-version
static DEFAULT_ANIMATION_DURATION: u16 = 40;


#[derive(Debug)]
pub struct AnimationDurations {
	/// times in ms for frame
	pub durations: Vec<u16>,
}


/// Durations for each frame of animation
impl AnimationDurations {
	pub fn frames_num(&self) -> usize { self.durations.len() }
	pub fn default_with_frames(num: usize) -> Self { Self { durations: vec![DEFAULT_ANIMATION_DURATION; num], } }
}

impl Default for AnimationDurations {
	fn default() -> Self { Self::default_with_frames(0) }
}

// TODO: КОГДА NodePointer ДОСТАВЯТ
// impl<'a, 'b, R: Seek + Read> ReadNode<WapAnimationDurations, &'b mut R> for NodePointer<'a, 'b, R> {
// 	fn read(&mut self) -> Result<WapAnimationDurations> { read_as_ani(&self.0, &mut self.1) }
// }


pub fn read_as_ani<'a, 'b, R: Seek + Read>(node: &'a Node, r: &'b mut R) -> Result {
	r.seek(SeekFrom::Start(node.offset as u64))?;
	let mut buf = vec![0u8; node.size as usize];
	r.read_exact(&mut buf)?;

	let head = zero::read::<Header>(&buf[..size_of::<Header>()]);
	debug_assert!(head.frames_num > 0);

	let mut n = head.offset as usize + size_of::<Header>();
	let mut durations = vec![0u16; head.frames_num as usize];

	for i in 0..head.frames_num as usize {
		// read half:
		durations[i] = buf[n + 10] as u16 + ((buf[n + 10 + 1] as u16) << 8);

		// Check for effect:
		let mut j = 0;
		if (buf[n] & 2) == 2 {
			'jb: loop {
				let b = buf[n + 20 + j];
				j += 1;
				if b == 0 {
					break 'jb;
				}
			}
		}
		n += 20 + j;
	}

	debug_assert_eq!(durations.len(), head.frames_num as usize);

	let result = if *durations.iter().min().unwrap() == 0 {
		AnimationDurations::default_with_frames(head.frames_num as usize)
	} else {
		AnimationDurations { durations }
	};

	Ok(result)
}
