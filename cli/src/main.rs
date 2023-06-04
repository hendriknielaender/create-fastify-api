use std::{env::args_os, ffi::OsStr, path::Path, process::exit};

fn main() {
    let mut args = args_os().peekable();
    let bin_name = match args
        .next()
        .as_deref()
        .map(Path::new)
        .and_then(Path::file_stem)
        .and_then(OsStr::to_str)
    {
        Some("cargo-create-fastify-api") => {
            if args.peek().and_then(|s| s.to_str()) == Some("create-fastify-api") {
                // remove the extra cargo subcommand
                args.next();
                Some("cargo create-fastify-api".into())
            } else {
                Some("cargo-create-fastify-api".into())
            }
        }
        Some(stem) => Some(stem.to_string()),
        None => {
            eprintln!("cargo-node wrapper unable to read first argument");
            exit(1);
        }
    };

    create_fastify_api::run(args, bin_name);
}
