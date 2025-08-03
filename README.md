# CSV Converter
This is a small Rust program that converts the units of columns in a CSV file. So if, for instance, you download some CSV data where one column is in inches but you really want it in feet, this tool is here to help! Just be aware that as the conversion factors are hardcoded, while I think they should be correct, you may want to double-check your output, at least for now.

## Installation and Development
Once you've cloned the repo, it's just a standard cargo project, so go ahead and use standard commands like `cargo run`, `cargo build`, and `cargo test` (only used for an end-to-end test as conversion functions are self-explanatory).

## How to Use
Once you've built it with `cargo build --release` and added the directory with the binary to your `PATH` environment variable, the `csvc` command will be exposed to your terminal. The arguments are as follows:
1. The file you want to convert
2. The name of the column you want to convert (this will correspond to whatever is in the top row, so make sure you've got headers!)
3. The unit the column is currently in
4. The unit you want to convert it to
So usage is:
```sh
csvc [INPUTFILE] [HEADERNAME] [CURRENTUNIT] [NEWUNIT]
```

## License
CSV Converter is licensed under the [MIT License](LICENSE).
