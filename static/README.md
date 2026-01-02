# Static ISO 10383 Data

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->![License][license-image]

This crate contains static, `const`-friendly structures and data from the ISO 10383 MIC data distributed by the [ISO 20022 Website](https://www.iso20022.org/market-identifier-codes). More specifically, it contains a [`Code`] enum which enumerates all of the market identifier codes distributed, with accessors to read particular data, and a set of constants containing individual records.

If you're _only_ looking for a simple string-oriented type that will perform basic well-formedness checks on a MIC code string, but not ensure the given code actuall exists, you should use [`iso10383-types`](https://crates.io/crates/iso10383-types).

[//]: # (badges)

[license-image]: https://img.shields.io/github/license/jcape/iso10383?style=for-the-badge
[crate-image]: https://img.shields.io/crates/v/iso10383-static.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso10383-static/0.3.1
[docs-image]: https://img.shields.io/docsrs/iso10383-static/0.3.1?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso10383-static/0.3.1/iso10383_static/
[deps-image]: https://deps.rs/crate/iso10383-static/0.3.1/status.svg?style=for-the-badge
[deps-link]: https://deps.rs/crate/iso10383-static/0.3.1
