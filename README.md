Alternative to [`dbg!()`](https://doc.rust-lang.org/std/macro.dbg.html) but prints using [`log::debug!()`](https://github.com/rust-lang/log) so that it works on WASM and other non-std targets.

```rust
// WASM example
use debug::debug;
wasm_logger::init(wasm_logger::Config::default());
debug!();
```