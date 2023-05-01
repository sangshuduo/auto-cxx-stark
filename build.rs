fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    let libstark_path = std::path::PathBuf::from("../libSTARK");
    let bair_path = std::path::PathBuf::from("../libSTARK/libstark/src/languages/Bair/");
    let algebralib_path =
        std::path::PathBuf::from("../libSTARK/algebra/algebralib/headers/");
    let fft_path = std::path::PathBuf::from("../libSTARK/algebra/FFT/src/");
    let omp_path = std::path::PathBuf::from("/usr/local/opt/libomp/include/");
    let libstark_src_path = std::path::PathBuf::from("../libSTARK/libstark/src");

    let mut b = autocxx_build::Builder::new(
        "src/main.rs",
        [
            &path,
            &libstark_path,
            &bair_path,
            &algebralib_path,
            &fft_path,
            &omp_path,
            &libstark_src_path,
        ],
    )
    .build()?;
    b.flag_if_supported("-std=c++14").compile("auto-cxx-stark");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/input.h");
    Ok(())
}
