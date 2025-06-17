//! ISO 10383 Market Identifier Codes XML Parser
#![doc = include_str!("../README.md")]

use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};

/// The type of MIC
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Kind {
    /// A top-level owner/operator organization
    #[serde(alias = "OPRT")]
    Operating,
    /// A market segment MIC subsidiary of an owner/operator MIC
    #[serde(alias = "SGMT")]
    Segment,
}

/// The market category of a MIC
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Category {
    /// ATSS - Alternative Trading System
    #[serde(alias = "ATSS")]
    AlternativeTradingSystem,

    /// APPA - Approved Publication Arrangement
    #[serde(alias = "APPA")]
    ApprovedPublicationArrangement,

    /// ARMS - Approved Reporting Mechanism
    #[serde(alias = "ARMS")]
    ApprovedReportingMechanism,

    /// CTPS - Consolidated Tape Provider
    #[serde(alias = "CTPS")]
    ConsolidatedTapeProvider,

    /// CASP - Crypto Asset Services Provider
    #[serde(alias = "CASP")]
    CryptoAssetServicesProvider,

    /// DCMS - Designated Contract Market
    #[serde(alias = "DCMS")]
    DesignatedContractMarket,

    /// IDQS - Inter Dealer Quotation System
    #[serde(alias = "IDQS")]
    InterDealerQuotationSystem,

    /// MLTF - Multilateral Trading Facility
    #[serde(alias = "MLTF")]
    MultilateralTradingFacility,

    /// NSPD - Not Specified
    #[serde(alias = "NSPD")]
    NotSpecified,

    /// OTFS - Organised Trading Facility
    #[serde(alias = "OTFS")]
    OrganisedTradingFacility,

    /// OTHR - ,
    #[serde(alias = "OTHR")]
    Other,

    /// RMOS - Recognised Market Operator
    #[serde(alias = "RMOS")]
    RecognisedMarketOperator,

    /// RMKT - Regulated Market
    #[serde(alias = "RMKT")]
    RegulatedMarket,

    /// SEFS - Swap Execution Facility
    #[serde(alias = "SEFS")]
    SwapExecutionFacility,

    /// SINT - Systematic Internaliser
    #[serde(alias = "SINT")]
    SystematicInternaliser,

    /// TRFS - Trade Reporting Facility
    #[serde(alias = "TRFS")]
    TradeReportingFacility,
}

/// The status of a MIC
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Status {
    /// The MIC is active
    #[serde(alias = "ACTIVE")]
    Active,

    /// The MIC has been updated
    #[serde(alias = "UPDATED")]
    Updated,

    /// The MIC has expired
    #[serde(alias = "EXPIRED")]
    Expired,
}

/// A list of MICs which can be parsed from the distributed XML file.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MicList {
    #[serde(alias = "ISO10383_MIC")]
    mics: Vec<Mic>,
    #[serde(skip)]
    by_mics: RefCell<HashMap<String, Mic>>,
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
                .insert(mic.mic.clone(), mic.clone());
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
    pub fn mics(&self) -> &[Mic] {
        &self.mics
    }
}

/// A structure representing a Market Identifier record.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Mic {
    /// The MIC itself
    #[serde(alias = "MIC")]
    pub mic: String,

    /// The "owning/operating" MIC which controls this entry.
    #[serde(alias = "OPERATING_x0020_MIC")]
    pub operating_mic: String,

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
    pub country: String,

    /// The city where this market is located
    #[serde(alias = "CITY")]
    pub city: String,

    /// The website of this market
    #[serde(alias = "WEBSITE")]
    pub website: Option<String>,

    /// The current status of this code
    #[serde(alias = "STATUS")]
    pub status: Status,

    /// The date this code was originally created
    #[serde(alias = "CREATION_x0020_DATE")]
    pub creation_date: String,

    /// The last update date
    #[serde(alias = "LAST_x0020_UPDATE_x0020_DATE")]
    pub last_update_date: String,

    /// The date this MIC was last verified for correctness
    #[serde(alias = "LAST_x0020_VALIDATION_x0020_DATE")]
    pub last_validation_date: Option<String>,

    /// The date when this MIC was marked inactive
    #[serde(alias = "EXPIRY_x0020_DATE")]
    pub expiry_date: Option<String>,

    /// Additional details or comments.
    #[serde(alias = "COMMENTS")]
    pub comments: Option<String>,
}
