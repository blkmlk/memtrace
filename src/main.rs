mod flamegraph;

use crate::flamegraph::build_flamegraph;
use anyhow::Context;
use clap::Parser;
use memtrack_utils::common::download_lib_if_needed;
use memtrack_utils::interpret::Interpreter;
use std::env;
use std::fs::remove_file;
use std::path::PathBuf;

const LIB_VERSION: &str = "v0.1.0";

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

    let lib_dir = PathBuf::from(cargo_home).join("lib");

    let lib_path =
        download_lib_if_needed(&lib_dir, LIB_VERSION).context("failed to load library")?;

    let pid = std::process::id();
    let trace_filepath = format!("/tmp/{}.trace", pid);

    let mut interpret = Interpreter::new(&trace_filepath).context("failed to create trace file")?;

    let cwd = std::env::current_dir().context("failed to get current directory")?;

    interpret
        .exec(opt.cmd, opt.args, cwd, &lib_path)
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
