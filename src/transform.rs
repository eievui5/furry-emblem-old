#![allow(dead_code)]

// Represents one of four cardinal directions.
use core::fmt::Debug;
use core::ops::*;

pub enum Direction4 {
	Up, Right, Down, Left
}

impl Direction4 {
	pub fn rotate_right(self) -> Self {
		match self {
			Direction4::Up => Direction4::Right,
			Direction4::Right => Direction4::Down,
			Direction4::Down => Direction4::Left,
			Direction4::Left => Direction4::Up,
		}
	}

	pub fn rotate_left(self) -> Self {
		match self {
			Direction4::Up => Direction4::Left,
			Direction4::Right => Direction4::Up,
			Direction4::Down => Direction4::Right,
			Direction4::Left => Direction4::Down,
		}
	}

	pub fn rotate_180(self) -> Self {
		match self {
			Direction4::Up => Direction4::Down,
			Direction4::Right => Direction4::Left,
			Direction4::Down => Direction4::Up,
			Direction4::Left => Direction4::Right,
		}
	}
}

pub enum AxisX {
	Left, Right
}

pub enum AxisY {
	Up, Down
}

/// A vector of two points: (x, y) represented by integers or fixed point numbers
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Vector2D<T> {
	/// The x coordinate
	pub x: T,
	/// The y coordinate
	pub y: T,
}

impl<T: From<u8>> Vector2D<T> {
	pub fn new() -> Self {
		Self { x: T::from(0), y: T::from(0) }
	}
}

impl<T: Add<Output = T>> Add<Vector2D<T>> for Vector2D<T> {
	type Output = Vector2D<T>;
	fn add(self, rhs: Vector2D<T>) -> Self::Output {
		Vector2D {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T: AddAssign> AddAssign<Self> for Vector2D<T> {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T: Sub<Output = T>> Sub<Vector2D<T>> for Vector2D<T> {
	type Output = Vector2D<T>;
	fn sub(self, rhs: Vector2D<T>) -> Self::Output {
		Vector2D {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T: SubAssign> SubAssign<Self> for Vector2D<T> {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

