fn main() {
	// Use the linker script.
 	println!("cargo:rustc-link-arg=-Tsrc/linker.ld");
 	// Don't do any magic linker stuff.
	println!("cargo:rustc-link-arg=--omagic");
}
