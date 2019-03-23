use std::marker::PhantomData;
use crate::point::*;
use self::format::PxFmt;


// #[cfg(not(feature = "image"))]
pub mod format {
	#![allow(non_snake_case)]

	// TODO: Store primitive as generic in the RGB
	// e.g. RGB<u8> , RGB<u32> , RGBA<u32>

	#[derive(Debug)]
	pub struct RGBA;

	#[derive(Debug)]
	pub struct ARGB;

	#[derive(Debug)]
	pub struct RGB;

	/// Indices (u8) pinting into palette
	#[derive(Debug)]
	pub struct I;

	/// Only alpha. Using as lights/shadow-texture.
	#[derive(Debug)]
	pub struct A;


	pub trait PxFmt {
		/// Bytes per pixel.
		/// Pixel components num in bytes.
		fn components() -> u8;
	}

	impl PxFmt for RGBA {
		#[inline]
		fn components() -> u8 { 4 }
	}

	impl PxFmt for ARGB {
		#[inline]
		fn components() -> u8 { 4 }
	}

	impl PxFmt for RGB {
		#[inline]
		fn components() -> u8 { 3 }
	}

	impl PxFmt for I {
		#[inline]
		fn components() -> u8 { 1 }
	}

	impl PxFmt for A {
		#[inline]
		fn components() -> u8 { 1 }
	}
}


pub trait Pxls<T>: Sized {
	/// Raw pixels
	fn pixels(&self) -> &[T];
}

pub trait PxlsSized<T>: Pxls<T> {
	/// Dimensions. Raw point - `&[width, height]`.
	fn size(&self) -> &RawPoint<u32>;

	/// Pos. Raw point - `&[x, y]`.
	fn offset(&self) -> &RawPoint<i32>;
}

pub trait PxComponents<T>: PxlsSized<T> {
	/// Elements per pixel. Not in bytes.
	/// e.g. for u8: 1 / 3 / 4 for A / RGB / RGBA
	/// and for u32 all is eq 1.
	fn color_components(&self) -> u8;
}

// auto-impls
impl<F: PxFmt> PxComponents<u8> for PxBufSized<F, u8> {
	fn color_components(&self) -> u8 { F::components() }
}
impl<F: PxFmt> PxComponents<u32> for PxBufSized<F, u32> {
	fn color_components(&self) -> u8 { 1 }
}
// pub trait PxlsConvert<A, B>: PxComponents<A> {
// 	fn convert(mut self);
// 	fn convert_cloned(&self) -> PxComponents<B>;
// }


pub trait PxlsMut<T>: Sized {
	/// Raw pixels
	fn pixels_mut(&mut self) -> &mut [T];
}


#[derive(Debug)]
// TODO: PxBuf<F: PxFmt, C: AsRef<[T]>, T>
// for support small arrays
pub struct PxBuf<F: PxFmt, T> {
	buffer: Vec<T>,
	_format: PhantomData<*const F>,
}

pub struct PxBufSized<F: PxFmt, T> {
	inner: PxBuf<F, T>,
	size: RawPoint<u32>,
}

impl<F: PxFmt, T> PxBuf<F, T> {
	pub fn new(pixels: Vec<T>) -> Self {
		Self { buffer: pixels,
		       _format: PhantomData, }
	}

	pub fn into_inner(self) -> Vec<T> { self.buffer }
	pub fn as_inner(&self) -> &[T] { &self.buffer }
	pub fn as_inner_mut(&mut self) -> &mut [T] { &mut self.buffer }
}

impl<F: PxFmt, T> PxBufSized<F, T> {
	pub fn into_inner(self) -> Vec<T> { self.inner.into_inner() }
	pub fn as_inner(&self) -> &[T] { self.inner.as_inner() }
	pub fn as_inner_mut(&mut self) -> &mut [T] { self.inner.as_inner_mut() }
}

impl<F: PxFmt> PxBufSized<F, u8> {
	pub fn new(pixels: Vec<u8>, width: u32, height: u32) -> Self {
		debug_assert_eq!(
		                 height,
		                 pixels.len() as u32 / width / F::components() as u32,
		                 "incorrect dimensions"
		);
		PxBufSized { inner: PxBuf::new(pixels),
		             size: [width, height], }
	}

	pub fn new_rect(pixels: Vec<u8>, side: u32) -> Self { Self::new(pixels, side, side) }
}

impl<F: PxFmt, T> Pxls<T> for PxBuf<F, T> {
	#[inline]
	fn pixels(&self) -> &[T] { self.buffer.as_slice() }
}

impl<F: PxFmt, T> Pxls<T> for PxBufSized<F, T> {
	#[inline]
	fn pixels(&self) -> &[T] { self.inner.pixels() }
}

impl<F: PxFmt, T> PxlsSized<T> for PxBufSized<F, T> {
	#[inline]
	fn size(&self) -> &RawPoint<u32> { &self.size }
	#[inline]
	fn offset(&self) -> &RawPoint<i32> { &ZERO }
	// fn color_components(&self) -> u8 { }
}

impl<F: PxFmt, T> PxlsMut<T> for PxBuf<F, T> {
	fn pixels_mut(&mut self) -> &mut [T] { self.buffer.as_mut_slice() }
}

impl<F: PxFmt, T> PxlsMut<T> for PxBufSized<F, T> {
	fn pixels_mut(&mut self) -> &mut [T] { self.inner.pixels_mut() }
}


pub type Indices = PxBuf<format::I, u8>;
pub type Palette<Format> = PxBuf<Format, u8>;


// ------- conversions ------- //

impl<T> From<PxBuf<format::I, T>> for PxBuf<format::A, T> {
	fn from(buf: PxBuf<format::I, T>) -> Self {
		// safe transmute because changing `_format:PhantomData` only
		// and it exists in compile-time only.
		unsafe { std::mem::transmute(buf) }
	}
}

impl<T> From<PxBuf<format::A, T>> for PxBuf<format::I, T> {
	fn from(buf: PxBuf<format::A, T>) -> Self {
		// safe transmute because changing `_format:PhantomData` only
		// and it exists in compile-time only.
		unsafe { std::mem::transmute(buf) }
	}
}


// ------- extra info ------- //

pub trait PxMeta {
	fn mirror(&self) -> bool;
	fn invert(&self) -> bool;
	fn lightmap(&self) -> bool;
	fn transparent(&self) -> bool;
}
