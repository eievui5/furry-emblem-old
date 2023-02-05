/// Includes a resource from cargo's output directory.
macro_rules! include_aligned_resource {
	($file:expr $(,)?) => {
		gba::include_aligned_bytes!(concat!(env!("OUT_DIR"), "/res/", $file))
	};
}

pub(crate) use include_aligned_resource;
