/// Includes a resource from cargo's output directory.
macro_rules! include_aligned_resource {
	($file:expr $(,)?) => {
		gba::include_aligned_bytes!(concat!(env!("OUT_DIR"), "/assets/", $file))
	};
}

pub(crate) use include_aligned_resource;

/// Includes a resource from cargo's output directory.
macro_rules! include_resource {
	($file:expr $(,)?) => {
		include!(concat!(env!("OUT_DIR"), "/assets/", $file))
	};
}

pub(crate) use include_resource;

macro_rules! load_level {
	($file:expr $(,)?) => {
		include!(concat!(env!("OUT_DIR"), "/assets/maps/", $file, ".rs"))
	};
}

pub(crate) use load_level;
