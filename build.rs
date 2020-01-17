use std::{
    env,
    path::{Path, PathBuf},
    time::Instant,
};

use rust_swig::{CppConfig, CppOptional, CppStrView, CppVariant, LanguageConfig};

fn main() {
    env_logger::init();

    let out_dir = env::var("OUT_DIR").expect("no OUT_DIR, but cargo should provide it");
    // let jni_h_path = Path::new("/");
    let include_dirs = [Path::new("/usr/local/include")];

    gen_binding(
        &include_dirs[..],
        &Path::new(&out_dir).join("cpp_c_header.rs"),
    )
    .expect("gen_binding failed");

    let now = Instant::now();
    let gen_path = Path::new(&out_dir).join("lib.rs");
    rust_swig_expand(Path::new("src/lib.rs.in"), &gen_path);
    let expand_time = now.elapsed();
    println!(
        "rust swig expand time: {}",
        expand_time.as_secs() as f64 + (expand_time.subsec_nanos() as f64) / 1_000_000_000.
    );
    println!("cargo:rerun-if-changed=src");
    //rebuild if user removes generated code
    println!("cargo:rerun-if-changed={}", gen_path.display());
}

fn search_file_in_directory<P: AsRef<Path>>(dirs: &[P], file: &str) -> Result<PathBuf, ()> {
    for dir in dirs {
        let dir = dir.as_ref().to_path_buf();
        let file_path = dir.join(file);
        if file_path.exists() && file_path.is_file() {
            return Ok(file_path);
        }
    }
    Err(())
}

fn gen_binding<P: AsRef<Path>>(
    include_dirs: &[P],
    // c_file_path: &Path,
    output_rust: &Path,
) -> Result<(), String> {
    let bindings: bindgen::Builder = bindgen::Builder::default()
        //bindgen::builder().header(c_file_path.to_str().unwrap());
        // bindings = include_dirs.iter().fold(bindings, |acc, x| {
        //     acc.clang_arg("-I".to_string() + x.as_ref().to_str().unwrap())
        // });
        .clang_args(
            include_dirs
                .iter()
                .map(|x| "-I".to_string() + x.as_ref().to_str().unwrap()),
        );

    println!("Bindings: {:?}", &bindings);

    let generated_bindings = bindings
        .generate()
        .map_err(|_| "Failed to generate bindings".to_string())?;

    println!("Generated bindings: {:?}", &generated_bindings);

    generated_bindings
        .write_to_file(output_rust)
        .map_err(|err| err.to_string())?;

    Ok(())
}

fn rust_swig_expand(from: &Path, out: &Path) {
    println!("Run rust_swig_expand");
    let cpp_cfg = CppConfig::new(
        // get_cpp_codegen_output_directory(),
        Path::new("..").join("cpp-part").join("rust-api"),
        "sdk".into(),
    )
    .cpp_optional(CppOptional::Boost)
    .cpp_variant(CppVariant::Boost)
    .cpp_str_view(CppStrView::Boost);

    let swig_gen = rust_swig::Generator::new(LanguageConfig::CppConfig(cpp_cfg))
        .merge_type_map("chrono_support", include_str!("src/chrono-include.rs"));
    swig_gen.expand("c++-api-for-ironoxide", from, out);
    // swig_gen.expand("rust_swig_test_jni", from, out);
}

fn get_cpp_codegen_output_directory() -> PathBuf {
    let path = Path::new("cpp").join("sdk");
    if !path.exists() {
        std::fs::create_dir_all(&path)
            .expect("Couldn't create codegen output directory at cpp/sdk/.");
    }
    path.to_path_buf()
}
