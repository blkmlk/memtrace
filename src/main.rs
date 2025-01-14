mod flamegraph;

use crate::flamegraph::build_flamegraph;
use anyhow::Context;
use clap::Parser;
use memtrack::common::interpret::Interpreter;
use std::fs::remove_file;
use std::path::PathBuf;

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

    let pid = std::process::id();
    let trace_filepath = format!("/tmp/{}.trace", pid);

    let mut interpret = Interpreter::new(&trace_filepath).context("failed to create trace file")?;

    let cwd = std::env::current_dir().context("failed to get current directory")?;

    let lib_path = env!("LIB_PATH");

    interpret.exec(opt.cmd, opt.args, cwd, lib_path).context("failed to execute process")?;

    let data = memtrack::common::parser::Parser::new()
        .parse_file(&trace_filepath)
        .context("failed to parse trace file")?;

    let output_file = if let Some(file) = opt.out_file {
        PathBuf::from(file)
    } else {
        PathBuf::from("/tmp/flamegraph.svg")
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
