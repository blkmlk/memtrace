# memtrace


## MemTrace


![example](example.svg)

A Rust-based tool for visualizing heap memory consumption using flamegraphs. It helps you profile your app.

The tool is using the [flamegraph](https://github.com/flamegraph-rs/flamegraph) crate for building flamegraphs

> ℹ️ **Info:** So far, the tool works only on MacOS.

> ⚠️ **Warning:** At the moment, this tool requires downloading a dynamic library to function. The library is open source and can be found [here](https://github.com/blkmlk/memtrace-lib).
>
> If you know a better solution - feel free to open a pull request

### Installation

```bash
cargo install memtrace
```

### Usage
> ℹ️ **Info:** Make sure your app is built in debug mode
```bash
memtrace <your_program>
```

License: MIT
