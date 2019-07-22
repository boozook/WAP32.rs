pub static DELIMITER: u8 = b'\\'; // 92
pub static OBJ_TRAIT_URI_DELIMITER: u8 = b'_';


pub type Uri = Vec<u8>;
pub type UriRef<'a> = &'a [u8];
