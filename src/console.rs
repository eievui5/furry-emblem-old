#![allow(dead_code)]
pub use gba::bios::VBlankIntrWait as wait_vblank;

use gba::bios::IntrWait as wait_intr;
use gba::prelude::IrqBits;
use voladdress::{Safe, VolBlock};

/// Block 0 of VRAM as a u32 array.
pub const VRAM_BLOCK0: VolBlock<u32, Safe, Safe, 4196> = unsafe { VolBlock::new(0x06000000) };

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

/// Shorthand for [`IntrWait(true, IrqBits::HBLANK)`](wait_intr)
pub fn wait_hblank() {
	wait_intr(true, IrqBits::new().with_hblank(true));
}
