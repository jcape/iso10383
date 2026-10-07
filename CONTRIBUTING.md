# Contributing to ISO 17442 Crates

## Generative AI Policy

_This is a copy-paste of the [Debian policy on LLM usage](https://www.debian.org/vote/2026/vote_002#texte), with the name changed, and applies to version 0.6 of this crate. Prior versions do not contain any AI-assisted source code or data._

This project neither endorses nor prohibits the use of generative AI tools in the development, maintenance, or documentation of software, packaging, documentation, and other media published within the this project Project. We recognize that such tools can substantially improve the productivity of contributors when used responsibly, allowing volunteers to spend more of their limited time on work that requires technical expertise, judgment, review, and collaboration.

This project nevertheless expects that all contributions submitted to this project, regardless of how and with which tools they were produced, satisfy the same standards of quality, correctness, maintainability, and legal compliance. The use of a generative AI tool does not diminish the contributor's responsibility for the work they submit. Contributors are expected to understand, review, test, and, where appropriate, modify AI-assisted output before incorporating it into this project. Blindly accepting or uploading AI-generated material without appropriate human review is inconsistent with this project's established development practices. We enourage our contributors to disclose whether a contribution was made with AI assitance, but do not require them to do so.

This project acknowledges that the legal status of material produced by generative AI systems remains the subject of ongoing discussion in many jurisdictions, including questions relating to copyright, authorship, licensing, and potential reproduction of training material. The project does not seek to resolve these unsettled legal questions through this General Resolution, nor does it adopt a position on whether AI-generated output is, in whole or in part, copyrightable or derived from copyrighted works.

Instead, this project continues to rely on the judgment and responsibility of its individual contributors. Project members are expected to exercise appropriate care when using generative AI tools, to consider the provenance and licensing implications of material they contribute, and to avoid introducing content whose legal status they cannot reasonably justify. Existing this project policies governing licensing, copyright, software freedom, and the acceptance of contributions continue to apply irrespective of the tools used to produce those contributions.

Contributors are expected to exercise appropriate care when designing and implementing workflows that incorporate generative AI tools. In particular, they should ensure that confidential information, private communications, security-sensitive information (such as embargoed information about security bugs that is not yet public), cryptographic keys, credentials, and other non-public material relating to the this project Project, its infrastructure, or its community are not disclosed to third-party AI services unless such disclosure has been explicitly authorized and is consistent with this project's security and privacy requirements.

The use of generative AI does not alter this project's established expectations regarding large-scale or automated project actions. Contributors intending to perform actions with broad project impact, such as mass bug filing or patch submission, large-scale code modifications, or other automated changes or requests affecting many packages or contributors, should seek prior discussion and consensus through the appropriate project channels before proceeding. Any such automated process should be overseen by a human who remains accountable for its behavior and output.

This resolution therefore affirms that generative AI is neither exempt from nor subject to special rules beyond the standards already expected of this project contributors. The responsibility for every contribution rests with the contributor who submits it, who remains accountable for its technical quality, legal acceptability, and suitability for inclusion in this project.

## Coding Style

Part of submitting a PR to this repository is ensuring that the formatting is correct.

### Automated Checks

The easiest part of ensuring the style guide is followed is running the following utilities, which are checked for every PR:

- `rustfmt`: Reformats the code. If the repo is "dirty" after this has been run, the PR cannot be merged.
- `cargo clippy`: An in-depth checking utility that will look for code which the authors (The Rust Foundation) think are not idiomatic rust. In practice this is a lot like PEP-8.

### Rust's Style Guide

The Rust Foundation has a [WIP style guide](https://doc.rust-lang.org/1.0.0/style/style/README.html), and we should follow it's recommendations unless there's a good reason not to:

- [Avoid `use *`, except in tests](https://doc.rust-lang.org/1.0.0/style/style/imports.html#avoid-use-*,-except-in-tests.)
- [Prefer fully importing types/traits while module-qualifying functions](https://doc.rust-lang.org/1.0.0/style/style/imports.html#prefer-fully-importing-types/traits-while-module-qualifying-functions.)
- [Always separately bind RAII (lock) guards](https://doc.rust-lang.org/1.0.0/style/features/let.html#always-separately-bind-raii-guards.-[fixme:-needs-rfc]) -- Note that you should use brace scopes.

### Our Style Guide

In addition (and sometimes overruling) the Rust Style Guide, we have our own rules:

#### Sort your inputs

The Rust Style Guide asks developers to [sort their inputs](https://doc.rust-lang.org/1.0.0/style/style/imports.html), but in this situation the sorting is considered sub-optimal. We would prefer to sort our inputs in a similar, but distinct manner:

- `extern crate` directives (typically this is just `extern crate alloc;` in no-std crates)
- `pub use` (re-)exports
- `pub mod` exports
- `mod` definitions
- `use` imports

For example:

```rust
extern crate alloc;

pub use crate::{
    module::TypeToExport,
};
pub use dependency::TypeWereUsing;

mod module;

use dependency::SomeTypeWeUseOurselves;
```

### Re-export types when necessary

Before re-exporting types, consider is whether there is an advantage to be gained by creating a newtype wrapper, e.g.:

```rust
/// This is meant to restrict the types of math which can performed on a block
/// height (to eliminate off-by-one errors).
pub struct BlockHeight(NotZero64);

/// A way to implement serde on ExternalNonSerdeThing for others
pub struct SerdeThing(ExternalNonSerdeThing);

impl Serialize for SerdeThing { /* ... */ }
impl DeserializeOwned for SerdeThing { /* ... */ }
```

If there _is_ such an advantage, then it's usually better to wrap the types. If there isn't an advantage, then you should re-export any types you're using (or expecting your users to use).

Don't:

```rust
use other_crate::ExternalThing;

pub fn my_function(thing: ExternalThing) -> bool {
    /* ... */
}
```

Do:

```rust
pub use other_crate::ExternalThing;

pub fn my_function(thing: ExternalThing) -> bool {
    /* ... */
}
```

#### Export types at the crate level

The Rust Style Guide contains the admonition to [Reexport the most important types at the crate level](https://doc.rust-lang.org/1.0.0/style/style/organization.html#reexport-the-most-important-types-at-the-crate-level.).

Don't:

```rust
pub use crate::module::SomeType;

mod module {
    pub struct SomeType;

    // Users will have to access this using `thecrate::module::Error`?
    pub enum Error {
        Error1,
        Error2,
    }

    enum Strategy {
        Strat1(SomeType),
        Strat2,
    }
}
```

Do:

```rust
pub use crate::module::{SomeType, Error as SomeError};

mod module {
    pub struct SomeType;
    pub enum Error {
        Error1,
        Error2,
    }

    enum Strategy {
        Strat1(SomeType),
        Strat2,
    }
}
```

#### Use public modules to group functions

This is the flip side of the Rust style guide's admonition to [prefer fully importing types/traits while module-qualifying functions](https://doc.rust-lang.org/1.0.0/style/style/imports.html#prefer-fully-importing-types/traits-while-module-qualifying-functions.) and our own admonition to [export types at the crate level](#export-types-at-the-crate-level).

Types should only be exported at the top level, but if you have lots of related bare functions, consider grouping them into modules exposed via a `pub mod`.

Don't:

```rust
pub struct SomeType;

pub fn foonary_frobnicate() -> bool {
    /* ... */
}

pub fn foonary_widgify() -> SomeType {
    /* ... */
}
```

Don't:

```rust
mod foonary {
    pub struct SomeType;

    pub fn frobnicate() -> SomeType {
        /* ... */
    }

    pub fn widgify() -> SomeType {
        /* ... */
    }
}

pub use crate::foonary::{
    SomeType,
    frobnicate as foonary_frobnicate,
    widgify as foonary_widgify,
};
```

Do:

```rust
pub use crate::foonary::SomeType;

pub mod foonary {
    pub(crate) struct SomeType;

    pub fn frobnicate() -> SomeType {
        /* ... */
    }

    pub fn widgify() -> SomeType {
        /* ... */
    }
}
```

#### Avoid Manual Drops

Earlier there is a reference to the Rust style guide item "[Always separately bind RAII (lock) guards](https://doc.rust-lang.org/1.0.0/style/features/let.html#always-separately-bind-raii-guards.-[fixme:-needs-rfc])." This rule should be modified slightly to indicate the use of scoping for the critical section, and expanded to be a general admonition against the use of `core::drop()` in favor of `{}`-braced scopes.

Don't:

```rust
fn use_mutex(m: sync::mutex::Mutex<int>) {
    let guard = m.lock();
    do_work(guard);
    drop(guard); // unlock the lock
    // do other work
}
```

Do:

```rust
fn use_mutex(m: sync::mutex::Mutex<int>) {
    {
        let guard = m.lock();
        do_work(guard);
    } // unlocking will happen automatically when the lock falls out of scope
    // do other work
}
```
