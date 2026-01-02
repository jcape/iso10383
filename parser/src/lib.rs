//! ISO 10383 Market Identifier Codes XML Parser

#![doc = include_str!("../README.md")]

use iso3166_static::Alpha2;
use iso10383_types::{Category, Kind, Mic, Status};
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};

/// A list of MICs which can be parsed from the distributed XML file.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MicList {
    #[serde(alias = "ISO10383_MIC")]
    mics: Vec<MicRecord>,
    #[serde(skip)]
    by_mics: RefCell<HashMap<String, MicRecord>>,
    #[serde(skip)]
    by_mics_loaded: AtomicBool,
}

impl MicList {
    fn update_cache(&self) {
        if self.by_mics_loaded.load(Ordering::Acquire) {
            return;
        }

        for mic in &self.mics {
            self.by_mics
                .borrow_mut()
                .insert(mic.name.clone(), mic.clone());
        }
        self.by_mics_loaded.store(true, Ordering::Release);
    }

    /// Get the size of the cache
    pub fn len(&self) -> usize {
        self.update_cache();

        self.mics.len()
    }

    /// Get whether or not the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.update_cache();

        self.mics.is_empty()
    }

    /// Retrieve a slice of the parsed MICs.
    pub fn mics(&self) -> &[MicRecord] {
        &self.mics
    }
}

/// A structure representing a Market Identifier record.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MicRecord {
    /// The MIC itself
    #[serde(alias = "MIC")]
    pub mic: Mic,

    /// The "owning/operating" MIC which controls this entry.
    #[serde(alias = "OPERATING_x0020_MIC")]
    pub operating_mic: Mic,

    /// What type of MIC this is
    #[serde(alias = "OPRT_x002F_SGMT")]
    pub kind: Kind,

    /// The human-readable name of this MIC
    #[serde(alias = "MARKET_x0020_NAME-INSTITUTION_x0020_DESCRIPTION")]
    pub name: String,

    /// The name of the legal entity responsible for this MIC
    #[serde(alias = "LEGAL_x0020_ENTITY_x0020_NAME")]
    pub legal_entity_name: Option<String>,

    /// The ISO 17442 LEI code for the legal entity.
    ///
    /// This is a string because quick-xml treats `<LEI></LEI>` as `Some("")` and tries to parse
    /// the `""`. A simple `deserialize_with` that returns an `Option<Lei>` also did not work...
    #[serde(alias = "LEI")]
    pub legal_entity_id: Option<String>,

    /// The market category this MIC is operating in
    #[serde(alias = "MARKET_x0020_CATEGORY_x0020_CODE")]
    pub category: Category,

    /// Known acronym of the market
    #[serde(alias = "ACRONYM")]
    pub acronym: Option<String>,

    /// ISO 3166-2 alpha-2 code
    #[serde(alias = "ISO_x0020_COUNTRY_x0020_CODE_x0020__x0028_ISO_x0020_3166_x0029_")]
    pub country: Alpha2,

    /// The city where this market is located
    #[serde(alias = "CITY")]
    pub city: String,

    /// The website of this market
    #[serde(alias = "WEBSITE")]
    pub website: Option<String>,

    /// The current status of this code
    #[serde(alias = "STATUS")]
    pub status: Status,

    /// The date this code was originally created in the ASCII decimal format YYYYMMDD.
    #[serde(alias = "CREATION_x0020_DATE")]
    pub creation_date: String,

    /// The last update date in the ASCII decimal format YYYYMMDD.
    #[serde(alias = "LAST_x0020_UPDATE_x0020_DATE")]
    pub last_update_date: String,

    /// The date this MIC was last verified for correctness in the ASCII decimal format YYYYMMDD.
    #[serde(alias = "LAST_x0020_VALIDATION_x0020_DATE")]
    pub last_validation_date: Option<String>,

    /// The date when this MIC was marked inactive in the ASCII decimal format YYYYMMDD.
    #[serde(alias = "EXPIRY_x0020_DATE")]
    pub expiry_date: Option<String>,

    /// Additional details or comments.
    #[serde(alias = "COMMENTS")]
    pub comments: Option<String>,
}
