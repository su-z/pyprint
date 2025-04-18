# pyprint

Getting tired of writine printing statements with format strings in rust? This is a library to enable python-style printing in rust. It is implemented using rust macros. Anything with the `Display` trait implemented can be printed.

## Usage

Install with `cargo add pyprint`.

Simply write like python in rust.

```rust
use pyprint::pprn;
let a = 5;
pprn!("Progress:", a, sep=" ", end="\r");
```
