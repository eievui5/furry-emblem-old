use evgfx::convert;
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
			concat!("src/res/", $resource, ".png"),
			&[
				&env::var("OUT_DIR").unwrap(),
				concat!("res/", $resource, ".4bpp"),
			]
			.iter()
			.collect(),
			&[
				&env::var("OUT_DIR").unwrap(),
				concat!("res/", $resource, ".pal"),
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

	make_image!(&config, "luvui");

	let config = convert::Config::new()
		.with_transparency_color(0xFF, 0x00, 0xFF);

	make_image!(&config, "cursor");
}
