use std::env;

fn main() {
    if Ok("thumbv7m-none-eabi".to_owned()) == env::var("TARGET") {
        println!("cargo:rustc-link-arg-bins=--nmagic");
        println!("cargo:rustc-link-arg-bins=-Tlink.x");
        println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
    }
}
