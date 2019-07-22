// TODO: #![no_std]
use std::io::{Read, Seek, SeekFrom};

extern crate byteorder;
extern crate libflate;
extern crate zero;

use self::byteorder::{LittleEndian, ReadBytesExt};

extern crate wap_res;
use wap_res::node::Node;

extern crate wap_common as common;
use common::utils::size_of;


pub(crate) mod ds;
use self::ds::*;


pub type RawWwd = (Vec<RawTileAttrs>, RawWwdHeader, Vec<WapPlane>);
type Result = std::result::Result<RawWwd, std::io::Error>;


#[derive(Debug)]
pub struct WapObjectTraits {
	pub name: Vec<u8>,
	pub logic: Vec<u8>,
	pub graphic: Vec<u8>,
	pub animation: Vec<u8>,
}

pub struct WapObject {
	pub i: u32,
	pub traits: WapObjectTraits,
	pub properties: PlaneObjectProperties,
}

pub struct WapPlane {
	pub i: usize,
	pub properties: PlaneHead,
	pub tiles: Vec<u32>,
	// TODO: use &[u8]
	pub tileset: Vec<String>,
	// TODO: use &[u8]
	pub tileset_names: Vec<String>,
	pub objects: Vec<WapObject>,
}

#[derive(Debug)]
pub enum RawTileAttrs {
	Single(TilePropertiesSingle),
	Double(TilePropertiesDouble),
}


// TODO: КОГДА NodePointer ДОСТАВЯТ
// impl<'a, 'b, R: Seek + Read> ReadNode<RawWwd, &'b mut R> for NodePointer<'a, 'b, R> {
// 	fn read(&mut self) -> Result<RawWwd> { Ok(read_as_wwd(&self.0, &mut self.1)?) }
// }


