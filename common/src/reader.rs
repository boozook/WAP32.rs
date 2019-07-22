pub trait Reader<R, T>: Sized {
	type Error: std::error::Error;
	fn read(&mut self, from: &mut R) -> Result<T, Self::Error>;
}

pub trait ReaderTo<R, T>: Sized {
	type Error: std::error::Error;
	fn read_to(&mut self, from: &mut R, to: &mut T) -> Result<(), Self::Error>;
}
