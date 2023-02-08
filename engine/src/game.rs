use crate::console::*;
use crate::tools::include_aligned_resource;
use crate::transform::{AxisX, AxisY, Vector2D};
use gba::mmio::TextScreenblockAddress;
use gba::video::TextEntry;
use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2};
use gba::Align4;

struct Cursor {
	position: Vector2D<i16>,
	sprite_position: Vector2D<i16>,
	tile_id: u16,
	palette: u16,
	bounce_timer: u8,
}

enum CursorState {
	Idle,
	Open,
	Closed,
}

impl Cursor {
	fn new(vram: &mut Vram) -> Self {
		Self {
			position: Vector2D { x: 0, y: 0 },
			sprite_position: Vector2D { x: 0, y: 0 },
			tile_id: vram.load_4bpp_obj_texture(
				&include_aligned_resource!("gfx/cursor.4bpp").as_u32_slice(),
			),
			palette: vram.load_obj_palette(&include_aligned_resource!("gfx/cursor.pal").as_u16_slice()),
			bounce_timer: 0,
		}
	}

	fn draw(&mut self, oam: &mut Oam, state: CursorState) {
		fn bounce_offset(timer: &mut u8) -> i16 {
			*timer += 1;
			match timer {
				0..5 => 0,
				5..10 => 1,
				10..15 => 2,
				15..20 => 3,
				20..40 => 4,
				40..45 => 3,
				45..50 => 2,
				50..55 => 1,
				55..75 => 0,
				_ => { *timer = 0; 0 }
			}
		}

		self.sprite_position.move_towards(self.position * 16, 4);

		let make_corner = |x_off, y_off, hflip, vflip| {
			let mut cursor_base = ObjAttr::new();
			cursor_base.0 = ObjAttr0::new()
				.with_y((self.sprite_position.y + y_off) as u16);
			cursor_base.1 = ObjAttr1::new()
				.with_x((self.sprite_position.x + x_off) as u16)
				.with_hflip(hflip)
				.with_vflip(vflip)
				.with_size(S8x8);
			cursor_base.2 = ObjAttr2::new()
				.with_tile_id(self.tile_id)
				.with_palbank(self.palette);
			cursor_base
		};

		let offset = match state {
			CursorState::Idle => bounce_offset(&mut self.bounce_timer),
			CursorState::Open => {
				self.bounce_timer = 40;
				4
			}
			CursorState::Closed => {
				self.bounce_timer = 0;
				2
			}
		};

		let sprite = oam.reserve_entry();
		*sprite = make_corner(-offset, -offset, false, false);
		let sprite = oam.reserve_entry();
		*sprite = make_corner(8 + offset, -offset, true, false);
		let sprite = oam.reserve_entry();
		*sprite = make_corner(-offset, 8 + offset, false, true);
		let sprite = oam.reserve_entry();
		*sprite = make_corner(8 + offset, 8 + offset, true, true);
	}
}
 
#[derive(Debug)]
pub struct UnitData<'a> {
	pub name: &'a str,
	pub x: u16,
	pub y: u16,
	/// Determines whether or not a unit is marked as a boss.
	/// Object Property: boss (boolean).
	pub is_boss: bool,
	pub level: u8,
}

#[derive(Debug)]
pub struct LevelData<'a> {
	pub width: u16,
	pub height: u16,
	pub map: &'a [u8],
	pub units: &'a [UnitData<'a>]
}

struct Unit {
	position: Vector2D<i16>,
	tile_id: u16,
	palette: u16,
	animation_timer: u8,
}

impl Unit {
	fn new(vram: &mut Vram) -> Self {
		Self {
			position: Vector2D { x: 0, y: 0 },
			tile_id: vram.load_4bpp_obj_texture(
				&include_aligned_resource!("gfx/luvui.4bpp").as_u32_slice(),
			),
			palette: vram.load_obj_palette(&include_aligned_resource!("gfx/luvui.pal").as_u16_slice()),
			animation_timer: 0,
		}
	}

	fn draw(&mut self, oam: &mut Oam, selected: bool) {
		let sprite = oam.reserve_entry();
		sprite.0 = ObjAttr0::new()
			.with_y((self.position.y * 16) as u16);
		sprite.1 = ObjAttr1::new()
			.with_x((self.position.x * 16) as u16)
			.with_size(S16x16);
		sprite.2 = ObjAttr2::new()
			.with_tile_id(self.tile_id
				+ if selected {8} else {0}
				+ if self.animation_timer & 0x10 != 0 {4} else {0})
			.with_palbank(self.palette);
		self.animation_timer = self.animation_timer.wrapping_add(1);
	}
}

pub struct GameState<'a> {
	cursor: Cursor,
	units: [Unit; 2],
	selected_unit: Option<usize>,
	tileset_id: u16,
	tileset_palette: u16,
	level: &'a LevelData<'a>
}

impl<'a> GameState<'a> {
	pub fn new(level: &'a LevelData) -> Self {
		let mut vram = Vram::new();
		vram.load_4bpp_bg_texture(
			&[0, 0, 0, 0, 0, 0, 0, 0],
		);
		let tileset_id = vram.load_4bpp_bg_texture(
			&include_aligned_resource!("gfx/tree_tiles.4bpp").as_u32_slice(),
		);
		let tileset_palette = vram.load_bg_palette(
			&include_aligned_resource!("gfx/tree_tiles.pal").as_u16_slice(),
		);

		for y in 0..level.height {
			for x in 0..level.width {
				let tile = level.map[(x + y * level.width) as usize] as u16;
				let x = x * 2;
				let y = y * 2;
				TextScreenblockAddress::new(8)
					.row_col(y.into(), x.into())
					.write(
						TextEntry::new()
							.with_tile(tileset_id + tile * 4 + 0)
							.with_palbank(tileset_palette)
					);
				TextScreenblockAddress::new(8)
					.row_col(y.into(), (x + 1).into())
					.write(
						TextEntry::new()
							.with_tile(tileset_id + tile * 4 + 1)
							.with_palbank(tileset_palette)
					);
				TextScreenblockAddress::new(8)
					.row_col((y + 1).into(), x.into())
					.write(
						TextEntry::new()
							.with_tile(tileset_id + tile * 4 + 2)
							.with_palbank(tileset_palette)
					);
				TextScreenblockAddress::new(8)
					.row_col((y + 1).into(), (x + 1).into())
					.write(
						TextEntry::new()
							.with_tile(tileset_id + tile * 4 + 3)
							.with_palbank(tileset_palette)
					);
			}
		}

		Self {
			cursor: Cursor::new(&mut vram),
			units: [Unit::new(&mut vram), Unit::new(&mut vram)],
			selected_unit: None,
			tileset_id,
			tileset_palette,
			level
		}
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

		if input.new.a() {
			if let Some(selected_unit) = self.selected_unit {
				self.units[selected_unit].position = self.cursor.position;
				self.selected_unit = None;
			} else {
				for (i, unit) in self.units.iter().enumerate() {
					if self.cursor.position == unit.position {
						self.selected_unit = Some(i);
						break;
					}
				}
			}
		}

		{
			let mut cursor_state = CursorState::Idle;
			for unit in &self.units {
				if self.cursor.position == unit.position {
					cursor_state = CursorState::Open;
				}
			}
			if self.selected_unit.is_some() {
				cursor_state = CursorState::Closed;
			}
			self.cursor.draw(oam, cursor_state);
		}

		for (i, unit) in self.units.iter_mut().enumerate() {
			unit.draw(oam, Some(i) == self.selected_unit);
		}
	}
}