pub fn read_as_wwd<'a, 'b, T: Seek + Read>(node: &'a Node, from: &'b mut T) -> Result {
	from.seek(SeekFrom::Start(node.offset as u64))?;

	// head/meta:
	let mut buf = [0u8; 8]; // 8 = Head::size_of()
	from.read_exact(&mut buf)?;
	let head = zero::read::<Head>(&buf);

	debug_assert_eq!(size_of::<RawWwdHeader>(), head.size as usize - size_of::<Head>());

	// header:
	let mut buf = vec![0u8; head.size as usize - size_of::<Head>()];
	from.read_exact(&mut buf)?;
	let header = zero::read::<RawWwdHeader>(&buf);
	let compressed = (header.flags & header_flags::COMPRESS) == header_flags::COMPRESS;
	// let use_z = (header.flags & header_flags::USE_Z_COORDS) == header_flags::USE_Z_COORDS;
	// log!("compressed = {}, useZ = {}", compressed, use_z);

	// prepare URI-prefixes:
	use std::collections::HashMap;
	let mut uri_prefix_map = HashMap::new();
	for i in 0..header.img_sets_prefixes.len() {
		{
			let prefix = &header.img_sets_prefixes[i];
			// get first zero - end of string:
			let prefix_len = prefix.iter().position(|b| b == &0u8).unwrap();
			let prefix = &prefix[..prefix_len];

			let replace = &header.img_sets[i];
			let replace_len = replace.iter().position(|b| b == &0u8).unwrap();
			let replace = &replace[..replace_len];

			if prefix_len > 0 && replace_len > 0 {
				uri_prefix_map.insert(prefix, replace);
			}
		}
	}


	// body:
	// not needed because already here:
	// from.seek(SeekFrom::Start(node.offset as u64))?;
	let mut body = vec![0u8; (node.size - header.planes_offset) as usize];
	from.read_exact(&mut body)?;

	debug_assert_eq!(node.size - header.planes_offset, body.len() as u32);

	// WWD uses the deflate algorithm, so use an inflater:
	if compressed {
		let mut target = Vec::with_capacity(header.all_planes_length as usize);
		body = {
			// https://docs.rs/libflate/0.1.14/libflate/zlib/index.html
			let mut decoder = libflate::zlib::Decoder::new(&body[..])?;
			decoder.read_to_end(&mut target)?;
			target
		}
	};

	debug_assert_eq!(header.all_planes_length as usize, body.len());

	// heads of planes:
	// log!("reading: heads of planes...");
	let std_plane_size: usize = 160;
	let mut plane_heads: Vec<&PlaneHead> = {
		let mut heads = Vec::with_capacity(header.planes_num as usize);
		for i in 0..header.planes_num {
			let pos = i as usize * std_plane_size;
			let buf = &body[pos..pos + std_plane_size];
			let plane = zero::read::<PlaneHead>(&buf);

			debug_assert_eq!(std_plane_size, plane.size as usize);

			if plane.tilesets_num != 1 {
				unimplemented!("Supported only planes using only one image-set.");
			}
			heads.push(plane);
		}
		heads
	};
	// log!("loaded {} plane heads.", plane_heads.len());
	debug_assert_eq!(header.planes_num as usize, plane_heads.len());


	// DEBUG: print planes info & positions:
		/* {
			log!("print planes info & positions:");
			for i in 0..header.planes_num {
				let head = plane_heads[i as usize];
				// log!("\t plane {} , head: \n\t{:?}", i, head);
				log!("\t\t offset: {} ( {} )", head.offset, head.offset - header.planes_offset);
				log!(
					"\t\t objects offset : {} ( {} )",
					head.objects_offset,
					head.objects_offset as i32 - header.planes_offset as i32
				);
				log!(
					"\t\t image sets names offset: {} ( {} )",
					head.image_sets_names_offset,
					head.image_sets_names_offset as i32 - header.planes_offset as i32
				);
			}
		} */


	// tilesets:
	// log!("reading: tilesets of planes...");
	let mut position: usize = 0;
	let mut tiles = {
		let mut tiles_for_plane: Vec<Vec<u32>> = Vec::with_capacity(header.planes_num as usize);
		for i in 0..header.planes_num as usize {
			let head = plane_heads[i];
			let pos_start = head.offset - header.planes_offset;
			let pos_end = pos_start + (head.tile_wide * head.tile_high) * 4;
			// log!("\t read tiles for plane {} at pos: {} to {} = {}", i, pos_start, pos_end, pos_end - pos_start);
			// log!("\t\t ( total is {} )", body.len());

			let tiles_bytes = &body[pos_start as usize..pos_end as usize];
			let mut tiles = vec![0; tiles_bytes.len() / 4];
			for i in 0..tiles.len() {
				let mut part = &tiles_bytes[i * 4..i * 4 + 4];
				tiles[i] = part.read_u32::<LittleEndian>()?;
				// log!("{}\t: {:?}  \t{}", i, &tiles_bytes[i * 4..i * 4 + 4], tiles[i]);
			}
			// log!("\t loaded indexes of {} tiles for plane {}", tiles.len(), i);
			// tiles_for_plane[i] = tiles;
			tiles_for_plane.push(tiles);
			debug_assert_eq!(i + 1, tiles_for_plane.len());

			position = pos_end as usize;
		}
		tiles_for_plane
	};
	// log!("loaded tiles for {} planes.", tiles.len());


	// sets-names:
	// log!("reading: tilesets & its names of planes...");
	let (mut tilesets, mut tilesets_names) = {
		let mut sets_for_plane: Vec<Vec<String>> = Vec::with_capacity(header.planes_num as usize);
		let mut sets_names_for_plane: Vec<Vec<String>> = Vec::with_capacity(header.planes_num as usize);

		for i in 0..header.planes_num as usize {
			let head = plane_heads[i];

			// log!("\t plane {}, position: {}", i, position);
			debug_assert_eq!(head.image_sets_names_offset as i32 - header.planes_offset as i32, position as i32);

			let mut sets = Vec::new();
			let mut names = Vec::with_capacity(head.tilesets_num as usize);
			for _ in 0..head.tilesets_num {
				let buf = &body[position..(position + 32)];
				// unsafe { zero::read_str_unsafe(buf) }
				let name = zero::read_str(buf);

				// log!("\t\t name: {} ({}, {})", name, name.len(), buf.len());

				names.push(name.to_owned());
				position += name.len() + 1;

				// log!("\t\t new position: {}", position);

				// TODO: move it into the UriBuilder
				for t in 0..header.img_sets.len() {
					let imageset = zero::read_str(&header.img_sets[t]);
					// ! XXX: Hack for buildin levels - swap "IMAGEZ" to "TILEZ":
					if imageset.len() != 0 && imageset.ends_with("IMAGEZ") {
						let cutted = unsafe { imageset.slice_unchecked(0, imageset.len() - "IMAGEZ".len()) };
						// let cutted = unsafe { imageset.get_unchecked(0..imageset.len() - "IMAGEZ".len()) };
						sets.push([cutted, name].concat());
						// log!("imageset: '{}' => '{}'", imageset, sets[sets.len() - 1]);
					}
				}
			}
			sets_for_plane.push(sets);
			sets_names_for_plane.push(names);
		}
		println!("tileset {:?} for plane: {:?}", sets_names_for_plane, sets_for_plane);
		(sets_for_plane, sets_names_for_plane)
	};


	// objects:
	const BLOCK_SIZE: usize = 284; // ObjectProperties::size_of()

	// log!("reading: objects of planes...");
	// log!("   and construct Planes");

	//       [head], [planesTiles[i]], planesTilesets[i], [objects], [i]
	let mut planes: Vec<WapPlane> = Vec::with_capacity(header.planes_num as usize);
	{
		let mut i: usize = 0;
		// while let Some(head) = plane_heads.pop() {
		// ! we should to get a first head from front instead `pop`ping.
		while plane_heads.len() > 0 {
			let head = plane_heads.remove(0);
			let objects = if head.objects_num == 0 {
				Vec::with_capacity(0)
			} else {
				debug_assert_eq!(head.objects_offset as i32 - header.planes_offset as i32, position as i32);

				let mut objects: Vec<WapObject> = Vec::with_capacity(head.objects_num as usize);
				for i in 0..head.objects_num {
					let pos_start = position;
					let pos_end = pos_start + BLOCK_SIZE;

					debug_assert_eq!(pos_end - pos_start, BLOCK_SIZE);

					let block = zero::read::<PlaneObjectProperties>(&body[pos_start..pos_end]);
					position += BLOCK_SIZE;


					// names of object's traits:
					let traits = {
						let mut read_to = |len: usize| -> Vec<u8> {
							if len > 0 {
								let buf = &body[position..position + len];
								let mut result = Vec::with_capacity(len);
								result.extend(buf);
								position += len;
								result
							} else {
								Vec::with_capacity(0)
							}
						};

						let name = { read_to(block.name_length as usize).to_owned() };
						let logic = { read_to(block.logic_length as usize).to_owned() };
						let graphic = { read_to(block.graphic_length as usize).to_owned() };
						let animation = { read_to(block.ani_length as usize).to_owned() };

						// {
						// 	use std::str::from_utf8_unchecked;
						// 	// log!("\t\t name: '{:?}'", unsafe { from_utf8_unchecked(&name[..]) });
						// 	// log!("\t\t logic: '{:?}'", unsafe { from_utf8_unchecked(&logic[..]) });
						// 	log!("\t\t graphic: '{:?}'", unsafe { from_utf8_unchecked(&graphic[..]) });
						// 	// log!("\t\t animation: '{:?}'", unsafe { from_utf8_unchecked(&animation[..]) });
						// }

						WapObjectTraits { name,
						                  logic,
						                  graphic,
						                  animation, }
					};

					let properties = *block;
					objects.push(WapObject { i,
					                         traits,
					                         properties, });
				}
				objects
			};

			// let properties = *plane_heads[i as usize];
			let properties = *head;
			let tiles = tiles.remove(0);
			let tileset = tilesets.remove(0);
			let tileset_names = tilesets_names.remove(0);
			// let tiles = tiles.pop().unwrap();
			// let tileset = tilesets.pop().unwrap();
			// let tileset_names = tilesets_names.pop().unwrap();

			let plane = WapPlane { i,
			                       properties,
			                       tiles,
			                       tileset,
			                       tileset_names,
			                       objects, };
			planes.push(plane);
			i += 1;
		}
	}


	// tile attributes:
	// log!("reading: tile attributes...");
	let tile_attrs = {
		debug_assert_eq!(header.tile_descriptions_offset - header.planes_offset, position as u32);

		let buf = &body[position..position + size_of::<TilePropertiesHead>()];
		let head = zero::read::<TilePropertiesHead>(&buf);
		let mut attrs: Vec<RawTileAttrs> = Vec::with_capacity(head.tile_descriptions_num as usize);

		// println!("B: {:?}", &body[position..position + 32]);

		// validate header:
		unsafe { debug_assert_eq!(32, head.length) };
		debug_assert!(head.tile_descriptions_num > 0);

		// 4 * 8 = length of header
		position += 4 * 8;
		const SINGLE_SIZE: usize = 20; // TilePropertiesSingle::size_of();
		const DOUBLE_SIZE: usize = 40; // TilePropertiesDouble::size_of();
		for i in 0..head.tile_descriptions_num {
			let props_s = zero::read::<TilePropertiesSingle>(&body[position..position + SINGLE_SIZE]);
			let t_attr = match props_s.dtype {
				TilePropertiesType::Single => {
					position += SINGLE_SIZE;
					RawTileAttrs::Single(*props_s)
				},
				TilePropertiesType::Double => {
					let props_d = zero::read::<TilePropertiesDouble>(&body[position..position + DOUBLE_SIZE]);
					position += DOUBLE_SIZE;
					RawTileAttrs::Double(*props_d)
				},
				_ => panic!("Unknown type of tile properties: tp={}, pos={}", i, position),
			};
			attrs.push(t_attr);
		}
		// validate result:
		debug_assert_eq!(head.tile_descriptions_num as usize, attrs.len());

		// log!("loaded {} tile attributes.", attrs.len());

		attrs
	};


	// wwd:
	// собираем struct Wwd with:
	//       [header], [planes], [attrs], objects, ?main_plane?


	// log!("COMPLETE WITHOUT ANY PANIC!");
	Ok((tile_attrs, *header, planes))
}
