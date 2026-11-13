use std::{env, fmt::Write as _, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // The directory where `omni_api_build::generate_bindings`'s output is
    // stored. Assume [tagref redacted] [tagref redacted] first.
    let mut gen_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    gen_dir.push("gen");

    if !gen_dir.is_dir() {
        cfg_select! {
            feature = "_internal_generate" => {
                // The bindings are missing. We are in
                // [tagref redacted], so generate bindings.
                omni_api_build::generate_bindings(
                    omni_api_build::BindingTy::Ffi,
                    &mut omni_api_build::BuildScriptOutput,
                );
                gen_dir.clone_from(&out_dir);
            }
            _ => {
                eprintln!(
                    "error: broken OmniKit installation: bindings are missing, and binding \
                     generator is disabled"
                );
                std::process::exit(1);
            }
        }
    }

    // Read the symbol list
    // [tagref redacted]
    let symbols_path = gen_dir.join("api_symbols.txt");
    let symbols = std::fs::read_to_string(&symbols_path)
        .unwrap_or_else(|e| panic!("failed to read {symbols_path:?}: {e}"));

    // The SDK is built without access to real `libomnikit.so`.
    // Create an import library supplying its symbols and SONAME.
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "linux" => {
            // Generate a C source file defining all plugin API symbols
            let mut source = "#include <stdlib.h>\n".to_owned();
            for symbol in symbols.lines() {
                writeln!(&mut source, "void {symbol}(void) {{ exit(0); }}").unwrap();
            }
            let source_path = out_dir.join("omnikit-link.c");
            std::fs::write(&source_path, source).expect("write link-time placeholder source");

            // Build a stub shared library
            //
            // [tagref redacted]
            // Explicitly sepcify the SONAME `libomnikit.so` and give the
            // stub library a different file name. This ensures that, when a
            // user is running Cargo on a downstream crate, binaries built by
            // Cargo do not accidentally load this stub library.
            let import_lib = out_dir.join("libomnikit_link.so");
            let mut cmd = std::process::Command::new("cc");
            let status = cmd
                .args(["-shared", "-fPIC", "-Wl,-soname,libomnikit.so"])
                .arg(&source_path)
                .arg("-o")
                .arg(&import_lib)
                .status()
                .unwrap_or_else(|e| panic!("failed to spawn {cmd:?}: {e}"));
            assert!(status.success(), "command {cmd:?} failed");

            println!("cargo:rustc-link-search=native={}", out_dir.display());
            println!("cargo:rustc-link-lib=dylib=omnikit_link");
        }
        "windows" => {
            // Create a .def file listing all symbols
            let def_path = out_dir.join("omnikit.def");
            let mut def = String::from("LIBRARY omnikit.dll\nEXPORTS\n");
            for symbol in symbols.lines() {
                writeln!(&mut def, "    {symbol}").unwrap();
            }
            std::fs::write(&def_path, def).expect("write omnikit.def");

            // Create an import library using dlltool
            let import_lib = out_dir.join("libomnikit.dll.a");
            let mut cmd = std::process::Command::new("dlltool");
            let status = cmd
                .args(["-d", &def_path.display().to_string()])
                .args(["-l", &import_lib.display().to_string()])
                .args(["-m", "i386:x86-64"])
                .status()
                .unwrap_or_else(|e| panic!("failed to spawn {cmd:?}: {e}"));
            assert!(status.success(), "command {cmd:?} failed");

            println!("cargo:rustc-link-search=native={}", out_dir.display());
            println!("cargo:rustc-link-lib=dylib=omnikit");
        }
        _ => {
            println!(
                "cargo:warning=don't know how to create import library for this platform. link \
                 error may occur"
            );
        }
    }

    // Generate source files to import the `gir`-generated modules.
    // (`#[path = ...] mod ident;` can't use `concat!` or `env!` in the path)
    //
    //                 this part of code
    //                    in build.rs               gir
    // src/lib.rs              |                     |
    //     |                   v                     |
    //     '----------> $OUT_DIR/gen.rs              |
    //   include!              |                     v
    //                         '----------> $gen_dir/src/auto.rs
    //                    #[path = ...]
    //                    mod lib;
    //
    // The file to be included in `lib.rs`
    let proxy_path = out_dir.join("gen.rs");

    // The root module generated by `generate_bindings`
    let target_path = gen_dir.join("src/lib.rs");

    std::fs::write(&proxy_path, format!("#[path = {target_path:?}] mod lib;"))
        .unwrap_or_else(|_| panic!("failed to write '{}'", proxy_path.display()));

    println!("cargo:rustc-env=GEN_PATH={}", proxy_path.display());
}
