# Procedural Macros for ISO 10383 Data

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->![MSRV 1.88.0][msrv-image]

This crate provides code generation based on the [ISO 10383 XML file](https://www.iso20022.org/market-identifier-codes) distributed by the ISO 200022 organization, and parsed by the [`iso10383-parser`](https://crates.io/crates/iso10383-parser) crate.

As an end-user, this probably isn't the crate you're looking for, you probably want either the owned/borrowed MIC strings featured in the [`iso10383-types`](https://crates.io/crates/iso10383-types) crate, or the fully enumerated data found in the [`iso10383-static`](https://crates.io/crates/iso10383-static) crate, which uses this crate to generate its data.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/iso10383-macros.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso10383-macros/0.4.0
[docs-image]: https://img.shields.io/docsrs/iso10383-macros?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso10383-macros/0.4.0/iso10383_macros
[msrv-image]: https://img.shields.io/crates/msrv/iso10383-macros/0.4.0?style=for-the-badge
