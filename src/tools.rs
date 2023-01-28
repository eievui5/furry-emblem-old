/// Includes a resource from cargo's output directory.
#[macro_export] macro_rules! include_aligned_resource {
	($file:expr $(,)?) => {
		gba::include_aligned_bytes!(concat!(env!("OUT_DIR"), "/", $file))
	}
}
