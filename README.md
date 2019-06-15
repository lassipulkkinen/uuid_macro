uuid\_macro
===========

[![Crates.io](https://img.shields.io/crates/v/uuid_macro.svg)](https://crates.io/crates/uuid_macro)
[![Docs.rs](https://docs.rs/uuid_macro/badge.svg)](https://docs.rs/uuid_macro)
![License](https://img.shields.io/crates/l/uuid_macro.svg)

Generate UUIDs at compile time.

```rust
use uuid_macro::uuid_v4;

let mut uuids = Vec::new();

for _ in 0..2 {
    uuids.push(uuid_v4!());
}

assert_eq!(uuids[0], uuids[1]);
```

License
-------

MIT.
