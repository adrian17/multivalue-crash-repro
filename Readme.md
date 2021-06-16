How to reproduce:

- clone the repo
- install wasm32-unknown-unknown target for rustc (1.52 or current nightly)
- `cd program`
- `cargo rustc --release --target wasm32-unknown-unknown -- -C target-feature=+multivalue`

Result:
```
error: could not compile `program`

Caused by:
  process didn't exit successfully: `rustc --crate-name program --edition=2018 program/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C target-feature=+multivalue -C metadata=fbd1b1570305f35f -C extra-filename=-fbd1b1570305f35f --out-dir /home/adrian/multivalue-crash-repro/target/wasm32-unknown-unknown/release/deps --target wasm32-unknown-unknown -L dependency=/home/adrian/multivalue-crash-repro/target/wasm32-unknown-unknown/release/deps -L dependency=/home/adrian/multivalue-crash-repro/target/release/deps --extern dec2flt=/home/adrian/multivalue-crash-repro/target/wasm32-unknown-unknown/release/deps/libdec2flt-e579b974813a05c6.rmeta` (signal: 11, SIGSEGV: invalid memory reference)
```