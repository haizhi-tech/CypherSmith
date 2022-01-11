fn compile_protos_to_rust() {
    let protos_path = concat!(env!("CARGO_MANIFEST_DIR"), "/protos");
    let mut protos: Vec<String> = vec![];
    for dir_entry in std::fs::read_dir(protos_path).unwrap().into_iter().flatten() {
        if let Some(file_ext) = std::path::Path::new(&dir_entry.file_name()).extension() {
            if file_ext == "proto" {
                protos.push(dir_entry.file_name().to_str().unwrap().to_string());
            }
        }
    }

    tonic_build::configure()
        .out_dir("src/protos_code_gen")
        .compile(&protos, &[protos_path.to_string()])
        .unwrap();

    let mut lib_rs = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("src/protos_code_gen/mod.rs")
        .unwrap();
    protos.sort_unstable();
    for proto in protos {
        let line = format!(
            "pub mod {};\n",
            std::path::Path::new(&proto).file_stem().unwrap().to_str().unwrap()
        );
        std::io::Write::write_all(&mut lib_rs, line.as_bytes()).unwrap();
    }
}

fn main() {
    compile_protos_to_rust();

    // Tells cargo to only rebuild if the proto file changed
    println!("cargo:rerun-if-changed=protos");
}
