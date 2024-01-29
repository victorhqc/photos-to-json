# photos-to-json
Reads a directory with images and generates a JSON file with their basic information

## How to build installer for windows

Make sure to Have wix installed

```shell
choco install wixtoolset
```

And follow the instructions for [cargo-wix](https://volks73.github.io/cargo-wix/cargo_wix/index.html)

```shell
cargo build --release
cargo wix
```