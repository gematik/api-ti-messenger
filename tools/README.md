# Usage

From this folder, run

```
cargo run ../../images/generated/TI-M_Pro/UC_10059_Seq.png
```

and follow the prompts.

To run from another folder use the `--manifest-path` flag, e.g.

```
cargo run --manifest-path tools/upload-polarion-attachment/Cargo.toml images/generated/TI-M_Pro/UC_10059_Seq.png
```

Only works while on the VPN and when `POLARION_ACCESS_TOKEN` is available in the environment.

# Development

To format and lint locally, run

```
cargo fmt
cargo clippy --all-targets --all-features
```
