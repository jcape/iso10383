//! ISO 10383 Market Identifier Codes

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

pub const XML_SOURCE_URL: &str =
    "https://www.iso20022.org/sites/default/files/ISO10383_MIC/ISO10383_MIC.xml";

/// The type of MIC
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Kind {
    /// An operating MIC indicates a top-level organization
    #[serde(rename = "OPRT")]
    Operating,
    /// A segment MIC indicates a listing or trading segment under an operating MIC
    #[serde(rename = "SGMT")]
    Segment,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Category {
    #[serde(rename = "ATSS")]
    AlternativeTradingSystem,

    #[serde(rename = "APPA")]
    ApprovedPublicationArrangement,

    #[serde(rename = "ARMS")]
    ApprovedReportingMechanism,

    #[serde(rename = "CTPS")]
    ConsolidatedTapeProvider,

    #[serde(rename = "CASP")]
    CryptoAssetServicesProvider,

    #[serde(rename = "DCMS")]
    DesignatedContractMarket,

    #[serde(rename = "IDQS")]
    InterDealerQuotationSystem,

    #[serde(rename = "MLTF")]
    MultilateralTradingFacility,

    #[serde(rename = "NSPD")]
    NotSpecified,

    #[serde(rename = "OTFS")]
    OrganisedTradingFacility,

    #[serde(rename = "OTHR")]
    Other,

    #[serde(rename = "RMOS")]
    RecognisedMarketOperator,

    #[serde(rename = "RMKT")]
    RegulatedMarket,

    #[serde(rename = "SEFS")]
    SwapExecutionFacility,

    #[serde(rename = "SINT")]
    SystematicInternaliser,

    #[serde(rename = "TRFS")]
    TradeReportingFacility,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Status {
    Active,
    Updated,
    Expired,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Mic {
    /// The MIC itself
    #[serde(rename = "MIC")]
    mic: String,

    /// The "owning/operating" MIC which controls this entry.
    #[serde(rename = "OPERATING_x0020_MIC")]
    operating_mic: String,

    /// What type of MIC this is
    #[serde(rename = "OPRT_x002F_SGMT")]
    kind: Kind,

    /// The human-readable name of this MIC
    #[serde(rename = "MARKET_x0020_NAME-INSTITUTION_x0020_DESCRIPTION")]
    name: String,

    /// The name of the legal entity responsible for this MIC
    #[serde(rename = "LEGAL_x0020_ENTITY_x0020_NAME")]
    legal_entity_name: String,

    /// The ISO 17442 LEI code for the legal entity.
    #[serde(rename = "LEI")]
    legal_entity_id: String,

    #[serde(rename = "MARKET_x0020_CATEGORY_x0020_CODE")]
    category: Category,

    /// Known acronym of the market
    #[serde(rename = "ACRONYM")]
    acronym: String,

    /// ISO 3166-2 alpha-2 code
    #[serde(rename = "ISO_x0020_COUNTRY_x0020_CODE_x0020__x0028_ISO_x0020_3166_x0029_")]
    country: [char; 2],

    /// The city where this market is located
    #[serde(rename = "CITY")]
    city: String,

    /// The website of this market
    #[serde(rename = "WEBSITE")]
    website: String,

    /// The current status of this code
    #[serde(rename = "STATUS")]
    status: Status,

    /// The date this code was originally created
    creation_date: String,

    /// The last update date
    last_update_date: String,

    /// The date this MIC was last verified for correctness
    last_validation_date: String,

    /// The date when this MIC was marked inactive
    expiry_date: Option<String>,
    comments: String,
}

pub struct MicLookup {
    data: HashMap<String, Mic>,
}
