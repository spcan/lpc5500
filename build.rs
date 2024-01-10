//! Build script for all LPC5500 variants.



#![allow(unreachable_code)]



use std::{
    fs::File,
    io::Read,
};


fn main() {
    // Build the linker script based on the feature flags.
    buildscript();

    // Rerun flags.
    println!("cargo:rerun-if-changed=build.rs");
}




/// Creates the build script based on the active configuration flags.
fn buildscript() {
    use std::{
        io::{ BufWriter, Write, },
        path::PathBuf,
    };

    // Get the output path of the linkerscript.
    let path = PathBuf::from( std::env::var_os( "OUT_DIR" ).expect( "Could not find output path: env var \"OUT_DIR\" is not set" ) );

    // Create the output file, overwriting if needed.
    let file = File::create( path.join("link.x") ).expect( "Could not create output linker file" );

    // Create the 'MEMORY' string (1 kB) and the 'SECTIONS' string (7 kB) and the 'ASSERT' string (1 kB).
    let mut memory   = Vec::with_capacity(1 * 1024);
    let mut sections = Vec::with_capacity(7 * 1024);
    let mut asserts  = Vec::with_capacity(1 * 1024);

    // Build the FLASH configuration.
    buildflash(&mut memory, &mut sections, &mut asserts);

    // Build the core configuration.
    buildcores(&mut memory, &mut sections, &mut asserts);

    // Build the RAM configuration.
    buildram(&mut memory, &mut sections, &mut asserts);

    // Build the miscellaneous configuration.
    buildmisc(&mut memory, &mut sections, &mut asserts);

    // Wrap the file in a buffered writer (8 kB) to avoid excessivo I/O.
    let mut writer = BufWriter::with_capacity(8 * 1024, file);

    // Write the declarations section.
    {
        write!(&mut writer, "ENTRY( Reset );").expect( "Failed to write the entry" );
    }

    // Write the MEMORY section.
    {
        write!(&mut writer, "MEMORY {{\n").expect("Failed to write to linker script output file");
        writer.write_all(&memory).expect("Failed to write to linker script output file");
        write!(&mut writer, "}}\n\n").expect("Failed to write to linker script output file");
    }

    // Write the SECTIONS section.
    {
        write!(&mut writer, "SECTIONS {{\n").expect("Failed to write to linker script output file");
        writer.write_all(&sections).expect("Failed to write to linker script output file");
        write!(&mut writer, "}}\n\n").expect("Failed to write to linker script output file");
    }

    // Write the ASSERTS section.
    {
        writer.write_all(&asserts).expect("Failed to write to linker script output file");
    }

    // 

    // Set the link search.
    println!("cargo:rustc-link-search={}", path.display());
}



/// Builds the FLASH configuration linker script based on the active feature flags.
fn buildflash(memory: &mut Vec<u8>, sections: &mut Vec<u8>, asserts: &mut Vec<u8>) {
    // Ensure that at least one flash feature flag is active.
    #[cfg(not(any( feature = "flash-640", feature = "flash-512", feature = "flash-256" )))]
    panic!("One FLASH feature flag must be active");

    // Select the 'memory.ld' file path based on the active feature flags.
    #[cfg(feature = "flash-640")]
    let path = ".linker/flash/memory/640.ld";

    #[cfg(feature = "flash-512")]
    let path = ".linker/flash/memory/512.ld";

    #[cfg(feature = "flash-256")]
    let path = ".linker/flash/memory/256.ld";

    // Copy the 'memory.ld' and 'sections.ld' files.
    File::open( path )
        .expect( "Could not open FLASH 'memory.ld' file" )
        .read_to_end( memory )
        .expect( "Could not copy FLASH 'memory.ld' file" );

    File::open( ".linker/flash/sections.ld" )
        .expect( "Could not open FLASH 'sections.ld' file" )
        .read_to_end( sections )
        .expect( "Could not copy FLASH 'sections.ld' file" );

    File::open( ".linker/flash/asserts.ld" )
        .expect( "Could not open FLASH 'asserts.ld' file" )
        .read_to_end( asserts )
        .expect( "Could not copy FLASH 'asserts.ld' file" );

    // Set the rerun flags.
    println!("cargo:rerun-if-changed=.linker/flash/memory/256.ld");
    println!("cargo:rerun-if-changed=.linker/flash/memory/512.ld");
    println!("cargo:rerun-if-changed=.linker/flash/memory/640.ld");
    println!("cargo:rerun-if-changed=.linker/flash/sections.ld");
    println!("cargo:rerun-if-changed=.linker/flash/asserts.ld");
}



