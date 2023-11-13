use std::{
    env,

    fs::{
        read_to_string, write,
    },

    path::PathBuf,
};


fn main() {
    // Put the linker script somewhere the linker can find it.
    let out = &PathBuf::from( env::var_os("OUT_DIR").expect("Could not find output directory") );

    // Embed the link.x file in the build binary.
    let link = read_to_string("link.x").expect("Could not read linker file");

    // Create the file and write the contents to it.
    //let mut f = File::create(out.join("link.x")).expect("Could not create the linker file");
    write(out.join("link.x"), link).expect("Failed to write linker file");

    // Set the link search.
    println!("cargo:rustc-link-search={}", out.display());

    // Rerun flags.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.x");
}
