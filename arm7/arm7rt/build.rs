use cc::Build;

use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());
    File::create(out_dir.join("link.ld"))?.write_all(include_bytes!("link.ld"))?;
    //Build::new().file("asm.s").compile("asm"); // <- NEW!
    //println!("cargo:rerun-if-changed=asm.s"); // <- NEW!
    Ok(())
}
