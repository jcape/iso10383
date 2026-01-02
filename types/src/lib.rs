//! ISO 10383 Types

#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod alloc_;

#[cfg(feature = "serde")]
mod serde_;

use core::{
    borrow::Borrow,
    fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
    ops::Deref,
    str::FromStr,
};
use ref_cast::{RefCastCustom, ref_cast_custom};
use thiserror::Error as ThisError;

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

const MIC_SIZE: usize = 4;

const fn check_mic(bytes: &[u8]) -> Result<(), Error> {
    if bytes.len() != MIC_SIZE {
        return Err(Error::InvalidLength(bytes.len(), MIC_SIZE));
    }

    let mut i = 0;
    while i < MIC_SIZE {
        if !bytes[i].is_ascii_digit() && !bytes[i].is_ascii_uppercase() {
            return Err(Error::InvalidCharacter(i));
        }

        i += 1;
    }

    Ok(())
}

/// An enumeration of errors when validating a MIC
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, ThisError)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Error {
    /// Invalid length.
    #[error("Invalid length.")]
    InvalidLength(usize, usize),
    /// Invalid character at position {0}.
    #[error("Invalid character at position {0}.")]
    InvalidCharacter(usize),
}

/// A MIC reference
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd, RefCastCustom)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct mic([u8]);

impl mic {
    #[ref_cast_custom]
    pub(crate) const fn from_bytes_unchecked(src: &[u8]) -> &Self;

    /// Create a new MIC from the given bytes
    pub const fn from_bytes(src: &[u8]) -> Result<&Self, Error> {
        if let Err(e) = check_mic(src) {
            Err(e)
        } else {
            Ok(Self::from_bytes_unchecked(src))
        }
    }

    /// Create a new MIC from the given string
    pub const fn from_str(src: &str) -> Result<&Self, Error> {
        let bytes = src.as_bytes();

        if let Err(e) = check_mic(bytes) {
            Err(e)
        } else {
            Ok(Self::from_bytes_unchecked(bytes))
        }
    }

    /// Borrow this MIC as a byte slice.
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Borrow this MIC as a string slice.
    #[allow(unsafe_code)]
    pub const fn as_str(&self) -> &str {
        // SAFETY: a mic slice is validated before construction
        unsafe { str::from_utf8_unchecked(&self.0) }
    }

    /// Get an owned copy of this MIC.
    pub const fn to_mic(&self) -> Mic {
        Mic::from_bytes_unchecked(&self.0)
    }
}

impl AsRef<[u8]> for mic {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for mic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for mic {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", str::from_utf8(&self.0).map_err(|_e| FmtError)?)
    }
}

/// An owned MIC value
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, RefCastCustom)]
#[repr(transparent)]
pub struct Mic([u8; MIC_SIZE]);

impl Mic {
    pub(crate) const fn from_byte_array_unchecked(bytes: [u8; MIC_SIZE]) -> Self {
        Self(bytes)
    }

    pub(crate) const fn from_bytes_unchecked(src: &[u8]) -> Self {
        let mut bytes = [0u8; MIC_SIZE];
        let (value, _reject) = src.split_at(MIC_SIZE);
        bytes.copy_from_slice(value);
        Self::from_byte_array_unchecked(bytes)
    }

    /// Create a new MIC by taking ownership of a byte array.
    pub const fn from_byte_array(bytes: [u8; MIC_SIZE]) -> Result<Self, Error> {
        if let Err(e) = check_mic(&bytes) {
            Err(e)
        } else {
            Ok(Self::from_byte_array_unchecked(bytes))
        }
    }

    /// Create a new owned MIC from the given byte slice.
    pub const fn from_bytes(src: &[u8]) -> Result<Self, Error> {
        if let Err(e) = check_mic(src) {
            Err(e)
        } else {
            Ok(Self::from_bytes_unchecked(src))
        }
    }

    /// Create a new owned MIC from the given string slice.
    pub const fn from_str_slice(s: &str) -> Result<Self, Error> {
        Self::from_bytes(s.as_bytes())
    }

