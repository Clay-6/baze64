# Baze64

The baze64 rust library & accompanying CLI

## Usage

### Library

Simply add

```toml
baze64 = "<VERSION>"
```

to your `Cargo.toml` where `<VERSION>` is the latest version of the crate. Alternatively,
run

```shell
cargo add baze64
```

for this to be done for you.

### CLI

Run `baze64 encode <STRING>` to encode a string or `baze64 encode -f <FILE>` to
encode a file. Decode a base64 string by running

```shell
baze64 decode <STRING>
```

where `<STRING>` is a base64 encoded string, adding `-o <FILE>` to output to `<FILE>`
