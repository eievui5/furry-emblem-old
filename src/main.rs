#![no_std]
#![no_main]

mod console;
mod game;
mod tools;
mod transform;

use core::fmt::Write;
use crate::console::wait_vblank;
use gba::interrupts::IrqBits;
use gba::mgba::MgbaBufferedLogger;
use gba::mgba::MgbaMessageLevel;
use gba::mmio;
use gba::video::BackgroundControl;
use gba::video::Color;
use gba::video::DisplayControl;
use gba::video::DisplayStatus;
use gba::video::VideoMode::_0 as VideoMode0;

fn rotate_rgb_color(color: Color) -> Color {
	match (color.red(), color.green(), color.blue()) {
		// Green from Red
		(last, 0x1F, 0x00) if last > 0 => color.with_red(color.red() - 1),
		// Blue from Green
		(0x00, last, 0x1F) if last > 0 => color.with_green(color.green() - 1),
		// Red from Blue
		(0x1F, 0x00, last) if last > 0 => color.with_blue(color.blue() - 1),
		// Red to Green
		(0x1F, _, 0x00) => color.with_green(color.green() + 1),
		// Green to Blue
		(0x00, 0x1F, _) => color.with_blue(color.blue() + 1),
		// Blue to Red
		(_, 0x00, 0x1F) => color.with_red(color.red() + 1),
		// If the state is invalid, reset to red.
		_ => Color::RED,
	}
}

#[no_mangle]
extern "C" fn main() -> ! {
	mmio::DISPCNT.write(
		DisplayControl::new()
			.with_video_mode(VideoMode0)
			.with_show_bg0(true)
			.with_show_obj(true)
			.with_obj_vram_1d(true)
	);

	mmio::BG0CNT.write(
		BackgroundControl::new()
			.with_charblock(0)
			.with_screenblock(8)
	);

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

	let mut input = console::Input::new();
	let mut oam = console::Oam::new();
	let mut game_state = game::GameState::new();

	loop {
		input.update();
		oam.clean();

		game_state.tick(&mut input, &mut oam);

		mmio::BG_PALETTE.index(0).apply(|color| {
			*color = rotate_rgb_color(*color)
		});
		wait_vblank();
		oam.commit();
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