/// Builds the COREs configuration linker script based on the active feature flags.
fn buildcores(memory: &mut Vec<u8>, sections: &mut Vec<u8>, asserts: &mut Vec<u8>) {
    // Build the main core's (CORE 0) configuration.
    {
        // Copy the 'memory.ld' and 'sections.ld' files.
        File::open( ".linker/main/memory.ld" )
            .expect( "Could not open CORE 0 'memory.ld' file" )
            .read_to_end( memory )
            .expect( "Could not copy CORE 0 'memory.ld' file" );

        File::open( ".linker/main/sections.ld" )
            .expect( "Could not open CORE 0 'sections.ld' file" )
            .read_to_end( sections )
            .expect( "Could not copy CORE 0 'sections.ld' file" );

        File::open( ".linker/main/asserts.ld" )
            .expect( "Could not open CORE 0 'asserts.ld' file" )
            .read_to_end( asserts )
            .expect( "Could not copy CORE 0 'asserts.ld' file" );

        // Set the rerun flags.
        println!("cargo:rerun-if-changed=.linker/main/memory.ld");
        println!("cargo:rerun-if-changed=.linker/main/sections.ld");
        println!("cargo:rerun-if-changed=.linker/main/asserts.ld");
    }

    // Build the secondary core's (CORE 1) configuration.
    #[cfg(feature = "dual-core")]
    {
        // Copy the 'memory.ld' and 'sections.ld' files.
        File::open( ".linker/sub/memory.ld" )
            .expect( "Could not open CORE 1 'memory.ld' file" )
            .read_to_end( memory )
            .expect( "Could not copy CORE 1 'memory.ld' file" );

        File::open( ".linker/sub/sections.ld" )
            .expect( "Could not open CORE 1 'sections.ld' file" )
            .read_to_end( sections )
            .expect( "Could not copy CORE 1 'sections.ld' file" );

        // Set the rerun flags.
        println!("cargo:rerun-if-changed=.linker/sub/memory.ld");
        println!("cargo:rerun-if-changed=.linker/sub/sections.ld");
        println!("cargo:rerun-if-changed=.linker/sub/asserts.ld");
    }

    // Dummy secondary core configuration.
    #[cfg(not(feature = "dual-core"))]
    {
        // Copy the 'empty.ld' file.
        File::open( ".linker/main/empty.ld" )
            .expect( "Could not open CORE 0 'empty.ld' file" )
            .read_to_end( memory )
            .expect( "Could not copy CORE 0 'empty.ld' file" );

        // Set the rerun flags.
        println!("cargo:rerun-if-changed=.linker/main/empty.ld");
    }
}



/// Builds the RAM sections of the buildscript based on the active feature flags.
fn buildram(memory: &mut Vec<u8>, sections: &mut Vec<u8>, asserts: &mut Vec<u8>) {
    // Ensure that at least one sram feature flag is active.
    #[cfg(not(any( feature = "sram-320", feature = "sram-256", feature = "sram-144" )))]
    panic!("One SRAM feature flag must be active");

    #[cfg(feature = "sram-320")]
    let path = ".linker/ram/memory/320.ld";

    #[cfg(feature = "sram-256")]
    let path = ".linker/ram/memory/256.ld";

    #[cfg(feature = "sram-144")]
    let path = ".linker/ram/memory/144.ld";

    // Copy the 'memory.ld' and 'sections.ld' files.
    File::open( path )
        .expect( "Could not open RAM 'memory.ld' file" )
        .read_to_end( memory )
        .expect( "Could not copy RAM 'memory.ld' file" );

    File::open( ".linker/ram/sections.ld" )
        .expect( "Could not open RAM 'sections.ld' file" )
        .read_to_end( sections )
        .expect( "Could not copy RAM 'sections.ld' file" );

    File::open( ".linker/ram/asserts.ld" )
        .expect( "Could not open RAM 'asserts.ld' file" )
        .read_to_end( asserts )
        .expect( "Could not copy RAM 'asserts.ld' file" );

    // Set the rerun flags.
    println!("cargo:rerun-if-changed=.linker/ram/memory/320.ld");
    println!("cargo:rerun-if-changed=.linker/ram/memory/256.ld");
    println!("cargo:rerun-if-changed=.linker/ram/memory/144.ld");
    println!("cargo:rerun-if-changed=.linker/ram/sections.ld");
    println!("cargo:rerun-if-changed=.linker/ram/asserts.ld");
}



/// Builds the miscellanous sections of the buildscript.
fn buildmisc(_: &mut Vec<u8>, sections: &mut Vec<u8>, asserts: &mut Vec<u8>) {
    // Copy the 'memory.ld', 'sections.ld' and 'asserts.ld' files.
    File::open( ".linker/misc/sections.ld" )
        .expect( "Could not open miscellanouse 'sections.ld' file" )
        .read_to_end( sections )
        .expect( "Could not copy miscellanouse 'sections.ld' file" );

    File::open( ".linker/misc/asserts.ld" )
        .expect( "Could not open miscellanouse 'asserts.ld' file" )
        .read_to_end( asserts )
        .expect( "Could not copy miscellanouse 'asserts.ld' file" );

    // Set the rerun flags.
    println!("cargo:rerun-if-changed=.linker/misc/asserts.ld");
    println!("cargo:rerun-if-changed=.linker/misc/sections.ld");
}
