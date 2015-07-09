include!(concat!(env!("OUT_DIR"), "/test_var.rs"));


fn main() {
    println!("test var is {}", TEST_VAR);
}
