# memtrace


## MemTrace


![example](example.svg)

A Rust-based tool for visualizing heap memory consumption using flamegraphs. It helps you profile your app

> ⚠️ **Warning:** at the moment the tool requires to download a dynamic library to work. The library is opensource, you can find it [here](https://github.com/blkmlk/memtrace-lib)
>
> If you know a better solution - welcome to pull requests

### Installation

```bash
cargo install memtrace
```

### Usage
```bash
memtrace <your_program>
```

License: MIT
