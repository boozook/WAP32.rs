/// [width, height] or [x, y]
pub type RawPoint<T> = [T; 2];

pub trait Point<T: Sized> {
	fn x(&self) -> &T;
	fn y(&self) -> &T;
}

pub trait Size<T: Sized> {
	fn width(&self) -> &T;
	fn height(&self) -> &T;
}

impl<T> Point<T> for RawPoint<T> {
	fn x(&self) -> &T { &self[0] }
	fn y(&self) -> &T { &self[1] }
}

impl<T> Size<T> for RawPoint<T> {
	fn width(&self) -> &T { &self[0] }
	fn height(&self) -> &T { &self[1] }
}


pub const ZERO: RawPoint<i32> = [0, 0];
