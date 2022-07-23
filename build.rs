fn main() {
    use std::path::PathBuf;
    use std::env;
    println!("cargo:rerun-if-changed=src/home.fl");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("src/home.fl", out_path.join("home").to_str().unwrap()).expect("Failed to generate rust from fl file!");
}
