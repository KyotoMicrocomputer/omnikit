use std::{env, fmt::Write as _, fs, path::PathBuf};

fn main() {
    // Find `Omni-*.gir`
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let meta_dir = manifest_dir.join("../../meta");
    let symbols = omni_api_build::api_symbols(&meta_dir);

    // Emit code
    let mut output = String::new();
    for (index, symbol) in symbols.iter().enumerate() {
        writeln!(
            output,
            r#"#[unsafe(export_name = {symbol:?})]
            pub extern "C" fn stub_{index}() -> ! {{
                called({symbol:?});
            }}"#
        )
        .unwrap();
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("stubs.rs"), output).expect("write generated stubs");
}
