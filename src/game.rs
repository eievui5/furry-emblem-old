use crate::console::{Input, Oam, S8x8, Vram};
use crate::tools::include_aligned_resource;
use crate::transform::{AxisX, AxisY, Vector2D};
use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2};
use gba::Align4;

struct Cursor {
	position: Vector2D<i16>,
	tile_id: u16,
	palette: u16,
	bounce_timer: u8,
}

impl Cursor {
	fn draw(&mut self, oam: &mut Oam) {
		fn bounce_offset(timer: &mut u8) -> i16 {
			macro_rules! range_value {
				{$name:ident: $default:literal, $($low:literal - $high:literal => $value:literal),*$(,)?} => {
					match *$name {
						#[allow(unused_comparisons)]
						$($name if $name >= $low && $name < $high => $value,)+
						_ => {
							*$name = 0;
							$default
						}
					}
				}
			}

			*timer += 1;
			range_value! {
				timer: 0,
				 0 -  5 => 0,
				 5 - 10 => 1,
				10 - 15 => 2,
				15 - 20 => 3,
				20 - 40 => 4,
				40 - 45 => 3,
				45 - 50 => 2,
				50 - 55 => 1,
				55 - 75 => 0,
			}
		}

		let make_corner = |x_off, y_off, hflip, vflip| {
			let mut cursor_base = ObjAttr::new();
			cursor_base.0 = ObjAttr0::new().with_y((self.position.y * 16 + y_off) as u16);
			cursor_base.1 = ObjAttr1::new()
				.with_x((self.position.x * 16 + x_off) as u16)
				.with_hflip(hflip)
				.with_vflip(vflip)
				.with_size(S8x8);
			cursor_base.2 = ObjAttr2::new()
				.with_tile_id(self.tile_id)
				.with_palbank(self.palette);
			cursor_base
		};

		let offset = bounce_offset(&mut self.bounce_timer);

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

pub struct GameState {
	cursor: Cursor,
}

impl GameState {
	pub fn new() -> Self {
		let mut vram = Vram::new();

		Self {
			cursor: Cursor {
				position: Vector2D::new(),
				tile_id: vram.load_4bpp_obj_texture(
					&include_aligned_resource!("cursor.4bpp").as_u32_slice(),
				),
				palette: vram.load_palette(&include_aligned_resource!("cursor.pal").as_u16_slice()),
				bounce_timer: 0,
			},
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

		self.cursor.draw(oam);
	}
}
