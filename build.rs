use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let test_var = env::var("TEST_VAR").unwrap_or("not set".to_string());
    let out_path = Path::new(&out_dir).join("test_var.rs");
    let mut f = File::create(&out_path).unwrap();

    f.write_all(format!("
        static TEST_VAR: &'static str = \"{}\";
    ", test_var).as_bytes()).unwrap();
}
