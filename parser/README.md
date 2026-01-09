# ISO 10383 Parser

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->![License][license-image]

This crate provides the data structures necessasry to parse ISO 10383 XML data with [`quick-xml`](https://docs.rs/quick-xml).

As an end-user, this probably isn't the crate you're looking for, you probably want either the owned/borrowed MIC strings featured in the [`iso10383-types`](https://crates.io/crates/iso10383-types) crate, or the fully enumerated data found in the [`iso10383-static`](https://crates.io/crates/iso10383-static) crate, which uses this crate (via a proc-macro) to generate enumerations and static data.

[//]: # (badges)

[license-image]: https://img.shields.io/github/license/jcape/iso10383?style=flat-square
[crate-image]: https://img.shields.io/crates/v/iso10383-parser.svg?style=flat-square
[crate-link]: https://crates.io/crates/iso10383-parser/0.3.3
[docs-image]: https://img.shields.io/docsrs/iso10383-parser?style=flat-square
[docs-link]: https://docs.rs/crate/iso10383-parser/0.3.3/iso10383_parser
[deps-image]: https://deps.rs/crate/iso10383-parser/0.3.3/status.svg?style=flat-square
[deps-link]: https://deps.rs/crate/iso10383-parser/0.3.3
