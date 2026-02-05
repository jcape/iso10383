# Static ISO 10383 Market Identifier Code Data

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->![MSRV 1.88.0][msrv-image]

This crate contains `no-std` and `no-std::no-alloc` structures and data from the ISO 10383 MIC data distributed by the [ISO 20022 Website](https://www.iso20022.org/market-identifier-codes). More specifically, it contains a [`Code`](crate::Code) enum which enumerates all of the market identifier codes distributed, with accessors to read particular data, and a set of constants containing individual records.

If you're _only_ looking for a simple string-oriented type that will perform basic well-formedness checks on a MIC code string, but not ensure the given code actuall exists, you should use [`iso10383-types`](https://crates.io/crates/iso10383-types).

## Features

- **`default`**: Enables the `serde` feature.
- **`serde`**: Enables the serialization and deserialization traits of the code enum and MIC record data.
- **`alloc`**: Enables the variants of serde serialization and deserialization which require `alloc`. This should be enabled if serde has it's own `alloc` feature enabled.

## Examples

```rust
use iso10383_types::mic;
use iso10383_static::Code;

const MIC: &mic = match mic::from_str("IEXG") {
    Ok(mic) => mic,
    Err(_) => panic!("Static MICs should parse"),
};

let code = Code::from_mic(MIC).expect("valid code");
assert_eq!(Code::Iexg, code);
```

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/iso10383-static.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso10383-static/0.4.1
[docs-image]: https://img.shields.io/docsrs/iso10383-static/0.4.1?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso10383-static/0.4.1/iso10383_static/
[msrv-image]: https://img.shields.io/crates/msrv/iso10383-static/0.4.1?style=for-the-badge