    /// Borrow this MIC as a byte slice.
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Borrow this MIC as a string slice.
    #[allow(unsafe_code)]
    pub const fn as_str(&self) -> &str {
        // SAFETY: We validate the internal byte array contains only ASCII digits and uppercase
        // characters on construction.
        unsafe { str::from_utf8_unchecked(&self.0) }
    }

    /// Borrow this MIC as a MIC slice.
    pub const fn as_mic(&self) -> &mic {
        mic::from_bytes_unchecked(&self.0)
    }
}

impl AsRef<[u8]> for Mic {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for Mic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for Mic {
    type Target = mic;

    fn deref(&self) -> &Self::Target {
        mic::from_bytes_unchecked(&self.0)
    }
}

impl Borrow<mic> for Mic {
    fn borrow(&self) -> &mic {
        mic::from_bytes_unchecked(&self.0)
    }
}

impl Display for Mic {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", str::from_utf8(&self.0).map_err(|_e| FmtError)?)
    }
}

impl FromStr for Mic {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_slice(s)
    }
}

/// The type of MIC
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Kind {
    /// A top-level owner/operator organization
    #[cfg_attr(feature = "serde", serde(alias = "OPRT"))]
    Operating,
    /// A market segment MIC subsidiary of an owner/operator MIC
    #[cfg_attr(feature = "serde", serde(alias = "SGMT"))]
    Segment,
}

/// The market category of a MIC
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Category {
    /// ATSS - Alternative Trading System
    #[cfg_attr(feature = "serde", serde(alias = "ATSS"))]
    AlternativeTradingSystem,

    /// APPA - Approved Publication Arrangement
    #[cfg_attr(feature = "serde", serde(alias = "APPA"))]
    ApprovedPublicationArrangement,

    /// ARMS - Approved Reporting Mechanism
    #[cfg_attr(feature = "serde", serde(alias = "ARMS"))]
    ApprovedReportingMechanism,

    /// CTPS - Consolidated Tape Provider
    #[cfg_attr(feature = "serde", serde(alias = "CTPS"))]
    ConsolidatedTapeProvider,

    /// CASP - Crypto Asset Services Provider
    #[cfg_attr(feature = "serde", serde(alias = "CASP"))]
    CryptoAssetServicesProvider,

    /// DCMS - Designated Contract Market
    #[cfg_attr(feature = "serde", serde(alias = "DCMS"))]
    DesignatedContractMarket,

    /// IDQS - Inter Dealer Quotation System
    #[cfg_attr(feature = "serde", serde(alias = "IDQS"))]
    InterDealerQuotationSystem,

    /// MLTF - Multilateral Trading Facility
    #[cfg_attr(feature = "serde", serde(alias = "MLTF"))]
    MultilateralTradingFacility,

    /// NSPD - Not Specified
    #[cfg_attr(feature = "serde", serde(alias = "NSPD"))]
    NotSpecified,

    /// OTFS - Organised Trading Facility
    #[cfg_attr(feature = "serde", serde(alias = "OTFS"))]
    OrganisedTradingFacility,

    /// OTHR - ,
    #[cfg_attr(feature = "serde", serde(alias = "OTHR"))]
    Other,

    /// RMOS - Recognised Market Operator
    #[cfg_attr(feature = "serde", serde(alias = "RMOS"))]
    RecognisedMarketOperator,

    /// RMKT - Regulated Market
    #[cfg_attr(feature = "serde", serde(alias = "RMKT"))]
    RegulatedMarket,

    /// SEFS - Swap Execution Facility
    #[cfg_attr(feature = "serde", serde(alias = "SEFS"))]
    SwapExecutionFacility,

    /// SINT - Systematic Internaliser
    #[cfg_attr(feature = "serde", serde(alias = "SINT"))]
    SystematicInternaliser,

    /// TRFS - Trade Reporting Facility
    #[cfg_attr(feature = "serde", serde(alias = "TRFS"))]
    TradeReportingFacility,
}

/// The status of a MIC
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Status {
    /// The MIC is active
    #[cfg_attr(feature = "serde", serde(alias = "ACTIVE"))]
    Active,

    /// The MIC has been updated
    #[cfg_attr(feature = "serde", serde(alias = "UPDATED"))]
    Updated,

    /// The MIC has expired
    #[cfg_attr(feature = "serde", serde(alias = "EXPIRED"))]
    Expired,
}
