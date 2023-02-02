#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_macros)]
#![allow(unused_imports)]
pub use gba::bios::VBlankIntrWait as wait_vblank;

use core::cmp::max;
use crate::transform::AxisX;
use crate::transform::AxisY;
use crate::transform::Direction4;
use gba::bios::IntrWait as wait_intr;
use gba::interrupts::IrqBits;
use gba::mmio;
use gba::keys::KeyInput;
use gba::video::obj::{ObjAttr, ObjAttr0, ObjDisplayStyle};
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
macro_rules! println {
	($($args:expr),+) => {
		let log_level = gba::mgba::MgbaMessageLevel::Info;
		if let Ok(mut logger) = gba::prelude::MgbaBufferedLogger::try_new(log_level) {
			writeln!(logger, $($args),+).ok();
		}
	}
}

pub(crate) use println;

/// Formats and prints a message to the emulator.
/// The message is marked as "Error".
macro_rules! eprintln {
	($($args:expr),+) => {
		let log_level = gba::mgba::MgbaMessageLevel::Error;
		if let Ok(mut logger) = gba::prelude::MgbaBufferedLogger::try_new(log_level) {
			writeln!(logger, $($args),+).ok();
		}
	}
}

pub(crate) use eprintln;

/// Stores a working copy of OAM (Shadow OAM) that can be sent to the PPU at the end of a frame.
pub struct Oam {
	index: usize,
	last_index: usize,
	entries: [ObjAttr; 128],
}

impl Oam {
	pub fn new() -> Self {
		Oam {
			index: 128,
			last_index: 0,
			entries: [(ObjAttr::new()); 128]
		}
	}

	/// Clears all dirty oam entries.
	pub fn clean(&mut self) {
		for i in 0..self.index {
			self.entries[i].0 = ObjAttr0::new()
				.with_style(ObjDisplayStyle::NotDisplayed);
		}
		self.last_index = self.index;
		self.index = 0;
	}

	/// Pushes all entries to OAM, allowing the PPU to display them.
	pub fn commit(&self) {
		for i in 0..max(self.index, self.last_index) {
			mmio::OBJ_ATTR0.index(i).write(self.entries[i].0);
			mmio::OBJ_ATTR1.index(i).write(self.entries[i].1);
			mmio::OBJ_ATTR2.index(i).write(self.entries[i].2);
		}
	}

	/// Returns an OAM entry for the calling code to use as needed.
	pub fn reserve_entry(&mut self) -> &mut ObjAttr {
		let result = &mut self.entries[self.index];
		self.index += 1;
		result
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
