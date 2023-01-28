#![no_std]
#![no_main]

mod console;
mod tools;

use gba::Align4;
use core::fmt::Write;
use crate::console::VRAM_BLOCK0;
use crate::console::wait_vblank;
use gba::interrupts::IrqBits;
use gba::mgba::MgbaBufferedLogger;
use gba::mgba::MgbaMessageLevel;
use gba::mmio;
use gba::video::BackgroundControl;
use gba::video::Color;
use gba::video::DisplayControl;
use gba::video::DisplayStatus;
use gba::video::TextEntry;
use gba::video::VideoMode::_0 as VideoMode0;

#[no_mangle]
extern "C" fn main() -> ! {
	mmio::DISPCNT.write(
		DisplayControl::new()
			.with_video_mode(VideoMode0)
			.with_show_bg0(true)
	);
	// Transparency color
	mmio::BG_PALETTE.index(0).write(
		Color::new()
			.with_red(31)
	);

	mmio::BG0CNT.write(
		BackgroundControl::new()
			.with_charblock(0)
			.with_screenblock(8)
	);

	for (i, word) in include_aligned_resource!("res/luvui.4bpp").as_u32_slice().iter().enumerate() {
		VRAM_BLOCK0.index(i).write(*word);
	}

	for y in 0..32 {
		for x in 0..32 {
			mmio::TextScreenblockAddress::new(8)
				.row_col(x, y)
				.write(
					TextEntry::new()
						.with_tile(((y & 1) + 2 * (x & 1)) as u16)
				);
		}
	}

	mmio::IF.write(IrqBits::new());
	mmio::IE.write(
		IrqBits::new()
			.with_vblank(true)
			.with_hblank(true)
	);
	mmio::IME.write(true);
	mmio::DISPSTAT.write(
		DisplayStatus::new()
			.with_irq_vblank(true)
			.with_irq_hblank(true)
	);

	loop {
		for _ in 0..31 {
			let color = mmio::BG_PALETTE.index(0).read();
			mmio::BG_PALETTE.index(0).write(color.with_green(color.green() + 1));
			for _ in 0..4 { wait_vblank(); }
		}
		for _ in 0..31 {
			let color = mmio::BG_PALETTE.index(0).read();
			mmio::BG_PALETTE.index(0).write(color.with_red(color.red().saturating_sub(1)));
			for _ in 0..4 { wait_vblank(); }
		}
		for _ in 0..31 {
			let color = mmio::BG_PALETTE.index(0).read();
			mmio::BG_PALETTE.index(0).write(color.with_blue(color.blue() + 1));
			for _ in 0..4 { wait_vblank(); }
		}
		for _ in 0..31 {
			let color = mmio::BG_PALETTE.index(0).read();
			mmio::BG_PALETTE.index(0).write(color.with_green(color.green().saturating_sub(1)));
			for _ in 0..4 { wait_vblank(); }
		}
		for _ in 0..31 {
			let color = mmio::BG_PALETTE.index(0).read();
			mmio::BG_PALETTE.index(0).write(color.with_red(color.red() + 1));
			for _ in 0..4 { wait_vblank(); }
		}
		for _ in 0..31 {
			let color = mmio::BG_PALETTE.index(0).read();
			mmio::BG_PALETTE.index(0).write(color.with_blue(color.blue().saturating_sub(1)));
			for _ in 0..4 { wait_vblank(); }
		}
	}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
	let log_level = MgbaMessageLevel::Fatal;
	if let Ok(mut logger) = MgbaBufferedLogger::try_new(log_level) {
		writeln!(logger, "{info}").ok();
	}
	loop {}
}
