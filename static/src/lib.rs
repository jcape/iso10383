//! Static ISO 10383 Data

#![doc = include_str!("../README.md")]
#![no_std]

iso10383_macros::generate!(xml = "ISO10383_MIC.xml");
