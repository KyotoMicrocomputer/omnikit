#![expect(unsafe_code, reason = "loading a dynamic library runs foreign code")]

use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

const SAMPLE: &str = cfg_select! {
    unix => "libsample.so",
    windows => "sample.dll",
};

const OMNIKIT: &str = cfg_select! {
    unix => "libomnikit.so",
    windows => "omnikit.dll",
};

fn main() {
    if env::var_os("OMNIKIT_LOAD_CHILD").is_none() {
        // Find the Cargo build output directory
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let target = env::var_os("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| workspace.join("target"));
        let target = if target.is_absolute() {
            target
        } else {
            workspace.join(target)
        };
        let output = target.join("debug");

        // [tagref redacted]
        // Use it `libomnikit.so` is provided externally
        let omnikit_dir = env::var_os("OMNIKIT_DIR").map(PathBuf::from);
        match &omnikit_dir {
            Some(dir) => {
                eprintln!("using {OMNIKIT} from {dir:?}");
            }
            None => {
                eprintln!("building {OMNIKIT} using `omni_api_stub`");
            }
        }

        // `cargo test` does not produce cdylib output for library crates.
        // Invoke Cargo to build shared libraries required for tests.
        let mut cmd = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
        let status = cmd
            .current_dir(workspace)
            .args(["build", "-p", "sample"])
            .args(if omnikit_dir.is_some() {
                &[][..]
            } else {
                // Build a stub if `libomnikit.so` is not provided
                &["-p", "omni_api_stub"]
            })
            .status()
            .unwrap_or_else(|e| panic!("failed to spawn {cmd:?}: {e}"));
        assert!(status.success(), "command {cmd:?} failed");

        let library_path_var = match () {
            #[cfg(windows)]
            () => "PATH",
            #[cfg(unix)]
            () => "LD_LIBRARY_PATH",
        };

        // Append the build output directory to `LD_LIBRARY_PATH`
        let mut library_paths = vec![if let Some(dir) = &omnikit_dir {
            dir.clone()
        } else {
            output.clone()
        }];
        if let Some(old) = env::var_os(library_path_var) {
            library_paths.extend(env::split_paths(&old));
        }
        let library_paths = env::join_paths(library_paths).unwrap();

        // Changing `LD_LIBRARY_PATH` in this process is too late at this point.
        // Launch it again to apply the change in `LD_LIBRARY_PATH`.
        let mut cmd = Command::new(env::current_exe().unwrap());
        let status = cmd
            .env("OMNIKIT_LOAD_CHILD", "1")
            .env(library_path_var, library_paths)
            .arg(output.join(SAMPLE))
            .status()
            .unwrap_or_else(|e| panic!("failed to spawn {cmd:?}: {e}"));

        assert!(status.success(), "command {cmd:?} failed");

        return;
    }

    let sample = env::args_os().nth(1).expect("sample library path");

    // SAFETY: We load the plugin module but do not call their functions.
    unsafe {
        // Load the plugin module
        let plugin = libloading::Library::new(&sample)
            .unwrap_or_else(|e| panic!("failed to load {sample:?}: {e:#}"));

        let entry: libloading::Symbol<unsafe extern "C" fn()> = plugin
            .get(b"omni_plugin_load")
            .expect("find omni_plugin_load");
        println!("Successfully loaded a plugin module.");
        println!("omni_plugin_load = {:p}", entry.try_as_raw_ptr().unwrap());
    }

    // The plugin module should depend on `libomnikit.so`
    // (and not `libomnikit_link.so` [tagref redacted])
    #[cfg(target_os = "linux")]
    {
        let mut cmd = Command::new("ldd");
        let dependencies = cmd
            .arg(&sample)
            .output()
            .unwrap_or_else(|e| panic!("failed to spawn {cmd:?}: {e}"));
        assert!(dependencies.status.success(), "command {cmd:?} failed");

        let dependencies = String::from_utf8(dependencies.stdout).expect("ldd output is UTF-8");
        assert!(
            dependencies
                .lines()
                .any(|line| line.trim_start().starts_with(&format!("{OMNIKIT} => "))),
            "sample does not depend on {OMNIKIT}:\n{dependencies}"
        );
    }
}
