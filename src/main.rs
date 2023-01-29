#![no_std]
#![no_main]

mod console;
mod tools;
mod transform;

use gba::Align4;
use core::fmt::Write;
use crate::console::VRAM_BLOCK0;
use crate::console::VRAM_OBJS;
use crate::console::wait_vblank;
use crate::transform::{AxisX, AxisY};
use crate::transform::Vector2D;
use gba::interrupts::IrqBits;
use gba::mgba::MgbaBufferedLogger;
use gba::mgba::MgbaMessageLevel;
use gba::mmio;
use gba::video::BackgroundControl;
use gba::video::Color;
use gba::video::DisplayControl;
use gba::video::DisplayStatus;
use gba::video::obj::{ObjAttr0, ObjAttr1, ObjAttr2, ObjDisplayStyle};
use gba::video::TextEntry;
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

	// Clear OAM
	for i in 0..128 {
		mmio::OBJ_ATTR0.index(i).write(ObjAttr0::new()
			.with_style(ObjDisplayStyle::NotDisplayed));
	}

	for (i, word) in include_aligned_resource!("res/luvui.4bpp").as_u32_slice().iter().enumerate() {
		VRAM_BLOCK0.index(8 + i).write(*word);
	}
	for (i, word) in include_aligned_resource!("res/luvui.4bpp").as_u32_slice().iter().enumerate() {
		VRAM_OBJS.index(i).write(*word);
	}

	// Set the First object palette to be all red
	for i in 1..16 {
		mmio::OBJ_PALETTE.index(i).write(Color::RED);
	}

	for y in 0..32 {
		for x in 0..32 {
			mmio::TextScreenblockAddress::new(8)
				.row_col(x, y)
				.write(
					TextEntry::new()
						.with_tile(1 + ((y & 1) + 2 * (x & 1)) as u16)
				);
		}
	}

	let mut position = Vector2D::<u16> { x: 0, y: 0 };
	let mut input = console::Input::new();

	loop {
		input.update();

		match input.get_held_x() {
			Some(AxisX::Left) => position.x -= 1,
			Some(AxisX::Right) => position.x += 1,
			_ => {}
		}

		match input.get_held_y() {
			Some(AxisY::Up) => position.y -= 1,
			Some(AxisY::Down) => position.y += 1,
			_ => {}
		}

		if input.new.a() {
			println!("Meow!");
		}

		mmio::BG_PALETTE.index(0).write(rotate_rgb_color(mmio::BG_PALETTE.index(0).read()));
		wait_vblank();
		// TODO: Make this not ugly
		// I'd like to use shadow OAM w/DMA just to make accesses more elegant,
		// but that may not be necessary.
		mmio::OBJ_ATTR0.index(0).write(ObjAttr0::new()
			.with_y(position.y));
		mmio::OBJ_ATTR1.index(0).write(ObjAttr1::new()
			.with_x(position.x)
			.with_size(console::S16x16));
		mmio::OBJ_ATTR2.index(0).write(ObjAttr2::new()
			.with_tile_id(0));
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
