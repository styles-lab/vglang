fn ml_gen() {
    use std::path::PathBuf;

    use mlang_rs::lang::{compile, rustgen::CodeGen};

    let target = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ml");

    compile(
        include_str!("./vglang.ml"),
        CodeGen::default().target(target),
    )
    .unwrap();
}

fn main() {
    if !std::env::var("DOCS_RS").is_ok() {
        println!("cargo::rerun-if-changed=vglang.ml");
        ml_gen();
    }
}
