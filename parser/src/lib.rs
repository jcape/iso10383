//! ISO 10383 Market Identifier Codes

use std::{
    cell::RefCell,
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};

use serde::{Deserialize, Serialize};

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
    #[serde(alias = "ATSS")]
    AlternativeTradingSystem,

    #[serde(alias = "APPA")]
    ApprovedPublicationArrangement,

    #[serde(alias = "ARMS")]
    ApprovedReportingMechanism,

    #[serde(alias = "CTPS")]
    ConsolidatedTapeProvider,

    #[serde(alias = "CASP")]
    CryptoAssetServicesProvider,

    #[serde(alias = "DCMS")]
    DesignatedContractMarket,

    #[serde(alias = "IDQS")]
    InterDealerQuotationSystem,

    #[serde(alias = "MLTF")]
    MultilateralTradingFacility,

    #[serde(alias = "NSPD")]
    NotSpecified,

    #[serde(alias = "OTFS")]
    OrganisedTradingFacility,

    #[serde(alias = "OTHR")]
    Other,

    #[serde(alias = "RMOS")]
    RecognisedMarketOperator,

    #[serde(alias = "RMKT")]
    RegulatedMarket,

    #[serde(alias = "SEFS")]
    SwapExecutionFacility,

    #[serde(alias = "SINT")]
    SystematicInternaliser,

    #[serde(alias = "TRFS")]
    TradeReportingFacility,
}

/// The status of a MIC
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Status {
    #[serde(alias = "ACTIVE")]
    Active,
    #[serde(alias = "UPDATED")]
    Updated,
    #[serde(alias = "EXPIRED")]
    Expired,
}

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

    /// Retrieve the size of the cache
    pub fn len(&self) -> usize {
        self.update_cache();

        self.mics.len()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Mic {
    /// The MIC itself
    #[serde(alias = "MIC")]
    mic: String,

    /// The "owning/operating" MIC which controls this entry.
    #[serde(alias = "OPERATING_x0020_MIC")]
    operating_mic: String,

    /// What type of MIC this is
    #[serde(alias = "OPRT_x002F_SGMT")]
    kind: Kind,

    /// The human-readable name of this MIC
    #[serde(alias = "MARKET_x0020_NAME-INSTITUTION_x0020_DESCRIPTION")]
    name: String,

    /// The name of the legal entity responsible for this MIC
    #[serde(alias = "LEGAL_x0020_ENTITY_x0020_NAME")]
    legal_entity_name: Option<String>,

    /// The ISO 17442 LEI code for the legal entity.
    #[serde(alias = "LEI")]
    legal_entity_id: Option<String>,

    #[serde(alias = "MARKET_x0020_CATEGORY_x0020_CODE")]
    category: Category,

    /// Known acronym of the market
    #[serde(alias = "ACRONYM")]
    acronym: Option<String>,

    /// ISO 3166-2 alpha-2 code
    #[serde(alias = "ISO_x0020_COUNTRY_x0020_CODE_x0020__x0028_ISO_x0020_3166_x0029_")]
    country: String,

    /// The city where this market is located
    #[serde(alias = "CITY")]
    city: String,

    /// The website of this market
    #[serde(alias = "WEBSITE")]
    website: Option<String>,

    /// The current status of this code
    #[serde(alias = "STATUS")]
    status: Status,

    /// The date this code was originally created
    #[serde(alias = "CREATION_x0020_DATE")]
    creation_date: String,

    /// The last update date
    #[serde(alias = "LAST_x0020_UPDATE_x0020_DATE")]
    last_update_date: String,

    /// The date this MIC was last verified for correctness
    #[serde(alias = "LAST_x0020_VALIDATION_x0020_DATE")]
    last_validation_date: Option<String>,

    /// The date when this MIC was marked inactive
    #[serde(alias = "EXPIRY_x0020_DATE")]
    expiry_date: Option<String>,

    /// Additional details or comments.
    #[serde(alias = "COMMENTS")]
    comments: Option<String>,
}
