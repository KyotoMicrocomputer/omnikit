#![doc = include_str!("../README.md")]
use std::{
    env,
    io::{BufRead, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use xot::Xot;

const CORE_NS: &str = "http://www.gtk.org/introspection/core/1.0";
const C_NS: &str = "http://www.gtk.org/introspection/c/1.0";
const GLIB_NS: &str = "http://www.gtk.org/introspection/glib/1.0";

/// Enumerate the C symbols exported by `libomnikit.so`.
///
/// This function emits Cargo build script directives to stdout.
pub fn api_symbols(meta_dir: &Path) -> Vec<String> {
    // Find `Omni-*.gir`
    let mut matches = std::fs::read_dir(meta_dir)
        .expect("read meta directory")
        .map(|entry| entry.expect("read meta entry").path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("Omni-") && name.ends_with(".gir"))
        })
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "expected exactly one meta/Omni-*.gir");
    let gir = matches.pop().unwrap();
    println!("cargo:rerun-if-changed={}", gir.display());

    // Read `Omni-*.gir`
    let xml = std::fs::read_to_string(&gir).expect("read GIR");
    let mut xot = Xot::new();
    let document = xot.parse(&xml).expect("parse GIR");
    let c_identifier = xot
        .name_ns("identifier", xot.namespace(C_NS).unwrap())
        .unwrap();
    let glib_get_type = xot
        .name_ns("get-type", xot.namespace(GLIB_NS).unwrap())
        .unwrap();

    // Enumerate C symbol names
    let mut symbols = Vec::new();

    for node in xot.descendants(document) {
        let Some(element) = xot.element(node) else {
            continue;
        };
        let (local_name, namespace) = xot.name_ns_str(element.name());
        if namespace != CORE_NS {
            continue;
        }

        // Callables have their exported C symbol names in `c:identifier`.
        if matches!(local_name, "function" | "method" | "constructor") {
            let symbol = xot
                .get_attribute(node, c_identifier)
                .unwrap_or_else(|| panic!("{local_name} has no c:identifier"));
            symbols.push(symbol.to_owned());
        }

        // GLib types also name their registration functions in `glib:get-type`.
        if let Some(symbol) = xot.get_attribute(node, glib_get_type) {
            symbols.push(symbol.to_owned());
        }
    }

    symbols.sort_unstable();
    symbols.dedup();
    assert!(!symbols.is_empty(), "no C functions found in GIR");

    for symbol in &symbols {
        assert!(
            symbol.starts_with("omni_")
                && symbol
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_'),
            "unexpected C symbol: {symbol}"
        );
    }

    symbols
}

pub trait Output {
    /// Get the directory to store generated binding code in.
    fn out_dir(&mut self) -> PathBuf;

    /// Register an environment variable as an input dependency.
    fn rerun_if_env_changed(&mut self, name: &str) {
        _ = name;
    }

    /// `spawn` and `wait` a given [`Command`].
    ///
    /// Panics if `spawn` or `wait` fails or the command exits with an
    /// unsuccessful exit status.
    fn spawn_and_wait_command(&mut self, cmd: &mut Command) {
        let status = cmd
            .spawn()
            .unwrap_or_else(|e| panic!("command {cmd:?} failed to start: {e}"))
            .wait()
            .unwrap_or_else(|e| panic!("failed to wait for command {cmd:?} to end: {e}"));
        if !status.success() {
            panic!("command {cmd:?} exited with {status}");
        }
    }
}

pub struct BuildScriptOutput;

impl Output for BuildScriptOutput {
    fn out_dir(&mut self) -> PathBuf {
        env::var_os("OUT_DIR").expect("OUT_DIR is not set").into()
    }

    fn rerun_if_env_changed(&mut self, name: &str) {
        println!("cargo:rerun-if-env-changed={name}");
    }

    fn spawn_and_wait_command(&mut self, cmd: &mut Command) {
        // A Cargo build script's stderr is normally hidden
        spawn_and_forward_stderr_as_warnings(cmd);
    }
}

/// The type of GIR binding to generate in [`generate_bindings`].
pub enum BindingTy {
    /// Rust API binding (safe wrapper for functions and interfaces)
    Api,
    /// FFI binding (`extern "C"`, raw structs, ...)
    Ffi,
}

