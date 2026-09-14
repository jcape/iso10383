//! Serde support for generated types.

#[cfg(feature = "alloc")]
extern crate alloc;

use crate::Code;
use core::fmt::{Formatter, Result as FmtResult};
use iso10383_types::mic;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Visitor},
};

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

impl Serialize for Code {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_mic().as_str())
    }
}

impl<'de> Deserialize<'de> for Code {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CodeVisitor)
    }
}

/// A visitor used to parse the code enum.
struct CodeVisitor;

impl<'de> Visitor<'de> for CodeVisitor {
    type Value = Code;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("A 4-ascii-character MIC code")
    }

    #[cfg(feature = "alloc")]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        let value = mic::from_bytes(&v)
            .map_err(|_error| DeError::custom("Could not parse &mic from borrowed bytes"))?;
        Code::from_mic(value).map_err(|_error| DeError::custom("Deserialized value is not valid."))
    }

    #[cfg(feature = "alloc")]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        let value = mic::from_str(&v)
            .map_err(|_error| DeError::custom("Could not parse &mic from borrowed bytes"))?;
        Code::from_mic(value).map_err(|_error| DeError::custom("Deserialized value is not valid."))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        let value = mic::from_str(v)
            .map_err(|_error| DeError::custom("Could not parse &mic from borrowed bytes"))?;
        Code::from_mic(value).map_err(|_error| DeError::custom("Deserialized value is not valid."))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        let value = mic::from_bytes(v)
            .map_err(|_error| DeError::custom("Could not parse &mic from borrowed bytes"))?;
        Code::from_mic(value).map_err(|_error| DeError::custom("Deserialized value is not valid."))
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        let value = mic::from_bytes(v)
            .map_err(|_error| DeError::custom("Could not parse &mic from borrowed bytes"))?;
        Code::from_mic(value).map_err(|_error| DeError::custom("Deserialized value is not valid."))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        let value = mic::from_str(v)
            .map_err(|_error| DeError::custom("Could not parse &mic from borrowed str"))?;

        Code::from_mic(value).map_err(|_error| DeError::custom("Deserialized value is not valid."))
    }
}

#[cfg(test)]
mod test {
    use crate::Code;

    #[test]
    fn serde_roundtrip() {
        let start = Code::Iexg;
        let serde = serde_json::to_string(&start).expect("ser");
        assert_eq!("\"IEXG\"", serde);

        let end = serde_json::from_str::<Code>(&serde).expect("de");
        assert_eq!(start, end);
    }
}
