# ISO 10383 Market Identifier Codes Parser

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->![MSRV 1.88.0][msrv-image]

This crate provides the data structures necessasry to parse ISO 10383 XML data with [`quick-xml`](https://docs.rs/quick-xml).

As an end-user, this probably isn't the crate you're looking for, you probably want either the owned/borrowed MIC strings featured in the [`iso10383-types`](https://crates.io/crates/iso10383-types) crate, or the fully enumerated data found in the [`iso10383-static`](https://crates.io/crates/iso10383-static) crate, which uses this crate (via a proc-macro) to generate enumerations and static data.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/iso10383-parser.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso10383-parser/0.4.0
[docs-image]: https://img.shields.io/docsrs/iso10383-parser?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso10383-parser/0.4.0/iso10383_parser
[msrv-image]: https://img.shields.io/crates/msrv/iso10383-parser/0.4.0?style=for-the-badge
