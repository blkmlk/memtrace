//!
//! # MemTrace
//!
//!
//! ![example](example.svg)
//!
//! A Rust-based tool for visualizing heap memory consumption using flamegraphs. It helps you profile your app.
//!
//! The tool is using the [flamegraph](https://github.com/flamegraph-rs/flamegraph) crate for building flamegraphs
//!
//! > ℹ️ **Info:** So far, the tool works only on MacOS.
//!
//! > ⚠️ **Warning:** At the moment, this tool requires downloading a dynamic library to function. The library is open source and can be found [here](https://github.com/blkmlk/memtrace-lib).
//! >
//! > If you know a better solution - feel free to open a pull request
//!
//! ## Installation
//!
//! ```bash
//! cargo install memtrace
//! ```
//!
//! ## Usage
//! ```bash
//! memtrace <your_program>
//! ```

mod flamegraph;

use crate::flamegraph::build_flamegraph;
use anyhow::Context;
use clap::Parser;
use memtrace_utils::common::download_lib_if_needed;
use memtrace_utils::interpret::Interpreter;
use std::env;
use std::fs::remove_file;
use std::path::PathBuf;

#[cfg(not(target_os = "macos"))]
compile_error!("This binary works only on MacOS!");

const LIB_VERSION: &str = "v0.2.0";

#[derive(Parser)]
struct Opt {
    #[clap(short, long, default_value = "false")]
    no_open: bool,
    #[clap(short, long)]
    out_file: Option<PathBuf>,

    cmd: String,
    args: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    let Some(home) = env::var_os("HOME") else {
        anyhow::bail!("missing $HOME");
    };

    let lib_dir = PathBuf::from(home).join(".cargo").join("lib");

    let lib_path =
        download_lib_if_needed(&lib_dir, LIB_VERSION).context("failed to load library")?;

    let pid = std::process::id();
    let trace_filepath = format!("/tmp/{}.trace", pid);

    let mut interpret = Interpreter::new(&trace_filepath).context("failed to create trace file")?;

    let cwd = std::env::current_dir().context("failed to get current directory")?;

    interpret
        .exec(opt.cmd, opt.args, cwd, &lib_path)
        .context("failed to execute process")?;

    let data = memtrace_utils::parser::Parser::new()
        .parse_file(&trace_filepath)
        .context("failed to parse trace file")?;

    let output_file = if let Some(file) = opt.out_file {
        PathBuf::from(file)
    } else {
        PathBuf::from(format!("/tmp/flamegraph_{}.svg", pid))
    };

    build_flamegraph(data, &output_file).context("failed to build flamegraph")?;

    println!(
        "Successfully stored memory flamegraph to {}",
        output_file.display()
    );

    if !opt.no_open {
        open::that(output_file).context("failed to open output file")?;
    }

    remove_file(trace_filepath).context("failed to remove trace file")?;

    Ok(())
}
