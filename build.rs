use std::{
  env,
  path::{Path, PathBuf},
  time::Instant,
};

use rust_swig::{CppConfig, LanguageConfig};

fn main() {
  env_logger::init();

  let out_dir = env::var("OUT_DIR").expect("no OUT_DIR, but cargo should provide it");

  let gen_path = Path::new(&out_dir).join("lib.rs");
  let cpp_cfg = CppConfig::new(get_cpp_codegen_output_directory(), "sdk".into()); //.use_boost();

  let swig_gen = rust_swig::Generator::new(LanguageConfig::CppConfig(cpp_cfg))
    .merge_type_map("chrono_support", include_str!("src/chrono-include.rs"));

  swig_gen.expand(
    "c++-api-for-ironoxide",
    Path::new("src/lib.rs.in"),
    &gen_path,
  );

  println!("cargo:rerun-if-changed=src");
  //rebuild if user removes generated code
  println!("cargo:rerun-if-changed={}", gen_path.display());
}

fn get_cpp_codegen_output_directory() -> PathBuf {
  let out_dir = env::var("OUT_DIR").expect("no OUT_DIR, but cargo should provide it");
  let path = Path::new("cpp").join("sdk");
  if !path.exists() {
    std::fs::create_dir_all(&path).expect("Couldn't create codegen output directory at cpp/sdk/.");
  }
  println!("Output dir: {:?}", &path);
  path.to_path_buf()
}
