use std::fmt::{Display, Formatter};
use tiled::LayerType;
use tiled::{Loader, Properties, PropertyValue, TileLayer};

/// Mirrors the UnitData struct on the GBA side.
#[derive(Debug)]
pub struct UnitData {
	pub name: String,
	pub x: u16,
	pub y: u16,
	/// Determines whether or not a unit is marked as a boss.
	/// Object Property: boss (boolean).
	pub is_boss: bool,
	pub level: u8,
}

/// Mirrors the LevelData struct on the GBA side.
#[derive(Debug)]
pub struct LevelData {
	pub width: u16,
	pub height: u16,
	pub map: Vec<u8>,
	pub units: Vec<UnitData>,
}

impl Display for LevelData {
	fn fmt(&self, out: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		let mut map = String::from("&[");
		for i in &self.map {
			map += &format!("{i},");
		}
		map += "]";

		let mut units = String::from("&[");
		for i in &self.units {
			units += &format!("{i:?},");
		}
		units += "]";

		write!(out,
"LevelData {{
	width: {},
	height: {},
	map: {map},
	units: {units},
}}", self.width, self.height)
	}
}

fn bool_property(properties: &Properties, property: &str) -> Option<bool> {
	match properties.get(property) {
		Some(PropertyValue::BoolValue(value)) => Some(*value),
		_ => None,
	}
}

pub fn load_level(path: &str) -> LevelData {
	let mut loader = Loader::new();
	let map = loader.load_tmx_map(path).unwrap();

	// Set true once a map layer is processed.
	// Used for error checking.
	let mut has_map_layer = false;

	let mut level = LevelData {
		width: 0,
		height: 0,
		map: Vec::new(),
		units: Vec::new(),
	};

	for layer in map.layers() {
		match layer.layer_type() {
			LayerType::TileLayer(TileLayer::Finite(map)) => {
				if has_map_layer {
					eprintln!("More than one tile layer. Skipping extra...");
					continue;
				}
				has_map_layer = true;

				level.width = map.width() as u16;
				level.height = map.height() as u16;

				for y in 0..(map.height() as i32) {
					for x in 0..(map.width() as i32) {
						let id = if let Some(tile) = map.get_tile(x, y) {
							tile.id()
						} else {
							eprintln!("Tile at ({}: {x}, {y}) is blank", layer.name);
							0
						};
						level.map.push(id as u8);
					}
				}
			}
			LayerType::ObjectLayer(objects) => {
				for unit in objects.object_data() {
					let unit_name = if unit.name.len() > 0 {
						&unit.name
					} else {
						"Enemy"
					};

					let is_boss = bool_property(&unit.properties, "boss").unwrap_or(false);

					let unit_data = UnitData {
						name: unit_name.to_string(),
						// Offset units by 8 so that they're centered if a unit is misplaced.
						x: ((unit.x + 8.0) as u16) / 16,
						y: ((unit.y + 8.0) as u16) / 16,
						is_boss,
						level: 1,
					};

					level.units.push(unit_data);
				}
			}
			LayerType::ImageLayer(_) => {
				eprintln!("Image layers are not supported");
			}
			LayerType::GroupLayer(_) => {
				eprintln!("Group layers are not supported");
			}
			LayerType::TileLayer(TileLayer::Infinite(_)) => {
				eprintln!("Infinite maps are not supported");
			}
		}
	}

	if !has_map_layer {
		panic!("No tile layer is defined");
	}

	level
}

