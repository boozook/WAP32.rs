#[derive(Debug)]
pub struct Node {
	// Идентификатор или флаги.
	// pub id: [u8; 4],
	/// Seek-position rel by start.
	pub offset: u32,

	/// Size of block.
	pub size: u32,

	// /// Type of node
	// pub t: Format,
}

pub struct TypedNode<'n>
{
	pub n: &'n Node,
	pub t: Format,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Debug)]
pub enum Format {
	/// Animation
	Ani,
	/// Palette
	Pal,
	Pid,
	Pcx,
	Wav,
	/// X-Midi
	Xmi,
	/// World (level)
	Wwd,
	/// Text. Can be encrypted.
	Txt,
	/// e.g. `.BAT`, `.BMP`,..
	Unknown,
}


pub fn type_from_ext(ext: &[u8]) -> Format {
	use self::Format::*;

	match &ext[..3] {
		b"INA" => Ani,
		b"LAP" => Pal,
		b"DIP" => Pid,
		b"XCP" => Pcx,
		b"VAW" => Wav,
		b"DWW" => Wwd,
		b"IMX" => Xmi,
		b"TXT" => Txt,
		_ => Unknown,
	}
}
