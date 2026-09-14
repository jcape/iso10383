//! Static ISO 10383 Data.

#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]

#[cfg(feature = "serde")]
mod _serde;

iso10383_macros::generate!(xml = "ISO10383_MIC.xml", zerocopy = true);
