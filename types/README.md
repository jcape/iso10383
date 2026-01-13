# ISO 10383 Types

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->![MSRV 1.88.0][msrv-image]

This crate provides basic `no-std`, `no-std::no-alloc` capable types for working with ISO 10383 data, including an owned [`Mic`](crate::Mic) and it's associated [`&mic`](crate::mic) borrow, as well as additional enums for handling [`Kind`], [`Category`], and [`Status`] of a code.

If you're looking for stronger validation and smaller storage requirements, you probably want the [`iso10383-static`](https://crates.io/crates/iso10383-static) crate.

## Features

- **`default`**: Enables the `serde` feature.
- **`serde`**: Enables the serialization and deserialization traits of the code enum and MIC record data.
- **`alloc`**: Enables the variants of serde serialization and deserialization which require `alloc`. This should be enabled if serde has it's own `alloc` feature enabled.
- **`zerocopy`**: Enables the derivation of traits from the [`zerocopy`](https://crates.io/crate/zerocopy/), namely [`IntoBytes`](zerocopy::IntoBytes) and [`KnownLayout`](zerocopy::KnownLayout). Notably, this does not include [`TryFromBytes`](zerocopy::TryFromBytes) as it is not possible to provide a custom validity check at this time.

## Examples

```rust
use iso10383_types::{Mic, mic};

const SRC: &str = "IEXG";

let mcode = mic::from_str(SRC).expect("valid MIC");
assert_eq!(SRC, mcode.as_str());
```

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/iso10383-types.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso10383-types/0.3.3
[docs-image]: https://img.shields.io/docsrs/iso10383-types/0.3.3?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso10383-types/0.3.3/iso10383_types/
[msrv-image]: https://img.shields.io/crates/msrv/iso10383-types/0.3.3?style=for-the-badge
