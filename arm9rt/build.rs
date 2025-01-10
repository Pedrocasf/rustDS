use std::{
    env,
    error::Error,
    fs::{self},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());
    fs::copy("libmpusetup.a", out_dir.join("libmpusetup.a"))?;
    println!("cargo:rustc-link-lib=static=mpusetup");
    println!("cargo:rerun-if-changed=libmpusetup.a");
    fs::copy("libamiga.a", out_dir.join("libamiga.a"))?;
    println!("cargo:rustc-link-lib=static=amiga");
    println!("cargo:rerun-if-changed=libmamiga.a");
    /*fs::copy("libsetsp.a", out_dir.join("libsetsp.a"))?;
    println!("cargo:rustc-link-lib=static=setsp");
    println!("cargo:rerun-if-changed=libsetsp.a");
    */
    Ok(())
}