/// Run common build steps for `omni_api[_sys]`.
pub fn generate_bindings(ty: BindingTy, out: &mut dyn Output) {
    let pkg_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let out_dir = out.out_dir();

    // Read artifacts from `omni_api`
    let gir_dir = pkg_dir.parent().unwrap().parent().unwrap().join("meta");
    let (gir_namespace, gir_version) = std::fs::read_dir(&gir_dir)
        .unwrap_or_else(|e| panic!("failed to read {gir_dir:?}: {e}"))
        .find_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().into_string().unwrap();
            let (namespace, version) = file_name.strip_suffix(".gir")?.split_once('-').unwrap();
            Some((namespace.to_owned(), version.to_owned()))
        })
        .unwrap();

    // Generate a `gir` configuration file
    let mut gir_conf = tempfile::NamedTempFile::new().expect("NamedTempFile::new");
    let glib_gir_path = pkg_dir.join("../../vendor/gir-files");
    assert!(
        glib_gir_path.join("GLib-2.0.gir").is_file(),
        "directory '{}' does not contain GIR files. please make sure you have run 'git submodule \
         update --init'",
        glib_gir_path.display(),
    );
    #[expect(
        clippy::write_literal,
        reason = "avoid misrecognition of this file as generated"
    )]
    (|| -> std::io::Result<()> {
        // `Gir.toml` ends with a `[options]` section, to which we'll append
        // [ref:api_rs_gir_options_autogen]
        write!(
            gir_conf,
            r#"# {at}generated by `{pkg_name}`
            {partial}
            library = "{gir_namespace}"
            version = "{gir_version}"
            target_path = {out_dir:?}
            girs_directories = [{gir_dir:?}, {glib_gir_path:?}]
            work_mode = "{work_mode}"
            "#,
            at = '@',
            partial = include_str!("../../omni_api_rs/Gir.toml"),
            pkg_name = env!("CARGO_PKG_NAME"),
            work_mode = match ty {
                BindingTy::Api => "normal",
                BindingTy::Ffi => "sys",
            },
        )?;
        gir_conf.flush()?;
        Ok(())
    })()
    .unwrap_or_else(|e| panic!("failed to write '{}': {e}", gir_conf.path().display()));
    let gir_conf_path = gir_conf.into_temp_path();

    // `gir` log level
    let gir_log_var = "GIR_LOG";
    let gir_log_level = env::var_os(gir_log_var);
    out.rerun_if_env_changed(gir_log_var);

    // Invoke `gir`
    //
    // It should be somehow made available by the developer.
    // [ref:api_build_prerequisite_gir]
    // Allow overriding via an environment variable.
    let gir = env::var_os("GIR").unwrap_or_else(|| "gir".into());
    let mut cmd = Command::new(&gir);
    if let Some(x) = gir_log_level {
        cmd.env("RUST_LOG", x);
    } else {
        cmd.env_remove("RUST_LOG");
    }
    spawn_and_forward_stderr_as_warnings(cmd.arg("-c").arg(&gir_conf_path));

    // Check for unbound API
    match ty {
        BindingTy::Api => {
            let mut cmd = Command::new(&gir);
            let output = cmd
                .arg("-c")
                .arg(&gir_conf_path)
                .args(["-m", "not_bound"])
                .stdout(Stdio::piped())
                .output()
                .unwrap_or_else(|_| panic!("failed to spawn command {cmd:?}"));
            assert!(
                output.status.success(),
                "command {cmd:?} exited with {}",
                output.status
            );

            std::io::stdout().write_all(&output.stdout).unwrap();
            assert!(
                output.stdout.is_empty(),
                "could not generate bindings for some items"
            );
        }
        BindingTy::Ffi => {}
    }

    // Write API symbol list
    // [tag:api_build_symbol_list]
    match ty {
        BindingTy::Ffi => {
            let symbols_path = out_dir.join("api_symbols.txt");
            let symbols = api_symbols(&gir_dir).join("\n") + "\n";
            std::fs::write(&symbols_path, symbols)
                .unwrap_or_else(|e| panic!("failed to write {symbols_path:?}: {e}"));
        }
        BindingTy::Api => {}
    }

    // Wrap `gir`-generated code with a `patch_auto!` invocation
    // [ref:api_rs_patch_auto]
    match ty {
        BindingTy::Api => {
            let auto_dir = out_dir.join("src").join("auto");
            for entry in std::fs::read_dir(&auto_dir)
                .unwrap_or_else(|e| panic!("failed to scan {auto_dir:?}: {e}"))
            {
                let entry = entry.unwrap_or_else(|e| panic!("failed to scan {auto_dir:?}: {e}"));
                let path = entry.path();
                if path.extension().is_some_and(|x| x == "rs") {
                    let mut data = std::fs::read_to_string(&path)
                        .unwrap_or_else(|e| panic!("failed to read {path:?}: {e}"));

                    if data.contains("patch_auto!") {
                        continue;
                    }

                    data = format!("patch_auto! {{ {:?}; {data} }}", path.file_name().unwrap());

                    std::fs::write(&path, data)
                        .unwrap_or_else(|e| panic!("failed to read {path:?}: {e}"));
                }
            }
        }
        BindingTy::Ffi => {}
    }
}

fn spawn_and_forward_stderr_as_warnings(cmd: &mut Command) {
    let mut child = cmd
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("command {cmd:?} failed to start: {e}"));
    let stderr = std::io::BufReader::new(child.stderr.take().unwrap());
    let forward_thread = std::thread::spawn(move || {
        for line in stderr.lines().map_while(Result::ok) {
            println!("cargo:warning={line}");
        }
    });
    let status = child
        .wait()
        .unwrap_or_else(|e| panic!("failed to wait for command {cmd:?} to end: {e}"));
    if !status.success() {
        panic!("command {cmd:?} exited with {status}");
    }
    forward_thread.join().unwrap();
}
