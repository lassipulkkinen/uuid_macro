uuid\_macro
===========

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
