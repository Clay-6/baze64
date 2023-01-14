# Baze64

A Rust library & binary crate for encoding & decoding
base64

## Usage

Run `baze64 encode <STRING>` to encode a string or `baze64 encode -f <FILE>` to
encode a file. Decode a base64 string by running

```shell
baze64 decode <STRING>
```

where `<STRING>` is a base64 encoded string, adding `-o <FILE>` to output to `<FILE>`

## Installation

### Through Cargo

Run the command

```shell
cargo install baze64
```

### Precompiled Binary

Install the binary for your system from the [latest GitHub release](https://github.com/Clay-6/baze64/releases/latest)
and extract the binary to a location on your `PATH`
