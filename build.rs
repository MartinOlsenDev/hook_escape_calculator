use std::env;
use std::path::PathBuf;

fn main() {
    let ui_name = String::from("hook_calculator_ui");
    let ui_dir = String::from("src/ui");
    let inpath: String = format!("{}/{}.fl", &ui_dir, &ui_name);
    println!("cargo:rerun-if-changed={}", &inpath);
    let inpath: PathBuf = inpath.into();
    let outpath = PathBuf::from(env::var("OUT_DIR").unwrap()).join(format!("{}.rs", ui_name));
    let g = fl2rust::Generator::default();
    g.in_out(inpath, outpath)
        .expect("Failed to generate rust from fl file!");
}
