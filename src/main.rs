mod flamegraph;

use crate::flamegraph::build_flamegraph;
use anyhow::Context;
use clap::Parser;
use interpret::interpret::Interpreter;
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

fn main() {
    let opt = Opt::parse();

    let pid = std::process::id();
    let trace_filepath = format!("/tmp/{}.trace", pid);

    let mut interpret = Interpreter::new(&trace_filepath).unwrap();

    let cwd = std::env::current_dir().unwrap();

    let lib_path = env!("LIB_PATH");

    interpret.exec(opt.cmd, opt.args, cwd, lib_path).unwrap();

    let data = common::parser::Parser::new()
        .parse_file(&trace_filepath)
        .unwrap();

    let output_file = if let Some(file) = opt.out_file {
        PathBuf::from(file)
    } else {
        PathBuf::from("/tmp/flamegraph.svg")
    };

    build_flamegraph(data, &output_file).unwrap();

    println!(
        "Successfully stored memory flamegraph to {}",
        output_file.display()
    );

    if !opt.no_open {
        open::that(output_file).unwrap();
    }

    remove_file(trace_filepath).unwrap();
}
