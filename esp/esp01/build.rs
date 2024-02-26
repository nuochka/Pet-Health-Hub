//! Build script
//!
//! This script provides necessary files to linker scripts, which define memory layout
//! and sections for ESP8266EX target. If 'RAMONLY' feature is defined, it aliases out
//! FLASH memory and makes linker to write all code to RAM memory only.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Main linker script file entry.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("ld/linker.ld"))
        .unwrap();

    // Main memory definitions script
    File::create(out.join("memory.ld"))
        .unwrap()
        .write_all(include_bytes!("ld/memory.ld"))
        .unwrap();
 

    // Memory regions aliases
    File::create(out.join("alias.ld"))
        .unwrap()
        .write_all(if cfg!(feature = "RAMONLY") {
            include_bytes!("ld/ram.ld")
        } else {
            include_bytes!("ld/rom.ld")
        })
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");
}
