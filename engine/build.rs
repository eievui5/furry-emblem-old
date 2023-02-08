use evgfx::convert;
use level_converter::load_level;
use std::env;
use std::fs;
use std::path::PathBuf;

fn convert_image(
	config: &convert::Config,
	input_path: &str,
	output_path: &PathBuf,
	palette_path: &PathBuf,
) {
	println!("cargo:rerun-if-changed={input_path}");
	fs::create_dir_all(output_path.parent().unwrap()).unwrap();
	fs::create_dir_all(palette_path.parent().unwrap()).unwrap();

	let (palettes, tiles, _) = config.convert_image(input_path).unwrap();

	tiles.write_4bpp(output_path.to_str().unwrap()).unwrap();
	palettes
		.write_rgb555(palette_path.to_str().unwrap(), true)
		.unwrap();
}

macro_rules! make_image {
	($config:expr, $resource:expr) => {
		convert_image(
			$config,
			concat!("src/assets/", $resource, ".png"),
			&[
				&env::var("OUT_DIR").unwrap(),
				concat!("assets/", $resource, ".4bpp"),
			]
			.iter()
			.collect(),
			&[
				&env::var("OUT_DIR").unwrap(),
				concat!("assets/", $resource, ".pal"),
			]
			.iter()
			.collect(),
		);
	};
}

fn main() {
	let config = convert::Config::new()
		.with_tilesize(16, 16)
		.with_transparency_color(0xFF, 0x00, 0xFF);

	make_image!(&config, "gfx/luvui");
	make_image!(&config, "gfx/tree_tiles");

	let config = convert::Config::new()
		.with_transparency_color(0xFF, 0x00, 0xFF);

	make_image!(&config, "gfx/cursor");

	println!("cargo:rerun-if-changed={}/assets/levels/debug-level.rs", &env::var("OUT_DIR").unwrap());
	let level = load_level("src/assets/levels/debug-level.tmx").to_string();
	fs::create_dir_all(&format!("{}/assets/levels/", &env::var("OUT_DIR").unwrap())).unwrap();
	fs::write(
		&format!("{}/assets/levels/debug-level.rs", &env::var("OUT_DIR").unwrap()),
		level,
	).unwrap();
}
