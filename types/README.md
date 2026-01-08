# ISO 10383 Types

[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->![License][license-image]

This crate provides basic no-std, no-alloc capable types for working with ISO 10383 data, including an owned [`Mic`](crate::Mic) and it's associated [`&mic`](crate::mic) borrow, as well as additional enums for handling [`Kind`], [`Category`], and [`Status`] of a code.

If you're looking for stronger validation and smaller storage requirements, you probably want the [`iso10383-static`](https://crates.io/crates/iso10383-static) crate.

[//]: # (badges)

[license-image]: https://img.shields.io/github/license/jcape/iso10383?style=flat-square
[crate-image]: https://img.shields.io/crates/v/iso10383-types.svg?style=flat-square
[crate-link]: https://crates.io/crates/iso10383-types/0.3.2
[docs-image]: https://img.shields.io/docsrs/iso10383-types/0.3.2?style=flat-square
[docs-link]: https://docs.rs/crate/iso10383-types/0.3.2/iso10383_types/
[deps-image]: https://deps.rs/crate/iso10383-types/0.3.2/status.svg?style=flat-square
[deps-link]: https://deps.rs/crate/iso10383-types/0.3.2
