//! `[tag:omni_api_generate_bindings]`
//! A utility to call [`omni_api_build::generate_bindings`] and generate
//! bindings for `omni_api_{rs,sys}`.

use std::{env, path::PathBuf};

fn main() {
    let mut args = env::args_os().skip(1);
    let ty = match &*args.next().expect("ty not provided").to_string_lossy() {
        "ffi" => omni_api_build::BindingTy::Ffi,
        "api" => omni_api_build::BindingTy::Api,
        unknown => panic!("invalid binding type: {unknown:?}"),
    };
    let gen_dir = PathBuf::from(args.next().expect("gen_dir not provided"));

    std::fs::create_dir_all(&gen_dir).expect("create_dir");

    // TODO: Ensure incompletely generated binding code does not cause
    // confusing build error

    omni_api_build::generate_bindings(ty, &mut Output { out_dir: gen_dir });
}

struct Output {
    out_dir: PathBuf,
}

impl omni_api_build::Output for Output {
    fn out_dir(&mut self) -> PathBuf {
        self.out_dir.clone()
    }
}
