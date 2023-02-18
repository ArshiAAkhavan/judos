# JUDOS
a judge for OS  

this is a work-pool based,multi-threaded judge framework designed to run grading on the specified inputs in the config file

## abstract model 
basically what you do is you define a pipeline with a list of git repository

## installation
```bash
cargo build --release
```
## run
```bash
cargo run --release
```
or simply run the binary after build
```bash
target/release/judos
```

In order to specify the log level set the `RUST_LOG` environment variable accordingly
```bash
RUST_LOG=info cargo run --release 
```
