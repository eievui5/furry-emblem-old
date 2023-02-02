use gba::mmio;
use gba::video::Color;
use gba::Align4;
use crate::VRAM_OBJS;
use crate::console::{Input, Oam, S16x16};
use crate::transform::{AxisX, AxisY, Vector2D};
use crate::tools::include_aligned_resource;

use gba::video::obj::{ObjAttr0, ObjAttr1, ObjAttr2};

struct Cursor {
	position: Vector2D<u16>,
}

impl Cursor {
	fn draw(&self, oam: &mut Oam) {
		let sprite = oam.reserve_entry();
		sprite.0 = ObjAttr0::new()
			.with_y(self.position.y * 16);
		sprite.1 = ObjAttr1::new()
			.with_x(self.position.x * 16)
			.with_size(S16x16);
		sprite.2 = ObjAttr2::new()
			.with_tile_id(0);
	}
}

pub struct GameState {
	cursor: Cursor,
}

impl GameState {
	pub fn new() -> Self {
		let result = Self {
			cursor: Cursor { position: Vector2D::new() },
		};

		for (i, word) in include_aligned_resource!("luvui.4bpp").as_u32_slice().iter().enumerate() {
			VRAM_OBJS.index(i).write(*word);
		}
		for (i, word) in include_aligned_resource!("luvui.pal").as_u16_slice().iter().enumerate() {
			mmio::OBJ_PALETTE.index(i).write(Color(*word));
		}

		result
	}

	pub fn tick(&mut self, input: &Input, oam: &mut Oam) {
		match input.get_new_x() {
			Some(AxisX::Left) => self.cursor.position.x -= 1,
			Some(AxisX::Right) => self.cursor.position.x += 1,
			_ => {}
		}

		match input.get_new_y() {
			Some(AxisY::Up) => self.cursor.position.y -= 1,
			Some(AxisY::Down) => self.cursor.position.y += 1,
			_ => {}
		}

		self.cursor.draw(oam);
	}
}
