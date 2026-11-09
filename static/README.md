# Static ISO 10383 Market Identifier Code Data

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->![MSRV 1.88.0][msrv-image]

This crate contains `no-std` and `no-std::no-alloc` structures and data from the ISO 10383 MIC data distributed by the [ISO 20022 Website](https://www.iso20022.org/market-identifier-codes). More specifically, it contains a [`Code`](crate::Code) enum which enumerates all of the market identifier codes distributed, with accessors to read particular data, and a set of constants containing additional record details.

If you're _only_ looking for a simple string-oriented type that will perform basic well-formedness checks on a MIC code string, but not ensure the given code actually exists, you should use [`iso10383-types`](https://crates.io/crates/iso10383-types).

## Features

- **`default`**: Enables the `serde` and `details` features.
- **`serde`**: Enables the serialization and deserialization traits of the code enum and MIC record data.
- **`alloc`**: Enables the variants of serde serialization and deserialization which require `alloc`. This should be enabled if serde has it's own `alloc` feature enabled.
- **`details`**: Enables `const` structures containing the relevant types, and provides methods on the `Code` enum to retrieve them.

## Examples

### Basic usage

This example shows building a `mic` borrow from a static string, and getting the correpsonding `Code` value from that.

```rust
use iso10383_types::mic;
use iso10383_static::Code;

const MIC: &mic = match mic::from_str("IEXG") {
    Ok(mic) => mic,
    Err(_) => panic!("Static MICs should parse"),
};

const CODE: Code = match Code::from_mic(MIC) {
    Ok(code) => code,
    Err(_) => panic!("IEXG is a current MIC"),
};

let code = Code::from_mic(MIC).expect("valid code");
assert_eq!(Code::Iexg, code);
assert_eq!(CODE, code);
```

### Building a lookup cache

This example shows building a cache of exchange details from static details. An application concerned with displaying details of an exchange (e.g. the corporate name) can use this to build a fast-lookup cache of exchange details.

```rust
use iso10383_static::{ALL_MICS, Code};
use std::collections::HashMap;

let names_by_code = ALL_MICS.iter()
    .map(|record| (record.code, record.name.clone()))
    .collect::<HashMap<_, _>>();

let iexg_name = names_by_code.get(&Code::Iexg)
    .expect("The name should be found");

assert_eq!("INVESTORS EXCHANGE", *iexg_name);
```

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/iso10383-static.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso10383-static/0.7.1
[docs-image]: https://img.shields.io/docsrs/iso10383-static/0.7.1?style=for-the-badge
[docs-link]: https://docs.rs/iso10383-static/0.7.1/iso10383_static/
[msrv-image]: https://img.shields.io/crates/msrv/iso10383-static/0.7.1?style=for-the-badge
