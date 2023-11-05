//! Build script.



#![deny(warnings)]



use std::fs::{ self, File };
use std::io::Write;
use std::path::PathBuf;



fn main() {
    // Get output directory.
    let out = PathBuf::from( std::env::var("OUT_DIR").expect("Could not get output directory.") );

    // Get current directory.
    let current = std::env::current_dir().expect("Could not get current directory.");

    // Relocate the linker.
    linker(&out, &current);

    // Build script changes.
    println!("cargo:rerun-if-changed=build.rs");

    // Linker changes.
    println!("cargo:rerun-if-changed=.linker/main.ld");
}



fn linker(out: &PathBuf, current: &PathBuf) {
    // Create the path of the output file.
    let path = out.join("memory.x");

    // Create the output file.
    let mut file = File::create(path).expect("Could not create linker output file");

    // Look for the linker file.
    let input =  current.join(".linker").join("main.ld");

    // Copy the linker.
    file.write_all(&fs::read(input).expect("Could not read content of 'main.ld'")).expect("Failed to write 'main.ld' to output file");

    println!("cargo:rustc-link-search={}", out.display());
}
