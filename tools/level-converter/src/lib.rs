#![crate_type = "proc-macro"]
use proc_macro2::{Delimiter, Group, Ident, Literal, Spacing, Span, Punct, TokenStream};
use syn::{parse_macro_input, LitStr};
use syn::parse::{Parse, ParseStream};
use tiled::LayerType;
use tiled::{Loader, Properties, PropertyValue, TileLayer};
use quote::{quote, TokenStreamExt, ToTokens};

fn add_struct_literal(tokens: &mut TokenStream, ident: &str, lit: Literal) {
	tokens.append(Ident::new(ident, Span::call_site()));
	tokens.append(Punct::new(':', Spacing::Alone));
	tokens.append(lit);
	tokens.append(Punct::new(',', Spacing::Alone));
}

/// Mirrors the UnitData struct on the GBA side.
#[derive(Debug)]
struct UnitData {
	name: String,
	x: u16,
	y: u16,
	/// Determines whether or not a unit is marked as a boss.
	/// Object Property: boss (boolean).
	is_boss: bool,
	level: u8,
}

impl ToTokens for UnitData {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append(Ident::new("UnitData", Span::call_site()));

		let mut sub_tokens = TokenStream::new();

		add_struct_literal(&mut sub_tokens, "name", Literal::string(&self.name));
		add_struct_literal(&mut sub_tokens, "x", Literal::u16_suffixed(self.x));
		add_struct_literal(&mut sub_tokens, "y", Literal::u16_suffixed(self.y));

		sub_tokens.append(Ident::new("is_boss", Span::call_site()));
		sub_tokens.append(Punct::new(':', Spacing::Alone));
		sub_tokens.append(Ident::new(if self.is_boss { "true" } else { "false" }, Span::call_site()));
		sub_tokens.append(Punct::new(',', Spacing::Alone));

		add_struct_literal(&mut sub_tokens, "level", Literal::u8_suffixed(self.level));

		tokens.append(Group::new(Delimiter::Brace, sub_tokens));
	}
}

/// Mirrors the LevelData struct on the GBA side.
#[derive(Debug)]
struct LevelData {
	width: u16,
	height: u16,
	map: Vec<u8>,
	units: Vec<UnitData>,
}

impl ToTokens for LevelData {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append(Ident::new("LevelData", Span::call_site()));

		let mut sub_tokens = TokenStream::new();

		add_struct_literal(&mut sub_tokens, "width", Literal::u16_suffixed(self.width));
		add_struct_literal(&mut sub_tokens, "height", Literal::u16_suffixed(self.height));

		sub_tokens.append(Ident::new("map", Span::call_site()));
		sub_tokens.append(Punct::new(':', Spacing::Alone));
		sub_tokens.append(Punct::new('&', Spacing::Alone));
		let mut map_tokens = TokenStream::new();
		for tile in &self.map {
			map_tokens.append(Literal::u8_suffixed(*tile));
			map_tokens.append(Punct::new(',', Spacing::Alone));
		}
		sub_tokens.append(Group::new(Delimiter::Bracket, map_tokens));
		sub_tokens.append(Punct::new(',', Spacing::Alone));

		sub_tokens.append(Ident::new("units", Span::call_site()));
		sub_tokens.append(Punct::new(':', Spacing::Alone));
		sub_tokens.append(Punct::new('&', Spacing::Alone));
		let mut unit_tokens = TokenStream::new();
		for unit in &self.units {
			unit.to_tokens(&mut unit_tokens);
			unit_tokens.append(Punct::new(',', Spacing::Alone));
		}
		sub_tokens.append(Group::new(Delimiter::Bracket, unit_tokens));
		sub_tokens.append(Punct::new(',', Spacing::Alone));

		tokens.append(Group::new(Delimiter::Brace, sub_tokens));
	}
}

fn bool_property(properties: &Properties, property: &str) -> Option<bool> {
	match properties.get(property) {
		Some(PropertyValue::BoolValue(value)) => Some(*value),
		_ => None,
	}
}

struct LoadLevelArgs {
	path: LitStr,
}

impl Parse for LoadLevelArgs {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		Ok(Self {
			path: input.parse()?,
		})
	}
}

#[proc_macro]
pub fn load_level(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let args = parse_macro_input!(item as LoadLevelArgs);
	let mut loader = Loader::new();
	let map = loader.load_tmx_map(args.path.value()).unwrap();

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
						x: (unit.x as u16) / 16,
						y: (unit.y as u16) / 16,
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

	proc_macro::TokenStream::from(quote! {
		#level
	})
}