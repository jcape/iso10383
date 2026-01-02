//! A collection of parsed records

use chrono::NaiveDate;
use heck::ToTitleCase;
use iso10383_parser::MicRecord;
use iso17442_types::lei;
use proc_macro2::{Span, TokenStream};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use syn::{Error, Ident, LitByteStr, Result};

/// A struct-of-arrays collection of parsed record data suitable for use with the
/// [`quote!`](quote::quote!) macro.
#[derive(Default)]
pub(crate) struct RecordSet {
    doc: Vec<String>,
    const_ident: Vec<Ident>,
    code_bytes: Vec<LitByteStr>,
    code_ident: Vec<Ident>,
    oper_bytes: Vec<LitByteStr>,
    oper_ident: Vec<Ident>,
    kind: Vec<Ident>,
    name: Vec<String>,
    legal_name: Vec<TokenStream>,
    legal_id: Vec<TokenStream>,
    category: Vec<Ident>,
    acronym: Vec<TokenStream>,
    alpha2: Vec<Ident>,
    city: Vec<String>,
    website: Vec<TokenStream>,
    status: Vec<Ident>,
    creation: Vec<TokenStream>,
    last_update: Vec<TokenStream>,
    last_validation: Vec<TokenStream>,
    expiry: Vec<TokenStream>,
    comments: Vec<TokenStream>,
}

impl RecordSet {
    /// Create a new record set with the given capacity.
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            doc: Vec::with_capacity(capacity),
            const_ident: Vec::with_capacity(capacity),
            code_bytes: Vec::with_capacity(capacity),
            code_ident: Vec::with_capacity(capacity),
            oper_bytes: Vec::with_capacity(capacity),
            oper_ident: Vec::with_capacity(capacity),
            kind: Vec::with_capacity(capacity),
            name: Vec::with_capacity(capacity),
            legal_name: Vec::with_capacity(capacity),
            legal_id: Vec::with_capacity(capacity),
            category: Vec::with_capacity(capacity),
            acronym: Vec::with_capacity(capacity),
            alpha2: Vec::with_capacity(capacity),
            city: Vec::with_capacity(capacity),
            website: Vec::with_capacity(capacity),
            status: Vec::with_capacity(capacity),
            creation: Vec::with_capacity(capacity),
            last_update: Vec::with_capacity(capacity),
            last_validation: Vec::with_capacity(capacity),
            expiry: Vec::with_capacity(capacity),
            comments: Vec::with_capacity(capacity),
        }
    }

    /// Append the data of a new record to the given recordset
    pub(crate) fn push_record(&mut self, record: &MicRecord, span: Span) -> Result<()> {
        let doc = format!("{} - {}", record.mic, record.name);
        let const_ident = quote::format_ident!("{}", cleanup_mic(record.mic.as_str()));
        let code_bytes = LitByteStr::new(record.mic.as_bytes(), span);
        let code_ident =
            quote::format_ident!("{}", cleanup_mic(&record.mic.as_str().to_title_case()));
        let oper_bytes = LitByteStr::new(record.operating_mic.as_bytes(), span);
        let oper_ident = quote::format_ident!(
            "{}",
            cleanup_mic(&record.operating_mic.as_str().to_title_case())
        );
        let kind = Ident::new(&format!("{:?}", record.kind), span);
        let legal_name = optional_str(record.legal_entity_name.as_deref());
        let legal_id = optional_lei(record.legal_entity_id.as_deref(), span)?;
        let category = Ident::new(&format!("{:?}", record.category), span);
        let acronym = optional_str(record.acronym.as_deref());
        let alpha2 = Ident::new(&format!("{:?}", record.country), span);
        let website = optional_str(record.website.as_deref());
        let status = Ident::new(&format!("{:?}", record.status), span);
        let creation = required_date(&record.creation_date, span)?;
        let last_update = required_date(&record.last_update_date, span)?;
        let last_validation = optional_date(record.last_validation_date.as_deref(), span)?;
        let expiry = optional_date(record.expiry_date.as_deref(), span)?;
        let comments = optional_str(record.comments.as_deref());

        self.doc.push(doc);
        self.const_ident.push(const_ident);
        self.code_bytes.push(code_bytes);
        self.code_ident.push(code_ident);
        self.oper_bytes.push(oper_bytes);
        self.oper_ident.push(oper_ident);
        self.kind.push(kind);
        self.name.push(record.name.clone());
        self.legal_name.push(legal_name);
        self.legal_id.push(legal_id);
        self.category.push(category);
        self.acronym.push(acronym);
        self.alpha2.push(alpha2);
        self.city.push(record.city.clone());
        self.website.push(website);
        self.status.push(status);
        self.creation.push(creation);
        self.last_update.push(last_update);
        self.last_validation.push(last_validation);
        self.expiry.push(expiry);
        self.comments.push(comments);

        Ok(())
    }

    /// Documentation strings
    pub(crate) fn doc(&self) -> &[String] {
        &self.doc
    }

    /// The MIC as an all-caps identifier
    pub(crate) fn const_ident(&self) -> &[Ident] {
        &self.const_ident
    }

    /// The MIC as bytes
    pub(crate) fn code_bytes(&self) -> &[LitByteStr] {
        &self.code_bytes
    }

    /// The MIC as a title-case identifier
    pub(crate) fn code_ident(&self) -> &[Ident] {
        &self.code_ident
    }

    /// The Operating MIC as bytes
    pub(crate) fn oper_bytes(&self) -> &[LitByteStr] {
        &self.oper_bytes
    }

    /// The operating MIC as a title-case identifier
    pub(crate) fn oper_ident(&self) -> &[Ident] {
        &self.oper_ident
    }

    /// The kind invariant, as an identifier
    pub(crate) fn kind(&self) -> &[Ident] {
        &self.kind
    }

    /// The name of the entity, as a string
    pub(crate) fn name(&self) -> &[String] {
        &self.name
    }

    /// The name of the entry's legal entity, as tokens for an optional string slice
    pub(crate) fn legal_name(&self) -> &[TokenStream] {
        &self.legal_name
    }

    /// The LEI of the entry's legal entity, as tokens for an optional static
    /// [`&lei`](::iso17442_types::lei) reference.
    pub(crate) fn legal_id(&self) -> &[TokenStream] {
        &self.legal_id
    }

    /// The market category variant of the entry, as an identifier
    pub(crate) fn category(&self) -> &[Ident] {
        &self.category
    }

    /// The acronym of the variant, as tokens for an optional string slice
    pub(crate) fn acronym(&self) -> &[TokenStream] {
        &self.acronym
    }

    /// The [`Alpha2`](iso3166_static::Alpha2) enum variant, as an identifier
    pub(crate) fn alpha2(&self) -> &[Ident] {
        &self.alpha2
    }

    /// The city, as a string
    pub(crate) fn city(&self) -> &[String] {
        &self.city
    }

    /// The website, as tokens for an optional string slice.
    pub(crate) fn website(&self) -> &[TokenStream] {
        &self.website
    }

    /// The
    pub(crate) fn status(&self) -> &[Ident] {
        &self.status
    }

    pub(crate) fn creation(&self) -> &[TokenStream] {
        &self.creation
    }

    pub(crate) fn last_update(&self) -> &[TokenStream] {
        &self.last_update
    }

    pub(crate) fn last_validation(&self) -> &[TokenStream] {
        &self.last_validation
    }

    pub(crate) fn expiry(&self) -> &[TokenStream] {
        &self.expiry
    }

    pub(crate) fn comments(&self) -> &[TokenStream] {
        &self.comments
    }
}

