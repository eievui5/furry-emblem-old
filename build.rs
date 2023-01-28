use std::env;
use std::io::{stdout, stderr, Write};
use std::process::{Command, exit};

fn main() {
	let out_dir = format!("OUT={}", env::var("OUT_DIR").unwrap());

	let output = Command::new("make")
		.args(["resources", "-j", &out_dir])
		.output()
		.expect("failed to execute `make`");

	stdout().write_all(&output.stdout).unwrap();
	stderr().write_all(&output.stderr).unwrap();

	exit(!output.status.success() as i32);
}