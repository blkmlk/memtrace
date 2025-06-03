mod flamegraph;

use crate::flamegraph::build_flamegraph;
use anyhow::Context;
use clap::Parser;
use memtrack_utils::interpret::Interpreter;
use std::fs::{remove_file, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

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

    let Some(cargo_home) = env::var_os("CARGO_HOME") else {
        anyhow::bail!("missing $CARGO_HOME");
    };

    let lib_path = PathBuf::from(cargo_home)
        .join("lib")
        .join("libmemtrack.dylib");

    load_lib_if_needed(&lib_path).context("failed to load library")?;

    let pid = std::process::id();
    let trace_filepath = format!("/tmp/{}.trace", pid);

    let mut interpret = Interpreter::new(&trace_filepath).context("failed to create trace file")?;

    let cwd = std::env::current_dir().context("failed to get current directory")?;

    interpret
        .exec(opt.cmd, opt.args, cwd, lib_path.to_str().unwrap())
        .context("failed to execute process")?;

    let data = memtrack_utils::parser::Parser::new()
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

fn load_lib_if_needed(path: impl AsRef<Path>) -> Result<(), anyhow::Error> {
    if path.as_ref().is_file() {
        return Ok(());
    }
    println!("Loading flamegraph from {}", path.as_ref().display());

    fs::create_dir_all(path.as_ref().parent().unwrap()).context("failed to create dirs")?;

    let mut response = reqwest::blocking::get(
        "https://github.com/blkmlk/memtrack-rs/releases/download/v0.1.1/libmemtrack.dylib",
    )
    .context("failed to download libmemtrack.dylib")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "failed to download libmemtrack.dylib. status: {}",
            response.status()
        );
    }

    let mut out_file = BufWriter::new(File::create(path).context("failed to create output file")?);

    io::copy(&mut response, &mut out_file).context("failed to write output file")?;

    println!("Successfully loaded libmemtrack.dylib");

    Ok(())
}