impl Debug for RecordSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("RecordSet")
            .field("doc", &self.doc)
            .field("const_ident", &self.const_ident)
            .field("code_bytes", &"...")
            .field("code_ident", &self.code_ident)
            .field("oper_bytes", &"...")
            .field("oper_ident", &self.oper_ident)
            .field("kind", &self.kind)
            .field("name", &self.name)
            .field("legal_name", &self.legal_name)
            .field("legal_id", &self.legal_id)
            .field("category", &self.category)
            .field("acronym", &self.acronym)
            .field("alpha2", &self.alpha2)
            .field("city", &self.city)
            .field("website", &self.website)
            .field("status", &self.status)
            .field("creation", &self.creation)
            .field("last_update", &self.last_update)
            .field("last_validation", &self.last_validation)
            .field("expiry", &self.expiry)
            .field("comments", &self.comments)
            .finish()
    }
}

/// Prefix a MIC with an underscore if the first character is an ASCII digit.
fn cleanup_mic(value: &str) -> String {
    let mut retval = value.to_owned();
    if retval
        .chars()
        .next()
        .is_some_and(|ref ch| ch.is_ascii_digit())
    {
        retval.insert(0, '_');
    }

    retval
}

/// Create the token stream necessary to render an optional string.
fn optional_str(src: Option<&str>) -> TokenStream {
    if let Some(value) = src {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            quote::quote! { None }
        } else {
            quote::quote! { Some(#trimmed) }
        }
    } else {
        quote::quote! { None }
    }
}

/// Create a token stream necessary to render an optional [`&lei`](iso17442_types::lei) from the
/// optional [`&str`](core::primitive::str).
fn optional_lei(legal_entity_id: Option<&str>, span: Span) -> Result<TokenStream> {
    if let Some(le_id) = legal_entity_id
        && !le_id.trim().is_empty()
    {
        lei::from_str_slice(le_id)
            .map(lei::as_bytes)
            .map(|value| LitByteStr::new(value, span))
            .map(|lit| {
                // This is all const-compatible, and will be resolved at compile time.
                quote::quote! { match ::iso17442_types::lei::from_bytes(#lit) {
                    Ok(value) => Some(value),
                    Err(error) => panic!("Invalid LEI bytes encoded in constant."),
                } }
            })
            .map_err(|error| {
                let message = format!(
                    "Unable to parse legal entity ID '{}': {error}",
                    legal_entity_id.expect("Already verified LEI was Some()")
                );

                Error::new(span, message)
            })
    } else {
        Ok(quote::quote! { None })
    }
}

fn required_date(date: &str, span: Span) -> Result<TokenStream> {
    let dateval = date.parse::<u32>().map_err(|error| {
        let message = format!("Unable to parse date: {error}");
        Error::new(span, message)
    })?;

    // 20260102 /    10_000 = 2026
    // 20260102 %    10_000 = 102
    // 20260102 % 1_000_000 = 2
    let year = (dateval / 10_000).cast_signed();
    let month = (dateval % 10_000) / 100;
    let day = dateval % 100;

    let _date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
        let message = format!("Date `{date}` ({year}, {month}, {day} out of range, likely invalid");
        Error::new(span, message)
    })?;

    Ok(quote::quote! {
        match ::chrono::NaiveDate::from_ymd_opt(#year, #month, #day) {
            Some(value) => value,
            None => panic!("Invalid date in constant data"),
        }
    })
}

fn optional_date(date: Option<&str>, span: Span) -> Result<TokenStream> {
    if let Some(date) = date
        && !date.trim().is_empty()
    {
        let value = required_date(date, span)?;
        Ok(quote::quote! { Some(#value) })
    } else {
        Ok(quote::quote! { None })
    }
}
