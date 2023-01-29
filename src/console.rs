#![allow(dead_code)]
#![allow(non_upper_case_globals)]
pub use gba::bios::VBlankIntrWait as wait_vblank;

use crate::transform::AxisX;
use crate::transform::AxisY;
use crate::transform::Direction4;
use gba::bios::IntrWait as wait_intr;
use gba::mmio;
use gba::keys::KeyInput;
use gba::prelude::IrqBits;
use voladdress::{Safe, VolBlock};

// Sprite sizes.
pub const S8x8: u16 = 0;
pub const S16x16: u16 = 1;
pub const S32x32: u16 = 2;
pub const S64x64: u16 = 3;
pub const H16x8: u16 = 0;
pub const H32x8: u16 = 1;
pub const H32x16: u16 = 2;
pub const H64x32: u16 = 3;
pub const V8x16: u16 = 0;
pub const V8x32: u16 = 1;
pub const V16x32: u16 = 2;
pub const V32x64: u16 = 3;

pub const VRAM_BLOCK0: VolBlock<u32, Safe, Safe, 0x1000> =
	unsafe { VolBlock::new(0x06000000) };
pub const VRAM_OBJS: VolBlock<u32, Safe, Safe, 0x1000> =
	unsafe { VolBlock::new(0x06010000) };

/// Formats and prints a message to the emulator.
/// The message is marked as "Info".
#[macro_export] macro_rules! println {
	($($args:expr),+) => {
		let log_level = gba::mgba::MgbaMessageLevel::Info;
		if let Ok(mut logger) = gba::prelude::MgbaBufferedLogger::try_new(log_level) {
			writeln!(logger, $($args),+).ok();
		}
	}
}

/// Formats and prints a message to the emulator.
/// The message is marked as "Error".
#[macro_export] macro_rules! eprintln {
	($($args:expr),+) => {
		let log_level = gba::mgba::MgbaMessageLevel::Error;
		if let Ok(mut logger) = gba::prelude::MgbaBufferedLogger::try_new(log_level) {
			writeln!(logger, $($args),+).ok();
		}
	}
}

/// Contains the current frame's input state.
/// Must be updated once (and only once) each frame with the .update() function.
pub struct Input {
	pub held: KeyInput,
	pub new: KeyInput,
	pub released: KeyInput,
	last: KeyInput,
}

impl Input {
	pub fn new() -> Self {
		Self {
			held: KeyInput::new(),
			last: KeyInput::new(),
			new: KeyInput::new(),
			released: KeyInput::new(),
		}
	}

	/// Re-read the keys for this frame.
	pub fn update(&mut self) {
		self.last = self.held;
		self.held = mmio::KEYINPUT.read();
		self.new = self.held & !self.last;
		self.released = !self.held & self.last;
	}

	fn get_direction4(input: KeyInput) -> Option<Direction4> {
		if input.up() {
			Some(Direction4::Up)
		} else if input.right() {
			Some(Direction4::Right)
		} else if input.down() {
			Some(Direction4::Down)
		} else if input.left() {
			Some(Direction4::Left)
		} else {
			None
		}
	}

	pub fn get_held_direction4(&self) -> Option<Direction4> {
		Self::get_direction4(self.held)
	}

	pub fn get_new_direction4(&self) -> Option<Direction4> {
		Self::get_direction4(self.new)
	}

	pub fn get_released_direction4(&self) -> Option<Direction4> {
		Self::get_direction4(self.released)
	}

	fn get_x(input: KeyInput) -> Option<AxisX> {
		if input.left() {
			Some(AxisX::Left)
		} else if input.right() {
			Some(AxisX::Right)
		} else {
			None
		}
	}

	pub fn get_held_x(&self) -> Option<AxisX> {
		Self::get_x(self.held)
	}

	pub fn get_new_x(&self) -> Option<AxisX> {
		Self::get_x(self.new)
	}

	pub fn get_released_x(&self) -> Option<AxisX> {
		Self::get_x(self.released)
	}

	fn get_y(input: KeyInput) -> Option<AxisY> {
		if input.up() {
			Some(AxisY::Up)
		} else if input.down() {
			Some(AxisY::Down)
		} else {
			None
		}
	}

	pub fn get_held_y(&self) -> Option<AxisY> {
		Self::get_y(self.held)
	}

	pub fn get_new_y(&self) -> Option<AxisY> {
		Self::get_y(self.new)
	}

	pub fn get_released_y(&self) -> Option<AxisY> {
		Self::get_y(self.released)
	}
}

/// Shorthand for [`IntrWait(true, IrqBits::HBLANK)`](wait_intr)
pub fn wait_hblank() {
	wait_intr(true, IrqBits::new().with_hblank(true));
}
