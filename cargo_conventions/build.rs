use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("generated.rs");

    let mut file = File::create(&dest_path).expect("Could not create file");
    file.write_all(b"pub fn get_message() -> &'static str { \"Hello from build.rs!\" }")
        .expect("Could not write to file");

    println!("cargo::rerun-if-changed=build.rs");
}
