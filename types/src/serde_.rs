//! Serde Implementations

use crate::{mic, Error, Mic};
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
use core::fmt::{Formatter, Result as FmtResult};
use serde::{
    de::{Error as DeError, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl Serialize for mic {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for Mic {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

struct MicVisitor;

impl<'v> Visitor<'v> for MicVisitor {
    type Value = &'v mic;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a 4-character string or byte-string value")
    }

    fn visit_borrowed_bytes<E: DeError>(self, v: &'v [u8]) -> Result<Self::Value, E> {
        mic::from_bytes(v).map_err(|err| match err {
            Error::InvalidCharacter(pos) => {
                DeError::invalid_value(Unexpected::Bytes(&[v[pos]]), &"A-Z, 0-9")
            }
            Error::InvalidLength(actual, _expected) => {
                DeError::invalid_length(actual, &"4 ascii digits or upper-case characters")
            }
        })
    }

    fn visit_borrowed_str<E: DeError>(self, v: &'v str) -> Result<Self::Value, E> {
        self.visit_borrowed_bytes(v.as_bytes())
    }
}

impl<'de> Deserialize<'de> for &'de mic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(MicVisitor)
    }
}

struct OwnedMicVisitor;

impl<'v> Visitor<'v> for OwnedMicVisitor {
    type Value = Mic;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a 4-character string or byte-string value")
    }

    fn visit_borrowed_bytes<E: DeError>(self, v: &'v [u8]) -> Result<Self::Value, E> {
        Mic::from_bytes(v).map_err(|err| match err {
            Error::InvalidCharacter(pos) => {
                DeError::invalid_value(Unexpected::Bytes(&[v[pos]]), &"A-Z, 0-9")
            }
            Error::InvalidLength(actual, _expected) => {
                DeError::invalid_length(actual, &"4 ascii digits or upper-case characters")
            }
        })
    }

    fn visit_borrowed_str<E: DeError>(self, v: &'v str) -> Result<Self::Value, E> {
        self.visit_borrowed_bytes(v.as_bytes())
    }

    fn visit_bytes<E: DeError>(self, v: &[u8]) -> Result<Self::Value, E> {
        self.visit_borrowed_bytes(v)
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        self.visit_borrowed_str(v)
    }

    #[cfg(feature = "alloc")]
    fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
        self.visit_borrowed_str(&v)
    }

    #[cfg(feature = "alloc")]
    fn visit_byte_buf<E: DeError>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        self.visit_borrowed_bytes(&v)
    }
}

impl<'de> Deserialize<'de> for Mic {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(OwnedMicVisitor)
    }
}
