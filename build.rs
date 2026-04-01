use std::env;
use std::path::PathBuf;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "x86_64" {
        let asm_files = [
            "asm/hash_crc64.asm",
            "asm/memcpy_erms.asm",
            "asm/header_scan_avx2.asm",
        ];

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        for file in asm_files.iter() {
            let obj_file = out_dir.join(format!(
                "{}.o",
                file.trim_end_matches(".asm").replace("/", "_")
            ));

            // Compilar con nasm
            let status = std::process::Command::new("nasm")
                .args(&[
                    "-f",
                    "elf64",
                    "-o",
                    obj_file.to_str().unwrap(),
                    file,
                ])
                .status()
                .expect("Failed to execute nasm");
            if !status.success() {
                panic!("nasm failed for {}", file);
            }

            // Enlazar el objeto
            println!("cargo:rustc-link-search=native={}", out_dir.display());
            println!("cargo:rustc-link-lib=static={}", obj_file.file_stem().unwrap().to_str().unwrap());
        }

        // Enlazar las bibliotecas estáticas generadas
        println!("cargo:rustc-link-lib=static=hash_crc64");
        println!("cargo:rustc-link-lib=static=memcpy_erms");
        println!("cargo:rustc-link-lib=static=header_scan_avx2");
    } else {
        println!("cargo:warning=Assembly optimizations only available on x86_64");
    }
}